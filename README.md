<h1 align="center">Assignment 6</h1>

<!-- *Note: Allow some time for the GIFs to load if viewing HTML or [GitHub Markdown versions](https://github.com/Jesperoka/tdt4195/tree/assignment_6) of report.<br> -->
<!-- PDF version does not support GIFs, so the other options are recommended.* -->

<br>

<h2 align="left">Task 1: Theory</h2>

<h3 align="left">a)</h3>

**Question:**

Define opening and closing in terms of erosion and dilation. What happens when
opening and closing are applied multiple times on the same image?

**Answer:**

*Opening* and *closing*, as first described by Georges Matheron (building on and formalizing his own, Jean Serra's, and André Haas' earlier work), are the two functional compositions of *erosion* and *dilation*, applied to one set by *another set*, denoted by G.M. as a *structuring element* ([The Birth of Mathematical Morphology](https://smil.cmm.minesparis.psl.eu/publis/C-72.pdf), (*Random Sets and Integral Geometry*, Matheron G., 1974, pp. 16-17)). 

Specifically, in image processing, with an image described by the set $A$ and a structuring element (aka. filter/kernel) $B\text{,}$ the *opening* $A_B$ of $A$ by $B$ is
```math
\begin{align}
     f \circ g
\end{align}
```
and the *closing* $A^B$ of $A$ by $B$ is
```math
\begin{align}
     g \circ f
\end{align}
```
where $f$ is the erosion of $A$ by $B$ and $g$ is the dilation of $A$ by $B$, defined as
```math
\begin{align}
     f &= A \oplus \check{B} \\[3pt]
     g &= A \ominus \check{B} 
\end{align}
```
respectively, where $\check{B}$ is the symmetrical set of $B$ w.r.t. the origin, and where $\oplus$ and $\ominus$ are the [*Minkowski sum* and *Minkowski difference*](https://en.wikipedia.org/wiki/Minkowski_addition) operators.

Here I've defined the opening and closing operations as function compositions of erosion and dilation, but Georges Matheron originally defined them more directly as
```math
\begin{align}
     A_B &= (A \ominus \check{B}) \oplus B \\[3pt]
     A^B &= (A \oplus \check{B}) \ominus B
\end{align}
```
and notes that the mappings are [*idempotent*](https://en.wikipedia.org/wiki/Idempotence) (*Random Sets and Integral Geometry*, Matheron G., 1974, p. 18). Thus, repeated applications of either mapping has no further effect than the initial application.

<h3 align="left">b)</h3>

**Question:**

Smoothing of an image is often done before performing edge detection. What is the
purpose of smoothing the image before edge detection?

**Answer:**

Smoothing is primarily done to mitigate the effects of noise on edge detection algorithms, under the assumption that noise is not relevant to the edges we want to detect (reasonable). 

Since edges are essentially sharp gradients in the image, it's natural that edge detectors use gradient information, and we can therefore understand why traditional edge detection algorithms are sensitive to noise, just like any other gradient-based processing of a noisy signal (because noise is assumed to be high-frequency in this case $\rightarrow$ large derivatives).

In general smoothing the image may also help remove detection of *false edges*, or rather, bias us towards larger more defined structures, but of course this will depend on the specifics of the smoothing and edges we want to detect.

<h3 align="left">c)</h3>

**Question:**

The Canny edge detector uses a method called *hysteresis thresholding*. Shortly explain
how hysteresis thresholding works.

**Answer:**

Hysteresis thresholding in the Canny edge detector is the usage of two gradient magnitude thresholds when determining whether to keep or filter out potential edges. 

After having found the magnitude of the intensity gradients (in horizontal, vertical and 45 deg. diagonal directions), all pixels are compared with neighboring pixels with the same gradient direction, and all non-maximal pixels are filtered out.

Then, the hysteresis double threshold is applied to the remaining pixels (gradient amplitude), where pixels above the high threshold are defined as strong edges and kept, while pixels below the low threshold are removed. Finally, pixels in-between the two thresholds are checked for being directly next to any strong edge pixel, if yes, the pixel is also denoted strong and can be used to connect other pixels (i.e. the whole connected segment is included, atleast in [Canny's original description, p. 690](http://dx.doi.org/10.1109/TPAMI.1986.4767851). 


<h3 align="left">d)</h3>

**Question:**

Why do we use hysteresis thresholding instead of a single treshold?

**Answer:**

The point is to make it less likely that the algorithm outputs edges for gradients that appear only as a result of noise, and to avoid breaking up edges due to gradient magnitudes fluctuating along an edge.

Quoting Canny's article:
```
[end of page 689]

    [...]

    Streaking is the breaking up of an edge contour caused by
    the operator output fluctuating above and below the
    threshold along the length of the contour.

    [...]

[middle of page 690]

    [...]

    The probability of streaking is greatly reduced 
    because for a contour to be broken it must now fluctuate 
    above the high threshold and below the low threshold. 
    Also the probability of isolated false edge points is reduced 
    because the strength of such points must be above a higher threshold.

    [...]

Canny, J. (1986) A Computational Approach to Edge Detection. 
IEEE Transactions on Pattern Analysis and Machine Intelligence, 8, 679-698. 
http://dx.doi.org/10.1109/TPAMI.1986.4767851
```

<h3 align="left">e)</h3>

**Question:**

Determine the dilation A $\oplus$ B of the binary image in [Figure 1a](#figure-1a). Use the structuring
element in [Figure 1b](#figure-1b).

<h3 align="center">Binary Image and Structuring Element</h3>
<a name="figure-1a"></a><a name="figure-1b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/imgs/figure_1ab.png?raw=true" width=450>
</p>
<p align="center"><b>Figure 1ab: </b>A binary image and a structuring element. The foreground is colored white and given the
symbol 1. The background has the symbol 0 and is colored black. The reference pixel in the structuring
element (b) is indicated by a black circle.<p>

**Answer:**

The result of the dilation is shown in [Figure 2](#figure-2).

<h3 align="center">Dilated Binary Image</h3>
<a name="figure-2"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/imgs/task1e_dilation.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 2: </b>Result of the dilation.</p>


<h2 align="left">Task 2: Programming</h2>

<h3 align="left">a)</h3>

**Question:**

Implement a function in task2a.py/task2a.ipynb that implements Otsu's algorithm
for thresholding, and returns a single threshold value.

Segment the images thumbprint.png and rice-shaded.png, and include the results in your report.

**Answer:**

Results shown in [Figure 3](#figure-3).

<h3 align="center">Otsu's Method Results</h3>
<a name="figure-3"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/rice-shaded-segmented.png?raw=true" width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/thumbprint-segmented.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 3: </b>Result of the dilation. <br>Left: rice-shaded.png (segmented) | Right: thumbprint.png (segmented).</p>

<h3 align="left">b)</h3>

**Question:**

Region growing is a region-based segmentation algorithm that uses a set of seed points and a homogeneity
criteria $H(R_i)$ to perform segmentation. For each seed point, a region is grown by inspecting neighboring
pixels and evaluating whether to include them in the region $R_i$ using $H(R_i)$. The neighboring pixels
that are currently being evaluated are typically called candidate pixels. The growing of a region stops
when there are no more candidate pixels to inspect. The simplest homogeneity criteria is a threshold,
where the threshold defines the maximum absolute difference in intensity between the seed point and
the current candidate pixel.

Implement a function in task2b.py/task2b.ipynb that segments a grayscale image using the region growing method outlined above. 
Use a Moore neighborhood (8-connectedness) to expand your set of candidate pixels around each seed point.

Apply it on the image defective-weld.png and show the result in your report. 
Use the 4 seed points given in the starter code and use a pixel intensity threshold of 50.

**Answer:**

Results shown in [Figure 4](#figure-4).

<h3 align="center">Region Growing Result</h3>
<a name="figure-4"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/defective-weld-segmented.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 4: </b>Result of the region growing algorithm.</p>


<h2 align="left">Task 3: Morphology</h2>

<h3 align="left">a)</h3>

**Question:**

Use what you know about erosion, dilation, opening, and closing to remove the noisy
elements from the image in [Figure 5](#figure-5). 

<h3 align="center">Noisy Binary Image</h3>
<a name="figure-5"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/images/noisy.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 5: </b>noisy.png</p>

Implement this in the file task3a.py/task3a.ipynb. 
Include the resulting image in your report and shortly explain how you removed the noise.

**Answer:**

To create a noise removal function using the four basic morphological operations one can apply closing to remove negative noise (holes) in the desired features, and opening to remove positive noise (grains) in the desired background.

By applying opening and closing successively with different structuring elements, one can promote different changes to the features in the images and remove unwanted features.

In order to get a reasonably general function that allows for easy tuning to the image in question, the function iteratively grows (by dilation) the structuring element, and I've defined 3 parameters.
```python
def remove_noise(im: np.ndarray,
                 num_morphologies: int = 6,
                 structure_dilator: NDArray[np.bool_] = np.ones((9, 9), dtype=np.bool_),
                 closing_first: bool = False) -> np.ndarray:
```
where some reasonable defaults are shown above. 

The parameters are as follows 
```python
num_morphologies    # How many iterations of opening, closing and dilation of the structuring element to perform.
structure_dilator   # Structuring element for the structuring element, this is the kernel for dilating the structuring element.
closing_first       # Whether to perform closing and then opening, or the opposite, at every iteration.
```
where `num_morphologies` ultimately determines the range of sizes for the structuring element, and is arguably the most important parameter.
If shape information about the desired features is known, one can define the `structure_dilator`, which is the basic shape of the growing structuring element applied to the image.

In [Figure 6](#figure-6) we can see how the function performs with the defaults and with somewhat tuned parameters, especially the use of a `structure_dilator` shape that almost matches the shape of the desired feature in the image (i.e. a triangle).

<h3 align="center">Morphological Noise Removal with Defaults and Tuned Parameters</h3>
<a name="figure-6"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/noisy-filtered.png?raw=true" width=350>
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/tuned-noisy-filtered.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 6: </b>Comparison of using default parameters vs using a shape informed structuring element.</p>

<h3 align="left">b)</h3>

**Question:**

The distance transform is an operation typically applied on a binary image and creates a grayscale image
where each foreground pixel shows the distance to the closest boundary pixel. 

One inefficient way of calculating the distance transform is to use erosion. Intuitively, by using erosion
the distance transform for a pixel is simply the number of erosion operations it took to remove it from
the foreground of the original image.

Implement the distance transform using the erosion method explained above, in the
file task3b.py/task3b.ipynb. You can use a 3×3 structuring element of all ones to get chessboard
distance (*Manhattan distance*).

Test the function on the noise-free binary image you got from the task (a) and show the result in
your report.

**Answer:**

I am testing on the result using default parameters from the previous task. This is show in [Figure 7](#figure-7)

<h3 align="center">Distance Transform of Binary Image Using Erosion Counting</h3>
<a name="figure-7"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/noisy-distance.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 7: </b>Map of distance from feature borders using erosion counting with a square kernel for chessboard/Manhattan distance.</p>

<h3 align="left">c)</h3>

**Question:**

Mathematical operations can be used for extracting boundary information from images. The operation
for extracting the inner boundary extraction can be seen below, where $\ominus$ is erosion.
```math
\begin{align}
    A_{\text{boundary}} = A - (A \ominus B)
\end{align}
```
Implement a function that extracts the boundary from a binary image, as defined in the equation above, 
in the file task3c.py/task3c.ipynb. You can use a 3 × 3 structuring element of all ones.

Show the boundary on the image blood-vessels.png and include the result in your report.

**Answer:**

The result can be seen in [Figure 8](#figure-8)

<h3 align="center">Boundary Extraction</h3>
<a name="figure-8"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_6/src/image_processed/blood-vessels-boundary.png?raw=true" width=350>
</p>
<p align="center"><b>Figure 8: </b>Extracted boundary of blood-vessels.png</p>

<h3 align="left">d)</h3>

**Question:**

Hole filling is a method to fill in holes in segmentation, and is very useful for post-processing imperfect
segmentations. The algorithm below outlines a basic algorithm that takes a binary image and fills all known holes
indicated by a set of starting points.

```math
\begin{align}
    &\text{\textbf{Algorithm: }}&&\text{An algorithm to fill holes in a binary image.} \\[8pt]
    &1:&&\textbf{Input: } \text{Image } I, \text{ number of iterations } K, \text{ starting points } S, \text{ and structuring element } B. \\
    &2:&&\text{Form an array, } X_0, \text{ of 0’s (the same size as the Image } I\text{)} \\
    &3:&&\textbf{for } \text{row, column in } S \textbf{ do } \\
    &4:&&\quad X_0[\text{row, column}] \leftarrow 1 \\
    &5:&&\textbf{for } k \leftarrow 1 \text{ to } K \textbf{ do } \\
    &6:&&\quad X_k \leftarrow (X_{k-1} \oplus B) \cap I^c \\
    &7:&&\textbf{return } X_k \cup I
\end{align}
```

**Answer:**
