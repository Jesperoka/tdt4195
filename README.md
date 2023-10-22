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

<h3 align="left">Task 2: Programming</h3>

<h3 align="left">a)</h3>

**Question:**

Implement a function that converts an RGB image to grayscale. Use [Equation 1](#eq1). Implement
this in the function `greyscale`.

<a name="eq1"></a>
$$\text{GRAY}[i][j] = 0.212\cdot\text{R}[i][j] + 0.7152\cdot\text{G}[i][j] + 0.0722\cdot\text{B}[i][j]$$

**In your report**, include the image duck.jpeg as a grayscale image.

**Answer:**

[Figure 2a](#figure-2a) shows the result of the conversion to greyscale.

<h3 align="center">Temp</h3>
<a name="figure-2a"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/images/duck.jpeg?raw=true" width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_greyscale.jpeg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2a:</b>Before and after greyscale transformation.</p>

<h3 align="left">b)</h3>

**Question:**

Implement a function that takes a grayscale image and applies the following intensity transformation <p align="center">
    $T(p) = 1 − p$</p> Implement this in the function `inverse`.

**In your report**, apply the transformation on duck.jpeg, and include in your report.

**Answer:**

[Figure 2b](#figure-2b) shows the result of the conversion to intensity inverse.

<h3 align="center">Temp</h3>
<a name="figure-2b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_greyscale.jpeg?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/duck_inverse.jpeg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2b:</b>Before and after intensity inversion transformation.</p>

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
$$h_a = \begin{bmatrix} -1 & 0 & 1 \\ -2 & 0 & 2 \\ -1 & 0 & 1
\end{bmatrix}, h_b=\frac{1}{256}\begin{bmatrix} 1 & 4 & 6 & 4 & 1 \\ 4 & 16 & 24 & 16 & 4 \\ 6 & 24 & 36 & 24 & 6 \\ 4 &
16 & 24 & 16 & 4 \\ 1 & 4 & 6 & 4 & 1 \end{bmatrix}$$

**In your report**, apply the transformation on duck.jpeg, and include in your report.

**Answer:**

The convolution I implemented uses zero-padding and unit stride. Convolutions are [embarrasingly
parallel](https://en.wikipedia.org/wiki/Embarrassingly_parallel) (alas, see the note below ;( ), and the separation
between rgb channels even more so, thus we can easily process each channel concurrently.

[Figure 2c](#figure-2c) shows the result of the result of the convolutions.

<h3 align="center">Temp</h3>
<a name="figure-2c"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/im_sobel.jpg?raw=true"
        width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_4/src/image_solutions/im_smoothed.jpg?raw=true"
        width=350>
</p>
<p align="center"><b>Figure 2c:</b>Convolutions with Sobel and smoothing kernel</p>

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

<h3 align="center">Part 2: Neural Networks</h3>

<h3 align="left">Task 3: Theory</h3>

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

[Figure 3](#figure-3) shows a simple neural network. Perform a forward pass and backward pass on this
network with the given input values. Use [Equation 3](#eq3) as the cost function and let the target value
be y = 1.

<p align="center">$T(p)=p$</p>

**Answer:**


