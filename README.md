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



<h3 align="left">b)</h3>

**Question:**

You are told that the spatial dimensions of the feature maps in the first layer are 506 × 506,
and that there are 12 feature maps in the first layer. Assuming that no padding is used, the stride
is 1, and the kernel used are square, and of an odd size, what are the spatial dimensions of these
kernels? Give the answer as (Height) × (Width).

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

[Figure 2a](#figure-2a) shows the result of the conversion to greyscale.

<h3 align="center">Greyscale</h3>
<a name="figure-2a"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/images/duck.jpeg?raw=true" width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_greyscale.jpeg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2a: </b>Before and after greyscale transformation.</p>


<h2 align="left">Task 2: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Implement the network in Table 1. Implement this in the jupyter notebook (or python file)
task2.py/ipynb. Report the final accuracy on the validation set for the trained network. Include
a plot of the training and validation loss during training.
By looking at the final train/validation loss/accuracy, do you see any evidence of overfitting?
Shortly summarize your reasoning

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


