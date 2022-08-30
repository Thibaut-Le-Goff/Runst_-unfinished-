//pub async fn cal_for_row(matrix: &Vec<f64>, vector: &Vec<f64>) -> f64 {
    //let mut x: f64 = 0.0;

    //for j in 0..= vector.len() - 1 { // column
        //x = (vector[j] * matrix[j]) + x;
    //}
    //return x;
//}

pub fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {
    use tokio::runtime::Runtime;
    //use tokio::*;

    let mut result: Vec<f64> = vec![0.0; matrix.len()];
    //let mut thread = Runtime::new().unwrap(); 

    for i in 0..= matrix.len() - 1 { // row
        //thread.block_on(async move {
            //let result[i]: Vec<f64> = tokio::spawn(async move { cal_for_row(&matrix[i], &vector).await });
        //});
        //loop{}
        let mut x: f64 = 0.0;
        for j in 0..= vector.len() - 1 {
            x = (vector[j] * matrix[i][j]) + x;
        }
        result[i] = x;
    }
    return result;
}

pub mod weight_init;
