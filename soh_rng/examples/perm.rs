use soh_rng::prelude::*;

fn main() {
    let matches = clap::Command::new("perm")
        .about("Generate a permutation table")
        .arg(
            clap::Arg::new("size")
                .long("size")
                .value_name("NUMBER")
                .help("Size of the permutation table table")
                .default_value("256"),
        )
        .arg(
            clap::Arg::new("seed")
                .long("seed")
                .value_name("NUMBER")
                .help("Random generator seed"),
        )
        .get_matches();

    let table_size = matches
        .get_one::<String>("size")
        .unwrap()
        .parse::<u32>()
        .expect("Size value should be a number!")
        .clamp(2, 1024);

    let mut rng = if let Some(seed_arg) = matches.get_one::<String>("seed") {
        let seed = seed_arg
            .parse::<u64>()
            .expect("Seed value should be a number!");

        RNG64::new(seed)
    } else {
        RNG64::new_from_time()
    };

    let mut table = (0..table_size).collect::<Vec<u32>>();

    rng.shuffle(&mut table);

    for i in table.iter() {
        print!("{} ", i);
    }

    print!("\n");
}
