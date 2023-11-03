use rand::Rng;
use regex::Regex;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::cmp::{max, min};

fn main() {
    let dice_re_1 = Regex::new(r"(?i)(\d*)[d](\d+)\s*([\+-]?)\s*(\d*)\s*-*([dac]*)").unwrap();
    let dice_re_d20 = Regex::new(r"(?i)([+-]?)\s*(\d*)\s*-*([dac]?)").unwrap();
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(" 🎲 ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let (nd, ns, pm, w, ad): (u8, u8, &str, u8, &str);
                if line == "\n" || line == "\r\n" {
                    (nd, ns, pm, w, ad) = (1u8, 20u8, "", 0u8, "");
                } else if let Some(val) = dice_re_1.captures(&line) {
                    let (_, [nds, nss, pms, ws, ads]) = val.extract();

                    (nd, ns, pm, w, ad) = (
                        nds.parse::<u8>().unwrap_or(1u8),
                        nss.parse::<u8>().unwrap_or(20u8),
                        pms,
                        ws.parse::<u8>().unwrap_or(0u8),
                        ads,
                    );
                } else if let Some(val) = dice_re_d20.captures(&line) {
                    let (_, [pms, ws, ads]) = val.extract();

                    (nd, ns, pm, w, ad) = (1, 20, pms, ws.parse::<u8>().unwrap_or(0u8), ads);
                } else {
                    println!();
                    continue;
                }

                // Advantage and disadvantage handling
                match ad.to_lowercase().as_str() {
                    "a" => {
                        print!("Roll 1: ");
                        let roll_1 = roll_dice(nd, ns, pm, w);
                        println!();
                        print!("Roll 2: ");
                        let roll_2 = roll_dice(nd, ns, pm, w);
                        println!();
                        let result = max(roll_1, roll_2);
                        println!("Result: {result}");
                        println!();
                    }
                    "d" => {
                        print!("Roll 1: ");
                        let roll_1 = roll_dice(nd, ns, pm, w);
                        println!();
                        print!("Roll 2: ");
                        let roll_2 = roll_dice(nd, ns, pm, w);
                        println!();
                        let result = min(roll_1, roll_2);
                        println!("Result: {result}");
                        println!();
                    }
                    "c" => {
                        print!("Roll: ");
                        let result = roll_dice(nd * 2, ns, pm, w * 2);
                        println!();
                        println!("Result: {result}");
                        println!();
                    }
                    _ => {
                        print!("Roll: ");
                        let result = roll_dice(nd, ns, pm, w);
                        println!();
                        println!("Result: {result}");
                        println!();
                    }
                }
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

fn roll_dice(nd: u8, ns: u8, pm: &str, w: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    for _ in 0..nd {
        let roll = rng.gen_range(1..=ns);
        result += roll;
        print!("{roll} ");
    }
    match pm {
        "+" => result += w,
        "-" => result -= w,
        _ => (),
    }
    result
}
