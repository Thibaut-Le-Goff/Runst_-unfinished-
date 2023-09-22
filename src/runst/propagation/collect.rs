// linking the functions(FunType) to their name(&str):

use crate::runst::propagation::activ_fun::*;
use crate::runst::objects::FunType;

//pub fn activ_fun() -> Vec<(FunType, &'static str)> {
pub fn activ_fun() -> [(FunType, &'static str); 7] {

    /*
    //let mut activ_fun_list: Vec<(FunType, &str)> = Vec::new();
    let mut activ_fun_list: Vec<(FunType, &str)> = vec![
        (Box::new(none), "none"),
        (Box::new(relu), "relu"), 
        (Box::new(leaky_relu), "leaky_relu"),
        (Box::new(silu), "silu"),
        (Box::new(softplus), "softplus"),
        (Box::new(sigmoid), "sigmoid"),
        (Box::new(softmax), "softmax")
    ];    
    */

    let mut activ_fun_list: [(FunType, &'static str); 7] = [
        (Box::new(none), "none"),
        (Box::new(relu), "relu"), 
        (Box::new(leaky_relu), "leaky_relu"),
        (Box::new(silu), "silu"),
        (Box::new(softplus), "softplus"),
        (Box::new(sigmoid), "sigmoid"),
        (Box::new(softmax), "softmax")
    ];

    activ_fun_list
}
