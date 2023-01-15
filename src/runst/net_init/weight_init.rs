pub fn random(layer_n: usize, layer_n1: usize, a: f32, b: f32) -> Vec<f32> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let matrix_length: usize = layer_n * layer_n1;
    //let matrix_length: usize = columtn * rotw;
    let mut matrix: Vec<f32> = vec![0.0; matrix_length];

    for i in 0..matrix_length {
        let rand: f32 = rng.gen_range(a..=b);
        matrix[i] = rand;
    }
    
    return matrix;
}

pub fn normal_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = 1.0;

    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);

    return matrix;
}

pub fn uniform_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = -1.0 / (layer_n as f32).sqrt();
    let b: f32 = 1.0 / (layer_n as f32).sqrt();
    // .sqrt() only works with float
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    return matrix;
}

pub fn xav_gro_normal_dis(layer_n: usize, layer_n1: usize, layer_n2: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (layer_n  + layer_n2) as f32).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    return matrix;
}

pub fn xav_gro_uniform_dis(layer_n: usize, layer_n1: usize, layer_n2: usize) -> Vec<f32> {
    let a: f32 = -(6.0 / (layer_n  + layer_n2) as f32).sqrt();
    let b: f32 = (6.0 / (layer_n  + layer_n2) as f32).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    return matrix;
}

pub fn he_normal_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (layer_n as f32)).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    return matrix;
}

pub fn he_uniform_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = -(6.0 / (layer_n as f32)).sqrt();
    let b: f32 = (6.0 / (layer_n as f32)).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);

    return matrix;
}