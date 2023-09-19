use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub fn random(layer_n: usize, layer_n1: usize, a: f32, b: f32) -> Vec<f32> {

    let matrix_length: usize = layer_n * layer_n1;
    let mut matrix: Vec<f32> = vec![0.0; matrix_length];

    matrix.par_iter_mut().for_each(|(weight)| {
        *weight = thread_rng().gen_range(a..=b);
    });
    
    matrix
}

pub fn normal_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = 1.0;

    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);

    matrix
}

pub fn uniform_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = -1.0 / (layer_n as f32).sqrt();
    let b: f32 = 1.0 / (layer_n as f32).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    matrix
}

pub fn xav_gro_normal_dis(layer_n: usize, layer_n1: usize, layer_n2: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (layer_n  + layer_n2) as f32).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    matrix
}

pub fn xav_gro_uniform_dis(layer_n: usize, layer_n1: usize, layer_n2: usize) -> Vec<f32> {
    let a: f32 = -(6.0 / (layer_n  + layer_n2) as f32).sqrt();
    let b: f32 = (6.0 / (layer_n  + layer_n2) as f32).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    matrix
}

pub fn he_normal_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = 0.0;
    let b: f32 = (2.0 / (layer_n as f32)).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    matrix
}

pub fn he_uniform_dis(layer_n: usize, layer_n1: usize) -> Vec<f32> {
    let a: f32 = -(6.0 / (layer_n as f32)).sqrt();
    let b: f32 = (6.0 / (layer_n as f32)).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);

    matrix
}