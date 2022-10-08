use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    length: u32
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let num_elements = cli.length;

    let mut rng = thread_rng();
    let mut nums: Vec<u8> = vec![];

    for _i in 0..num_elements {
        nums.push(rng.gen());
    }

    let nums = nums.chunks(12);
    let mut lines = vec![];


    for num_line in nums {
        lines.push(
            String::from("    ")
                + &num_line
                    .into_iter()
                    .map(|x| format!("0x{:02x}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
        );
    }

    println!("uint8_t foo[{}] = {{\n{}\n}};", num_elements, lines.join(",\n"));
    Ok(())
}
