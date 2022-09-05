mod runst;

fn main() {

    const LAYER: [usize; 3] = [2, 3, 1];

    let matrix: Vec<f64> = runst::weight_init::normal_dis(LAYER[0], LAYER[1]);
    let vec6: Vec<f64> = vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let vec3: Vec<f64> = vec![2.0, 1.0, 1.0];
    let vec2: Vec<f64> = vec![2.0, 1.0];
    let vec1: Vec<f64> = vec![3.0];

    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec6);

    println!("Font :");
    let mat6: Vec<f64> = runst::multiply(&matrix, &vec6);
    println!("{:?}", mat6);



    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec3);

    println!("Font :");
    let mat3: Vec<f64> = runst::multiply(&matrix, &vec3);
    println!("{:?}", mat3);




    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec2);

    println!("Font :");
    let mat2: Vec<f64> = runst::multiply(&matrix, &vec2);
    println!("{:?}", mat2);





    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec1);

    println!("Font :");
    let mat1: Vec<f64> = runst::multiply(&matrix, &vec1);
    println!("{:?}", mat1);
}