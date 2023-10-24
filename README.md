<h1 align="center">Assignment 4</h1>

<!-- *Note: Allow some time for the GIFs to load if viewing HTML or [GitHub Markdown versions](https://github.com/Jesperoka/tdt4195/tree/assignment_4) of report.<br> -->
<!-- PDF version does not support GIFs, so the other options are recommended.* -->

<br>

<h2 align="center">Part 1: Spatial Filtering</h2>

<h2 align="left">Task 1: Theory</h2>
<h3 align="left">a)</h3>

**Question:**

Explain in one sentence what sampling is.

**Answer:**

Sampling (in the context of image processing) is the discretization of the continuous voltage waveform representing the
image, in the **spatial** domain.

<h3 align="left">b)</h3>

**Question:**

Explain in one sentence what quantization is.

**Answer:**

Quantization (in the context of image processing) is the discretization of the continuous voltage waveform representing
the image, in the **amplitude** domain.


<h3 align="left">c)</h3>

**Question:**

Looking at an image histogram, how can you see that the image has high contrast?

**Answer:**

Since an image histogram displays the distribution of pixel intensities in the image, we expect a high contrast image to
have a large amount of pixels at the extremes of the intensity spectrum, which means we can identify a high contrast
image as one with a valley in the middle range and peaks at the left and right sides. There is of course more nuance but
in general a high contrast image will have a larger spread of pixel intensities, while low constrast images will have
pixel intensities more bunched up. An image can also appear to have high contrast from its histogram, while not really
looking like it has that much contrast, if the contrast is mostly global contrast rather than local contrast (which is
more noticable to us).

<h3 align="left">d)</h3>

**Question:**

Perform histogram equalization by hand on the 3-bit (8 intensity levels) image in [Figure 1a](#figure-1a)
Your report must include all the steps you did to compute the histogram, the transformation, and
the transformed image. Round down any resulting pixel intensities that are not integers (use the
floor operator).


<div align="center">
    <a name="figure-1a"></a>
    <table border="1">
        <tr>
            <td>1</td>
            <td>7</td>
            <td>6</td>
            <td>3</td>
            <td>6</td>
        </tr>
        <tr>
            <td>7</td>
            <td>6</td>
            <td>5</td>
            <td>6</td>
            <td>4</td>
        </tr>
        <tr>
            <td>5</td>
            <td>4</td>
            <td>7</td>
            <td>7</td>
            <td>0</td>
        </tr>
    </table>
    <b>Figure 1a:</b> 3 x 5, 3-bit image to be histogram equalized.
</div>

**Answer:**

1. The image is a 3 x 5 image, so our probability mass function is<p align="center">$p_r(r_k)=\frac{n_k}{15}$</p>where
intensity levels<p align="center">$r_k\in(0,1,2,3,4,5,6,7)$</p>are mapped to their counts<p align="center">
    $n_k\in(1,1,0,1,2,2,4,4)$</p>through their index $k$. This gives us the following histrogram:
<table align="center" cellspacing="0" cellpadding="0">
    <tr style="border: none;">
        <td style="border: none;"><b>n</b></td>
        <td style="border-right: none; border-left: none; border-top: none; border-bottom: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
    </tr>
    <tr style="border: none;">
        <td style="border: none;">4</td>
        <td style="border-right: none; border-top: none; border-bottom: none;"> </td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;"></td>
    </tr>
    <tr style="border: none;">
        <td style="border: none;">3</td>
        <td style="border-right: none; border-top: none; border-bottom: none;"> </td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;"></td>
    </tr>
    <tr style="border: none;">
        <td style="border: none;">2</td>
        <td style="border-right: none; border-top: none; border-bottom: none;"> </td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;"></td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;"></td>
    </tr>
    <tr style="border: none;">
        <td style="border: none;">1</td>
        <td style="border-right: none; border-top: none; border-bottom: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;"></td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;">*</td>
        <td style="border: none;"></td>
    </tr>
    <tr style="border: none;">
        <td style="border: none;"></td>
        <td style="border-right: none; border-left: none; border-bottom: none;">0</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">1</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">2</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">3</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">4</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">5</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">6</td>
        <td style="border-right: none; border-left: none; border-bottom: none;">7</td>
        <td style="border: none;"><b>r</b></td>
    </tr>
</table>

2. The probability mass function can also be represented as<p align="center">
    $p_k\in(\frac{1}{15},\frac{1}{15},0,\frac{1}{15},\frac{2}{15},\frac{2}{15},\frac{4}{15},\frac{4}{15})$</p> which
gives a cumulative probability mass function <p align="center">$F_r(r_k)=\sum_{i=0}^{k}{p_i}$</p> or again represented
as an ordered set <p align="center">$F_{k}\in(\frac{1}{15}, \frac{2}{15}, \frac{2}{15}, \frac{3}{15}, \frac{5}{15},
    \frac{7}{15}, \frac{11}{15}, \frac{15}{15})$</p>

3. Then we construct the transformation as a scaling to a desired range <p align="center">$T(r_k)=(8-1)F_{k}$</p> which
is a mapping from old pixel intensity levels to new pixel intensity levels after histogram equalization. This creates
the following transformed intensity map
<div align="center">
    <table border="1">
        <tr>
            <td>0.933...</td>
            <td>7.0</td>
            <td>5.133...</td>
            <td>1.4</td>
            <td>5.133...</td>
        </tr>
        <tr>
            <td>7.0</td>
            <td>5.133...</td>
            <td>3.266...</td>
            <td>5.133...</td>
            <td>2.333...</td>
        </tr>
        <tr>
            <td>3.266...</td>
            <td>2.333...</td>
            <td>7.0</td>
            <td>7.0</td>
            <td>0.466...</td>
        </tr>
    </table>
</div>

4. Finally applying the floor operator gives the output intensity image
<div align="center">
    <table border="1">
        <tr>
            <td>0</td>
            <td>7</td>
            <td>5</td>
            <td>1</td>
            <td>5</td>
        </tr>
        <tr>
            <td>7</td>
            <td>5</td>
            <td>3</td>
            <td>5</td>
            <td>2</td>
        </tr>
        <tr>
            <td>3</td>
            <td>2</td>
            <td>7</td>
            <td>7</td>
            <td>0</td>
        </tr>
    </table>
</div>

<h3 align="left">e)</h3>

**Question:**

What happens to the dynamic range if we apply a log transform to an image with a large
variance in pixel intensities?

**Answer:**

In general a log transform compresses a range of large numbers, and expands the range of small numbers (in terms of
absolute power). Since intensity
levels are positive integers, the dynamic range will be lowered by a log transform.

<h3 align="left">f)</h3>

**Question:**

Perform spatial convolution by hand on the image in [Figure 1a](#figure-1a) using the kernel in [Figure 1b](#figure-1b).
The convolved image should be 3 × 5. You are free to choose how you handle boundary conditions,
and state how you handle them in the report.

<div align="center">
    <a name="figure-1b"></a>
    <table border="1">
        <tr>
            <td>1</td>
            <td>0</td>
            <td>-1</td>
        </tr>
        <tr>
            <td>2</td>
            <td>0</td>
            <td>-2</td>
        </tr>
        <tr>
            <td>1</td>
            <td>0</td>
            <td>-1</td>
        </tr>
    </table>
    <b>Figure 1b:</b> A 3 x 3 Sobel kernel.
</div>

**Answer:**

Spatial discrete convolution is done by replacing all source pixels with a weighted sum of the neighboring pixels,
weighted elementwise by the kernel. Since the I am free to choose boundary conditions, I choose *valid* padding, aka. no
padding, since it greatly reduces the amount of work I have to do.

Thus the result of the convolution is a $(3 - (3-1)) \times (5 - (3 - 1)) = 1 \times 3$ image / feature map

<div align="center">
    <a name="figure-1b"></a>
    <table border="1">
        <tr>
            <td>1⋅1+0⋅7+(-1)⋅6+2⋅7+0⋅6+(-2)⋅5+1⋅5+0⋅4+(-1)⋅7</td>
            <td>1⋅7+0⋅6+(-1)⋅3+2⋅6+0⋅5+(-2)⋅6+1⋅4+0⋅7+(-1)⋅7</td>
            <td>1⋅6+0⋅3+(-1)⋅6+2⋅5+0⋅6+(-2)⋅4+1⋅7+0⋅7+(-1)⋅0</td>
        </tr>
    </table>
</div>

which, barring any mistakes, should evaluate to

<div align="center">
    <a name="figure-1b"></a>
    <table border="1">
        <tr>
            <td>-3</td>
            <td>1</td>
            <td>9</td>
        </tr>
    </table>
</div>

<h2 align="left">Task 2: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Implement a function that converts an RGB image to grayscale. Use [Equation 1](#eq1). Implement
this in the function `greyscale`.

<a name="eq1"></a>
$$\text{GRAY}[i][j] = 0.212\cdot\text{R}[i][j] + 0.7152\cdot\text{G}[i][j] + 0.0722\cdot\text{B}[i][j]$$

**In your report**, include the image duck.jpeg as a grayscale image.

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

<h3 align="left">b)</h3>

**Question:**

Implement a function that takes a grayscale image and applies the following intensity transformation <p align="center">
    $T(p) = 1 − p$</p> Implement this in the function `inverse`.

**In your report**, apply the transformation on duck.jpeg, and include in your report.

**Answer:**

[Figure 2b](#figure-2b) shows the result of the conversion to intensity inverse.

<h3 align="center">Inversion</h3>
<a name="figure-2b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_greyscale.jpeg?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_inverse.jpeg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2b: </b>Before and after intensity inversion transformation.</p>

<h3 align="left">c)</h3>

**Question:**

Implement a function that takes an RGB image and a convolutional kernel as input, and performs 2D spatial convolution.
Assume the size of the kernel is odd numbered, e.g. 3 × 3, 5 × 5, or 7 × 7. You must implement the convolution operation
yourself from scratch. Implement the function in `convolve_im`.

You are not required to implement a procedure for adding or removing padding (you can return zero in cases when the
convolutional kernel goes outside the original image).

**In your report**, test out the convolution function you made. Convolve the image duck.jpeg with the sobel kernel
($h_a$)
and the smoothing kernel ($h_b$) in [Equation 2](#eq2). Show both images in your report.

<a name="eq2"></a>
<p align="center">
    <math>h_a = \begin{bmatrix} -1 & 0 & 1 \\ -2 & 0 & 2 \\ -1 & 0 & 1
    \end{bmatrix}, h_b=\frac{1}{256}\begin{bmatrix} 1 & 4 & 6 & 4 & 1 \\ 4 & 16 & 24 & 16 & 4 \\ 6 & 24 & 36 & 24 & 6 \\
    4 &
    16 & 24 & 16 & 4 \\ 1 & 4 & 6 & 4 & 1 \end{bmatrix}</math>
</p>

**In your report**, apply the transformation on duck.jpeg, and include in your report.

**Answer:**

The convolution I implemented uses zero-padding and unit stride. Convolutions are [embarrasingly
parallel](https://en.wikipedia.org/wiki/Embarrassingly_parallel) (alas, see the note below ;( ), and the separation
between rgb channels even more so, thus we can easily process each channel concurrently.

[Figure 2c](#figure-2c) shows the result of the result of the convolutions.

<h3 align="center">Convolutions</h3>
<a name="figure-2c"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/im_sobel.jpg?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/im_smoothed.jpg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2c: </b>Convolutions with Sobel and smoothing kernel</p>

*Note:
I spent 2 whole days implementing the (toeplitz) matrix multiplication version of 2d convolution (with zero-padding and
unit stride), only to realize the block matrix you end up with as convolution operator is too large to fit in memory, so
naturally I spent another half a day making it work on image segments, only to realize that overlap needed between
segments scales with kernel size and image width, so the implementation is slower than just using (python) for loops.
wtf is wrong with me?*

<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/imgs/pain.png?raw=true" width=350>
</p>
<p align="center"><i>pain.png</i></p>

<h2 align="center">Part 2: Neural Networks</h2>

<h2 align="left">Task 3: Theory</h2>

<h3 align="left">a)</h3>

**Question:**

A single-layer neural network is a linear function. Which of these binary operation(s) can not be represented by a
single-layer neural network (AND, OR, NOT, NOR, NAND, or XOR).

**Answer:**

XOR, because it does not have a linear decision boundary.

<h3 align="left">b)</h3>

**Question:**

Explain in one sentence what a hyperparameter for a neural network is. Give two examples of a hyperparameter.

**Answer:**

A hyperparameter for a neural network is any changeable-by-you aspect of the neural network and its training and/or
inference process, that are not optimized by the optimization procedure (usually some form of gradient based
optimization) used to find the minimum of the loss function.

Examples of hyperparameters are number of layers, neurons
per layer, optimizer, step size, any parameters to non-trainable layers like batch normalization or dropout, pre- and
postprocessing of data and outputs like how data augmentation is performed or use of model ensembles, Platt Scaling, and
the list goes
on.

<h3 align="left">c)</h3>

**Question:**

Why is the softmax activation functioned used in the last layer for neural networks trained to classify objects?

**Answer:**

Minimizing (log) cross-entropy is equivalent (but computationally superior) to minimizing the Kullback Leibler
divergence, which is a distance function between probability distributions. Estimating a parameterized approximation of
a true distribution is usually how we model class prediction problems, especially in the context of neural networks.
Taking the softmax of the output forces the estimated approximate probability distribution to be a valid probability
distribution (depending on philosophical definitions and some other details), and this is why it is a common practice
(but it is not always [necessary](https://arxiv.org/abs/2011.11538) or [optimal](https://arxiv.org/abs/1711.03953)).
It's really just a normalization of logits, which can [be](https://arxiv.org/pdf/1812.05720.pdf)
[done](https://stats.stackexchange.com/a/251654) [in](https://en.wikipedia.org/wiki/Platt_scaling)
[multiple](https://arxiv.org/pdf/2205.09310.pdf) (better) [ways](https://proceedings.mlr.press/v70/guo17a/guo17a.pdf).

<h3 align="left">d)</h3>

**Question:**

[Figure 3a](#figure-3a) shows a simple neural network. Perform a forward pass and backward pass on this
network with the given input values. Use [Equation 3](#eq3) as the cost function and let the target value
be y = 1.

Find and report the final values for ∂C/∂w1, ∂C/∂w2, ∂C/∂w3, ∂C/∂w4, ∂C/∂b1, and ∂C/∂b2.

Explain each step in the computation, such that it is clear how you compute the derivatives.

<a name="eq3"></a>
<p align="center">$C(y_n, \hat{y}_n)=\frac{1}{2}(y_n - \hat{y}_n)^2$</p>

<h3 align="center">Task 3d Neural Network</h3>
<a name="figure-3a"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/imgs/simple_nn.png?raw=true" width=650>
</p>
<p align="center"><b>Figure 3a: </b>Simple neural network</p>

**Answer:**

In the forward pass (the output of) each node in the neural network is evaluated, as well as the value of the gradient
of each node with respect to each of its inputs, excluding the inputs to the network itself. This can be seen in [Figure
3b](#figure-3b).

<h3 align="center">Task 3d Forward Pass</h3>
<a name="figure-3b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/imgs/forward_pass.jpg?raw=true" width=650>
</p>
<p align="center"><b>Figure 3b: </b>Evaluation of a forward pass, with node output values in orange, and node
    derivatives w.r.t. their inputs in blue.</p>

Then, using the chain rule, the derivative of the cost function w.r.t. the weights and biases can be computed as a
product of the previously stored values for each path. This computation is shown in [Figure 3c](#figure-3c) with
reference to the values in [Figure 3b](#figure-3b).

<h3 align="center">Task 3d Backward Pass</h3>
<a name="figure-3c"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/imgs/backward_pass.jpg?raw=true" width=650>
</p>
<p align="center"><b>Figure 3c: </b>Evaluation of a backward pass, giving us the gradient of the cost function w.r.t.
    all the weights and biases.</p>


<h3 align="left">e)</h3>

**Question:**

Compute the updated weights w1, w3, and b1 \[note: *I am assuming this is a typo and we were meant to compute all the
weight updates*\] by using gradient descent and the values you
found in task d. Use α = 0.1

**Answer:**

The computation is shown in [Figure 3d](#figure-3d).

<h3 align="center">Task 3e Gradient Descent Step</h3>
<a name="figure-3d"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/imgs/gradient_descent.jpg?raw=true" width=650>
</p>
<p align="center"><b>Figure 3d: </b>A single gradient descent step.</p>

<h2 align="left">Task 4: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Use the given starter code and train a single-layer neural network with batch size of 64. Then, normalize every image
between a range of [-1. 1], and train the network again.

Plot the training and validation loss from both of the networks in the same graph. Include the graph in your report.

Do you notice any difference when training your network with/without normalization?

**Use normalization for every subsequent task.**

**Answer:**

[Figure 4a](#figure-4a) shows the result of normalization, which is faster convergence (and in theory more stable, but
not noticable in this case) for the same learning rate.

<h3 align="center">Task 4a Normalization</h3>
<a name="figure-4a"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4a_train.png?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4a_test.png?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 4a: </b>Comparison of train and test losses for without and with normalization of the input
    data. Note that "test" refers to "validation" in the convention used by the task description, i.e.
    train-validate-test vs train-test-validate.</p>

The reason for the faster (and more stable) learning is because scaling affects gradient based optimization in quite a
few ways. Some of the main ones are optimization problem preconditioning, acivation function gradient properties (i.e.
saturation), activation function output properties (i.e. dying ReLU). To some extent (but not solely), these are a
result of weight initialization not being a hand-tuned process. There are also other more complicated effects that take
place in larger neural nets, but in our specific case the most important part is the preconditioning and activation
function related properties.

Another thing to note is that our images are already on the range [0, 1] not [0, 255], which means the effect is not as
large as i could have been in other image color formats.

<h3 align="left">b)</h3>

**Question:**

The trained neural network will have one weight with shape [num classes, 28 × 28]. To visualize the learned weight, we
can plot the weight as a 28 × 28 grayscale image. For each digit (0-9), plot the learned weight as a 28 × 28 image. In
your report, include the image for each weight, and describe what you observe (1-2 sentences).

**Answer:**

The weights are visualized in [Figure 4b](#figure-4b).

<h3 align="center">Task 4b Weight maps</h3>
<a name="figure-4b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_0.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_1.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_2.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_3.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_4.png?raw=true"
        width=150>
    <br>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_5.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_6.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_7.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_8.png?raw=true"
        width=150>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4b_class_9.png?raw=true"
        width=150>
</p>
<p align="center"><b>Figure 4b: </b>Weights for each output class row in the weight matrix visualized as 2D linear color
    maps.</p>

We can see that the neural network has learned a mapping from pixel regions to class, which correspond to the regions
that are most likely to be bright or dark for a given number. Due to the variation between indivdual instances of each
number, the regions get blurred, and the effect is larger for some numbers than others.

<h3 align="left">c)</h3>

**Question:**

Set the learning rate to lr = 1.0, and train the network from scratch. Report the accuracy and average cross entropy
loss on the validation set. In 1-2 sentences, explain why the network achieves worse/better accuracy than previously.

**Answer:**

```
Final Test loss with lr = 0.0192: 0.28947194200602305. Final Test accuracy with lr = 0.0192: 0.9186
Final Test loss with lr = 1.0: 2.226700500160228. Final Test accuracy with lr = 1.0: 0.8977
```

<h3 align="center">Task 4c Learning Rates</h3>
<a name="figure-4c"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4c_train.png?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 4c: </b>Comparison of training loss for learning rates of 0.0192 and 1.0.</p>



The learning rate is too high for the optimization surface we are walking along, so the training is very unstable (and
nearly diverges). We are essentially stepping over minima during single training steps.

Not really much more to say about this other than that it is problem specific, and there are many techniques to change
the optimization landscape to a more preferable form. Too low a learning rate can result in slow convergence or getting
stuck in local minima, since then we are not able to utilize the stochastic nature of SGD to exit them.

<h3 align="left">d)</h3>

**Question:**

Include an hidden layer with 64 nodes in the network, with ReLU as the activation function for the first layer. Train
this network with the same hyperparameters as previously. Plot the training and validation loss from this network
together with the loss from task (a). Include the plot in your report. What do you observe?

**Answer:**

This question is worded a bit ambiguously, since number of "nodes" is not really precisely defined unless we specify
that the layer is fully connected etc., and "ReLU as activation function for the first layer" can be interpreted in two
ways, but presumably this is what is meant:

```python
model = nn.Sequential(nn.Flatten(), nn.Linear(28*28*1, 64), nn.ReLU(), nn.Linear(64, 10))
```

The result can be seen in [Figure 4d](#figure-4d).

<h3 align="center">Task 4d Adding a Hidden Layer</h3>
<a name="figure-4d"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4d_train.png?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/task4d_test.png?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 4d: </b>Comparison of train and test losses between the task a) model and a model with an
    added hidden layer and data normalization.</p>

We can see that adding the additional nonlinearity (softmax is the other one) means we can learn a more complicated
function approximation, which in this case leads to better performance since we do not observe any overfitting.
