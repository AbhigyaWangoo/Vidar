
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