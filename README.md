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
    <b>Figure 1a:</b> 3 x 5, 3-bit image to be equalized
</div>

**Answer:**

1. The image is a 3 x 5 image, so our probability mass function is<p align="center">$p_r(r_k)=\frac{n_k}{15}$</p>where
intensity levels<p align="center">$r_k\in\{0,1,2,3,4,5,6,7\}$</p>are mapped to their counts<p align="center">
    $n_k\in\{1,1,0,1,2,2,4,4\}$</p>through their index $k$. This gives us the following histrogram:

<table align="center" cellspacing="0" cellpadding="0">
    <tr style="border: none;">
        <td style="border: none;"><b >n</b></td>
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
        <td style="border: none;">5</td>
        <td style="border-right: none; border-top: none; border-bottom: none;"></td>
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


<h3 align="center">Temp</h3>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
</p>
<p align="center"></p>

<h3 align="left">Task 1d</h3>
