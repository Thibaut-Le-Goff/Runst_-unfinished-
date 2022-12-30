///////////////////// Network initialisation //////////////////////////
pub fn net_init(network_struct: &Vec<usize>, distrib: &str) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let mut weights_tensor: Vec<Vec<f32>> = Vec::new();
    let mut bias_matrix: Vec<Vec<f32>> = Vec::new();

    let mut next_layer: usize;

    // create the weights and the bias between the layers:
    for i in 0..network_struct.len() - 1 {
        // the things between x things is equal to x - 1

        next_layer = i + 1;

        // in this line :
        let weight_matrix: Vec<f32> = weight_init::normal_dis(network_struct[i], network_struct[next_layer]);
        // We want to call a function for the initialisation of the weights, but with a variable.
        // One of the solution find is here :
        // https://users.rust-lang.org/t/call-module-function-by-variable/60176

        // here is the block of code :
        /*
        use std::collections::HashMap;

        pub fn my_fn_0(x:u8, y:f32) -> String{
            "Hello".to_string()
        }

        pub fn my_fn_1(x:u8, y:f32) -> String{
            "Bye".to_string()
        }

        pub fn main() {
            type VoidFnPtr = Box<dyn Fn(u8, f32)->String>;
            let mut functions = HashMap::<String, VoidFnPtr>::new();
            functions.insert("my_fn_0".to_string(), Box::new(my_fn_0));
            functions.insert("my_fn_1".to_string(), Box::new(my_fn_1));

            let res = functions.get("my_fn_0").unwrap()(1, 2.0);
            println!("{}", res);

            let res = functions.get("my_fn_1").unwrap()(1, 2.0);
            println!("{}", res);
        }
        */

        // note :
        // HasMap seems more optimezed than vec for high number of iterations and big hashmap :
        // https://gist.github.com/daboross/976978d8200caf86e02acb6805961195
        // In our case using a vector is better:

        /*
        pub fn my_fn_0() -> String{
            "Hello".to_string()
        }

        pub fn my_fn_1() -> String{
            "Bye".to_string()
        }

        pub fn main() {
            type FunType = Box<dyn Fn()->String>;
            // must be of the type of the output of the function to call

            // linking the functions(FunType) to their name(&str):
            let mut functions: Vec<(FunType, &str)> = Vec::new();
            functions.push((Box::new(my_fn_0), "my_fn_0"));
            functions.push((Box::new(my_fn_1), "my_fn_1"));
            
            let wanted_fun: &str = "my_fn_1";
            let mut function_to_call: String = "".to_string();
            // must be of the type of the output of the function to call
            
            for i in 0..functions.len() {
                if functions[i].1 == wanted_fun {
                    function_to_call = functions[i].0();
                    // if arguments are needed they have to be put in the ()
                }
            }
            
            println!("{}", function_to_call);
        }
        */

        // a method to find exists :
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find 


        weights_tensor.push(weight_matrix);

        let bias_vector: Vec<f32> = vec![0.0; network_struct[next_layer]];
        bias_matrix.push(bias_vector);
    }

    return (weights_tensor, bias_matrix);
}


///////////////// for the propagation ///////////////////////////
pub fn multiply(matrix: &Vec<f32>, vector: &Vec<f32>) -> Vec<f32> {

    let result_lenght: usize = matrix.len() / vector.len();
    let mut result: Vec<f32> = vec![0.0; result_lenght];
    let mut counter: usize = 0;
    let mut x: f32;

    for i in 0..result_lenght {
        x = 0.0;
        for j in 0..vector.len() {
            x = (&vector[j] * &matrix[counter]) + x;
            counter = counter + 1;
        }
        result[i] = x;
    }
    return result;
}

pub fn bias_addition(vector: &Vec<f32>, bias: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = vector[i] + bias[i];
    }
    return result;
}

/* 
pub fn loss(received: &Vec<f32>, expected: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; expected.len()];

    for i in 0..result.len() {
        let res: f32 = &received[i] - &expected[i];
        result[i] = res * res;
    }
    return result;
}
*/

pub mod activ_fun;
pub mod weight_init;