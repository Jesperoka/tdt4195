<h1 align="center">Assignment 3</h1>

*Note: Allow some time for the GIFs to load if viewing HTML or [GitHub Markdown versions](https://github.com/Jesperoka/tdt4195/tree/assignment_3) of report.<br> 
PDF version does not support GIFs, so the other options are recommended.*

<br>

<h2 align="center">Task 1: More polygons than you can shake a stick at </h2>
<h3 align="left">Task 1c</h3>

**Question:**

        Extend the vertex shader to take the new normals as an input,
        and pass them straight on to the fragment shader through an output variable.

        Extend the fragment shader to take in the normals that the vertex shader outputs.

        After you’ve done this, visualize the normals for a model by using the x, y and z
        coordinates of the normals as the r, g and b values for our fragments.
        
        This will make some of the colors negative (i.e. black), but that’s fine for this example.
        When you do this, your scene should end up looking very colorful. Lots of green, with
        hints of red, and blue. Usually no grays, though. You should see for the most part the
        color green. If you see a rainbow pattern then you’ve most likely got an erroneous
        vertex attrib pointer or a wrong vector size in the shaders.

        Position your camera to point into one of the craters in the scene, and attach a
        screenshot of this immense natural beauty in your report.

**Answer:**

<h3 align="center">Task 1c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1c_2.png?raw=true" width=350>
</p>
<p align="center">Colored surface normals.</p>

<h3 align="left">Task 1d</h3>

**Question:**

        Finally, we want you to implement some very simple lighting.
        In the fragment shader, create a variable holding the direction that the light is coming
        from. 

        We’re creating a light source that’s infinitely far away here, which on the moon
        effectively will be the sun.

        vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

        Computing light accurately is immensely complicated and computationally intensive.
        So instead we’re going to use what’s known as a “lighting model”; an often crude
        approximation that looks plausible to the human eye.

        The lighting model we are going to be using assumes that every object reflects light
        equally much in all directions (obviously not the case, but it’s easy to compute). This is
        called the Lambertian model. You can read more about it at https://en.wikipedia.
        org/wiki/Lambert%27s_cosine_law, but it’s not required for this assignment.

        The equation we want you to use to compute the final color in the fragment shader
        looks like this, when written with mathematical notation:

$$\text{colorRGB} ∗ max(0, v_{\text{normal}} · −v_{\text{light direction}})$$

        When you set the fragment color to this, all of the details in the lunar surface should become a lot more visible.
        
        Attach a screenshot which shows that the surface is correctly lit.

 **Answer:**

<h3 align="center">Task 1d Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1d_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1d_2.png?raw=true" width=350>
</p>
<p align="center">Surface with lighting. Red background confirms shading does not make pixels transparent.</p>

<br>

<h2 align="center">Task 2: Helicopter Parenting </h2>
<h3 align="left">Task 2a</h3>

        This task is about setting up the scene graph and using it to render the models. 
        The figure below shows the loaded helicopter model.

<h3 align="center">Task 2a Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t2a.png?raw=true" width=350>
</p>
<p align="center">Loaded helicopter model.</p>

<h3 align="left">Task 2c</h3>

**Question:**

        Now that we have the scene structured into a scene graph
        hierarchy, we can use it to determine what to draw instead of just calling the draw
        function for each VAO manually.

        We won’t be needing the code you wrote to draw the VAOs in the earlier tasks, so you
        can now get rid of the drawing code in your main loop.

        Drawing the scene involves iterating over every SceneNode (starting from the root
        node), binding its VAO and then calling gl::DrawElements().

        This snippet should get you most of the way there:

```rust
unsafe fn draw_scene(node: &scene_graph::SceneNode, 
                     view_projection_matrix: &glm::Mat4, 
                     transformation_so_far: &glm::Mat4) {

        // Perform any logic needed before drawing the node
        // Check if node is drawable, if so: set uniforms, bind VAO and draw VAO
        // Recurse

        for &child in &node.children {
                draw_scene(&*child, view_projection_matrix, transformation_so_far);
        }
}
```
        Note, the view_projection_matrix parameter will be used later, but don’t worry
        about it for now. Just pass in the camera matrix you’re already computing in the main
        loop, and call it a day. Also note the transformation_so_far parameter. Same story
        here, simply pass in a 4×4 identity matrix and move on.

        If you use simple_shader.get_uniform_location("uniform_name") to locate your
        uniform variables: 

                try either to (1) pass the shader object in as a parameter to draw_scene,

                or (2) store the shader object as a global variable, 
                
                (3) define draw_scene inside of the rendering loop such that it has access to its closure/scope, 

                or (4) hardcode the location of the uniforms using the layout(location=X) qualifier in the vertex shader.
        
        Attach a screenshot showing the helicopter being drawn.

**Answers:**

<h3 align="center">Task 2c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t2c.png?raw=true" width=350>
</p>
<p align="center">Helicopter model rendered using scene graph traversal.</p>

<br>

<h2 align="center">Task 3: The (Model) Matrix: Revolutions </h2>

        We verify that the relative transformations of the child nodes correctly follow the parent nodes by rotating the helicopter, which can be seen in the figure below.

<h3 align="center">Task 3c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t3.png?raw=true" width=350>
</p>
<p align="center">Helicopter with rotated body, door and rotors.</p>

<br>

<h2 align="center">Task 4: Spinning into gear </h2>
<h3 align="left">Task 4b</h3>

<h3 align="center">Task 4b Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t4.gif?raw=true" width=350>
</p>
<p align="center">Animated helicopter following a figure-eight path.</p>

<h2 align="center">Task 5: Help! My lighting is wrong! </h2>
<h3 align="left">Task 5a</h3>

**Question:**

        Notice anything weird about how the helicopter is lit as it moves
        around? The lighting doesn’t change even though the helicopter is turning!

        This is because the normal vectors are defined in relation the model as if the model
        still had its original orientation. If we want the shadow to move across the helicopter
        when the helicopter rotates, we have to rotate the normals in the same way that we
        rotate the model.
        
        Attach two screenshots staken from the same camera position, one where we see the
        brightly lit side of the helicopter, and another taken when it has rotated and we see a
        darker side instead.

**Answer:**

<h3 align="center">Task 5a Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t5a_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t5a_2.png?raw=true" width=350>
</p>
<p align="center">Incorrect helicopter lighting.</p>

<h3 align="left">Task 5c</h3>

**Question:**

        Before we multiply the Model matrix with the vertex normal, we’ll
        need to make some minor modifications to it to ensure it does not scale or translate
        the normal vector.

        We do this by taking the top 3x3 part of the Model matrix, which gets us all of the
        rotation and scale applied to the model, and none of the translation. 
        
        We then multiply this by the normal vector, which results in it being rotated and scaled by the same
        amount that the object it’s attached to would be. 
        
        Finally, we normalize the result, since the normal vectors should always be of unit length (i.e. have a magnitude equal to 1).
        This has the effect of reverting any uniform scaling present in the Model matrix.

        Attach two new screenshots taken from the same camera position, where we see two sides of the helicopter.

**Answer:**

<h3 align="center">Task 5c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t5c_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t5c_2.png?raw=true" width=350>
</p>
<p align="center">More correct helicopter lighting.</p>

<h2 align="center">Task 6: Time to turn this thing up to 5 </h2>
<h3 align="left">Task 6a</h3>

**Question:**

        Everything you have done so far should be easily extendable to
        be done in loops, so that’s what we’re going to do for the final part of this assignment.

        Instead of creating a single helicopter, instantiate at least 5 helicopters that share
        the same VAOs / meshes. Keep track of the relevant scene nodes and animate them in the main loop. 

        Every helicopter should have a rotating tail rotor and main rotor, as
        well as follow some kind of path like in the previous task.

        Every one of the helicopters should follow the same path, but none of them should
        collide with any other. 
        
        You can accomplish this by feeding the animation function a carefully selected offset.

        Attach a screenshot of the five helicopters.

**Answer:**

        The figure below shows the still image of the five helicopters.

<h3 align="center">Task 6a Rendering 1</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t6.png?raw=true" width=350>
</p>
<p align="center">More correct helicopter lighting.</p>

        Assuming you are viewing the HTML or GitHub Markdown versions of this report,
        the figure below shows a GIF of the five animated helicopters.

<h3 align="center">Task 6a Rendering 2</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t6.gif?raw=true" width=350>
</p>
<p align="center">Five helicopters following a figure-eight path without colliding.</p>

<br>

<h2 align="center">Task 7: Optional Challenges </h2>

<h3 align="left">Task 7e</h3>

        I've been using the intrinsic rotation matrix:

$$
\begin{align}
R = R_z(\alpha) \, R_y(\beta) \, R_x(\gamma) = \begin{bmatrix}
        \cos\alpha\cos\beta &
        \cos\alpha\sin\beta\sin\gamma - \sin\alpha\cos\gamma &
        \cos\alpha\sin\beta\cos\gamma + \sin\alpha\sin\gamma \\
        \sin\alpha\cos\beta &
        \sin\alpha\sin\beta\sin\gamma + \cos\alpha\cos\gamma &
        \sin\alpha\sin\beta\cos\gamma - \cos\alpha\sin\gamma \\
-\sin\beta & \cos\beta\sin\gamma & \cos\beta\cos\gamma \\
\end{bmatrix}
\end{align}
$$

        directly the whole time.

<h3 align="left">Task 7g</h3>

        I decided to implement shadow mapping, which poses some challenges due to the size of the scene and the angle of the light.

<h3 align="center">Task 7g Rendering 1</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_shadow_acne.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_no_shadow_acne.png?raw=true" width=350>
</p>
<p align="center">Shadow mapping without and with front-face culling.</p>

        Using front-face culling removes a lot of the shadow acne, especially the shadow acne from self-shadowing. I didn't have much luck with biases to remove shadow acne at first, but it did end up helping in the final version.
        
        Another issue is how the shadow mapping plays with the lambertian shading. Because the lambertian shading only uses the angle of the surface normals to determine the lighting, we get some very bright spots and some very hard shadow edges. To mitigate this I first tried using a nonlinear blending of the lambertian shading and the shadow mapping.

<h3 align="center">Task 7g Rendering 2</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_linear_blend.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_nonlinear_blend.png?raw=true" width=350>
</p>
<p align="center">Nonlinear blending function of shadows and lambertian shading.</p>

        The results are not entirely convincing, and tehcnically the hard shadows are more physically correct on the moon where there is not atmosphere to diffuse the light. On the other hand, there could be some reflection due to metallic dust or something.

        For large scenes it can be tricky to have high enough resolution in the shadow map, especially if you want to shade the whole scene at once. To best utilize the pixels you have, it can help to align the view frustum of the light source with the scene itself. 

<h3 align="center">Task 7g Rendering 3</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_before_scene_fit_lightview.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_before_scene_fit.png?raw=true" width=350>
</p>
<p align="center">Scene from lightsource and on the ground before better fitting of the view frustum.</p>

        To best utilize the pixels you have, it can help to align the view frustum of the light source with the scene itself. 

<h3 align="center">Task 7g Rendering 4</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_scene_fit_lightview.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_scene_fit.png?raw=true" width=350>
</p>
<p align="center">Scene from lightsource and on the ground after better fitting of the view frustum.</p>

        It's not a major improvement, but the resolution of the shadows is effectively a bit higher than before. Note also that the rotation is done first, so as to not change the orthograpic projection of the scene, which is what the directional light uses. As you might have noticed, I also added some skins to the helicopters (by just using a separate "fancy" shader for them).

        Finally I changed from just simple shadow mapping to percentage closed filtering (PCF), with deterministic poisson sampling (randomized didn't look as good) and some additional nonlinear blending with the lambertian shading to mimick some reflection. I was at the very least happy with having a lighter region appear close to the base of the shadow, which looks like reflected light.

        The final result can be seen in the GIF below if viewing the report in HTML or GitHub Markdown formats.

<h3 align="center">Task 7g Rendering 5</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t7_final_result.gif?raw=true" width=450>
</p>
<p align="center">Scene from lightsource and on the ground after better fitting of the view frustum.</p>