use std::env;
use std::process;

mod dfs;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Value not provided");
        process::exit(0x0100)
    }

    let value =  args[1].parse::<i32>().unwrap();
    let node_1 = dfs::Node { value: 2, neighbors: vec![] };
    let node_2 = dfs::Node { value: 1, neighbors: vec![node_1] };
    let graph = dfs::Graph { entry: node_2 };
    let result = dfs::dfs(&graph, value);

    match result {
       Some(n)  => println!("Found a node with value {}", n.value),
       None     => println!("No node found with value {}", value),
    }
}
