///////////////////// Network initialisation //////////////////////////
use crate::runst::Network;

pub fn net_init(net: &Network) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    ///// list of the available functions /////
    type FunType = Box<dyn Fn(usize, usize)->Vec<f32>>;
    // must be of the type of the output and inputs of the function to call

    // linking the functions(FunType) to their name(&str):
    let mut dist_list: Vec<(FunType, &str)> = Vec::new();
    dist_list.push((Box::new(weight_init::uniform_dis), "uniform_dis"));
    dist_list.push((Box::new(weight_init::normal_dis), "normal_dis"));
    dist_list.push((Box::new(weight_init::he_normal_dis), "he_normal_dis"));
    dist_list.push((Box::new(weight_init::he_uniform_dis), "he_uniform_dis"));

    let mut dist_list_xav_gro: Vec<(FunTypeXavGro, &str)> = Vec::new();
    type FunTypeXavGro = Box<dyn Fn(usize, usize, usize)->Vec<f32>>;
    // had to create another type because the fun take 3 usize and not 2

    let mut function_to_call_i: usize = dist_list.len();
    // must be, at least, equal to dist_list.len() in order to keep the 
    // value unchange if the wanted dist is not found.

    for i in 0..dist_list.len() {
        if dist_list[i].1 == net.distrib {
            function_to_call_i = i;
        }
    }

    if function_to_call_i == dist_list.len() {
        // the fact the value didn't change tell  
        // the dist wanted has not been found

        dist_list_xav_gro.push((Box::new(weight_init::xav_gro_normal_dis), "xav_gro_normal_dis"));
        dist_list_xav_gro.push((Box::new(weight_init::xav_gro_uniform_dis), "xav_gro_uniform_dis"));

        for i in 0..dist_list_xav_gro.len() {
            if dist_list_xav_gro[i].1 == net.distrib {
                function_to_call_i = i;
            }
        }
    }

    /*
    Note to myself:
        send an error message if:
            function_to_call_i is still equal to 
            dist_list.len() in this area
        
        Because the distribution wanted is 
        not available
    */
    
    //////////////////// initialisation of the weights and bias ///////////////////////
    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut layer_n1: usize;
    let mut layer_n2: usize;

    // create the weights and the bias between the layers:
    for i in 0..net.network_struct.len() - 1 {
        // the number of things between x things is equal to x - 1
        layer_n1 = i + 1;

        if dist_list_xav_gro.len() > 0 && i == net.network_struct.len() - 2 {
            // if the dist_list_xav_gro lenght is higher than 0
            // that mean a xav_gro fun is wanted
            // and
            // if this is the last layer :
            // -1 because last element of a vec(or array) is vec.len() - 1
            // -1 because the number of things between x things is equal to x - 1

            let weight_matrix: Vec<f32> = dist_list_xav_gro[function_to_call_i].0(net.network_struct[i], net.network_struct[layer_n1], 0);
            weights_tensor.push(weight_matrix);
            
        } else if dist_list_xav_gro.len() > 0 && i < net.network_struct.len() - 2 {
            // if the dist_list_xav_gro lenght is higher than 0
            // that mean a xav_gro fun is wanted
            // and
            // this is not the last layer
            layer_n2 = layer_n1 + 1;

            let weight_matrix: Vec<f32> = dist_list_xav_gro[function_to_call_i].0(net.network_struct[i], net.network_struct[layer_n1], net.network_struct[layer_n2]);
            weights_tensor.push(weight_matrix);
        
        } else {
            let weight_matrix: Vec<f32> = dist_list[function_to_call_i].0(net.network_struct[i], net.network_struct[layer_n1]);
            weights_tensor.push(weight_matrix);
        };

        let bias_vector: Vec<f32> = vec![0.0; net.network_struct[layer_n1]];
        bias_matrix.push(bias_vector);
    }

    return (weights_tensor, bias_matrix);
}
pub mod weight_init;