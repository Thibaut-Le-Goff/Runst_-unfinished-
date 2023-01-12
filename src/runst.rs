////// Struct with the settings of the neural network //////
pub struct Network {
    pub network_struct: Vec<usize>,
    pub distrib: String,
       
    pub hidden_activ_fun: String,
    pub out_activ_fun: String,
}

///////////////////// Network initialisation //////////////////////////
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














///////////////// for the propagation ///////////////////////////
pub fn multiply(matrix: &Vec<f32>, vector: &Vec<f32>) -> Vec<f32> {

    let result_lenght: usize = matrix.len() / vector.len();
    let mut result: Vec<f32> = vec![0.0; result_lenght];
    let mut counter: usize = 0;
    let mut x: f32;

    for i in 0..result_lenght {
        x = 0.0;
        for j in 0..vector.len() {
            x = (&vector[j] * &matrix[counter]) + x;
            counter = counter + 1;
        }
        result[i] = x;
    }
    return result;
}

pub fn bias_addition(vector: &Vec<f32>, bias: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = vector[i] + bias[i];
    }
    return result;
}

pub fn propagation(net: &Network, inputs: &Vec<f32>, weights_tensor: &Vec<Vec<f32>>, bias_matrix: &Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    
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
        let mut neuron_out: Vec<f32> = vec![inputs[i]; net.network_struct[0]];
        println!("{:?}\n", &neuron_out);

        for y in 0..net.network_struct.len() - 1 {
            // for each hiden layer + the output layers:
            // - 1 : avoid the first layer

            println!("\nDans les neurones de la couche {} :", y + 2);
            let neuron_sum: Vec<f32> = multiply(&weights_tensor[y], &neuron_out);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = bias_addition(&neuron_sum, &bias_matrix[y]);
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