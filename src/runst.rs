///////////////////// Network initialisation //////////////////////////
pub fn net_init(network_struct: &Vec<usize>, distrib: &str) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {

    ///// list of the available functions /////
    type FunType = Box<dyn Fn(usize, usize)->Vec<f32>>;
    // must be of the type of the output and input of the function to call

    // linking the functions(FunType) to their name(&str):
    let mut functions: Vec<(FunType, &str)> = Vec::new();
    functions.push((Box::new(weight_init::uniform_dis), "uniform_dis"));
    functions.push((Box::new(weight_init::normal_dis), "normal_dis"));
    
    let mut function_to_call_i: usize = 0;
    
    for i in 0..functions.len() {
        if functions[i].1 == distrib {
            function_to_call_i = i;
        }
    }
    ///////////////////////////////////////////

    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut next_layer: usize;

    // create the weights and the bias between the layers:
    for i in 0..network_struct.len() - 1 {
        // the things between x things is equal to x - 1
        next_layer = i + 1;

        let weight_matrix: Vec<f32> = functions[function_to_call_i].0(network_struct[i], network_struct[next_layer]);
        weights_tensor.push(weight_matrix);

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