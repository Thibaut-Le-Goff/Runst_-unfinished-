/*
pub fn multiply(matrix: &Vec<f32>, vector: &Vec<f32>) -> Vec<f32> {

    let result_lenght: usize = matrix.len() / vector.len();
    let mut result: Vec<f32> = vec![0.0; result_lenght];
    let mut counter: usize = 0;
    //let mut x: f32;

    for i in 0..= result_lenght - 1 {
    
        let mut x: f32 = 0.0;
    
        for j in 0..= vector.len() - 1 {
            x += &vector[j] * &matrix[counter + j];
        }
    
        counter = counter + vector.len();
        result[i] = x;
    }

    result
}
*/


use rayon::prelude::*;

pub fn mat_vec(matrix: &[f32], vector: &[f32]) -> Vec<f32> {
    assert_eq!(matrix.len() % vector.len(), 0, "Dimensions of matrix and vector are not compatible for multiplication");
    
    let result_length: usize = matrix.len() / vector.len();
    let mut result: Vec<f32> = vec![0.0; result_length];

    result.par_iter_mut().enumerate().for_each(|(i, row)| {
        /*
        We passe throught each elements of the vector result 
        with for_each() in parallel and keep them mutable with par_iter_mut() 
        who permit the mutable access via the mutable reference, we use
        enumerate to attach to each elements an index, i is the index and 
        row is the element at result[i], the argument (i, res) means we
        pass throught both the index and result[index] in order to manipulate 
        both.
        */

        let x: f32 = (0..vector.len())
            .fold(0.0, |sum: f32, j: usize| sum + vector[j] * matrix[i * vector.len() + j]);
            // Here we create a fold method who loop from 0 to the length of the vector
            // The fold method is used to add the multiplication of the rows  
            // it's start with the variable sum at 0.0, j is the iterator of the vector.
        *row = x;

    });

    result
}