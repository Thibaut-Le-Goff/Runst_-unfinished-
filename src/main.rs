mod runst;

fn main() {
	let row: usize = 4; // nb neurones layer n
	let column: usize = 2; // nb neurones layer n-1, fan_in

	let test: Vec<Vec<f64>> = runst::weight_init::normal_dis(column, row);

    println!("La matrise :");
	for i in 0..= (row - 1) {
        println!("{:?}", test[i]);
    }

    println!("Multipliée par le vecteur :");
    let test1: Vec<f64> = vec![1.0, 2.0];
    println!("{:?}", test1);


    println!("Font :");
    //let res: Vec<f64> = runst::multiply(test, test1);
    //println!("{:?}", res);


    use ndarray::Array;

    //let array_vec = Array::from_vec(res);
    //println!("{}", array_vec);

    let array_mat = Array::from_vec(test);
    println!("{:?}", array_mat);
    let res: Vec<f64> = runst::multiply(array_mat, test1);
    println!("{:?}", res);


/*
    // test
    use rayon::prelude::*;

    fn multiply_vec(matrix: Vec<f64>, vector: Vec<f64>) -> f64 {
    //fn multiply(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
        matrix.par_iter()
            .map(|&i| i * vector[matrix[i]])
            .sum()
    }

    let test12: Vec<f64> = vec![2.0, 1.0];
    let test21: Vec<f64> = vec![7.0, 3.0];
    let test2: f64 = multiply_vec(test12, test21);
    println!("{:?}", test2);

 
    fn multiply_vec(matrix_vec: &Vec<f64>, vector: &Vec<f64>) -> f64 {
        matrix_vec.par_iter()
            .map(|&j| j * vector[(j as usize) - 1])
            .sum()
    }

    fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {
        //matrix.par_iter().for_each(
        let mut result: Vec<f64> = vec![0.0; matrix.len()];
        for i in 0..= matrix.len() - 1 {
            result[i] = multiply_vec(vector, matrix[i]);
        }
        return result;
    }

    let test12: Vec<f64> = vec![1.0, 2.0];
    let test21: Vec<f64> = vec![1.0, 2.0];
    let test2: f64 = multiply_vec(&test12, &test21);
    //let test2: Vec<f64> = multiply(test, test1);
    println!("{:?}", test2);
    */

}