///////////////// for the propagation ///////////////////////////
//use crate::runst::propagation::choose;
use crate::runst::calculations;
use crate::runst::Network;
use crate::runst::FunType;

pub fn propagation(net: &Network, inputs: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> Vec<f32> {
//pub fn propagation(net: &Network, inputs: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    
    //type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

    let activ_fun_list: [(FunType, &'static str); 7] = collect::activ_fun();

    let (hidden_activ_fun_i, out_activ_fun_i): (usize, usize) = choose::activ_fun(&net.hidden_activ_fun, &net.out_activ_fun, &activ_fun_list);

    /////////////// The propagation really start here //////////////
    
    let mut network_predictions: Vec<f32> = Vec::new();
    //let mut network_predictions: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..inputs.len() {
        // for each pair of datas in the data set
        // can be done in parrallel \o/
        println!("Propagation numéro {} des données d'entrée :", i + 1);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let mut neuron_out_to_input: Vec<f32> = inputs[i].clone();
        println!("{:?}\n", &neuron_out_to_input);

        for y in 0..weights.len() - 1 {
            // for each layers of weights but the last :

            // input :
            // &weights[y], &neuron_out_to_input, &bias[y]

            // output :
            // neuron_out_to_input

            //neuron_out_to_input = pass_through::forward(&neuron_out_to_input, &weights[y], &bias[y], &activ_fun_list, );

            println!("\nDans les neurones de la couche {} :", y + 2);
            let neuron_sum: Vec<f32> = calculations::multiplication::mat_vec(&weights[y], &neuron_out_to_input);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = calculations::addition::vec_vec(&neuron_sum, &bias[y]);
            println!("Après l'ajout des biais :");
            println!("{:?}\n", &neuron_sum_bias);

            neuron_out_to_input = activ_fun_list[hidden_activ_fun_i].0(&neuron_sum_bias);

                // We need two time the same output:
                // - one for the next iteration
                // - one for the output

            


            println!("Après le passage dans la function d'activation :");
            println!("{:?}\n", &neuron_out_to_input);

            /* 
            //I don't need the predictions of the hidden layers
            let neurons_outputs: Vec<f32> = neuron_out_to_input.clone();
                 
            for j in 0..neurons_outputs.len() {
                    network_predictions.push(neurons_outputs[j]);
            }   
            */
        }

        println!("\nDans les neurones de la dernière couche :");
        let neuron_sum: Vec<f32> = calculations::multiplication::mat_vec(&weights[weights.len() -1], &neuron_out_to_input);
        println!("Après La multiplication :");
        println!("{:?}\n", &neuron_sum);
        let neuron_sum_bias: Vec<f32> = calculations::addition::vec_vec(&neuron_sum, &bias[bias.len() -1]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", &neuron_sum_bias);

        let neurons_outputs = activ_fun_list[out_activ_fun_i].0(&neuron_sum_bias);
        



        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", &neurons_outputs);

            //network_predictions.push(neurons_outputs);

        for j in 0..neurons_outputs.len() {
            network_predictions.push(neurons_outputs[j]);
        }
    }

    network_predictions
}

pub mod activ_fun;
pub mod choose;
pub mod collect;