use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub value: String,
    pub neighbors: Vec<String>,
}

fn _dfs(graph: &HashMap<String, Node>, visited: &mut HashMap<String, bool>, stack: &mut Vec<String>) {
    let label = &stack[stack.len() - 1];
    visited.insert(label.to_string(), true);

    let node = &graph.get(label).unwrap();
    print!("{}", node.value);

    for neighbor in node.neighbors.iter() {
        let already_visited = visited.get(neighbor).unwrap_or(&false);
        if already_visited == &false  && graph.contains_key(neighbor) {
            print!(" ");
            stack.push(neighbor.to_string());
            _dfs(graph, visited, stack);
        }
    }
    stack.pop();
}

pub fn dfs(graph: &HashMap<String, Node>, entrypoint: &str,) {
    let mut visited = HashMap::with_capacity(graph.len() as usize);
    let mut stack : Vec<String> = vec![];

    stack.push(entrypoint.to_string());
    _dfs(&graph, &mut visited, &mut stack);
    print!("\n");
}
