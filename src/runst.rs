pub fn multiply(matrix: &Vec<f64>, vector: &Vec<f64>) -> Vec<f64> {

    let result_lenght: usize = matrix.len() / vector.len();
    let mut result: Vec<f64> = vec![0.0; result_lenght];
    let mut counter: usize = 0;

    for i in 0..= result_lenght - 1 {
        let mut x: f64 = 0.0;

        for j in 0..= vector.len() - 1 {
            x = (&vector[j] * &matrix[counter]) + x;
            counter = counter + 1;
        }
        result[i] = x;
    }
    return result;
}

pub mod activ_fun;
pub mod weight_init;