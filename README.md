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

$$w_{\text{output}} = \frac{w_{\text{input}} - w_{\text{kernel}} + w_{\text{padding start}} + w_{\text{padding end}} }{w_{\text{stride}}} + 1$$
$$h_{\text{output}} = \frac{h_{\text{input}} - h_{\text{kernel}} + h_{\text{padding start}} + h_{\text{padding end}} }{h_{\text{stride}}} + 1$$
$$c_{\text{output}} = \text{num\_filters}$$

We are given that

$$w_{\text{output}} = 506, h_{\text{output}} = 506, w_{stride}=1, h_{stride}=1, h_{\text{kernel}} = w_{\text{kernel}}$$

and there is no padding. 

We get

$$506 = \frac{512 - h_{\text{kernel}} + 0 + 0 }{1} + 1$$
$$\implies h_{\text{kernel}} = w_{\text{kernel}} = 512 - (506 - 1) = 7$$
$$\implies h_{\text{kernel}} \times w_{\text{kernel}} = 7 \times 7 $$

<h3 align="left">c)</h3>

**Question:**

If subsampling is done using neighborhoods of size 2 × 2, with a stride of 2, what are the
spatial dimensions of the pooled feature maps in the first layer? (assume the input has a shape of
506 × 506). Give the answer as (Height) × (Width).

**Answer:**

Pooling (subsampling) using a $2 \times 2$ kernel with stride $2$, assuming no padding or dilation (i.e. dilation=1), gives an  output of
$$h_{\text{output}} \times w_{\text{ouput}} = 253 \times 253 $$

<h3 align="left">d)</h3>

**Question:**

The spatial dimensions of the convolution kernels in the second layer are 3 × 3. Assuming
no padding and a stride of 1, what are the sizes of the feature maps in the second layer? (assume
the input shape is the answer from the last task). Give the answer as (Height) × (Width).

**Answer:**

Passing a $252 \times 253$ spatial dimension input from the previous layer and applying a $3 \times 3$ kernel with stride $1$ and no padding results in reduction of $2$ in both height and width, giving
$$h_{\text{output}} \times w_{\text{ouput}} = 251 \times 251 $$

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

$$W_{\text{shape}} = c_\text{output} \times c_\text{input} \times h_{\text{kernel}} \times w_{\text{kernel}}$$
$$\mathbf{b}_{\text{shape}} = c_\text{output} \times 1$$

The weight and bias tensors of Linear layers have shapes

$$W_{\text{shape}} = h_\text{output} \times h_{\text{input}}$$
$$\mathbf{b}_{\text{shape}} = h_\text{output} \times 1$$

MaxPool, ReLU and Softmax do not have any trainable parameters.

Thus, assuming RGB image inputs, for the Conv2D layers we have 

<p algin="left">

&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}}_1 = (32 \cdot 3 \cdot 5 \cdot 5) + 32 = 2432$

<br>&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}}_2 = (64 \cdot 32 \cdot 3 \cdot 3) + 64 = 18496$

<br>&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}}_3 = (128 \cdot 64 \cdot 3 \cdot 3) + 128 = 73856$
</p>

and the spatial dimension of the image will have been halved $3$ times from the max pooling operations by the time it's flattened, meaning we have a $4 \times 4$ image.
<br><br>This gives us

&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}}_4 = (64 \cdot (4 \cdot 4)) + 64 = 1088$ 

<br>&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}}_5 = (10 \cdot 64) + 10 = 650$ 

for a total of

&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}} = {N_{\text{params}}}_1 + {N_{\text{params}}}_2 + {N_{\text{params}}}_3 + {N_{\text{params}}}_4 + {N_{\text{params}}}_5$ 

&nbsp;&nbsp;&nbsp;&nbsp;${N_{\text{params}}} = 96522$ 

<h2 align="left">Task 2: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Implement the network in Table 1. Implement this in the jupyter notebook (or python file)
task2.py/ipynb. Report the final accuracy on the validation set for the trained network. Include
a plot of the training and validation loss during training.

By looking at the final train/validation loss/accuracy, do you see any evidence of overfitting?
Shortly summarize your reasoning.

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





<h2 align="center">Part 2: Filtering in the Frequency Domain</h2>

<h2 align="left">Task 1: Theory</h2>

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

In the forward pass (the output of) each node in the neural network is evaluated, as well as the value of the gradient

<h3 align="left">e)</h3>

**Question:**

**Answer:**

<h3 align="left">a)</h3>

**Question:**

**Use normalization for every subsequent task.**

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


<h3 align="center">Temp</h3>
<a name="figure-X"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/images/duck.jpeg?raw=true" width=350>
</p>
<p align="center"><b>Figure Xa: </b>Temp.</p>
