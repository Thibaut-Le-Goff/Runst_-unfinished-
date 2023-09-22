mod runst;
use crate::runst::objects::*;

//#![allow(dead_code)]
use std::env;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    ////////////////////////////// Data set ///////////////////////
    
    let inputs: Vec<Vec<f32>> = vec![vec![0.5], vec![2.3], vec![2.9]];

    let observed_values: Vec<Vec<f32>> = vec![vec![3.2, 1.9, 1.4]];


    ///////////// Network settings ///////////////////

    let network_struct: Vec<usize> = vec![1, 1];
    let distrib: &'static str = "he_normal_dis";
    let hidden_activ_fun: &'static str = "none";
    // useless in a 1-1 neural network because 
    //there is no hidden layers
    let out_activ_fun: &'static str = "none";


   let net_test: Network = Network::new( 
        network_struct,
        distrib,
        hidden_activ_fun,
        out_activ_fun,
    );

    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network

    let mut net_para: NetworkPara = NetworkPara::new(&net_test);
    
    println!("Les poids sont : \n {:?} \n Les biais sont \n {:?}", net_para.weights, net_para.bias);

    ////////////////////// PROPAGATION ////////////////////////////////////
    
    //let network_predictions: Vec<Vec<f32>> = runst::propagation::propagation(&net, &inputs ,&weights, &bias);
    let network_predictions: Vec<f32> = runst::propagation::propagation(&net_test, &inputs ,&net_para.weights, &net_para.bias);
    
    ////////////////////// BACK-PROPAGATION ////////////////////////////////////

    //let (mut trained_weights, mut trained_bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::back_prop::back_prop(&net, &observed_values, &network_predictions ,&weights, &bias);

    println!("Les donnés préditent en brut : \n {:?}", network_predictions);

    //let mut test: usize = runst::back_prop::back_prop(&net, &observed_values, &network_predictions ,&weights, &bias);
    
    //let mut test1: usize = runst::back_prop::gr

    ///////////////////// MONTRE LES DONNÉES À L'ENVERS ////////////////////
    //println!("\nLes données en brut (à l'endroit) :");
    //println!("{:?}", network_predictions);

    //println!("\n\nA l'envers :");

    //let network_predictions_concat: &Vec<f32> = &network_predictions.concat();
    
    /*
    // This block calculate the sum of neurons in the network without the input layer
    let mut number_neurons: usize = 0;
    for i in 0..weights.len() {
        // for every layer but the first one
        // or
        // for every layers of weights
        number_neurons += net.network_struct[i + 1];
    }

    let mut vec_predictions_neurons: Vec<Vec<f32>> = Vec::new();

    // This block order the datas, predictions, by neurons and by layers(but all the layers are in one vector), backward
    for i in 0..number_neurons {
        // for every neurons of the network
        let mut vec_predictions_neuron: Vec<f32> = Vec::new();

        for y in 0..inputs.len() {
            // for every propagation
            let test: usize = i + y * number_neurons;
            let test_reverse: usize = network_predictions.len() - 1 - i - y * number_neurons;

            vec_predictions_neuron.push(network_predictions[test_reverse]);
        }

        vec_predictions_neurons.push(vec_predictions_neuron)
    }

    // The predictions are now ordered by neurons and layers, now we need to seperate those layers backward
    let mut counter: usize = 0;
    let mut counter_reverse: usize = 0;

    for i in 0..net.network_struct.len() - 1 {
        // for every layers but the first one
        println!("\nAt the layer {}:", net.network_struct.len() - 1 - i);
        
        for y in 0..net.network_struct[net.network_struct.len() - 1 - i] {
            println!("The neuron number {} give these outputs: {:?}", net.network_struct[net.network_struct.len() - 1 - i] - y, vec_predictions_neurons[y + counter]);
        }

        counter += net.network_struct[net.network_struct.len() - 1 - i];
    }
    */
}