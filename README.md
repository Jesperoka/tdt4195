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

Specifically, in image processing, with an image described by the set $A$ and a structuring element (aka. filter/kernel) $B$, the *opening* $A_B$ of $A$ by $B$ is
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
     f &= A \oplus \v{B}
     g &= A \ominus \v{B} 
\end{align}
```
respectively, where $\v{B}$ is the symmetrical set of $B$ w.r.t. the origin, and where $\oplus$ and $\ominus$ are the [*Minkowski sum* and *Minkowski difference*](https://en.wikipedia.org/wiki/Minkowski_addition) operators.

Here I've defined the opening and closing operations as function compositions of erosion and dilation, but Georges Matheron originally defined them more directly as
```math
\begin{align}
     A_B &= (A \ominus \v{B}) \oplus B
     A^B &= (A \oplus \v{B}) \ominus B
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

The point is to make it less likely that the algorithm outputs edges for gradients that appear only as a result of noise. 

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

**Answer:**

<h3 align="center">Binary Image and Structuring Element</h3>
<a name="figure-1a"></a><a name="figure-1b"></a>
<p align="center">
    <img src="https://github.com/Jesperoka/tdt4195/blob/assignment_5/imgs/figure_1ab.png?raw=true" width=350>
</p>
