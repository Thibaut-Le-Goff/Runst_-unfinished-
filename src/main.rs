mod runst;

fn main() {
	let row: usize = 3; // nb neurones layer n
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
}
