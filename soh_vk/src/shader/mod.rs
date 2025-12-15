//-----------------------------------------------------------------------------
mod manager;
//-----------------------------------------------------------------------------
pub use manager::*;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

//-----------------------------------------------------------------------------

pub struct Shader {
    device: crate::DeviceRef,
    shader: vk::ShaderModule,
}

//-----------------------------------------------------------------------------
// Constructor
impl Shader {
    pub fn new(device: &crate::DeviceRef, shader_manager: &Manager, path: &str) -> Result<Shader> {
        let shader_code = shader_manager.get_shader(path)?;

        let create_info = vk::ShaderModuleCreateInfo::default().code(&shader_code);

        let shader = unsafe { device.create_shader_module(&create_info, None)? };

        return Ok(Shader {
            device: device.clone(),
            shader,
        });
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.shader, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Shader {
    type Target = vk::ShaderModule;

    fn deref(&self) -> &Self::Target {
        return &self.shader;
    }
}

//-----------------------------------------------------------------------------
