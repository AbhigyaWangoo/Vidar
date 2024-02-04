use super::super::cache;


// Finds the mean of a set of points
fn mean(points: &Vec<i64>) -> f64 {
    let mut sum = 0.0;

    for point in points {
        sum += *point as f64;
    }

    return sum / (points.len() as f64);
}

pub fn std(points: &Vec<i64>) -> f64 {
    let avg = mean(points);
    let mut running_numerator = 0.0;

    for point in points {
        let diff = *point as f64 - avg;

        running_numerator += diff.powf(2.0);
    }

    return (running_numerator / points.len() as f64).sqrt();
}

pub fn euclidean_dist(a: &cache::node::Vidvec, b: &cache::node::Vidvec) -> f64 {
    let a_vec = a.get_vector();
    let b_vec = b.get_vector();

    assert!(a_vec.len() == b_vec.len());

    // Euclidean distance between two vectors
    let sum_of_squares: i64 = a_vec.iter().zip(b_vec.iter()).map(|(&x, &y)| (x - y) * (x - y)).sum();

    (sum_of_squares as f64).sqrt()
}