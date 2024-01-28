use crate::bin::cache::node::Node;

mod bin;

fn main() {
    bin::cache::cache_hello_world();

    let node: Result<Node, String> = Node::new("node_id".to_string(), 123, 5, vec![0,1,2,3,4], "payload".to_string());

    match node {
        Ok(node_instance) => {
            // Accessing and printing the id field
            println!("ID: {}", node_instance.get_id());
        }
        Err(error) => {
            // Handling the error
            println!("Error creating node: {}", error);
        }
    }
}
