mod runst;

fn main() {
    ////////////////////////////// Data set ///////////////////////
    const DOSAGE: [f64; 3] = [0.0, 1.0, 2.0]; // ce qui est donné au réseau
    let mut observed_effect: Vec<f64> = vec![0.0; DOSAGE.len()]; // ce que le réseau donne

    const DESIRED_EFFECT: [f64; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne


    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network
    println!("Initialisation du réseaux de neurones :");
    println!("Le réseau :");
    const LAYER: [usize; 3] = [1, 2, 1];
    println!("Le nombre de neurones de la première couche à la dernière :");
    println!("{:?}\n", LAYER);

    // The weights:
    println!("Les poids :");
    let mut matrix_weight_l0: Vec<f64> = runst::weight_init::normal_dis(LAYER[0], LAYER[1]);
    println!("La matrix des poids entre la couche 0(input) et 1 :");
    println!("{:?}\n", matrix_weight_l0);
   
    let mut matrix_weight_l1: Vec<f64> = runst::weight_init::normal_dis(LAYER[1], LAYER[2]);
    println!("La matrix des poids entre la couche 1 et 2 :");
    println!("{:?}\n", matrix_weight_l1);

    // Creation of the tensor:
    let mut tensor: [Vec<f64>; 2] = [matrix_weight_l0, matrix_weight_l1];

    // The bias:
    let mut bias_l1: Vec<f64> = vec![0.0; LAYER[1]];
    println!("Les biais de la couche 1 sont :");
    println!("{:?}\n", bias_l1);

    let mut bias_l2: Vec<f64> = vec![0.0; LAYER[2]];
    println!("Les biais de la couche 2 sont :");
    println!("{:?}\n", bias_l2);

    // Creation of the bias of matrix:
    let mut bias_matrix: [Vec<f64>; 2] = [bias_l1, bias_l2];

    // details of the structure of the network
    let mut vec_input: Vec<f64> = vec![0.0; LAYER[0]];

    let mut sum_l1: Vec<f64> = vec![0.0; LAYER[1]];
    let mut sum_l1_bias: Vec<f64> = vec![0.0; LAYER[1]];
    let mut vec_l1: Vec<f64> = vec![0.0; LAYER[1]];

    let mut sum_l2: Vec<f64> = vec![0.0; LAYER[2]];
    let mut sum_l2_bias: Vec<f64> = vec![0.0; LAYER[2]];



    ////////////////////// PROPAGATION ////////////////////////////////////
    for i in 0..= DOSAGE.len() - 1 {
        println!("Propagation des données d'entrée :");

        // Input layer:
        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        vec_input[0] = DOSAGE[i];
        println!("{:?}\n", vec_input);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        sum_l1 = runst::multiply(&tensor[0], &vec_input);
        println!("Après La multiplication :");
        println!("{:?}\n", sum_l1);
        sum_l1_bias = runst::bias_addition(&sum_l1, &bias_matrix[0]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", sum_l1_bias);
        vec_l1 = runst::activ_fun::soft_plus(&sum_l1_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", vec_l1);

        println!("Dans les neurones de la couche 1 à 2 :");
        sum_l2 = runst::multiply(&tensor[1], &vec_l1);
        println!("Après La multiplication :");
        println!("{:?}\n", sum_l2);
        sum_l2_bias = runst::bias_addition(&sum_l2, &bias_matrix[1]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", sum_l2_bias);

        /* 
        let vec_l2: Vec<f64> = runst::activ_fun::soft_plus(&sum_l1_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", vec_l2);
        */

        println!("\n\nCe que le réseaux me donne :");
        println!("{:?}\n", sum_l1_bias);

        println!("Enregistrement de l'output :");
        observed_effect[i] = sum_l1_bias[0];
    }


 
        /////////////////////// BACKPROPAGATION //////////////////////////

        /* 
        const DOSAGE: [f64; 3] = [0.0, 1.0, 2.0]; // ce qui est donné au réseau
        let mut observed_effect: Vec<f64> = vec![0.0; 3]; // ce que le réseau donne

        const DESIRED_EFFECT: [f64; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne
        */

    let try_number: usize = 1000;

        //let mut weights_l0_find: Vec<bool> = vec![false; tensor[0].len()];
    let mut weights_l1_find: Vec<bool> = vec![false; tensor[1].len()];

        //let mut bias_l0_find: Vec<bool> = [false; bias_matrix[0].len()];
    let mut b3_l1_find: Vec<bool> = vec![false; bias_matrix[1].len()];

        // indication si les valeurs attendue ont 
        // était trouvées
        
    let precision_success: f64 = 0.001;
        // le programme s'arêtera lorsque que la somme
        // des dérivées du carré de la différence entre les 
        // données observées et prévues est entre cette 
        // valeur et sont négatif

    let mut step_size: f64;
        // taille des pas dans le rapprochement de 
        // sum_derivative_square_residual

    let learning_rate_weights: f64 = 0.1;
    let learning_rate_bias: f64 = 0.1;

    let mut sum_derivative_square_residual: f64;
    let mut derivative_square_residual: f64;
        // la somme des dérivés du carré de la différence 
        // entre la valeur observé et celle attendue
        // pour le calcule du coéficient directeur de la
        // courbes des prédictions a N-1 et N

        // <brouilon>
        // let batch_number: usize = 2;
        // pour mini batche :
        // for j in 0..= batch_number - 1 {  a la place de for j in WEIGHT.len() -1
        //  crée un nombre aléatoire x entre 0 et OBSERVED_HEIGHT.len()
        //  utilise x dans WEIGHT[x] et OBSERVED_HEIGHT[x]
        // </brouilon>

    for _i in 0..= try_number - 1 {
        // pour le nombre d'essayes indiqué

        println!("\n\nPour les poids :");
        for j in 0..= weights_l1_find.len() - 1 {
            if weights_l1_find[j] == false {

                    // met le "compteur" de la somme a zero
                sum_derivative_square_residual = 0.0;
       
                    // calcule d ssr
                for y in 0..= observed_effect.len() - 1 {
                    derivative_square_residual = (-2.0 * &vec_l1[j]) * (observed_effect[y] - DESIRED_EFFECT[y]);
        
                    sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                }
                println!("La somme des dérivées pour le calcule du du poid : {:?}", sum_derivative_square_residual);
        
                    // calcule step size, le pas
                step_size = sum_derivative_square_residual * learning_rate_weights;
                println!("Le step_size pour le calcule du poid : {:?}", step_size);
        
                    // determination de la prochaine valeur 
                tensor[1][j] = tensor[1][j] - step_size;

                println!("\nLe poid numéro {:?} de la couche 1 est {:?}", j, tensor[1][j]);

                if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                    //if step_size <= step_size_stop && step_size >= -step_size_stop {
                    println!("\n\nfini de trouver le bon poid numéro {:?} de la couche 1 !", j);
                    weights_l1_find[j] = true;
                    println!("Le poid : {:?}", tensor[1][j]);
                }
            }
        }

        println!("\n\nPour le biai :");
        if b3_l1_find[0] == false {
                // met le "compteur" de la somme a zero
            sum_derivative_square_residual = 0.0;

                // calcule d ssr
            for j in 0..= observed_effect.len() - 1 {
                derivative_square_residual = -2.0 * (observed_effect[j] - DESIRED_EFFECT[j]);

                sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
            }
            println!("La somme des dérivées pour le calcule du biai : {:?}", sum_derivative_square_residual);

                // calcule step size, le pas
            step_size = sum_derivative_square_residual * learning_rate_bias;
            println!("Le step_size pour le calcule du biai : {:?}", step_size);

                // determination de la prochaine valeur du coéficient directeur
            bias_matrix[1][0] = bias_matrix[1][0] - step_size;
            println!("\nLe biai : {:?}", bias_matrix[1][0]);

            if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                //if step_size <= step_size_stop && step_size >= -step_size_stop {
                println!("\n\nfini de trouver le bon biai numéro 3 !");
                b3_l1_find[0] = true;
                println!("Le biai : {:?}", bias_matrix[1]);
            }
        }
    }
}