#version 420 core

in vec4 rgba;

out vec4 color;

uniform float time;
uniform mat4 homography;
uniform vec2 resolution;

vec3 palette( float t ) {
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.263,0.416,0.557);

    return a + b * cos(6.28318 * (c * t + d));
}

void main()
{
    vec2 pos = (gl_FragCoord.xy - resolution * 0.5) / resolution; // Adjust fragment position and normalize by resolution
    vec2 uv = pos;
    vec2 uv0 = uv;
    vec3 finalColor = vec3(0.0);
    
    for (float j = 0.0; j < 3.0; j++) {
        uv = uv0 + vec2(j) * 0.1;  // Add an offset based on the outer loop iteration
        
        for (float i = 0.0; i < 4.0; i++) {
            uv = fract(uv * (1.5 + j * 0.5)) - 0.5;  // Modify UV based on outer loop iteration

            float d = length(uv) * exp(-length(uv0));

            vec3 col = palette(length(uv0) + i * 0.4 + time * 0.4);

            d = sin(d * (8.0 + j * 2.0) + time) / 8.0;  // Modify frequency based on outer loop iteration
            d = abs(d);

            d = pow(0.01 / d, 1.2);

            finalColor += col * d;
        }
    }
        
    color = mix(rgba, vec4(finalColor, 1.0), 0.5);
}
