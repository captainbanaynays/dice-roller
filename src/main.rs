use std::io::{self, Write};
use rand::Rng;
use regex::Regex;

fn main() {
    let dice_re = Regex::new(r"(\d*)d(\d+)\s*([\+-]?)\s*(\d*)").unwrap();
    let mut input_buffer = String::new();
    let mut rng = rand::thread_rng();
    

    loop {
        print!("r ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input_buffer).unwrap() == 0 {
            break;
        }

        let mut result = 0;

        let (nd, ns, pm, w): (_, _, _, _);
        if let Some(val) = dice_re.captures(&input_buffer) {
            (_, [nd, ns, pm, w]) = val.extract();
        } else {
            println!("");
            continue;
        }

        let (nd, ns, w) = (nd.parse::<u32>().unwrap_or(1),
                           ns.parse::<u32>().unwrap(),
                           w.parse::<u32>().unwrap_or(0));

        for _ in 0..nd {
            result += rng.gen_range(1..=ns);
        }

        match pm {
            "+" => result += w,
            "-" => result -= w,
            _ => (),
        }

        input_buffer.clear();
        println!("{result}");
    }
}
