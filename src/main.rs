use clap::App;
use rand::prelude::*;

fn main() -> Result<(), String> {
    let matches = App::new("Random C uint8_t array generator")
        .version(clap::crate_version!())
        .args_from_usage("<LENGTH>   'Length of array to generate'")
        .get_matches();

    let num_elements = clap::value_t!(matches.value_of("LENGTH"), u32)
        .map_err(|_| "Expected an integer for LENGTH")?;

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
