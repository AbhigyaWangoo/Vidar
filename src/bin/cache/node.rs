
pub struct Node {
    id: String,
    creation_time: u32,
    vector_size: u32,
    vector: Vec<u32>,
    payload_str: String,
}

impl Node {
    // Constructor method to create a new Node with a specified vector size
    pub fn new(id: String, creation_time: u32, vector_size: u32, vector: Vec<u32>, payload_str: String) -> Result<Self, String> {
        // Initialize the vector with zeros and a specified length
        if vector.len() != vector_size.try_into().unwrap() {
            // Throw this error here: The vector fed in was not of the provided vector size
            return Err("The vector size does not match the specified size.".to_string());
        }

        // Create and return a new Node instance
        Ok(Node {
            id,
            creation_time,
            vector_size,
            vector,
            payload_str,
        })
    }

    // Getter methods for each field
    pub fn get_id(&self) -> &String {
        &self.id
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_simple() {
        let node_id = "node_id";
        let node: Result<Node, String> = Node::new(node_id.to_string(), 123, 5, vec![0,1,2,3,4], "payload".to_string());
        match node {
            Ok(node_instance) => {
                // Accessing and printing the id field
                assert!(node_instance.get_id().as_str() == node_id.to_string())
            }
            Err(error) => {
                // Handling the error
                println!("Error creating node: {}", error);
            }
        }
    }
}
