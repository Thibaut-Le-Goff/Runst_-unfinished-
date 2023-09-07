//use crate::runst::DataSet;
use crate::runst::Network;

//pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
 
pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> usize {
    // This block calculate the sum of neurons in the network without the input layer
    let mut number_neurons: usize = 0;
    for i in 0..weights.len() {
        // for every layer but the first one
        // or
        // for every layers of weights
        number_neurons += net.network_struct[i + 1];
    }
    
    // the outpouts of each neurons are stored here:
    let mut vec_predictions_neurons: Vec<Vec<f32>> = Vec::new();

    // This block order the datas, predictions, by neurons and by layers(but all the layers are in one vector), backward
    let mut neurons_iterator_reverse: usize = 0;
    for i in 0..number_neurons {
        // for every neurons of the network
        
        // Create a vector to store the outputs of the neuron "neurons_iterator_reverse"
        let mut vec_predictions_neuron: Vec<f32> = Vec::new();

        //for y in 0..inputs.len() {
        for y in 0..observed_values[0].len() {
            // for every propagation
            neurons_iterator_reverse = network_predictions.len() - 1 - i - y * number_neurons;

            vec_predictions_neuron.push(network_predictions[neurons_iterator_reverse]);
        }

        vec_predictions_neurons.push(vec_predictions_neuron)
    }

    // The predictions are now ordered by neurons and layers, now we need to seperate those layers backward
    let mut counter_layers: usize = 0;
    let mut counter_neurons: usize = 0;

    for i in 0..net.network_struct.len() - 1 {
        // for every layers but the first one
        counter_neurons = net.network_struct.len() - 1 - i;

        println!("\nAt the layer {}:", counter_neurons);
        
        for y in 0..net.network_struct[counter_neurons] {
            println!("The neuron number {} give these outputs: {:?}", net.network_struct[counter_neurons] - y, vec_predictions_neurons[y + counter_layers]);
            //let (mut trained_weight, mut trained_bias): (f32, f32) = grad_descent(net, observed_values[y], vec_predictions_neurons[y + counter_layers], )
            //pub fn back_prop(net: &Network, observed_values: &Vec<Vec<f32>>, network_predictions: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> usize {
        }

        counter_layers += net.network_struct[counter_neurons];
    }
    
    0
}

//pub mod grad_descent;
