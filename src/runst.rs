////// Struct with the settings of the neural network //////
pub struct Network {
    pub network_struct: Vec<usize>,
    pub distrib: &'static str,
       
    pub hidden_activ_fun: &'static str,
    pub out_activ_fun: &'static str,
}


/* 
pub struct DataSet {
    pub inputs: Vec<Vec<f32>>,
    pub observed_values: Vec<Vec<f32>>,
}
*/

pub type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;


pub struct BackPropData {
    pub true_counter: &'static mut usize,
    pub layers_iterator: &'static mut usize,
    pub weight_bias_trouve: &'static mut Vec<bool>,
}

/* 
fn weights_update(true_counter: &mut usize, weight_bias_trouve: &mut Vec<bool>, 
    weights: &mut [f32], inputs: &Vec<Vec<f32>>, observed_values: &Vec<Vec<f32>>, 
    network_predictions: &Vec<Vec<f32>>)
*/



pub mod net_init;
pub mod propagation;
pub mod back_prop;
pub mod calculations;