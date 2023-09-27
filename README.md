<h1 align="center">Assignment 3</h1>

*Note: Allow some time for the GIFs to load if viewing HTML or GitHub Markdown versions of report.<br> 
PDF version does not support GIFs, so the other options are recommended.*

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
        hints of red, and blue. Usually no grays, though. (You should see for the most part the
        color green. If you see a rainbow pattern then you’ve most likely got an erroneous
        vertex attrib pointer or a wrong vector size in the shaders.)
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
        from. We’re creating a light source that’s infinitely far away here, which on the moon
        effectively will be the sun.
        vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
        Computing light accurately is immensely complicated and computationally intensive.
        So instead we’re going to use what’s known as a “lighting model”; an often crude
        approximation that looks plausible to the human eye.
        The lighting model we are going to be using assumes that every object reflects light
        equally much in all directions (obviously not the case, but it’s easy to compute). This is
        called the Lambertian model. You can read more about it at https://en.wikipedia.
        org/wiki/Lambert%27s_cosine_law, but it’s not required for this assignment.
        The equation we want you to use to compute the final color4
        in the fragment shader
        looks like this, when written with mathematical notation:
        colorRGB ∗ max(0, vnormal · −vlight direction)
        When you set the fragment color to this, all of the details in the lunar surface should
        become a lot more visible.
        Attach a screenshot which shows that the surface is correctly lit.

 **Answer:**

<h3 align="center">Task 1d Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1d_1.png?raw=true" width=350>
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t1d_2.png?raw=true" width=350>
</p>
<p align="center">Surface with lighting. Red background confirms shading does not make pixels transparent.</p>


<h2 align="center">Task 2: Helicopter Parenting </h2>
<h3 align="left">Task 2a</h3>

        This task is about setting up the scene graph and using it to render the models. 
        The figure below shows the loaded helicopter model.

<h3 align="center">Task 2a Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t2a.png?raw=true" width=350>
</p>
<p align="center">Loaded helicopter model.</p>

<h3 align="left">Task 2b</h3>

**Questions:**



**Answers:**



<h3 align="center">Task 2c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t2c.png?raw=true" width=350>
</p>
<p align="center">Helicopter model rendered using scene graph traversal.</p>


<h2 align="center">Task 3: The (Model) Matrix: Revolutions </h2>

        Something something model matrix

<h3 align="center">Task 3c Rendering</h3>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_3/imgs/a3_t3.png?raw=true" width=350>
</p>
<p align="center">Helicopter with rotated body, door and rotors.</p>


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
        rotation and scale applied to the model, and none of the translation. We then multiply
        this by the normal vector, which results in it being rotated and scaled by the same
        amount that the object it’s attached to would be. Finally, we normalize the result, since
        the normal vectors should always be of unit length (i.e. have a magnitude equal to 1).
        This has the effect of reverting any uniform scaling present in the Model matrix.8
        Attach two new screenshots taken from the same camera position, where we see two
        sides of the helicopter.

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
        the same VAOs / meshes. Keep track of the relevant scene nodes and animate them
        in the main loop. Every helicopter should have a rotating tail rotor and main rotor, as
        well as follow some kind of path like in the previous task.
        Every one of the helicopters should follow the same path, but none of them should
        collide with any other. You can accomplish this by feeding the animation function a
        carefully selected offset.
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

<h2 align="center">Task 7: Optional Challenges </h2>
<h3 align="left">Task 7a</h3>

<h3 align="left">Task 7e</h3>

        I think I already did this?