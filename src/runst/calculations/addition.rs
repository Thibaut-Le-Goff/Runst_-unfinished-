use rayon::prelude::*;

pub fn vec_vec(vector: &[f32], bias: &[f32]) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = vector[i] + bias[i];
    }
    result
}