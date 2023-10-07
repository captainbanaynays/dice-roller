use std::io::{self, Write};
use rand::Rng;
use regex::Regex;

fn main() {
    let dice_re_1 = Regex::new(r"(\d*)[dD](\d+)\s*([\+-]?)\s*(\d*)\s*-*([daDA]*
        )").unwrap();
    let dice_re_d20 = Regex::new(r"([+-]?)\s*(\d*)\s*-*([daDA]?)").unwrap();
    let mut input_buffer = String::new();

    loop {
        print!("Roll: ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input_buffer).unwrap() == 0 {
            break;
        }

        let mut result = 0;

        let (nd, ns, pm, w, ad): (u8, u8, &str, u8, &str);
        if input_buffer == "\n" || input_buffer == "\r\n" {
            (nd, ns, pm, w, ad) = (1u8, 20u8, "", 0u8, "");
        } else if let Some(val) = dice_re_1.captures(&input_buffer) {
            let (_, [nds, nss, pms, ws, ads]) = val.extract();

            (nd, ns, pm, w, ad) = (nds.parse::<u8>().unwrap_or(1u8),
                nss.parse::<u8>().unwrap_or(20u8),
                pms,
                ws.parse::<u8>().unwrap_or(0u8),
                ads);
        } else if let Some(val) = dice_re_d20.captures(&input_buffer) {
            let (_, [pms, ws, ads]) = val.extract();

            (nd, ns, pm, w, ad) = (1, 20, pms, ws.parse::<u8>().unwrap_or(0u8),
            ads);

        } else {
            println!();
            continue;
        }

        // Advantage and disadvantage handling
        if ad.to_lowercase() == "a" {
            print!("Roll 1: ");
        }
        print!("Rolls: ");
        for _ in 0..nd {
            let roll = rng.gen_range(1..=ns);
            result += roll;
            print!("{roll} ");
        }
        println!();

        match pm {
            "+" => result += w,
            "-" => result -= w,
            _ => (),
        }

        input_buffer.clear();
        println!("Result: {result}");
        println!();
    }
}

fn roll_dice(nd:u8, ns:u8) -> u8 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    for _ in 0..nd {
        let roll = rng.gen_range(1..=ns);
        result += roll;
        print!("{roll} ");
    }
    result
}
