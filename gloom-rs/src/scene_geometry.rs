
pub const VERTICES: &[f32; 39] = &[
    // Pyramid
    0.0, 0.25, 0.0,         // Apex
    -0.25, -0.25, -0.25,   // Base - Bottom Left
    0.25, -0.25, -0.25,    // Base - Bottom Right
    0.25, -0.25, 0.25,     // Base - Top Right
    -0.25, -0.25, 0.25,    // Base - Top Left

    // Cube (Positioned to the right of the pyramid)
    0.75, 0.0, 0.25,       // Front Top Left
    1.0, 0.0, 0.25,        // Front Top Right
    1.0, -0.25, 0.25,      // Front Bottom Right
    0.75, -0.25, 0.25,     // Front Bottom Left
    0.75, 0.0, 0.0,        // Back Top Left
    1.0, 0.0, 0.0,         // Back Top Right
    1.0, -0.25, 0.0,       // Back Bottom Right
    0.75, -0.25, 0.0       // Back Bottom Left
];

pub const INDICES: &[u32; 54] = &[
    // Pyramid
    0, 1, 2,  // Front
    0, 2, 3,  // Right
    0, 3, 4,  // Back
    0, 4, 1,  // Left
    1, 4, 3,  // Base Left triangle
    1, 3, 2,  // Base Right triangle

    // Cube
    5, 6, 7, 5, 7, 8,    // Front face
    9, 10, 11, 9, 11, 12, // Back face
    5, 9, 12, 5, 12, 8,  // Left face
    6, 10, 11, 6, 11, 7, // Right face
    5, 9, 10, 5, 10, 6,  // Top face
    8, 12, 11, 8, 11, 7  // Bottom face
];

pub const COLORS: &[f32; 52] = &[
    // Pyramid (Pastel Orange)
    1.0, 0.7, 0.278, 0.7,
    1.0, 0.7, 0.278, 0.7,
    1.0, 0.7, 0.278, 0.7,
    1.0, 0.7, 0.278, 0.7,
    1.0, 0.7, 0.278, 0.7,

    // Cube (Pastel Blue)
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,
    0.678, 0.847, 0.902, 0.7,

];

