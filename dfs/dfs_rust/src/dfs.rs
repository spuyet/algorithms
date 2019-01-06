pub struct Node {
    pub value: i32,
    pub neighbors: Vec<Node>,
}

pub struct Graph {
    pub entry: Node
}

fn _dfs(node: &Node, value: i32) -> Option<&Node> {
    for neighbor in node.neighbors.iter() {
        if neighbor.value == value {
            return Some(neighbor);
        } else {
            return _dfs(neighbor, value);
        }
    }
    None
}

pub fn dfs(graph: &Graph, value: i32) -> Option<&Node> {
    let entrypoint = &graph.entry;

    if entrypoint.value == value {
        return Some(entrypoint)
    }
    _dfs(entrypoint, value)
}
