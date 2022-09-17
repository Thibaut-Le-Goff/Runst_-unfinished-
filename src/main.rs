mod runst;

fn main() {
    // Network initialisation:
    println!("Initialisation du réseaux de neurones :");
    println!("Le réseau :");
    const LAYER: [usize; 4] = [3, 6, 2, 4];
    println!("Le nombre de neurones de la première couche à la dernière :");
    println!("{:?}\n", LAYER);

    // The weights:
    println!("Les poids :");
    let matrix_weight_l0_1: Vec<f64> = runst::weight_init::he_uniform_dis(LAYER[0], LAYER[1]);
    println!("La matrix des poids entre la couche 0(input) et 1 :");
    println!("{:?}\n", matrix_weight_l0_1);
   
    let matrix_weight_l1_2: Vec<f64> = runst::weight_init::he_uniform_dis(LAYER[1], LAYER[2]);
    println!("La matrix des poids entre la couche 1 et 2 :");
    println!("{:?}\n", matrix_weight_l1_2);

    let matrix_weight_l2_3: Vec<f64> = runst::weight_init::he_uniform_dis(LAYER[2], LAYER[3]);
    println!("La matrix des poids entre la couche 2 et 3 :");
    println!("{:?}\n", matrix_weight_l2_3);

    // Creation of the tensor:
    let tensor: [&Vec<f64>; 3] = [&matrix_weight_l0_1, &matrix_weight_l1_2, &matrix_weight_l2_3];

    // Input layer:
    println!("La couches des entrées, la numéros 0 :");
    let vec_input: Vec<f64> = vec![1.0; LAYER[0]];
    println!("La couche 0 a pour valeurs :");
    println!("{:?}\n", vec_input);

    // Propagation
    println!("Propagation des données d'entrée :");
    println!("De la couche 0(input) à 1, à la couche 1, les données sont, à la sortie des neurones :");
    let vec_l0_1: Vec<f64> = runst::multiply(tensor[0], &vec_input);
    let vec_l1: Vec<f64> = runst::activ_fun::leaky_relu(&vec_l0_1);
    println!("{:?}\n", vec_l1);

    println!("De la couche 1 à 2, à la couche 2, les données sont, à la sortie des neurones :");
    let vec_l1_2: Vec<f64> = runst::multiply(tensor[1], &vec_l1);
    let vec_l2: Vec<f64> = runst::activ_fun::leaky_relu(&vec_l1_2);
    println!("{:?}\n", vec_l2);

    println!("De la couche 2 à 3(output), à la couche 3, les données sont, à la sortie des neurones :");
    let vec_l2_3: Vec<f64> = runst::multiply(tensor[2], &vec_l2);
    let vec_l3: Vec<f64> = runst::activ_fun::softmax(&vec_l2_3);
    println!("{:?}\n", vec_l3);

    // Expectation    
    println!("Ce que le réseaux me donne :");
    println!("{:?}\n", vec_l3);

    println!("Ce que j'attend que le réseaux me donne :");
    let expected: Vec<f64> = vec![1.0; LAYER[3]];
    println!("{:?}\n", expected);

    println!("La fon,ction loss donne donc (reçu - attendu)^2 :");
    let loss: Vec<f64> = runst::loss(&vec_l3, &expected);
    println!("{:?}\n", loss);

}