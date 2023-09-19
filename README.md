<h1 align="center">Assignment 2</h1>

<h2 align="center">Task 1: Per-Vertex Colors </h2>
<h3 align="left">Task 1b</h3>

**Question:**

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

<h2 align="center">Task 2: Alpha Blending and Depth </h2>
<h3 align="left">Task 2a</h3>

**Question:**

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
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t2a.png?raw=true" width=350>
</p>
<p align="center">Semi-transparent triangles overlapping.</p>

<h3 align="left">Task 2b</h3>

**Questions:**

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

        i)
        The VBO for color can be modified by changing the color values suppled to create_vao(),
        so I've swapped the colors of the furthest and nearest triangles.

<h3 align="center">Task 2b i) Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t2bi.png?raw=true" width=350>
</p>
<p align="center">Semi-transparent triangles overlapping now with colors swapped.</p>

        The result is that colors drawn closer to viewpoint are more dominant, 
        so changing the color of the nearest triangle from green to be blue causes the
        overlapping regions to become dominated by blue instead of by green. This can
        be seen in the figure above.

        Since the alpha values of my triangles are greater than 0.5, the pixel will
        be weighted most heavily with the color of the triangle drawn last, which as
        per the task description is the closest triangle. This can be seen from the
        formula for the blending:

<!-- <p align="center"> -->
$$\mathbf{c}_{\text{new}} = \alpha_{\text{souce}} \cdot \mathbf{c}_{\text{source}} + (1-\alpha_{\text{souce}}) \cdot \mathbf{c}_{\text{destination}}$$
<!-- </p> -->

        where, somewhat confusingly, source refers to the pixel being drawn on top 
        of the already existing destination.

        ii)
        Swapping the depth of the triangles makes it so the closest triangle is drawn
        first, then the middle triangle, and finally the furthest triangle. With depth-testing
        and blending on this means the formula from before no longer has any destination colors
        from the triangles to influence the final pixel color, and as such the triangles appear
        to not be transparent to eachother. As shown in the figure below however, the triangles
        are indeed still transparent as shown by moving the rendering window above my code
        editor.

<h3 align="center">Task 2b ii) Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t2bii.png?raw=true" width=350>
</p>
<p align="center">Semi-transparent triangles overlapping now with z values swapped.</p>

<h2 align="center">Task 3: The Affine Transformation Matrix </h2>
<h3 align="left">Task 3b</h3>

**Question:**

        Individually modify each of the values marked with letters in the
        matrix in equation 2 below, one at a time. In each case use the identity matrix as a
        starting point.

        Observe the effect of modifying the value on the resulting rendered image. Deduce
        which of the four distinct transformation types discussed in the lectures and the
        book modifying the value corresponds to. Also write down the direction (axis) the
        transformation applies to.

$$A =\begin{bmatrix}a & b & 0 & c \\\ d  & e & 0 & f \\\ 0 & 0 & 1 & 0 \\\ 0 & 0 & 0 & 1 \end{bmatrix}$$

        Hint: You can use a “uniform” variable to pass a floating point value slowly oscillating
        between -1 and 1 from the main loop in main.rs into the Vertex Shader. Use the elapsed
        variable in the main loop, and compute the sine of it (elapsed.sin()) before sending
        it into the shader with a uniform variable. See the Hitchhikers guide on how to do
        this.

        It makes the effects of modifying individual values in the transformation matrix much
        easier to recognise. The guide includes a description on how to work with uniform
        variables.

**Answer:**

        All the transformations in this task (shown in the GIF below if viewing HTML or GitHub Markdown versions of this report)
        are homographies (projective transforms) and specifically they are all affine transformations.
        Further specifying the transformations are scaling, shear, translation and reflection, and even further
        specifying they only act along one axis.

<h3 align="center">Task 3b Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t3b.gif?raw=true" width=350>
</p>
<p align="center">GIF of each transform. Viewable at https://github.com/Jesperoka/tdt4195/tree/assignment_2 <br>
The elements a-f of the homography matrix are varied one-by-one in alphabetical order.</p>

        In order of application the transformations are:
                a: scaling along only x-axis
                b: xz shear along only x-axis
                c: translation along only x-axis
                d: yz shear along only y-axis
                e: scaling along only y-axis
                f: translation along only y-axis

<h3 align="left">Task 3c</h3>

**Question:**

        Why can you be certain that none of the transformations observed were rotations?

**Answer:**

        Because there is no general 3D rotation matrix that only differs from the identity matrix in exactly one element.
        This can be shown from orthogonality and the fact that reflections are not included in SO(3).
        The identity matrix itself is the only rotation matrix seen in this experiment.

        Note that one could also have concluded that there are no rotations because rotations preserve the origin,
        lengths, angles and chirality/handedness.


<h2 align="center">Task 4: Combinations of Transformations  </h2>
<h3 align="left">Task 4b</h3>

In the description of this task there is mention of how using 
```rs
glm::perspective()
// which is a wrapper for 
glm::perspective_rh_no()
```
causes the z-axis to be flipped because the clip-space is a left-handed coordinate system. To avoid this I have opted to use
```rs
glm::perspective_lh_no()
```
which conveniently does the z-axis negation for me, thus I don't have to reorder the drawing of objects in the scene.<br>
This is shown in the figure below.
<h3 align="center">Task 4b Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t4b_rh.gif?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t4b_lh.gif?raw=true" width=350>
</p>
<p align="center">Comparison og draw order for left- and right-handed coordinate systems.</p>

<h2 align="center">Task 4c and 5a: Optional Bonus Challenges  </h2>

<h3 align="left">Task 4c and 5a</h3>

**Question:**

        The camera you implemented in the previous task works well in many
        situations, however, it works a bit counterintuitively when the camera moves strictly
        along major axes without taking into account which direction it’s facing.

        Change the camera control scheme such that the “forward”, “backward”, “left”, “right”,
        “up”, and “down” motions move relative to the direction the camera is facing, rather
        than having all of them move you in the direction of each major axis regardless of
        camera orientation.

        Tip: You may want to limit range of motion of the vertical rotation of the camera (pitch)
        to between “straight down” and “straight up” to avoid weird camera movement.

        Also, there’s a really neat trick here if you properly understand your transformation
        matrices that allows you to turn movement relative to the camera into movement
        relative to the world, which solves this question in very few lines of code.

**Answer:**

        By keeping track of the rotation matrix during the rendering loop one can convert the
        movement commands from the keypresses to be relative to the camera pose by premultiplying
        the resulting translation vector by the rotation matrix, giving the transformed translation vector
        which can then be added to the total translation matrix. The movement that results can be seen in the
        figure below.

<h3 align="center">Task 4c and 5a Rendering 1</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t5b.gif?raw=true" width=350>
</p>
<p align="center">Movement in the 3D scene. GIF viewable at https://github.com/Jesperoka/tdt4195/tree/assignment_2</p>

        I also added rotation control via mouse press-and-drag in the direction you want to turn,
        to help with the movement jumping around if you move your cursor out of the screen.
        
        Additionally, I switched from event handling with DeviceEvent::MouseMotion struct 
        to using the WindowEvent::CursorMoved event and just keeping track of the previous cursor position
        to get the deltas, since it didn't work for me, probably because of WSL2.

        To see the objects while behind them as well I turned of back-culling, but I didn't bother trying
        any techniques to make blending work properly from both the front and back.

        Finally, instead of completing more bonus challenges, I had fun with shader art 
        (inspired by: https://www.shadertoy.com/view/mtyGWy) and scene geometry. The final result is seen
        in the figure below.

<h3 align="center">Task 4c and 5a Rendering 2</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_2/imgs/a2_t5b_2.gif?raw=true" width=350>
</p>
<p align="center">Another 3D scene with some fun shader art. GIF viewable at https://github.com/Jesperoka/tdt4195/tree/assignment_2</p>