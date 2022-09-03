//mod runst;

fn random(column_test: usize, row_test: usize, a: f64, b: f64) -> [[f64; COLUMN]; ROW] {
    //use ndarray::Array;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    //const rows: usize = row;
    //const columns: usize = column;

    //let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
    let mut matrix: [[f64; COLUMN]; ROW] = [[0.0; COLUMN]; ROW];

    //let array_matrix = Array::from_vec(matrix);

    for i in 0..= row_test - 1 {
        //array_matrix[i] = Array::from_vec(matrix[i]);
        for j in 0..= column_test - 1 {
            let rand: f64 = rng.gen_range(a..=b);
            matrix[i][j] = rand;
        }
    }
    return matrix;
}

const ROW: usize = 4; // nb neurones layer n
const COLUMN: usize = 2; // nb neurones layer n-1, fan_in

fn main() {
    let a: f64 = 0.0;
    let b: f64 = 1.0;

	//let test: [[f64; column]; row] = runst::weight_init::normal_dis(column, row);
    let test: [[f64; COLUMN]; ROW] = random(COLUMN, ROW, a, b);
    //let test1: Vec<f64> = vec![1.0, 2.0];
    let test1: [f64; COLUMN] = [1.0, 2.0];

    println!("La matrise :");
	for i in 0..= (ROW - 1) {
        println!("{:?}", test[i]);
    }

    println!("Multipliée par le vecteur :");
    println!("{:?}", test1);


    println!("Font :");
    //let res: [f64; row] = runst::multiply(column, row, &test, &test1);
    //let res: Vec<f64> = runst::multiply(test, test1);

    //println!("{:?}", res);


    /*test concluant :
    use ndarray::Array;

    let array_vec = Array::from_vec(res);
    println!("{}", array_vec);

    let array_mat = Array::from_vec(test);
    println!("{:?}", array_mat);
    let res: Vec<f64> = runst::multiply(array_mat, test1);
    println!("{:?}", res);

    */

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