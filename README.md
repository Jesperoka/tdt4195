<h1 align="center">Assignment 2</h1>

<h2 align="left">Task 1: Per-Vertex Colors </h2>
<h3 align="left">Task 1b</h3>

**Question**:

        Render a scene containing at least 3 different triangles, where
        each vertex of each triangle has a different color. Put a screenshot of the result in
        your report. All triangles should be visible on the screenshot. Briefly explain what
        OpenGL does in between the vertices for each fragment with the vertex attributes.

**Answer:**

<h3 align="center">Task 1b Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t1.png?raw=true" width=350>
</p>
<p align="center">Triangles with colors interpolated between vertices.</p>

        For a triangle with vertices A, B, and C, if you specify a color for each vertex, 
        OpenGL will interpolate these colors (linearly) for each fragment (pixel) inside the triangle.
        The closer a fragment is to a vertex, the more it will resemble the color of that vertex. 
        This results in a gradient effect across the triangle. The interpolated attribute does not have to be color.


<h2 align="left">Task 2: Alpha Blending and Depth </h2>
<h3 align="left">Task 2a</h3>

**Question**:

        For this task, we want to show you what happens to a blended
        color when multiple triangles overlap from the camera's perspective 
        (where the triangles are positioned at different distances from the camera).

        To this end, draw at least 3 triangles that satisfy the following conditions:

                • There exists a section on the xy-plane on which all triangles overlap.
                • For any single triangle, the three vertices of that triangle have the same zcoordinate.
                • No two triangles share the same z-coordinate.
                • All triangles have a transparent color (alpha < 1).
                • All triangles have a different color.
                • Each triangle's vertices have the same color.

        I do not recommend drawing the exact same triangle 3 times at different depths;
        it will make the next question more difficult to solve.

        Make sure your triangles are being drawn ordered back-to-front. That is, the triangle
        furthest away from the screen is drawn first, the second-furthest one next, and so on.
        You can change the draw order of triangles by modifying the index buffer. 
        
        Remember how the coordinate space of the Clipbox works here: Positive Z is deeper, negative is closer.
        Put a screenshot in your report.

**Answer:**

<h3 align="center">Task 2a Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t1.png?raw=true" width=350>
</p>
<p align="center">temp.</p>

<h3 align="left">Task 2b</h3>

**Questions**:

        i) 
        Swap the colors of different triangles by modifying the VBO containing the color
        Vertex Attribute. Observe the effect of these changes on the blended color of
        the area in which all triangles overlap. What effects on the blended color did you
        observe, and how did exchanging triangle colors cause these changes to occur?

        ii) 
        Swap the depth (z-coordinate) at which different triangles are drawn by modifying
        the VBO containing the triangle vertices. Again observe the effect of these
        changes on the blended color of the overlapping area. Which changes in the
        blended color did you observe, and how did the exchanging of z-coordinates
        cause these changes to occur? Why was the depth buffer the cause this effect?

**Answers:**


<h2 align="left">Task 3: The Affine Transformation Matrix </h2>
<h3 align="left">Task 3b</h3>

**Question**:

        Individually modify each of the values marked with letters in the
        matrix in equation 2 below, one at a time. In each case use the identity matrix as a
        starting point.

        Observe the effect of modifying the value on the resulting rendered image. Deduce
        which of the four distinct transformation types discussed in the lectures and the
        book modifying the value corresponds to. Also write down the direction (axis) the
        transformation applies to.

$$\begin{bmatrix}a & b & 0 & c \\\ d  & e & 0 & f \\\ 0 & 0 & 1 & 0 \\\ 0 & 0 & 0 & 1 \end{bmatrix}$$

        Hint: You can use a “uniform” variable to pass a floating point value slowly oscillating
        between -1 and 1 from the main loop in main.rs into the Vertex Shader. Use the elapsed
        variable in the main loop, and compute the sine of it (elapsed.sin()) before sending
        it into the shader with a uniform variable. See the Hitchhikers guide on how to do
        this.

        It makes the effects of modifying individual values in the transformation matrix much
        easier to recognise. The guide includes a description on how to work with uniform
        variables.