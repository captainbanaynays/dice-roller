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

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(" 🎲 ");
        //TODO: Implement string parsing into a tokenized string -- include special case for
        //(Add/Subtract Constant) to be a d20 and that. Additionally an empty input should be a
        //d20.
        //TODO: ensure order of operations is correctly followed, including parentheses
        //TODO: then finish collapsing the input into a final value

        match readline {
            Ok(line) => {}
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
