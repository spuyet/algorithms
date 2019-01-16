use std::process;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::collections::HashMap;

mod dfs;

fn graph_generator(filename: &String) -> std::io::Result<HashMap<String, dfs::Node>> {
    let f = File::open(&filename)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let mut graph = HashMap::with_capacity(line.trim().parse::<usize>().unwrap());

    for line in reader.lines() {
        let mut vec = line?.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let neighbor = vec.pop().unwrap();
        let value = vec.pop().unwrap();
        let v = value.clone();

        let node = graph.entry(value).or_insert(dfs::Node { value: v, neighbors: vec![] });
        node.neighbors.push(neighbor);
    }
    Ok(graph)
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("A file has to be given as argument.");
        process::exit(0x0100);
    } else {
        let filename = &args[1];
        let graph = graph_generator(filename).unwrap();
        dfs::dfs(&graph, "2");
    }
}
