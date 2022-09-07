pub fn relu(vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        };
    }
    return result;
}

pub fn leaky_relu(vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = 0.01 * vector[i];
        };
    }
    return result;
}

pub fn silu(vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = sigmoid(vector[i]);
        };
    }
    return result;
}

pub fn sigmoid(weight: f64) -> f64 {
    let result: f64 = 1.0 / (1.0 + (-1.0 * weight).exp());
    return result;
}
/* 
pub fn silu(vector: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = sigmoid(vector[i]);
        };
    }
    return result;
}*/