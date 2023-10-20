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

Perform histogram equalization by hand on the 3-bit (8 intensity levels) image in Figure 1a
Your report must include all the steps you did to compute the histogram, the transformation, and
the transformed image. Round down any resulting pixel intensities that are not integers (use the
floor operator).


<div align="center">
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
</div>

**Answer:**



<h3 align="center">Temp</h3>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
</p>
<p align="center"></p>

<h3 align="left">Task 1d</h3>
