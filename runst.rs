pub fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {
    use std::thread;

    let mut result: Vec<f64> = vec![0.0; matrix.len()];
    //let vec: Vec<f64> = vector.clone();
    let row: usize = matrix.len() - 1;
    let column: usize = vector.len() - 1;

    for i in 0..= row { // row
        let mut x: f64 = 0.0;
        thread::scope(|thread| {
            thread.spawn(|| {
                for j in 0..= column { // column
                    x = (vector[j] * matrix[i][j]) + x;
                }
                result[i] = x;
            });
        });
    }
    return result;
}

pub mod weight_init;
