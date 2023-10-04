use crate::mesh::{Mesh, Terrain, Helicopter};
use crate::create_vao;

pub const NUM_VAOS: usize = 5;

pub struct Scene {
    pub vao_ids: [u32; NUM_VAOS],
    pub triangle_counts: [i32; NUM_VAOS],
}

// Instantiate all VAOs in the scene
pub fn init_scene_geometry(terrain_model_path: &str, helicopter_model_path: &str) -> Scene {

    let terrain_mesh: Mesh = Terrain::load(terrain_model_path);
    let helicopter_meshes: Helicopter = Helicopter::load(helicopter_model_path);

    return Scene{
        vao_ids: create_vaos(&terrain_mesh, &helicopter_meshes),
        triangle_counts: count_indices(&terrain_mesh, &helicopter_meshes)
    };
}


fn create_vaos(terrain_mesh: &Mesh, helicopter_meshes: &Helicopter) -> [u32; NUM_VAOS] {

    let terrain_vao_id = unsafe { 
        create_vao(
            &terrain_mesh.vertices,
            &terrain_mesh.normals,
            &terrain_mesh.colors, 
            &terrain_mesh.indices,
            )};

    let helicopter_body_vao_id = unsafe { 
        create_vao(
            &helicopter_meshes.body.vertices,
            &helicopter_meshes.body.normals,
            &helicopter_meshes.body.colors, 
            &helicopter_meshes.body.indices,
            )};
    
    let helicopter_door_vao_id = unsafe { 
        create_vao(
            &helicopter_meshes.door.vertices,
            &helicopter_meshes.door.normals,
            &helicopter_meshes.door.colors, 
            &helicopter_meshes.door.indices,
            )};
    
    let helicopter_main_rotor_vao_id = unsafe { 
        create_vao(
            &helicopter_meshes.main_rotor.vertices,
            &helicopter_meshes.main_rotor.normals,
            &helicopter_meshes.main_rotor.colors, 
            &helicopter_meshes.main_rotor.indices,
            )};

    let helicopter_tail_rotor_vao_id = unsafe { 
        create_vao(
            &helicopter_meshes.tail_rotor.vertices,
            &helicopter_meshes.tail_rotor.normals,
            &helicopter_meshes.tail_rotor.colors, 
            &helicopter_meshes.tail_rotor.indices,
            )};

    return [terrain_vao_id, 
            helicopter_body_vao_id, 
            helicopter_door_vao_id, 
            helicopter_main_rotor_vao_id, 
            helicopter_tail_rotor_vao_id];
}

fn count_indices(terrain: &Mesh, heli: &Helicopter) -> [i32; NUM_VAOS] {
    return [
        terrain.indices.len() as i32, 
        heli.body.indices.len() as i32,
        heli.door.indices.len() as i32,
        heli.main_rotor.indices.len() as i32,
        heli.tail_rotor.indices.len() as i32,
    ];
}

// fn count_total_helicopter_indices(heli: &Helicopter) -> usize {
//     return heli.body.indices.len()
//         + heli.door.indices.len()
//         + heli.main_rotor.indices.len()
//         + heli.tail_rotor.indices.len();
// }
