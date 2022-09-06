pub fn relu(vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        };
    }
    return result;
}