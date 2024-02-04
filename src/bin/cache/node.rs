#[derive(Debug, Clone)]
pub struct Vidvec {
    id: String,
    vector_size: u32,
    vector: Vec<i64>,
    payload_str: String,   
}

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    vid_vector: Vidvec,
    radius: f64 
}

impl Node {
    pub fn new(left: Option<Box<Node>> , right: Option<Box<Node>> , vid_vector: Vidvec, radius: f64) -> Self {
        // Initialize the vector with zeros and a specified length
        return Node { left: left, right: right, vid_vector: vid_vector , radius: radius}
    }

    pub fn get_vector(&self) -> &Vidvec {
        &self.vid_vector
    }

    pub fn get_right(&self) -> &Option<Box<Node>> {
        &self.right
    }

    pub fn get_left(&self) -> &Option<Box<Node>> {
        &self.left
    }
}

impl Vidvec {
    pub fn new(id: String, vector_size: u32, vector: Vec<i64>, payload_str: String) -> Result<Self, String> {
        // Initialize the vector with zeros and a specified length
        if vector.len() != vector_size.try_into().unwrap() {
            // Throw this error here: The vector fed in was not of the provided vector size
            return Err("The vector size does not match the specified size.".to_string());
        }

        // Create and return a new Node instance
        Ok(Vidvec { id: id, vector_size: vector_size, vector: vector, payload_str: payload_str })
    }

    // Getter methods for each field
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_vector(&self) -> &Vec<i64> {
        &self.vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_simple() {
        let node_id = "node_id";
        let vect: Result<Vidvec, String> = Vidvec::new(node_id.to_string(), 5, vec![0,1,2,3,4], "payload".to_string());
        match vect {
            Ok(vect_instance) => {
                // Accessing and printing the id field
                assert!(vect_instance.get_id().as_str() == node_id.to_string())
            }
            Err(error) => {
                // Handling the error
                println!("Error creating node: {}", error);
            }
        }
    }

    #[test]
    fn test_size_mismatch() {
        let size = 5;
        let node_id: &str = "node_id";
        let vect: Result<Vidvec, String> = Vidvec::new(node_id.to_string(), size, vec![0,1,2,3], "payload".to_string());
        assert!(vect.is_err());
    }
}
