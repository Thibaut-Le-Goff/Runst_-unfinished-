//mod runst;

//const LAYER: [usize; 2] = [2, 3];

fn multiply(matrix: &Vec<f64>, vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; matrix.len() / vector.len()];
    let mut counter: usize = 0;
    
    for i in 0..= (matrix.len() / vector.len()) - 1 {
        let mut x: f64 = 0.0;

        for j in 0..= vector.len() - 1 {
            x = (vector[j] * matrix[counter]) + x;
            counter = counter + 1;
        }

        result[i] = x;
    }
    return result;
}

fn main() {

    let matrix: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let vec6: Vec<f64> = vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let vec3: Vec<f64> = vec![2.0, 1.0, 1.0];
    let vec2: Vec<f64> = vec![2.0, 1.0];
    let vec1: Vec<f64> = vec![3.0];

    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec6);

    println!("Font :");
    let mat6: Vec<f64> = multiply(&matrix, &vec6);
    println!("{:?}", mat6);



    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec3);

    println!("Font :");
    let mat3: Vec<f64> = multiply(&matrix, &vec3);
    println!("{:?}", mat3);




    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec2);

    println!("Font :");
    let mat2: Vec<f64> = multiply(&matrix, &vec2);
    println!("{:?}", mat2);





    println!("La matrise :");
    println!("{:?}", matrix);

    println!("Multipliée par le vecteur :");
    println!("{:?}", vec1);

    println!("Font :");
    let mat1: Vec<f64> = multiply(&matrix, &vec1);
    println!("{:?}", mat1);
}