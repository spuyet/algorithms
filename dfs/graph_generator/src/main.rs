use std::env;
use rand::Rng;

fn generate_nodes(size: u32) {
    let mut rng = rand::thread_rng();

    println!("{}", size);
    for node in 0..size {
        let neighbors_count = rng.gen_range(0, size);

        for _ in 0..neighbors_count {
            let neighbor = rng.gen_range(0, size);
            println!("{} {}", node, neighbor);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let size = args[1].parse::<u32>().unwrap();
        generate_nodes(size);
    } else {
        println!("Number of generated node has to be specified as parameter.")
    }
}
