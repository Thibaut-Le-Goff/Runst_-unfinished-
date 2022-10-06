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

    // The bias:
    let bias_l0: Vec<f64> = vec![0.0; LAYER[1]];
    println!("Les biais de la couche 0(input) sont :");
    println!("{:?}\n", bias_l0);

    let bias_l1: Vec<f64> = vec![1.0; LAYER[2]];
    println!("Les biais de la couche 0(input) sont :");
    println!("{:?}\n", bias_l1);

    let bias_l2: Vec<f64> = vec![2.0; LAYER[3]];
    println!("Les biais de la couche 0(input) sont :");
    println!("{:?}\n", bias_l2);

    // Creation of the bias of matrix:
    let bias_weight: [&Vec<f64>; 3] = [&bias_l0, &bias_l1, &bias_l2];
    
    // Input layer:
    println!("La couches des entrées, la numéros 0 a pour valeurs:");
    let vec_input: Vec<f64> = vec![1.0; LAYER[0]];
    println!("{:?}\n", vec_input);

    // Propagation
    println!("Propagation des données d'entrée :");

    println!("Dans les neurones de la couche 0(input) à 1 :");
    let vec_l0_1: Vec<f64> = runst::multiply(tensor[0], &vec_input);
    println!("Après La multiplication :");
    println!("{:?}\n", vec_l0_1);
    let vec_l0_1_bias: Vec<f64> = runst::bias_addition(&vec_l0_1, bias_weight[0]);
    println!("Après l'ajout des biais :");
    println!("{:?}\n", vec_l0_1_bias);
    let vec_l1: Vec<f64> = runst::activ_fun::leaky_relu(&vec_l0_1_bias);
    println!("Après le passage dans la function d'activation :");
    println!("{:?}\n", vec_l1);

    println!("Dans les neurones de la couche 1 à 2 :");
    let vec_l1_2: Vec<f64> = runst::multiply(tensor[1], &vec_l1);
    println!("Après La multiplication :");
    println!("{:?}\n", vec_l1_2);
    let vec_l1_2_bias: Vec<f64> = runst::bias_addition(&vec_l1_2, bias_weight[1]);
    println!("Après l'ajout des biais :");
    println!("{:?}\n", vec_l1_2_bias);
    let vec_l2: Vec<f64> = runst::activ_fun::leaky_relu(&vec_l1_2_bias);
    println!("Après le passage dans la function d'activation :");
    println!("{:?}\n", vec_l2);

    println!("Dans les neurones de la couche 2 à 3(output) :");
    let vec_l2_3: Vec<f64> = runst::multiply(tensor[2], &vec_l2);
    println!("Après La multiplication :");
    println!("{:?}\n", vec_l2_3);
    let vec_l2_3_bias: Vec<f64> = runst::bias_addition(&vec_l2_3, bias_weight[2]);
    println!("Après l'ajout des biais :");
    println!("{:?}\n", vec_l2_3_bias);
    let vec_l3: Vec<f64> = runst::activ_fun::leaky_relu(&vec_l2_3_bias);
    println!("Après le passage dans la function d'activation :");
    println!("{:?}\n", vec_l3);

    // Expectation    
    println!("Ce que le réseaux me donne :");
    println!("{:?}\n", vec_l3);

    println!("Ce que j'attend que le réseaux me donne :");
    let expected: Vec<f64> = vec![1.0; LAYER[3]];
    println!("{:?}\n", expected);

    println!("La fonction loss donne donc (reçu - attendu)^2 :");
    let loss: Vec<f64> = runst::loss(&vec_l3, &expected);
    println!("{:?}\n", loss);
}