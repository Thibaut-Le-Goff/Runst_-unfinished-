//Hidden layer activation functions:
pub fn relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        };
    }
    result
}

pub fn leaky_relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = 0.01 * vector[i];
        };
    }
    result
}

pub fn silu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = vector[i] * (1.0 / (1.0 + (-vector[i]).exp()));
    }
    result
}

pub fn softplus(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = (1.0 + (vector[i].exp())).ln();
    }
    result
}

// Last layer activation functions:
pub fn sigmoid(vector: &Vec<f32>) -> Vec<f32> {
    
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = 1.0 / (1.0 + (-vector[i]).exp());
    }
    result
}

pub fn none(vector: &Vec<f32>) -> Vec<f32> {
    
    return vector.to_vec();
}

pub fn softmax(vector: &Vec<f32>) -> Vec<f32> {
    /* 
    softmax calculate for each neuron, in the output layer,
    the probability that the information it indicates is the 
    right.

    ex :
    In a neural network which has to know how to differentiate
    the picture of a cat from the one of a dog, we can see 
    the results like: The picture have 70% to be one of a 
    dog and 30% of a cat.

    To calculate this function, for each neuron, we have to
    calculate the exponential of the neuron and divide it by
    the sum of the exponent of the neurons.
    */

    let mut sum: f32 = 0.0;
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        sum = sum + vector[i].exp();
    }

    for i in 0..vector.len() {
        result[i] = vector[i].exp() / sum;
    }
    result
}
