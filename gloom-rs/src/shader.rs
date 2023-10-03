use gl;
use std::{
    ptr,
    str,
    ffi::CString,
    path::Path,
    error::Error,
};

pub struct Shader {
    pub program_id: u32,
}

pub struct ShaderBuilder {
    program_id: u32,
    shaders: Vec::<u32>,
}

#[allow(dead_code)]
pub enum ShaderType {
    Vertex,
    Fragment,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
}

impl Shader {
    pub fn placeholder() -> Shader {
        Shader {program_id: u32::MAX - 69420} // placeholder program_id
    }
    // Make sure the shader is active before calling this
    pub unsafe fn get_uniform_location(&self, name: &str) -> i32 {
        let name_cstr = CString::new(name).expect("CString::new failed");
        gl::GetUniformLocation(self.program_id, name_cstr.as_ptr())
    }

    pub unsafe fn activate(&self) {
        gl::UseProgram(self.program_id);
    }
}

impl Into<gl::types::GLenum> for ShaderType {
    fn into(self) -> gl::types::GLenum {
        match self {
            ShaderType::Vertex                  => { gl::VERTEX_SHADER          },
            ShaderType::Fragment                => { gl::FRAGMENT_SHADER        },
            ShaderType::TessellationControl     => { gl::TESS_CONTROL_SHADER    },
            ShaderType::TessellationEvaluation  => { gl::TESS_EVALUATION_SHADER } ,
            ShaderType::Geometry                => { gl::GEOMETRY_SHADER        },
        }
    }
}

impl ShaderType {
    fn from_ext(ext: &std::ffi::OsStr) -> Result<ShaderType, String> {
        match ext.to_str().expect("Failed to read extension") {
            "vert" => { Ok(ShaderType::Vertex) },
            "frag" => { Ok(ShaderType::Fragment) },
            "tcs"  => { Ok(ShaderType::TessellationControl) },
            "tes"  => { Ok(ShaderType::TessellationEvaluation) },
            "geom" => { Ok(ShaderType::Geometry) },
            e => { Err(e.to_string()) },
        }
    }
}

impl ShaderBuilder {
    pub unsafe fn new() -> ShaderBuilder {
        ShaderBuilder {
            program_id: gl::CreateProgram(),
            shaders: vec![],
        }
    }

    pub unsafe fn attach_file(self, shader_path: &str) -> ShaderBuilder {
        let path = Path::new(shader_path);
        if let Some(extension) = path.extension() {
            let shader_type = ShaderType::from_ext(extension)
                .expect("Failed to parse file extension.");
            let shader_src = std::fs::read_to_string(path)
                .expect(&format!("Failed to read shader source. {}", shader_path));
            self.compile_shader(&shader_src, shader_type)
        } else {
            panic!("Failed to read extension of file with path: {}", shader_path);
        }
    }

    pub unsafe fn compile_shader(mut self, shader_src: &str, shader_type: ShaderType) -> ShaderBuilder {
        let shader = gl::CreateShader(shader_type.into());
        let c_str_shader = CString::new(shader_src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_shader.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        if !self.check_shader_errors(shader) {
            panic!("Shader failed to compile.");
        }

        self.shaders.push(shader);

        self
    }

    unsafe fn check_shader_errors(&self, shader_id: u32) -> bool {
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                shader_id,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            println!("ERROR::Shader Compilation Failed!\n{}", String::from_utf8_lossy(&info_log));
            return false;
        }
        true
    }

    unsafe fn check_linker_errors(&self) -> bool {
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetProgramiv(self.program_id, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetProgramInfoLog(
                self.program_id,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
            return false;
        }
        true
    }

    #[must_use = "The shader program is useless if not stored in a variable."]
    pub unsafe fn link(self) -> Shader {
        for &shader in &self.shaders {
            gl::AttachShader(self.program_id, shader);
        }
        gl::LinkProgram(self.program_id);

        // todo:: use this to make safer abstraction
        self.check_linker_errors();

        for &shader in &self.shaders {
            gl::DeleteShader(shader);
        }

        Shader {
            program_id: self.program_id
        }
    }
}

// Putting shadow mapping here because where else
pub const SHADOW_RES: i32 = 2*3840; 

pub fn create_depth_framebuffer() -> Result<(u32, u32), Box<dyn Error>> {
    // A separate framebuffer is needed for the shadow map 
    let mut framebuffer_id: u32 = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut framebuffer_id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);
    }

    // Depthmap is stored as a texture that can be sampled 
    let mut depth_texture: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut depth_texture);
        gl::BindTexture(gl::TEXTURE_2D, depth_texture);
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::DEPTH_COMPONENT32F as i32, 
            SHADOW_RES, 
            SHADOW_RES, 
            0, 
            gl::DEPTH_COMPONENT, 
            gl::FLOAT, 
            std::ptr::null()
            );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, depth_texture, 0);

        gl::DrawBuffer(gl::NONE);
    }

    // Check that the framebuffer is ok
    if unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) } != gl::FRAMEBUFFER_COMPLETE {
        return Err(Box::<dyn Error>::from("Framebuffer is not complete!".to_string()));
    }

    return Ok((framebuffer_id, depth_texture));
}

pub fn compute_depth_mvp_matrix() -> glm::Mat4 {
    let light_inv_dir = glm::vec3(0.8, -0.5, 0.6); // this is not inv_ but light direction itself
    let up = glm::vec3(0.0, 1.0, 0.0);
    let depth_projection_matrix = glm::ortho_lh_zo(-1200.0, 1200.0, -1200.0, 1200.0, 0.01, 1000.0);
    let depth_view_matrix = glm::look_at(&light_inv_dir, &glm::vec3(0.0, 0.0, 0.0), &up);

    // Rotating first allows for better alignment of view frustum with scene 
    let rotation_matrix = glm::rotate(&glm::Mat4::identity(), -0.995, &light_inv_dir);
    let depth_mvp = depth_projection_matrix * depth_view_matrix; // * rotation_matrix;

    return depth_mvp;
}

