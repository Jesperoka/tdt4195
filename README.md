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
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 2a:</b>Before and after greyscale transformation.</p>

<h3 align="left">b)</h3>

**Question:**

Implement a function that takes a grayscale image and applies the following intensity transformation <p align="center">$T(p) = 1 − p$</p> Implement this in the function `inverse`.

**In your report**, apply the transformation on duck.jpeg, and include in your report.

**Answer:**

[Figure 2b](#figure-2b) shows the result of the conversion to greyscale.

<h3 align="center">Temp</h3>
<a name="figure-2b"></a>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 2b:</b>Before and after intensity inversion transformation.</p>
