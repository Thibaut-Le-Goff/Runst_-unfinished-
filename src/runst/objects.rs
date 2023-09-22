////// Struct with the settings of the neural network //////
use crate::runst;

pub struct Network {
    pub network_struct: Vec<usize>,
    pub distrib: &'static str,
       
    pub hidden_activ_fun: &'static str,
    pub out_activ_fun: &'static str
}

impl Network {
    pub fn new(network_struct: Vec<usize>, distrib: &'static str, hidden_activ_fun: &'static str, out_activ_fun: &'static str) -> Network {
        Network { 
            network_struct,
            distrib,
            hidden_activ_fun,
            out_activ_fun
         }
    }
    fn net_init(&self) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        let (mut weights, mut bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = runst::net_init::net_init(&self);
    
        (weights, bias)
    }
}

pub struct NetworkPara {
    pub weights: Vec<Vec<f32>>,
    pub bias: Vec<Vec<f32>>
}

impl NetworkPara {
    pub fn new(network: &Network) -> NetworkPara {
        let (mut weights, mut bias): (Vec<Vec<f32>>, Vec<Vec<f32>>) = network.net_init();

        NetworkPara{weights, bias}
    }
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