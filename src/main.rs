use rand::Rng;
use regex::Regex;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::cmp::{max, min};
use std::str;

#[derive(PartialEq)]
enum Variant {
    Crit,
    Advantage,
    Disadvantage,
    None,
}

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

struct Dice {
    num_dice: u8,
    die_value: u8,
    variant: Variant,
}

impl Dice {
    fn from_string(a: &str) -> Option<Dice> {
        let dice_re = Regex::new(r"(\d*)d(\d+)([cad]?)").unwrap();
        if let Some(cap) = dice_re.captures(a) {
            let (_, [nd, dv, v]) = cap.extract();
            let nd = nd.parse::<u8>().unwrap_or_else(|_| 1u8);
            let dv = dv.parse::<u8>().unwrap();

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
        print!("Roll 1: ");
        for _ in 0..self.num_dice {
            let roll = (rng.gen::<i32>() % i32::from(self.die_value)) + 1;
            print!("{roll }");
            roll1 += roll;
        }
        println!("");
        if self.variant == Variant::None {
            return Token::Constant(roll1);
        }
        let mut roll2 = 0;
        println!("Roll 2: ");

        for _ in 0..self.num_dice {
            let roll = (rng.gen::<i32>() % i32::from(self.die_value)) + 1;
            print!("{roll }");
            roll2 += roll;
        }
        println!("");
        match self.variant {
            Variant::Crit => Token::Constant(roll1 + roll2),
            Variant::Advantage => Token::Constant(max(roll1, roll2)),
            Variant::Disadvantage => Token::Constant(min(roll1, roll2)),
            _ => panic!(),
        }
    }
}

fn tokenize(a: String) -> Result<Vec<Token>, &'static str> {
    let a = a.trim();
    let a = a.replace(" ", "");
    let mut ret: Vec<Token> = Vec::new();
    let mut splitter: Vec<String> = Vec::new();
    splitter.push(a.to_string());
    while let Some(i) = splitter[splitter.len() - 1]
        .find(|c| c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')')
    {
        let mut s1 = splitter.pop().unwrap();
        let mut s2 = s1.split_off(i);
        splitter.push(s1);
        if s2.bytes().len() > 2 {
            return Err("tokenize error: trailing operator");
        }
        let s3 = s2.split_off(1);
        splitter.push(s2);
        splitter.push(s3);
    }
    for sub in splitter {
        let new_token = match_sub(sub);
        match new_token {
            Some(t) => ret.push(t),
            None => return Err("tokenize error: incorrect format"),
        };
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

fn roll_expr(a: Vec<Token>) -> Vec<Token> {
    // Roll all dice
    for mut token in &a {
        match token {
            Token::Dice(d) => {
                token = &d.roll();
            }
            _ => {}
        };
    }
    a
}

fn evaluate(mut a: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    // Remove external parens
    if matches!(a[0], Token::LeftParen) && matches!(a[a.len() - 1], Token::RightParen) {
        let mut counter = 1;
        for i in 1..a.len() {
            match a[i] {
                Token::LeftParen => counter += 1,
                Token::RightParen => counter -= 1,
                _ => {}
            };
        }
        if counter != 0 {
            return Err("parens parsing error");
        }
        a.pop();
        a.remove(0);
    }
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
        rec = evaluate(rec).unwrap();
        for item in rec.into_iter().rev() {
            a.insert(lindex, item);
        }
    }
    // TODO
    // If multiplication
    // If division
    // If addition
    // If subtraction
    todo!()
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(" 🎲 ");

        match readline {
            Ok(line) => {
                let token_string = tokenize(line);
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
