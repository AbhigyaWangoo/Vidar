use super::node::Node;
use super::node::Vidvec;
use super::super::math;

// Finds the dimension in the provided nodes that has the maximum std
fn get_max_dimension_spread(points: &Vec<Vidvec>) -> u32 {
    let mut vec = Vec::new();
    let mut max_varience = 0.0;
    let mut max_var_idx = -1;

    for (index, point) in points.iter().enumerate() {
        for index in point.get_vector() {
            vec.push(index)
        }
        
        // Find std from vec
        let variance = math::std(point.get_vector());
        if variance > max_varience {
            max_varience = variance;
            max_var_idx = index as isize;
        }
    }

    return max_var_idx as u32;
}

// Find median idx from set of points. Returns all elements to the left and right of the median point.
fn points_median(points: &Vec<Vidvec>, dimension: i64) -> (usize, Vec<Vidvec>, Vec<Vidvec>) {
    // Check if the dimension is within bounds
    let length = points[0].get_vector().len() as i64;
    assert!(dimension < length, "Invalid dimension");

    // Sort the points based on the specified dimension
    let mut sorted_points = points.clone();
    sorted_points.sort_by_key(|p| p.get_vector()[dimension as usize]);

    // Calculate the median index
    let median_idx = (sorted_points.len() / 2);

    // Get the Leftmost values
    let lhs = sorted_points[0..median_idx].to_vec();
    let rhs = sorted_points[median_idx..sorted_points.len()].to_vec();

    // (median_idx, L.to_vec(), R.to_vec())
    return (median_idx, lhs, rhs);
}

// Constructs and returns a balltree's root node
fn construct(points: &Vec<Vidvec>) -> Box<Node> {
    if points.len() == 0 {
        // create a leaf B containing the single point in D
        let leaf = Node::new(None, None, points[0].clone(), 0.0);

        return Box::new(leaf); 
    } else {
        let vidvec_vec = &points.to_vec();

        // let c be the dimension of greatest spread
        let highest_spread_dimension = get_max_dimension_spread(vidvec_vec);

        // let p be the central point selected considering c
        // let L, R be the sets of points lying to the left and right of the median along dimension c
        let (mid, lhs, rhs) = points_median(vidvec_vec, highest_spread_dimension as i64);

        let mut max_dist = 0.0;
        let midpoint = points[mid].clone();
        for point in &lhs {
            let euc_dist = math::euclidean_dist(&point, &midpoint);
            if euc_dist > max_dist {
                max_dist = euc_dist;
            }
        }
        for point in &rhs {
            let euc_dist = math::euclidean_dist(&point, &midpoint);
            if euc_dist > max_dist {
                max_dist = euc_dist;
            }
        }
        
        // create B with two children:
        let b = Node::new(
            Some(construct(&lhs)), // B.child1 := construct_balltree(L),
            Some(construct(&rhs)), // B.child2 := construct_balltree(R),
            midpoint, // B.pivot := p
            max_dist // let B.radius be maximum distance from p among children
        );

        return Box::new(b);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_creation_simple() {
        assert!(true);
    }
}
