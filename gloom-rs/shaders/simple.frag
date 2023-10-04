#version 420 core

in vec4 rgba;
in vec3 nxnynz;
in vec3 xyz_light_space; 

out vec4 color;

uniform sampler2DShadow shadow_map;

vec2 poisson_disk[16] = vec2[]( 
   vec2( -0.94201624, -0.39906216 ), 
   vec2( 0.94558609, -0.76890725 ), 
   vec2( -0.094184101, -0.92938870 ), 
   vec2( 0.34495938, 0.29387760 ), 
   vec2( -0.91588581, 0.45771432 ), 
   vec2( -0.81544232, -0.87912464 ), 
   vec2( -0.38277543, 0.27676845 ), 
   vec2( 0.97484398, 0.75648379 ), 
   vec2( 0.44323325, -0.97511554 ), 
   vec2( 0.53742981, -0.47373420 ), 
   vec2( -0.26496911, -0.41893023 ), 
   vec2( 0.79197514, 0.19090188 ), 
   vec2( -0.24188840, 0.99706507 ), 
   vec2( -0.81409955, 0.91437590 ), 
   vec2( 0.19984126, 0.78641367 ), 
   vec2( 0.14383161, -0.14100790 ) 
);

void main()
{
    // Shadow mapping with PCF
    vec3 light_direction = normalize(vec3(0.8, -0.5, 0.6));
    vec4 shadow_color = vec4(0.1, 0.1, 0.1, 1.0f);

    float spread_factor = 1700.0; // just hardcoding this for now
    float sampled_depth = 0.0;
    int NUM_SAMPES = 16;
    float bias = clamp(0.05 * (1.0 - 1.1*dot(normalize(nxnynz), light_direction)), -0.005, 0.05);

    for(int i = 0; i < NUM_SAMPES; i++) {
        sampled_depth += texture(shadow_map, vec3(xyz_light_space.xy + poisson_disk[i]/spread_factor, xyz_light_space.z), bias);
    }
    sampled_depth = sampled_depth / NUM_SAMPES; 

    float dot_prod = dot(normalize(nxnynz), -light_direction);
    float shadow_mix_factor = min(1.0, 1.0 - (2.8*pow((dot_prod - 0.5), 2) + 0.3)); 

    vec4 lambertian_shade = vec4(rgba.rgb * max(0, dot(nxnynz, -light_direction)), rgba.a);

    color = mix(mix(lambertian_shade, shadow_color, 1.0 - shadow_mix_factor*sampled_depth), lambertian_shade, 0.93*lambertian_shade);
}
