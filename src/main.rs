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
            let nd = nd.parse::<u8>().unwrap();
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

    fn roll(a: Dice) -> u32 {
        let mut rng = rand::thread_rng();
        let mut roll1 = 0u32;
        print!("Roll 1: ");
        for _ in 0..a.num_dice {
            let roll: u32 = (rng.gen::<u32>() % u32::from(a.die_value)) + 1;
            print!("{roll }");
            roll1 += roll;
        }
        println!("");
        if a.variant == Variant::None {
            return roll1;
        }
        let mut roll2 = 0u32;
        println!("Roll 2: ");

        for _ in 0..a.num_dice {
            let roll: u32 = (rng.gen::<u32>() % u32::from(a.die_value)) + 1;
            print!("{roll }");
            roll2 += roll;
        }
        println!("");
        match a.variant {
            Variant::Crit => roll1 + roll2,
            Variant::Advantage => max(roll1, roll2),
            Variant::Disadvantage => min(roll1, roll2),
            _ => panic!(),
        }
    }
}

enum Token {
    Dice(Dice),
    Constant(u32),
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn tokenize(a: String) -> Result<Vec<Token>, &'static str> {
    let a = a.trim();
    let a = a.replace(" ", "");
    let ret: Vec<Token> = Vec::new();
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
        ret.push(match_sub(sub));
    }
    unimplemented!();
}

fn match_sub(a: String) -> Option(Token) {
    match a.as_str() {
        "+" => Some(Token::Add),
        "-" => Some(Token::Subtract),
        "*" => Some(Token::Multiply),
        "/" => Some(Token::Divide),
        "(" => Some(Token::LeftParen),
        ")" => Some(Token::RightParen),
        _ => {
            if let Some(d) = Dice::from_string(a) {
                Some(Token::Dice(d))
            } else if let Some(n: u32) = a.parse() {
                //TODO: Finish parsing!
            }
        }
    }
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
