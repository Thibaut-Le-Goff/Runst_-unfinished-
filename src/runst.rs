pub fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {

    let mut result: Vec<f64> = vec![0.0; matrix.len()];
    
    for i in 0..= matrix.len() - 1 { // row
        let mut x: f64 = 0.0;
        for j in 0..= vector.len() - 1 {
            x = (vector[j] * matrix[i][j]) + x;
        }
        result[i] = x;
    }
    return result;
}

pub mod weight_init;
