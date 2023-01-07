mod runst;
//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    let inputs: Vec<f32> = vec![0.0, 0.5, 1.0]; // ce qui est donné au réseau
    let _observed_values: Vec<f32> = vec![0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne
     
    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network
    let network_struct: Vec<usize> = vec![1, 2, 1];
    let distrib: &str = "he_uniform_dis";

    let (mut weights_tensor, mut bias_matrix): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::net_init(&network_struct, &distrib);

    println!("Les poids : {:?}\n\n", weights_tensor);
    println!("Les biais : {:?}\n\n", bias_matrix);

 
    ////////////////////// PROPAGATION ////////////////////////////////////
    
    let hiden_activ_fun: &str = "soft_plus";
    let out_activ_fun: &str = "none";

    let (mut network_outputs_sum_bias, mut network_outputs_neurons) = runst::propagation(&inputs, &network_struct, &weights_tensor, &bias_matrix, &hiden_activ_fun, &out_activ_fun);
    
    
    ///////////////////// MONTRE LES DONNÉES À L'ENVERS ////////////////////
    /*let outputs_sum_bias: usize = network_outputs_sum_bias.len();
    let outputs_neurons: usize = network_outputs_neurons.len();
    let couche_totale: usize = network_struct.len();
    // pour un nombre qui est :
    //   network_struct.len() = le nombre de couches dans le réseau
    //   network_struct.len() *  1 = multiplier par le nb de données enregistrées 
    //                      sum_bias et activ_fun pour un autre vecteur
    //   network_struct.len() = moins la donnée n'existants pas
    //                     à la couche input

    println!("\n\nCe que le réseaux me donne à l'enver :");
    for prop in 0..inputs.len() {
        // pour chaque propagation
        println!("\n\nÀ la propagation numéro {} :", (inputs.len() - 1) - prop);

        println!("Dans les neurones de la couche 1 à 2 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale))]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_neurons - ((prop * couche_totale))]);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + 1)]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + 1)]);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        println!("{:?}\n", inputs[inputs.len() - prop + 1]);

         
        for couche in 0..couche_totale {
            // pour chaque couches, network_struct.len()
            println!("Les neurons :");
            for neuron in 0..network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche + 1)].len() {
                // pour chaque neuron de la couche j de l'itération i
                println!("{:?}\n", network_outputs_neurons[outputs_neurons - ((prop * couche_totale) + couche + 1)][neuron]);
            }
            println!("Les sum_bias :");
            for sum_bias in 0..network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche + 2)].len() {
                // pour chaque sum_bias de la couche j de l'itération i
                println!("{:?}\n", network_outputs_sum_bias[outputs_sum_bias - ((prop * couche_totale) + couche + 2)][sum_bias]);
            }
        }
        println!("Les neurons :");
        println!("{:?}\n", inputs[(inputs.len()) - (prop + 1)]);
    }  */
}