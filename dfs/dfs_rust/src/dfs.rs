use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub value: String,
    pub neighbors: Vec<String>,
}

fn _dfs(graph: &HashMap<String, Node>, visited: &mut HashMap<String, bool>, stack: &mut Vec<String>) {
    let label = &stack[stack.len() - 1];
    visited.insert(label.to_string(), true);

    if let Some(node) = &graph.get(label) {
        print!("{}", node.value);

        for neighbor in node.neighbors.iter() {
            if let None = visited.get(neighbor) {
                print!(" ");
                stack.push(neighbor.to_string());
                _dfs(graph, visited, stack);
            }
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
