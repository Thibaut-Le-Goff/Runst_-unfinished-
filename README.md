 <center><h2><ins>Runst, a project to learn neural network and Rust</ins></h2></center>

As Runst is a big project, I must have a deep understanding of what I am doing and why.
And I learned through my school projects that explaining what I did force me to tackle subjects I skimmed over when I was doing those and have a better understanding of my project.

My goal is to create a module that will make easier the creation of a neural network.

<h5><ins>What is a neural network</ins></h5>
A neural network can be separated in three main area:
<p align="center">
    <img src="nn1.png" width="500"/>
</p>

- Informations relative to what we are looking for are put in the input layer; 
- The hidden layer do all the calculations to get a result with the information put in the input layer;
- The output layer will show the result and compare it to the result that was expected.

Each of those layers have a certain number of nods called neurones who are connected to the neurones of the adjacents layers by what is called weights.

To make the neural network work the neurones in the input layer will send the informations they hold to the neurones in the hidden layer through the weight. But the weight will have an effect on the informations sent, it will multiply it by the value of the weight:

Then the neurones in the hidden layer will make the sum of the informations sent by the neurones of the input layer multiplied by the weights:
<p align="center">
    <img src="nn2.png" width="300"/>
</p>

In this example, the first neurone sends the number 3 through a weight with the value 5, which means that the neurone in the hidden layer will take 3 * 5 = 15.

But, the neurone will also take the information sent by the second neurone of the input layer, 7, with the weight of the value 10, which makes 70.

The neurone will make the sum of 15 and 70 which is 85, the final value that will use the neurone with something called the activation function, a function that will determine the value that will be sent to the neuron of the output layer.

This can be sum up with matrices:
<p align="center">
    <img src="nn3.png" width="250"/>
</p>

In this example the matrice [5 10] is multiplied by the vector [3 7] , which give us 85.

Matrices can help us to calculate all the neurones in the hidden layer at the same time:
<p align="center">
    <img src="nn4.png" width="600"/>
</p>

With the result in the vector [33 85 53] , the neurones in the hidden layer can calculate values they will send to the next layer with a activation function.

But, before going further with the concepts of activation function, I want to explain my code.

<h5><ins>My code</ins></h5>
<ins>Weight initialisation:</ins> 

First I have to initialise the weitghs and they can be created in a matrix, I created à function that will take as input the number of column and the number of row and will out put the matrix which is a vecor of vectors of floats in 64 bits:
```rust
pub fn uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
```
We come to some things I didn't mention, the weights are initialised randomly betwen two numbers and those number are determined by the type of initialisation we choose, here I choose an uniform distribution.

The formula to detemine the two numbers are:
<p align="center">
    <img src="for1.png" width="300"/>
</p>

Here fan_in is equal to the number of weights coming toward one neuron, the number of neurons in the previous layer in short, for example:

<p align="center">
    <img src="nn5.png" width="600"/>
</p>
There is also fan_out which is equal to the number of weights coming from one neuron, the number of neurons in the next layer:
<p align="center">
    <img src="nn6.png" width="500"/>
</p>

In my code it's make:
```rust
    let a: f64 = -1.0 / (column as f64).sqrt();
    let b: f64 = 1.0 / (column as f64).sqrt();
    // .sqrt() only works with float
```
Here I use the number of columns in the matrix I want to create because in form of matrices, the number of weights going toward one neuron, fan_in, is the number of columns in the matrix of the weights.

After the calculation of a and b, I put the number of columns and rows in the matrix I want to create, a and b:

```rust
    let matrix: Vec<Vec<f64>> = random(column, row, a, b);
```

The function


<ins>The multiplication of a matrix by a vector:</ins> \
