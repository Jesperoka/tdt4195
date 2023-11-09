<h1 align="center">Assignment 5</h1>

<!-- *Note: Allow some time for the GIFs to load if viewing HTML or [GitHub Markdown versions](https://github.com/Jesperoka/tdt4195/tree/assignment_5) of report.<br> -->
<!-- PDF version does not support GIFs, so the other options are recommended.* -->

<br>

<h2 align="center">Part 1: Convolutional Neural Networks</h2>

<h2 align="left">Task 1: Theory</h2>

<h3 align="left">a)</h3>

**Question:**

Given a single convolutional layer with a stride of 1, kernel size of 7 × 7, and 6 filters. If I
want the output shape (Height × Width) of the convolutional layer to be equal to the input image,
how much padding should I use on each side?

**Answer:**

The padding $p_i$ required on each side of dimension $i$, where $i \le k$, of a $k$-dimensional unit-stride convolutional kernel $K_{a_0 \times a_1 \times ... \times a_k}$ if we want the output dimension to be equal to the input dimension, is $p_i = \lfloor \frac{a_i}{2} \rfloor$.

Thus for a unit stride convolutional kernel $K_{7 \times 7}$ we have $p_i = \lfloor \frac{7}{2} \rfloor = 3$ for $i \in {0,1}$ corresponding to height and width dimensions.

<h3 align="left">b)</h3>

Consider a CNN whose inputs are RGB color images of size 512×512. The network has two convolutional
layers. Using this information, answer the following:

**Question:**

You are told that the spatial dimensions of the feature maps in the first layer are 506 × 506,
and that there are 12 feature maps in the first layer. Assuming that no padding is used, the stride
is 1, and the kernel used are square, and of an odd size, what are the spatial dimensions of these
kernels? Give the answer as (Height) × (Width).

**Answer:**

The equations for the output shape of a 2D convolution layer are

```math
\begin{align}
    w_{\text{output}} &= \frac{w_{\text{input}} - w_{\text{kernel}} + w_{\text{padding start}} + w_{\text{padding end}} }{w_{\text{stride}}} + 1 \\[3pt]
    h_{\text{output}} &= \frac{h_{\text{input}} - h_{\text{kernel}} + h_{\text{padding start}} + h_{\text{padding end}} }{h_{\text{stride}}} + 1 \\[3pt]
    c_{\text{output}} &= \text{num\_filters}
\end{align}
```

We are given that

```math
\begin{align}
    w_{\text{output}} &= h_{\text{output}} = 506 \\[3pt]
    w_{stride} &= h_{stride} = 1, \\[3pt]
    h_{\text{kernel}} &= w_{\text{kernel}}
\end{align}
```

and there is no padding. 

We get

```math
\begin{align}
    &506 = \frac{512 - h_{\text{kernel}} + 0 + 0 }{1} + 1 \\[3pt]
    \implies &h_{\text{kernel}} = w_{\text{kernel}} = 512 - (506 - 1) = 7 \\[3pt]
    \implies &h_{\text{kernel}} \times w_{\text{kernel}} = 7 \times 7 
\end{align}
```

<h3 align="left">c)</h3>

**Question:**

If subsampling is done using neighborhoods of size 2 × 2, with a stride of 2, what are the
spatial dimensions of the pooled feature maps in the first layer? (assume the input has a shape of
506 × 506). Give the answer as (Height) × (Width).

**Answer:**

Pooling (subsampling) using a $2 \times 2$ kernel with stride $2$, assuming no padding or dilation (i.e. dilation=1), gives an  output of

```math
h_{\text{output}} \times w_{\text{ouput}} = 253 \times 253
```

<h3 align="left">d)</h3>

**Question:**

The spatial dimensions of the convolution kernels in the second layer are 3 × 3. Assuming
no padding and a stride of 1, what are the sizes of the feature maps in the second layer? (assume
the input shape is the answer from the last task). Give the answer as (Height) × (Width).

**Answer:**

Passing a $252 \times 253$ spatial dimension input from the previous layer and applying a $3 \times 3$ kernel with stride $1$ and no padding results in reduction of $2$ in bothe height and width, giving
```math
h_{\text{output}} \times w_{\text{ouput}} = 251 \times 251 
```

<h3 align="left">e)</h3>

**Question:**

[Table 1](#table-1) shows a simple CNN. How many parameters are there in the network? In this
network, the number of parameters is the number of weights + the number of biases. Assume the
network takes in an 32 × 32 image.

<a name="table-1"></a>
| Layer | Layer Type | Number of Hidden Units/Filters | Activation   |
|-------|------------|--------------------------------|--------------|
|   1   | Conv2D(kernel_size=5, stride=1, padding=2)  | 32 | ReLU    |
|   1   | MaxPool(kernel_size=2, stride=2)            |    |         |
|   2   | Conv2D(kernel_size=3, stride=1, padding=1)  | 64 | ReLU    |
|   2   | MaxPool(kernel_size=2, stride=2)            |    |         |
|   3   | Conv2D(kernel_size=3, stride=1, padding=1)  | 128| ReLU    |
|   3   | MaxPool(kernel_size=2, stride=2)            |    |         |
|       | Flatten()                                   |    |         |
|   4   | Linear(in_features=?, out_features=64)      | 64 | ReLU    |
|   5   | Linear(in_features=64, out_features=10)     | 10 | Softmax |
<p align="center"><b>Table 1</b>: Architecture of a simple convolutional neural network.</p>

**Answer:**

The weight and bias tensors of Conv2D layers have shapes 
```math
\begin{align}
    W_{\text{shape}} &= c_\text{output} \times c_\text{input} \times h_{\text{kernel}} \times w_{\text{kernel}} \\[3pt]
    \mathbf{b}_{\text{shape}} &= c_\text{output} \times 1
\end{align}
```
The weight and bias tensors of Linear layers have shapes
```math
\begin{align}
    W_{\text{shape}} &= h_\text{output} \times h_{\text{input}} \\[3pt]
    \mathbf{b}_{\text{shape}} &= h_\text{output} \times 1
\end{align}
```
MaxPool, ReLU and Softmax do not have any trainable parameters.

Thus, assuming RGB image inputs, for the Conv2D layers we have 
```math
\begin{align}
    {N_{\text{params}}}_1 &= (32 \cdot 3 \cdot 5 \cdot 5) + 32 = 2432 \\[3pt]
    {N_{\text{params}}}_2 &= (64 \cdot 32 \cdot 3 \cdot 3) + 64 = 18496 \\[3pt]
    {N_{\text{params}}}_3 &= (128 \cdot 64 \cdot 3 \cdot 3) + 128 = 73856 
\end{align}
```
and the spatial dimension of the image will have been halved $3$ times from the max pooling operations by the time it's flattened, meaning we have a $4 \times 4 \times 128$ feature map.
<br><br>This gives us
```math
\begin{align}
    {N_{\text{params}}}_4 &= (64 \cdot (4 \cdot 4 \cdot 128)) + 64 = 131136 \\[3pt]
    {N_{\text{params}}}_5 &= (10 \cdot 64) + 10 = 650
\end{align}
```
for a total of
```math
\begin{align}
    {N_{\text{params}}} &= {N_{\text{params}}}_1 + {N_{\text{params}}}_2 + {N_{\text{params}}}_3 + {N_{\text{params}}}_4 + {N_{\text{params}}}_5 \\[3pt]
    {N_{\text{params}}} &= 226570 
\end{align}
```

<h2 align="left">Task 2: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Implement the network in Table 1. Implement this in the jupyter notebook (or python file)
task2.py/ipynb. Report the final accuracy on the validation set for the trained network. Include
a plot of the training and validation loss during training.

By looking at the final train/validation loss/accuracy, do you see any evidence of overfitting?
Shortly summarize your reasoning.

**Answer:**

The final validation loss was $0.0348$, and the final validation accuracy was $0.9894$. [Figure 1](#figure-1) shows the training and validation losses during training.
<br>*Note: test and validation are used interchangably in these tasks.*

<h3 align="center">Task 2a Train and Test Losses</h3>
<a name="figure-1"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_5/src/image_processed/task2a_plot.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 1: </b>Five epochs of training is enough to learn the relatively simple MNIST dataset.</p>

Because the training and validation losses are closely aligned, we conclude that there is no evidence of overfitting. Furthermore the validation loss was still decreasing at the end of training.

<h3 align="left">b)</h3>

**Question:**

The optimizer in pytorch is the method we use to update our gradients. Till now, we have
used standard stochastic gradient descent (SGD). Understanding what the different optimizers do
is out of the scope of this course, but we want to make you aware that they exist. 

Adam is one of the most popular optimizers currently. Change the SGD optimizer to Adam (use
torch.optim.Adam instead of torch.optim.SGD), and train your model from scratch.

Use a learning rate of 0.001.

Plot the training/validation loss from both models (the model with Adam and the one with SGD)
in the same graph and include this in your report. (Note, you should probably change the plt.ylim
argument to [0, 0.1]).

**Answer:**

It wasn't quite clear weather SGD should be run with the same learning rate as Adam, or if we were meant to use the learning rate from before, so [Figure 2](#figure-2) includes both cases.

<h3 align="center">Task 2b Comparison of Optimizers and Learning Rates</h3>
<a name="figure-2"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_5/src/image_processed/task2b_plot.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 2: </b>Train and test losses for the same model using SGD with learning rates of 0.02 and 0.001, and Adam with a learning rate of 0.001.</p>

<h3 align="left">c)</h3>

**Question:**

Interpreting CNNs is a challenging task. One way of doing this, is to visualize the learned
weights in the first layer as a K × K × 3 image 2, where K is the kernel size.

Understanding what the filter does can be difficult. Therefore, we can visualize the activation by
passing an image through a given filter. The result of this will be a grayscale image.

Run the image zebra.jpg through the first layer of the ResNet50 network. Visualize the filter, and
the grayscale activation of a the filter, by plotting them side by side. 

Use the pre-trained network
ResNet50 and visualize the convolution filters with indices [5, 8, 19, 22, 34].

Include the visualized filters and activations in your report.

**Answer:**

[Figure-3](#figure-3) 

<h3 align="center">Task 2c Weights and Feature Maps of the First Layer of ResNet50</h3>
<a name="figure-3"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_5/src/image_processed/task2c_plot.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 3: </b>Visualization of the weights for filters 5, 8, 19, 22 and 34 of ResNet50 from PyTorch torchvision and the output feature map of the corresponding filter after passing zebra.jpg.</p>

<h3 align="left">d)</h3>

**Question:**

Looking at the visualized filter, and its corresponding activation on the zebra image, describe
what kind of feature each filter extracts. Explain your reasoning.

**Answer:**



<h2 align="center">Part 2: Filtering in the Frequency Domain</h2>

<h2 align="left">Task 3: Theory</h2>

<h3 align="left">a)</h3>

**Question:**


**Answer:**

**Question:**

**Answer:**

<h3 align="left">b)</h3>

**Question:**


**Answer:**

<h3 align="left">c)</h3>

**Question:**

**Answer:**

<h3 align="left">d)</h3>

**Question:**


**Answer:**


<h3 align="left">e)</h3>

**Question:**

**Answer:**

<h2 align="left">Task 4: Programming</h2>

<h3 align="left">a)</h3>

**Question:**


**Answer:**

<h3 align="left">b)</h3>

**Question:**

**Answer:**

<h3 align="left">c)</h3>

**Question:**

**Answer:**

<h3 align="left">d)</h3>

**Question:**

**Answer:**


