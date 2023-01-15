///////////////// for the propagation ///////////////////////////
use crate::runst::Network;

pub fn propagation(net: &Network, inputs: &Vec<Vec<f32>>, weights_tensor: &Vec<Vec<f32>>, bias_matrix: &Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    
    //////////////// Select the activation functions wanted ///////////
    type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

    // linking the functions(FunType) to their name(&str):
    let mut activ_fun_list: Vec<(FunType, &str)> = Vec::new();
    activ_fun_list.push((Box::new(activ_fun::none), "none"));
    activ_fun_list.push((Box::new(activ_fun::relu), "relu"));
    activ_fun_list.push((Box::new(activ_fun::leaky_relu), "leaky_relu"));
    activ_fun_list.push((Box::new(activ_fun::silu), "silu"));
    activ_fun_list.push((Box::new(activ_fun::softplus), "softplus"));
    activ_fun_list.push((Box::new(activ_fun::sigmoid), "sigmoid"));
    activ_fun_list.push((Box::new(activ_fun::softmax), "softmax"));

    let mut hidden_activ_fun_i: usize = activ_fun_list.len();
    let mut out_activ_fun_i: usize = activ_fun_list.len();

    for i in 0..activ_fun_list.len() {
        if activ_fun_list[i].1 == net.hidden_activ_fun {
            hidden_activ_fun_i = i;
        }
        // not else if because the same fun can be use in the
        // hidden and out layers
        if activ_fun_list[i].1 == net.out_activ_fun {
            out_activ_fun_i = i;
        }
    }

    /*
    Note to myself:
        send an error message if:
            hidden_activ_fun_i and/or out_activ_fun_i are still equal to 
            activ_fun_list.len() in this area
        
        Because the activation function wanted is 
        not available
    */

    /////////////// The propagation really start here //////////////
    
    let mut network_outputs_sum_bias: Vec<Vec<f32>> = Vec::new();
    let mut network_outputs_neurons: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..inputs.len() {
        // for each pair of datas in the data set
        println!("Propagation numéro {} des données d'entrée :", i + 1);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let mut neuron_out: Vec<f32> = inputs[i].clone();
        println!("{:?}\n", &neuron_out);

        for y in 0..net.network_struct.len() - 1 {
            // for each hiden layer + the output layers:
            // - 1 : avoid the first layer

            println!("\nDans les neurones de la couche {} :", y + 2);
            let neuron_sum: Vec<f32> = calculations::multiply(&weights_tensor[y], &neuron_out);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = calculations::bias_addition(&neuron_sum, &bias_matrix[y]);
            println!("Après l'ajout des biais :");
            println!("{:?}\n", &neuron_sum_bias);

            if y == net.network_struct.len() - 2 {
                // if this is the last layer (the last iteration)
                // last iteration is equal to net.network_struct.len() - 2
                // because :
                // - 1 : the iteration started at 0
                // - 1 : avoid the first layer exactly like above

                neuron_out = activ_fun_list[out_activ_fun_i].0(&neuron_sum_bias);

                // We need two time the same output:
                // - one for the next iteration
                // - one for the output
                let neuron_activ_fun: Vec<f32> = neuron_out.clone();
                
                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neuron_out);

                network_outputs_neurons.push(neuron_activ_fun);

            } else {
                neuron_out = activ_fun_list[hidden_activ_fun_i].0(&neuron_sum_bias);
                let neuron_activ_fun: Vec<f32> = neuron_out.clone();

                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neuron_out);

                network_outputs_neurons.push(neuron_activ_fun);
            }
            network_outputs_sum_bias.push(neuron_sum_bias);
        }
    }
    return (network_outputs_sum_bias, network_outputs_neurons);
}
pub mod activ_fun;
pub mod calculations;