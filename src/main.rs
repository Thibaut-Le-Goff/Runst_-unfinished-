mod runst;

fn main() {
	let row: usize = 4; // nb neurones layer n
	let column: usize = 4; // nb neurones layer n-1, fan_in

	let test: Vec<Vec<f64>> = runst::weight_init::normal_dis(column, row);

    println!("La matrise :");
	for i in 0..= (row - 1) {
        println!("{:?}", test[i]);
    }

    println!("Multipliée par le vecteur :");
    let test1: Vec<f64> = vec![1.0; column];
    println!("{:?}", test1);

    println!("Font :");
    let res: Vec<f64> = runst::multiply(test, test1);
    println!("{:?}", res);

    // test
    use rayon::prelude::*;
    fn sum_of_squares(input: &Vec<f64>) -> f64 {
        input.par_iter() // <-- just change that!
            //.map(|&i| i * i)
            .map(
                |&i| i + 2.0
            )
            .sum()
    }
    let test21: Vec<f64> = vec![1.0; 1000];
    let test2: f64 = sum_of_squares(&test21);
    println!("{:?}", test2);
}
