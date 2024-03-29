/*
use crate::runst::Network;
use crate::runst::BackPropData;
use rayon::prelude::*;
use crate::runst::calculations::*;

/* 
pub struct BackPropData {
    pub true_counter: &'static mut usize,
    pub layers_iterator: &'static mut usize,
    pub weight_bias_trouve: &'static mut Vec<bool>,
}
*/

fn weights_update(true_counter: &mut usize, weight_bias_trouve: &mut Vec<bool>, weights: &mut [f32], inputs: &Vec<Vec<f32>>, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<Vec<f32>>) {

    let precision_success: f32 = 0.001;

    let weights_learning_rate: f32 = 0.01;
    let mut derivative_square_residual: Vec<f32> = Vec::new();
    let mut sum_derivative_square_residual: Vec<f32> = Vec::new();
    let mut step_size: Vec<f32> = Vec::new();

    for j in 0..observed_values.len() {
        // for every matrixes of values observed

        derivative_square_residual = (multiplication::num_mat(-1.0, &inputs[j])) * (observed_values[j] - network_predictions[j]);
        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
    }

    step_size = sum_derivative_square_residual * weights_learning_rate;


    weights = weights - step_size;

    if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
        weight_bias_trouve[para] = true;
        true_counter = true_counter + 1;

        println!("\n\nfini de trouver le bon coéficient directeur de la droite de prediction  ! ");
        println!("Le coéficient directeur : {:?}", weights);
    }
}


fn bias_update(bias: &mut [f32], inputs: &Vec<Vec<f32>>, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<Vec<f32>>) {

    let precision_success: f32 = 0.001;

    let bias_learning_rate: f32 = 0.1;
    let mut derivative_square_residual: Vec<f32> = Vec::new();
    let mut sum_derivative_square_residual: Vec<f32> = Vec::new();
    let mut step_size: Vec<f32> = Vec::new();

    for j in 0..observed_values[0].len() {

        derivative_square_residual = (-1.0 * inputs[j]) * (observed_values[0][j] - predicted_value);
        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
        

    }

    step_size = sum_derivative_square_residual * bias_learning_rate;
    bias = bias - step_size;

    if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
        weight_bias_trouve[para] = true;
        true_counter = true_counter + 1;

        println!("\n\nfini de trouver le bon intercept de la droite de prediction  ! ");
        println!("L'intercept : {:?}", bias);
    }
}
 

*/