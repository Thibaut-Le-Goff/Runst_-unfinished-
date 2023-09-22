/* 
use crate::runst::Network;

//pub fn grad_descent(net: &Network, observed_values: &Vec<f32>, network_predictions: &Vec<f32>, weights: &Vec<f32>, bias: &Vec<f32>) -> (weight: Vec<f32>, bias: Vec<f32>) {
pub fn grad_descent(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> usize {

    let try_number: usize = 1000;
    let mut true_counter: usize = 0;
    let mut number_end: usize = 0;
    let weights_bias_matrix_nb: usize = weights.len() + bias.len();

    let optimal_weights: Vec<bool> = vec![false; weights.len()];
    println!("weights : {:?}", optimal_weights);


    let optimal_bias: Vec<bool> = vec![false; bias.len()];
    println!("bias : {:?}", optimal_bias);


    for trial in 0..try_number {
        // for every try
        //for y in 0..weight_bias.len() {
        for para in 0..weights_bias_matrix_nb {
            // for every matrix parameter

            // stop if every matrix are optimised
            if true_counter == weights_bias_matrix_nb {
                break;
            }

            // if the weight matrix para is not optimised :
            if optimal_weights[para] == false {

                weights_update(weights[para]);

            }

            // if the bias matrix is not optimised :
            if optimal_bias[para] == false {

                bias_update(bias[para]);
            }
        }

        if true_counter == weights_bias_matrix_nb {
            number_end = trial;
            break;
        }
    }
    
    if true_counter == (optimal_weights.len() + optimal_bias.len()) {
        println!("\nl'équation de la droite de prédiction est : y = a{:?} + {:?}", weights, bias);
        println!("L'algorithme a fait {} essaies pour trouver les bonnes données.", number_end + 1);
    }

    0
}

pub mod optimisation;
*/