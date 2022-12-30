pub fn random(column: usize, row: usize, a: f32, b: f32) -> Vec<f32> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let matrix_length: usize = column * row;
    let mut matrix: Vec<f32> = vec![0.0; matrix_length];

    for i in 0..matrix_length {
        let rand: f32 = rng.gen_range(a..=b);
        matrix[i] = rand;
    }
    
    return matrix;
}

pub fn normal_dis(column: usize, row: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = 1.0;

    let matrix: Vec<f32> = random(column, row, a, b);

    return matrix;
}

pub fn uniform_dis(column: usize, row: usize) -> Vec<f32> {
    let a: f32 = -1.0 / (column as f32).sqrt();
    let b: f32 = 1.0 / (column as f32).sqrt();
    // .sqrt() only works with float
    let matrix: Vec<f32> = random(column, row, a, b);
    
    return matrix;
}

pub fn xav_gro_normal_dis(column: usize, row: usize, fan_out: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (column as f32) + (fan_out as f32)).sqrt();
    let matrix: Vec<f32> = random(column, row, a, b);
    
    return matrix;
}

pub fn xav_gro_uniform_dis(column: usize, row: usize, fan_out: usize) -> Vec<f32> {
    let a: f32 = -(6_f32.sqrt()) / ((column as f32) + (fan_out as f32)).sqrt();
    let b: f32 = 6_f32.sqrt() / ((column as f32) + (fan_out as f32)).sqrt();
    let matrix: Vec<f32> = random(column, row, a, b);
    
    return matrix;
}

pub fn he_normal_dis(column: usize, row: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (column as f32)).sqrt();
    let matrix: Vec<f32> = random(column, row, a, b);
    
    return matrix;
}

pub fn he_uniform_dis(column: usize, row: usize) -> Vec<f32> {
    let a: f32 = -(6.0 / (column as f32)).sqrt();
    let b: f32 = (6.0 / (column as f32)).sqrt();
    let matrix: Vec<f32> = random(column, row, a, b);

    return matrix;
}