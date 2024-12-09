use colored::Colorize;
use core::panic;
use rand::Rng;
use regex::Regex;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::cmp::{max, min};
use std::fmt::Display;
use std::str;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Variant {
    Crit,
    Advantage,
    Disadvantage,
    None,
}

#[derive(Debug, Clone)]
enum Token {
    Dice(Dice),
    Constant(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::None => write!(f, "{}", ""),
            Variant::Crit => write!(f, "{}", "c"),
            Variant::Advantage => write!(f, "{}", "a"),
            Variant::Disadvantage => write!(f, "{}", "d"),
        }
    }
}

impl Token {
    fn constant(self) -> i32 {
        if let Token::Constant(c) = self {
            c
        } else {
            panic!("Passed value not a constant.")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Dice {
    num_dice: i32,
    die_value: i32,
    variant: Variant,
}

impl Dice {
    fn from_string(a: &str) -> Option<Dice> {
        let dice_re = Regex::new(r"(\d*)d(\d+)([cad]?)").unwrap();
        if let Some(cap) = dice_re.captures(a) {
            let (_, [nd, dv, v]) = cap.extract();
            let nd = nd.parse::<i32>().unwrap_or_else(|_| 1);
            let dv = dv.parse::<i32>().unwrap();

            let v = match v {
                "a" => Variant::Advantage,
                "c" => Variant::Crit,
                "d" => Variant::Disadvantage,
                "" => Variant::None,
                _ => Variant::None,
            };

            return Some(Dice {
                num_dice: nd,
                die_value: dv,
                variant: v,
            });
        }
        None
    }

    fn roll(&self) -> Token {
        let mut rng = rand::thread_rng();
        let mut roll1 = 0;
        let dstring = format!("{}d{}{}", self.num_dice, self.die_value, self.variant);
        println!("{}----------------", dstring.bold());
        print!("Roll: ");
        for _ in 0..self.num_dice {
            let roll = rng.gen_range(1..=self.die_value);
            print!("{roll} ");
            roll1 += roll;
        }
        println!();
        if self.variant == Variant::None {
            return Token::Constant(roll1);
        }
        let mut roll2 = 0;
        print!("Roll 2: ");

        for _ in 0..self.num_dice {
            let roll = rng.gen_range(1..=self.die_value);
            print!("{roll} ");
            roll2 += roll;
        }
        println!();
        match self.variant {
            Variant::Crit => Token::Constant(roll1 + roll2),
            Variant::Advantage => Token::Constant(max(roll1, roll2)),
            Variant::Disadvantage => Token::Constant(min(roll1, roll2)),
            _ => panic!("Passed variant is None, expecting a non-None value"),
        }
    }
}

fn tokenize(mut a: String) -> Result<Vec<Token>, &'static str> {
    // Special case for empty
    if a.is_empty() {
        a = "d20".to_string();
    }
    let a = a.trim();
    let a = a.replace(" ", "");
    let mut ret: Vec<Token> = Vec::new();
    let mut splitter: Vec<String> = Vec::new();
    splitter.push(a.to_string());
    while let Some(i) = splitter[splitter.len() - 1]
        .find(|c| c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')')
    {
        let mut s1 = splitter.pop().unwrap();
        if i == 0 {
            let s2 = s1.split_off(1);
            splitter.push(s1);
            splitter.push(s2);
        // Do not subtract 1 here because the first string is already removed
        // from splitter
        } else if i == splitter.len() {
            let s2 = s1.split_off(i);
            splitter.push(s1);
            splitter.push(s2);
        } else {
            let mut s2 = s1.split_off(i);
            let s3 = s2.split_off(1);
            splitter.push(s1);
            splitter.push(s2);
            splitter.push(s3);
        }
    }
    let last = splitter.pop().unwrap();
    if !last.is_empty() {
        splitter.push(last);
    }
    for sub in splitter {
        let new_token = match_sub(sub);
        match new_token {
            Some(t) => ret.push(t),
            None => return Err("tokenize error: incorrect format"),
        };
    }
    if ret.len() == 2 {
        match ret[0] {
            Token::Add | Token::Subtract | Token::Multiply | Token::Divide => match ret[1] {
                Token::Constant(_) => {
                    ret.insert(0, Token::Dice(Dice::from_string("1d20").unwrap()))
                }
                _ => {}
            },
            _ => {}
        }
    }
    return Ok(ret);
}

fn match_sub(a: String) -> Option<Token> {
    match a.as_str() {
        "+" => Some(Token::Add),
        "-" => Some(Token::Subtract),
        "*" => Some(Token::Multiply),
        "/" => Some(Token::Divide),
        "(" => Some(Token::LeftParen),
        ")" => Some(Token::RightParen),
        _ => {
            if let Some(d) = Dice::from_string(a.as_str()) {
                Some(Token::Dice(d))
            } else if let Ok(n) = a.parse::<i32>() {
                Some(Token::Constant(n))
            } else {
                None
            }
        }
    }
}

fn roll_expr(mut a: Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    loop {
        if i >= a.len() {
            break;
        }
        match a[i] {
            Token::Dice(d) => {
                a.remove(i);
                a.insert(i, d.roll());
            }
            _ => {}
        }
        i += 1;
    }
    a
}

fn evaluate(mut a: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    // Check to see if there is an internal parentheses, recursively call
    'a: loop {
        let mut lindex = usize::max_value();
        'b: for (i, token) in (0..a.len()).zip(a.iter()) {
            if matches!(token, Token::LeftParen) {
                lindex = i;
                break 'b;
            }
        }
        if lindex == usize::max_value() {
            break 'a;
        }

        let mut counter = 1;
        let mut i = 1;
        while counter != 0 {
            match a[lindex + i] {
                Token::LeftParen => counter += 1,
                Token::RightParen => counter -= 1,
                _ => {}
            }
            i += 1;
        }
        let rindex = lindex + i - 1;
        let mut rec: Vec<Token> = Vec::new();
        for _ in lindex..=rindex {
            rec.push(a.remove(lindex));
        }
        rec.pop();
        rec.remove(0);
        rec = evaluate(rec).unwrap();
        for item in rec.into_iter().rev() {
            a.insert(lindex, item);
        }
    }
    // If multiplication
    // If division
    let mut i = 0usize;
    let mut modified: bool = false;
    'a: loop {
        if i >= a.len() {
            if modified {
                modified = false;
                i = 0;
            } else {
                break 'a;
            }
        }
        let token = &a[i];
        match token {
            Token::Multiply => {
                let ro = a.remove(i + 1).constant();
                a.remove(i);
                let lo = a.remove(i - 1).constant();
                a.insert(i - 1, Token::Constant(lo * ro));
                modified = true;
            }
            Token::Divide => {
                let ro = a.remove(i + 1).constant();
                a.remove(i);
                let lo = a.remove(i - 1).constant();
                a.insert(i - 1, Token::Constant(lo / ro));
                modified = true;
            }
            _ => {}
        }
        i += 1;
    }
    // If addition
    // If subtraction
    let mut i = 0usize;
    let mut modified: bool = false;
    'a: loop {
        if a.len() == 1 {
            break 'a;
        }
        if i >= a.len() {
            if modified {
                modified = false;
            } else {
                return Err("Expression did not simplify");
            }
            i = 0;
        }
        let token = &a[i];
        match token {
            Token::Add => {
                let ro = a.remove(i + 1).constant();
                a.remove(i);
                let lo = a.remove(i - 1).constant();
                a.insert(i - 1, Token::Constant(lo + ro));
                modified = true;
            }
            Token::Subtract => {
                let ro = a.remove(i + 1).constant();
                a.remove(i);
                let lo = a.remove(i - 1).constant();
                a.insert(i - 1, Token::Constant(lo - ro));
                modified = true;
            }
            _ => {}
        }
        i += 1;
    }
    if a.len() != 1 {}
    Ok(a)
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(" 🎲 ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let tokenized = match tokenize(line) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{e}");
                        println!("");
                        continue;
                    }
                };
                let rolled = roll_expr(tokenized);
                let mut result = match evaluate(rolled) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("{e}");
                        println!("");
                        continue;
                    }
                };
                let roll = result.remove(0).constant();
                println!("--------------------");
                println!("Result: {roll}");
                println!("");
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupt signal received.");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Roll again with me sometime!");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}
