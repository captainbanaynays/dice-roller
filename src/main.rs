use std::io::{self, Write};
use rand::Rng;
use regex::Regex;

fn main() {
    let dice_re_1 = Regex::new(r"(\d*)d(\d+)\s*([\+-]?)\s*(\d*)").unwrap();
    let dice_re_d20 = Regex::new(r"([+-]?)\s*(\d+)").unwrap();
    let mut input_buffer = String::new();
    let mut rng = rand::thread_rng();
    

    loop {
        print!("r ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input_buffer).unwrap() == 0 {
            break;
        }

        let mut result = 0;

        let (nd, ns, pm, w): (u8, u8, &str, u8);
        if let Some(val) = dice_re_1.captures(&input_buffer) {
            let (_, [nds, nss, pms, ws]) = val.extract();

            (nd, ns, pm, w) = (nds.parse::<u8>().unwrap(),
                nss.parse::<u8>().unwrap(),
                pms,
                ws.parse::<u8>().unwrap());
        } else if let Some(val) = dice_re_d20.captures(&input_buffer) {
            let (_, [pms, ws]) = val.extract();

            (nd, ns, pm, w) = (1, 20, pms, ws.parse::<u8>().unwrap());
        } else {
            println!("");
            continue;
        }

        print!("Rolls: ");
        for _ in 0..nd {
            let roll = rng.gen_range(1..=ns);
            result += roll;
            print!("{roll} ");
        }
        print!("\n");
        io::stdout().flush().unwrap();

        match pm {
            "+" => result += w,
            "-" => result -= w,
            _ => (),
        }

        input_buffer.clear();
        println!("Result: {result}");
        println!("");
    }
}
