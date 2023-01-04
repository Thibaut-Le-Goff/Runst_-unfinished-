///////////////////// Network initialisation //////////////////////////
pub fn net_init(network_struct: &Vec<usize>, distrib: &str) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {


    
    ///// list of the available functions /////
    type FunType = Box<dyn Fn(usize, usize)->Vec<f32>>;
    // must be of the type of the output and input of the function to call

    // linking the functions(FunType) to their name(&str):
    let mut dist_list: Vec<(FunType, &str)> = Vec::new();
    dist_list.push((Box::new(weight_init::uniform_dis), "uniform_dis"));
    dist_list.push((Box::new(weight_init::normal_dis), "normal_dis"));
    dist_list.push((Box::new(weight_init::he_normal_dis), "he_normal_dis"));
    dist_list.push((Box::new(weight_init::he_uniform_dis), "he_uniform_dis"));

    type FunTypeXavGro = Box<dyn Fn(usize, usize, usize)->Vec<f32>>;
    // had to create another type because the fun take 3 usize and not 2
    let mut dist_list_xav_gro: Vec<(FunTypeXavGro, &str)> = Vec::new();

    let mut function_to_call_i: usize = dist_list.len() + 1;
    // must be higher than dist_list.len() in order to keep the value unchange
    // if the wanted dist is not found.
    
    for i in 0..dist_list.len() {
        if dist_list[i].1 == distrib {
            function_to_call_i = i;
        }
    }
    if function_to_call_i == dist_list.len() + 1 {
        // the fact the value didn't change tell  
        // the dist wanted has not been found

        //type FunTypeXavGro = Box<dyn Fn(usize, usize, usize)->Vec<f32>>;
        // had to create another type because the fun take 3 usize and not 2
        //let mut dist_list_xav_gro: Vec<(FunTypeXavGro, &str)> = Vec::new();
        dist_list_xav_gro.push((Box::new(weight_init::xav_gro_normal_dis), "xav_gro_normal_dis"));
        dist_list_xav_gro.push((Box::new(weight_init::xav_gro_uniform_dis), "xav_gro_uniform_dis"));

        for i in 0..dist_list_xav_gro.len() {
            if dist_list_xav_gro[i].1 == distrib {
                function_to_call_i = i;
            }
        }
    }
    
    ///////////////////////////////////////////
    
    //let (function_to_call_i, dist_list) = weight_init::which_dis(distrib);

    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut next_layer: usize;
    let mut fan_out: usize;

    // create the weights and the bias between the layers:
    for i in 0..network_struct.len() - 1 {
        // the number of things between x things is equal to x - 1
        next_layer = i + 1;

        if function_to_call_i == dist_list.len() + 1 {
            fan_out = next_layer + 1;

            let weight_matrix: Vec<f32> = dist_list_xav_gro[function_to_call_i].0(network_struct[i], network_struct[next_layer], network_struct[fan_out]);
            weights_tensor.push(weight_matrix);
            
        } else {
            let weight_matrix: Vec<f32> = dist_list[function_to_call_i].0(network_struct[i], network_struct[next_layer]);
            weights_tensor.push(weight_matrix);
        }

        let bias_vector: Vec<f32> = vec![0.0; network_struct[next_layer]];
        bias_matrix.push(bias_vector);
    }

    return (weights_tensor, bias_matrix);
}


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




pub mod activ_fun;
pub mod weight_init;