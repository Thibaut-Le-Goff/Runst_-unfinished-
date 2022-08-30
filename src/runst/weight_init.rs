pub fn random(column: usize, row: usize, a: f64, b: f64) -> Vec<Vec<f64>> {
    use rand::{thread_rng, Rng};
    use std::thread;
        
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
    let mut rng = thread_rng();

    for i in 0..= row - 1 {
        //thread::scope(|thread| {
            //thread.spawn(|| {
        for j in 0..= column - 1 {
            let rand: f64 = rng.gen_range(a..=b);
            matrix[i][j] = rand;
        }
            //});
        //});
    }
    return matrix;
}

pub fn normal_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
    let a: f64 = 0.0;
    let b: f64 = 1.0;
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);

    return matrix;
}

pub fn uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
    let a: f64 = -1.0 / (column as f64).sqrt();
    let b: f64 = 1.0 / (column as f64).sqrt();
    // .sqrt() only works with float
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);
    
    return matrix;
}

pub fn xav_gro_normal_dis(column: usize, row: usize, fan_out: usize) -> Vec<Vec<f64>> {
    let a: f64 = 0.0;
    let b: f64 = (2.0 / (column as f64) + (fan_out as f64)).sqrt();
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);
    
    return matrix;
}

pub fn xav_gro_uniform_dis(column: usize, row: usize, fan_out: usize) -> Vec<Vec<f64>> {
    let a: f64 = -(6_f64.sqrt()) / ((column as f64) + (fan_out as f64)).sqrt();
    let b: f64 = 6_f64.sqrt() / ((column as f64) + (fan_out as f64)).sqrt();
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);
    
    return matrix;
}

pub fn he_normal_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
    let a: f64 = 0.0;
    let b: f64 = (2.0 / (column as f64)).sqrt();
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);
    
    return matrix;
}

pub fn he_uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
    let a: f64 = -(6.0 / (column as f64)).sqrt();
    let b: f64 = (6.0 / (column as f64)).sqrt();
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);

    return matrix;
}
