use rayon::prelude::*;

pub fn vec_vec(vector1: &[f32], vector2: &[f32]) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector1.len()];

    result.par_iter_mut().enumerate().for_each(|(iterator, value)| {
        *value = vector1[iterator] - vector2[iterator];

    });
    
    result
}

/*


fn main() {
    let mut matrix: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let number: f32 = 2.0;
    
    println!("Matrix: {:?}", matrix);
    println!("Number: {}", number);

    let result: &Vec<f32> = mat_num(&mut &matrix, &number).unwrap_or_else(|| &matrix);

    println!("Result: {:?}", result);
}

pub fn mat_num<'a>(matrix: &mut &'a Vec<f32>, number: &f32) -> Option<&'a Vec<f32>> {
    use rayon::prelude::*;

    matrix.par_iter_mut().for_each(|value| {
        *value *= *number;
    });

    Some(matrix)
}

*/