<h1 align="center">Assignment 1</h1>

*This assignment was done after the deadline, so I'm not making a full report.*


**As for the theory questions:**<br><br>

Draw a single triangle passing through the following vertices:

$$v_0 = \begin{bmatrix}0.6 \\\ -0.8 \\\ -1.2\end{bmatrix}, v_0 = \begin{bmatrix}0 \\\ 0.4 \\\ 0\end{bmatrix}, v_0 = \begin{bmatrix}-0.8 \\\ -0.2 \\\ 1.2\end{bmatrix}$$

This shape does not entirely look like a triangle.
Explain in your own words:

    T2a)


        Q:

        i)          What is the name of this phenomenon?

        ii)         When does it occur?

        iii)        What is its purpose?



        A:

        i)      Clipping. 

        ii)     When normalized device coordinates (NDCs) are outside clip box ([-1, 1] in this case).

        iii)    Performance.

While drawing one or more triangles, try to swap the order in which
two of the vertices of a single triangle are drawn by modifying the index buffer. A
significant change in the appearance of the triangle should occur.

    T2b)

        Q:

        i) What happens?

        ii) Why does it happen?

        iii) What is the condition under which this effect occurs? Determine a rule.



        A:

        i)      Back-face Culling. 

        ii)     Faces that are facing away from the camera are discarded.

        iii)    By default OpenGL uses counter-clockwise vertex ordering.
                The ordering defines the notion of front and back used in the culling.

Explain the following in your own words:

    T2c)
        Q:
        i) Why does the depth buffer need to be reset each frame?

        ii) In which situation can the Fragment Shader be executed multiple times for the
        same pixel? (Assume we do not use multisampling.)
        
        iii) What are the two most commonly used types of Shaders?
        What are the responsibilities of each of them?
        
        iv) Why is it common to use an index buffer to specify which vertices should be
        connected into triangles, as opposed to relying on the order in which the vertices
        are specified in the vertex buffer(s)?

        v) While the last input of gl::VertexAttribPointer() is a pointer, we usually pass
        in a null pointer. Describe a situation in which you would pass a non-zero value
        into this function.



        A:

        i)      The depth buffer is used for depth testing to determine which pixels are behind other pixels and can be discarded.
                Because of this we need to clear it in order to not discard new pixels based on the previous frame.

        ii)     Overlapping geometry, transparent or semi-transparent objects, or more advanced shader processing such as         
                reflections, refractions, ray-tracing, bloom, motion blur, depth of field. 
                These are some situations where the fragment shader might be used multiple times for the same pixel.

        iii)    Fragment shader and vertex shader are the two most common types of shaders. 
        
                The primary responsibility of the fragment shader is to compute the final color of a pixel. 
                This involves taking interpolated vertex attributes (like texture coordinates or normals) 
                and using them to sample textures, apply lighting models, and combine various factors to produce a color.

                One of the primary responsibilities of the vertex shader is to transform vertex positions from 
                object space (local coordinates) to clip space. This often involves multiplying the 
                vertex position by a series of transformation matrices, such as the model, view, and projection matrices.

        iv)     Without an index buffer, vertices that are shared between multiple triangles would need to be duplicated in the
                vertex buffer. For complex models, this can lead to a significant amount of redundant data. 
                By using an index buffer, each unique vertex is stored only once in the vertex buffer, 
                and the indices determine how they are connected into triangles.

                Index buffers provide flexibility in specifying the order in which triangles are drawn. 
                This can be useful for techniques that require specific draw orders, such as transparency rendering.

        v)      If we have a single vertex buffer object that contains, either semantically or actually, 
                different types of elements that will have different attributes, when we would need to pass in the
                correct offset for any of the subsequent calls to gl::VertextAttribPointer().

                This interleaved approach, where multiple attributes are stored in a single VBO, is common because it can offer better performance due to improved memory locality and cache coherence. In such cases, using non-zero offsets with gl::VertexAttribPointer() is essential to correctly map the various attributes.

                

<h2 align="center">Final Rendering After Assingment</h2>
<p align="center">
<img src="https://github.com/Jesperoka/tdt4195/blob/assignment_1/gloom-rs/report/images/a1_final_result.gif?raw=true" width=350>
</p>
<p align="center">Using fragment and vertex shaders to change colors<br> as a function of time and normalized device coordinates,<br> and position reversal as a function of time.</p>
