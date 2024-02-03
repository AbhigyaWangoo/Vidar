use super::node::Node;
use super::node::Vidvec;

// Converts a list of points into a matrix
fn points_to_matrix(points: &Vec<Vidvec>) -> Vec<Vec<i64>> {
    let mut matrix = Vec::new();
    
    for i in points {
        matrix.push(*i.get_vector());
    }

    return matrix
}

// Finds the dimension in the provided nodes that has the maximum std
fn get_max_dimension_spread(points: &Vec<Vidvec>) -> u32 {
    return 0;
}

// Find median idx from set of points
fn points_median(points: &Vec<Vidvec>, dimension: i64) -> usize {
    // Check if the dimension is within bounds
    let length = points[0].get_vector().len() as i64;
    assert!(dimension < length, "Invalid dimension");

    // Sort the points based on the specified dimension
    let mut sorted_points = points.clone();
    sorted_points.sort_by_key(|p| p.get_vector()[dimension as usize]);

    // Calculate the median index
    let median_idx = sorted_points.len() / 2;

    median_idx
}

// Constructs and returns a balltree's root node
fn construct(points: &Vec<Vidvec>) -> Box<Node> {
    if points.len() == 0 {
        // create a leaf B containing the single point in D
        let leaf = Node {
            left: None,
            right: None,
            vid_vector: points[0]
        };
        // return B
        return Box::new(leaf); 
    } else {
        // let c be the dimension of greatest spread
        highest_spread_dimension = get_max_dimension_spread(points);
        
        // let p be the central point selected considering c
        let median_pt = points_median(points, highest_spread_dimension);

        // let L, R be the sets of points lying to the left and right of the median along dimension c
        
        // create B with two children: 
        //     B.pivot := p
        //     B.child1 := construct_balltree(L),
        //     B.child2 := construct_balltree(R),
        //     let B.radius be maximum distance from p among children

        // return B
    }
}

// fn find(vector_id: String, root: &Node) -> Option<&Node> {
//     if root.get_vector().get_id() == &vector_id {
//         return Some(root)
//     } else {
        
//     }

//     return None
// }