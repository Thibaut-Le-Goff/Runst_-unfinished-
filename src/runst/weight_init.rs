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
    let b: f32 = (2.0 / (layer_n as f32) + (layer_n2 as f32)).sqrt();
    let matrix: Vec<f32> = random(layer_n, layer_n1, a, b);
    
    return matrix;
}

pub fn xav_gro_uniform_dis(layer_n: usize, layer_n1: usize, layer_n2: usize) -> Vec<f32> {
    let a: f32 = -(6_f32.sqrt()) / ((layer_n as f32) + (layer_n2 as f32)).sqrt();
    let b: f32 = 6_f32.sqrt() / ((layer_n as f32) + (layer_n2 as f32)).sqrt();
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

/* 
pub fn which_dis(dis: &str) -> (usize, Vec<(Box<dyn Fn(usize, usize)->Vec<f32>>, &str)>) {
    ///// list of the available distribution /////
    type FunType = Box<dyn Fn(usize, usize)->Vec<f32>>;
    // must be of the type of the output and input of the function to call

    // linking the functions(FunType) to their name(&str):
    let mut dist_list: Vec<(FunType, &str)> = Vec::new();
    dist_list.push((Box::new(uniform_dis), "uniform_dis"));
    dist_list.push((Box::new(normal_dis), "normal_dis"));
    //dist_list.push((Box::new(xav_gro_normal_dis), "xav_gro_normal_dis"));
    //dist_list.push((Box::new(xav_gro_uniform_dis), "xav_gro_uniform_dis"));
    dist_list.push((Box::new(he_normal_dis), "he_normal_dis"));
    dist_list.push((Box::new(he_uniform_dis), "he_uniform_dis"));
    
    let mut function_to_call_i: usize = 0;
    
    for i in 0..dist_list.len() {
        if dist_list[i].1 == dis {
            function_to_call_i = i;
        }
    }

   return (function_to_call_i, dist_list);
}
*/