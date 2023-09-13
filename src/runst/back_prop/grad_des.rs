/* 
use crate::runst::Network;
use rayon::prelude::*;
use crate::runst::calculations;

//pub fn grad_descent(net: &Network, observed_values: &Vec<f32>, network_predictions: &Vec<f32>, weights: &Vec<f32>, bias: &Vec<f32>) -> (weight: Vec<f32>, bias: Vec<f32>) {
pub fn grad_descent(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> usize {

    let try_number: usize = 1000;

    /* 
    let optimal_weights: Vec<Vec<bool>> = weights.par_iter()
        .map(|&val| vec![false; val])
        // Here we loop through the weight matrix (par_iter())
        // as we loop in each vectors of the weight matrix 
        // we change each vectors by another one with the map method 
        .collect();
        // and collect the result into optimal_weights
    */    
    let optimal_weights: Vec<bool> = vec![false; weights.len()];
    println!("weights : {:?}", optimal_weights);

    /* 
    let optimal_bias: Vec<Vec<bool>> = bias.par_iter()
        .map(|&val| vec![false; val])
        .collect();
    */
    let optimal_bias: Vec<bool> = vec![false; bias.len()];
    println!("bias : {:?}", optimal_bias);

    //let mut optimal_weights: Vec<Vec<bool>> = vec![[new::vec()]; weights.len()];
    //let mut optimal_bias: Vec<Vec<bool>> = vec![[new::vec()]; bias.len()];

    //let mut weight_bias_trouve: [bool; 2] = [false, false];
    let mut true_counter: usize = 0;
    
    let precision_success: f32 = 0.001;

    let mut step_size: Vec<f32>;

    let power_dif : f32 = 1.0;

    let weight_bias_learning_rate: [f32; 2] = [0.01, 0.1];
    //let mut weight_bias: [f32; 2] = [weight[0][0], bias[0][0]];

    let mut derivative_square_residual: Vec<f32>;
    let mut sum_derivative_square_residual: Vec<f32>;

    let mut predicted_value: Vec<f32>;

    let mut number_end: usize = 0;

    let weights_bias_matrix_nb: usize = weights.len() + bias.len();
    let mut length_param_matrix: usize;

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

                /* 
                length_param_matrix = weights[para].len();

                derivative_square_residual = vec![0.0; length_param_matrix];
                sum_derivative_square_residual = vec![0.0; length_param_matrix];
                step_size = vec![0.0; length_param_matrix];
                */

                for j in 0..observed_values.len() {
                    // for

                    //predicted_value = (weight[z][j] * observed_values[0][j]) + bias[z][j];
                    // this value is already calculated by the propagation processe

                    derivative_square_residual = (-power_dif * inputs[j]) * (observed_values[0][j] - network_predictions);
                    sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                }

                step_size = sum_derivative_square_residual * weight_bias_learning_rate[para];
                weight_bias[para] = weight_bias[para] - step_size;

                if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                    weight_bias_trouve[para] = true;
                    true_counter = true_counter + 1;

                    println!("\n\nfini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                    println!("Le coéficient directeur : {}", weight_bias[para]);
                }

            }



            // if the bias matrix is not optimised :
            if optimal_bias[para] == false {

                sum_derivative_square_residual = 0.0;

                for j in 0..observed_values[0].len() {

                    //predicted_value = (weight[z][j] * observed_values[0][j]) + bias[z][j];
                    // this value is already calculated by the propagation processe

                    if y == 0 {
                        derivative_square_residual = (-power_dif * inputs[j]) * (observed_values[0][j] - predicted_value);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }

                    if y == 1 {
                        derivative_square_residual = -power_dif * (observed_values[j] - predicted_value);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }
                }

                step_size = sum_derivative_square_residual * weight_bias_learning_rate[para];
                weight_bias[para] = weight_bias[para] - step_size;

                if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                    weight_bias_trouve[para] = true;
                    true_counter = true_counter + 1;

                    if y == 0 {
                        println!("\n\nfini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                        println!("Le coéficient directeur : {}", weight_bias[para]);
                    }

                    if y == 1 {
                        println!("\n\nfini de trouver le bon intercept de la droite de prediction  ! ");
                        println!("L'intercept : {}", weight_bias[para]);
                    }
                }
            }
        }

        if true_counter == weight_bias_trouve.len() {
            number_end = trial;
            break;
        }
    }
    
    if true_counter == weight_bias_trouve.len() {
        println!("\nl'équation de la droite de prédiction est : y = a{} + {}", weight_bias[0], weight_bias[1]);
        println!("L'algorithme a fait {} essaies pour trouver les bonnes données.", number_end + 1);
    }

    //weights

    //return (weight_bias, weight_bias);
    0
}
*/