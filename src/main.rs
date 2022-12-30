mod runst;
//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    const DOSAGE: [f32; 3] = [0.0, 0.5, 1.0]; // ce qui est donné au réseau
    const OBSERVED_EFFECT: [f32; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne
     
    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network
    //const network_struct: [usize; 3] = [1, 2, 1]; // I will propably put it in a box
    let network_struct: Vec<usize> = vec![1, 2, 1];
    let distrib: Box<&str> = Box::new("normal_dis");

    let (mut weights_tensor, mut bias_matrix) = runst::net_init(&network_struct, &distrib);
    
    /*
    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut next_layer: usize;

    // create the weights and the bias between the layers:
    for i in 0..network_struct.len() - 2 {
        next_layer = i + 1;

        let weight_matrix: Vec<f32> = runst::weight_init::normal_dis(network_struct[i], network_struct[next_layer]);
        weights_tensor.push(weight_matrix);

        let bias_vector: Vec<f32> = vec![0.0; network_struct[next_layer]];
        bias_matrix.push(bias_vector);
    }
    */

    ////////////////////// PROPAGATION ////////////////////////////////////
    let mut network_outputs_sum_bias: Vec<Vec<f32>> = Vec::new();
    let mut network_outputs_neurons: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..DOSAGE.len() {
        // for each pair of datas in the data set
        println!("Propagation numéro {} des données d'entrée :", i + 1);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let mut neuron_out: Vec<f32> = vec![DOSAGE[i]; network_struct[0]];
        println!("{:?}\n", &neuron_out);

        for y in 0..network_struct.len() - 1 {
            // for each hiden layer + the output layers:
            // - 1 : avoid the first layer

            println!("\nDans les neurones de la couche {} à {} :", y, y + 1);
            let neuron_sum: Vec<f32> = runst::multiply(&weights_tensor[y], &neuron_out);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = runst::bias_addition(&neuron_sum, &bias_matrix[y]);
            println!("Après l'ajout des biais :");
            println!("{:?}\n", &neuron_sum_bias);

            if y == network_struct.len() - 2 {
                // if this is the last layer (the last iteration)
                // last iteration is equal to network_struct.len() - 2
                // because :
                // - 1 : the iteration started at 0
                // - 1 : avoid the first layer exactly like above

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
        }
    }


    /*
    let outputs_sum_bias: usize = network_outputs_sum_bias.len();
    let outputs_neurons: usize = network_outputs_neurons.len();

    let couche_totale: usize = network_struct.len();
    // pour un nombre qui est :
    //   network_struct.len() = le nombre de couches dans le réseau
    //   network_struct.len() *  1 = multiplier par le nb de données enregistrées 
    //                      sum_bias et activ_fun pour un autre vecteur
    //   network_struct.len() = moins la donnée n'existants pas
    //                     à la couche input

    println!("\n\nCe que le réseaux me donne à l'enver :");
    for prop in 0..DOSAGE.len() {
        // pour chaque propagation
        println!("\n\nÀ la propagation numéro {} :", (DOSAGE.len()) - prop);

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
        println!("{:?}\n", DOSAGE[(DOSAGE.len()) - prop]);

        for couche in 0..couche_totale {
            // pour chaque couches, network_struct.len()
            println!("Les neurons :");
            for neuron in 0..network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche)].len() {
                // pour chaque neuron de la couche j de l'itération i
                println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche)][neuron]);
            }
            println!("Les sum_bias :");
            for sum_bias in 0..network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche)].len() {
                // pour chaque sum_bias de la couche j de l'itération i
                println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche)][sum_bias]);
            }
        }
        println!("Les neurons :");
        println!("{:?}\n", DOSAGE[(DOSAGE.len()) - prop]);
    }
    */
}