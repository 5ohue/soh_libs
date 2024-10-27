//-----------------------------------------------------------------------------
mod manager;
//-----------------------------------------------------------------------------
pub use manager::*;
//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;

#[repr(transparent)]
pub struct Shader {
    shader: vk::ShaderModule,
}

// Constructor, destructor
impl Shader {
    pub fn new(device: &crate::Device, shader_manager: &Manager, path: &str) -> Result<Shader> {
        let shader_code = shader_manager.get_shader(path)?;

        let create_info = vk::ShaderModuleCreateInfo::default().code(&shader_code);

        let shader = unsafe { device.create_shader_module(&create_info, None)? };

        return Ok(Shader { shader });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            device.destroy_shader_module(self.shader, None);
        }
    }
}

// Deref
impl std::ops::Deref for Shader {
    type Target = vk::ShaderModule;

    fn deref(&self) -> &Self::Target {
        return &self.shader;
    }
}

//-----------------------------------------------------------------------------
