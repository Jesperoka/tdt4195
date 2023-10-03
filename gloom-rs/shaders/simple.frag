#version 420 core

in vec4 rgba;
in vec3 nxnynz;
in vec3 xyz_light_space; 

out vec4 color;

uniform sampler2D shadow_map;

void main()
{
    vec3 light_direction = normalize(vec3(0.8, -0.5, 0.6));

    // Shadow map testing
    float light_view_surface_depth = texture(shadow_map, xyz_light_space.xy).r;
    float bias = 0;// max(0.05 * (1.0 - dot(normalize(nxnynz), light_direction)), 0.005);
    float dot_prod = dot(normalize(nxnynz), -light_direction);
    float shadow_mix_factor = xyz_light_space.z + bias < light_view_surface_depth ? 0.0f : 0.5f*(1.0 - min(1.0, 1.8*pow(1.1, pow(dot_prod, dot_prod)) - 1.0 )  ); 
    vec4 shadow_color = vec4(0.1, 0.1, 0.1, 1.0f);

    vec4 lambertian_shade = vec4(rgba.rgb * max(0, dot(nxnynz, -light_direction)), rgba.a);

    color = mix(lambertian_shade, shadow_color, shadow_mix_factor);
}
