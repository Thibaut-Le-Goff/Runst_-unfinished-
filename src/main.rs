mod runst;
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    const DOSAGE: [f32; 3] = [0.0, 0.5, 1.0]; // ce qui est donné au réseau
    const OBSERVED_EFFECT: [f32; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne


    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network
    const NETWORK_STRUCT: [usize; 3] = [1, 2, 1];

    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut next_layer: usize;

    // create the weights and the bias between the layers:
    for i in 0..= NETWORK_STRUCT.len() - 2 {
        next_layer = i + 1;

        let weight_matrix: Vec<f32> = runst::weight_init::normal_dis(NETWORK_STRUCT[i], NETWORK_STRUCT[next_layer]);
        weights_tensor.push(weight_matrix);

        let bias_vector: Vec<f32> = vec![0.0; NETWORK_STRUCT[next_layer]];
        bias_matrix.push(bias_vector);
    }


    ////////////////////// PROPAGATION ////////////////////////////////////
    let mut network_outputs_sum_bias: Vec<Vec<f32>> = Vec::new();
    let mut network_outputs_neurons: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..= DOSAGE.len() - 1 {
        // for each pair of datas in the data set
        println!("Propagation numéro {} des données d'entrée :", i + 1);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let mut neuron_out: Vec<f32> = vec![DOSAGE[i]; NETWORK_STRUCT[0]];
        println!("{:?}\n", &neuron_out);

        for y in 0..= NETWORK_STRUCT.len() - 2 {
            // for each hiden layer + the output layers:
            // - 1 : because start at 0
            // - 1 : avoid the first layer

            println!("\nDans les neurones de la couche {} à {} :", y, y + 1);
            let neuron_sum: Vec<f32> = runst::multiply(&weights_tensor[y], &neuron_out);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = runst::bias_addition(&neuron_sum, &bias_matrix[y]);
            println!("Après l'ajout des biais :");
            println!("{:?}\n", &neuron_sum_bias);

            if y == NETWORK_STRUCT.len() - 2 {
                // if this is the last layer (the last iteration)
                // We need two time the same output:
                // - one for the next iteration
                // - one for the output
                neuron_out = runst::activ_fun::none(&neuron_sum_bias);
                let neuron_activ_fun: Vec<f32> = neuron_out.clone();
                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neuron_out);
                network_outputs_neurons.push(neuron_activ_fun);
            } else {
                neuron_out = runst::activ_fun::soft_plus(&neuron_sum_bias);
                let neuron_activ_fun: Vec<f32> = neuron_out.clone();
                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neuron_out);
                network_outputs_neurons.push(neuron_activ_fun);
            }
            network_outputs_sum_bias.push(neuron_sum_bias);
            //network_outputs_neurons.push(&neuron_activ_fun);
        }
 
        println!("\n\n\n\nréférence :");
        // Input NETWORK_STRUCT:
        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let vec_input: Vec<f32> = vec![DOSAGE[i]; NETWORK_STRUCT[0]];
        println!("{:?}\n", &vec_input);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        let vec_l0_sum: Vec<f32> = runst::multiply(&weights_tensor[0], &vec_input);
        println!("Après La multiplication :");
        println!("{:?}\n", &vec_l0_sum);
        let vec_l0_sum_bias: Vec<f32> = runst::bias_addition(&vec_l0_sum, &bias_matrix[0]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", &vec_l0_sum_bias);
        let vec_l0_activ_fun: Vec<f32> = runst::activ_fun::soft_plus(&vec_l0_sum_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", &vec_l0_activ_fun);

        println!("Dans les neurones de la couche 1 à 2 :");
        let vec_l1_sum: Vec<f32> = runst::multiply(&weights_tensor[1], &vec_l0_activ_fun);
        println!("Après La multiplication :");
        println!("{:?}\n", &vec_l1_sum);
        let vec_l1_sum_bias: Vec<f32> = runst::bias_addition(&vec_l1_sum, &bias_matrix[1]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", &vec_l1_sum_bias);
        let vec_l1_activ_fun: Vec<f32> = runst::activ_fun::none(&vec_l1_sum_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", &vec_l1_activ_fun);


        //enregistrement des données:
        //network_outputs_input_neurons.push(vec_input);
        /*
        //network_output.push(vec_l0_sum);
        network_outputs_sum_bias.push(vec_l0_sum_bias);
        network_outputs_neurons.push(vec_l0_activ_fun);

        //network_output.push(vec_l1_sum);
        network_outputs_sum_bias.push(vec_l1_sum_bias);
        network_outputs_neurons.push(vec_l1_activ_fun);
        */
    }


    /* 
    let outputs_sum_bias: usize = network_outputs_sum_bias.len() - 1;
    let outputs_neurons: usize = network_outputs_neurons.len() - 1;

    let couche_totale: usize = NETWORK_STRUCT.len() - 1;
    // pour un nombre qui est :
    //   NETWORK_STRUCT.len() = le nombre de couches dans le réseau
    //   NETWORK_STRUCT.len() *  1 = multiplier par le nb de données enregistrées 
    //                      sum_bias et activ_fun pour un autre vecteur
    //   NETWORK_STRUCT.len() - 1 = moins la donnée n'existants pas
    //                     à la couche input

    println!("\n\nCe que le réseaux me donne à l'enver :");
    for prop in 0..= DOSAGE.len() - 1 {
        // pour chaque propagation
        println!("\n\nÀ la propagation numéro {} :", (DOSAGE.len() - 1) - prop);

        println!("Dans les neurones de la couche 1 à 2 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - (prop * couche_totale)]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - (prop * couche_totale)]);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + 1)]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + 1)]);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        println!("{:?}\n", DOSAGE[(DOSAGE.len() - 1) - prop]);

        for couche in 0..= couche_totale - 1 {
            // pour chaque couches, NETWORK_STRUCT.len() - 1
            println!("Les neurons :");
            for neuron in 0..= network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche)].len() - 1 {
                // pour chaque neuron de la couche j de l'itération i
                println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche)][neuron]);
            }
            println!("Les sum_bias :");
            for sum_bias in 0..= network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche)].len() - 1 {
                // pour chaque sum_bias de la couche j de l'itération i
                println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche)][sum_bias]);
            }
        }
        println!("Les neurons :");
        println!("{:?}\n", DOSAGE[(DOSAGE.len() - 1) - prop]);
    }

    let weights_reverse: usize = weights_tensor.len() - 1;
    let bias_reverse: usize = bias_matrix.len() - 1;
    */
}