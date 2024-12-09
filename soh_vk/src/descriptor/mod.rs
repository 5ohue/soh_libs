//-----------------------------------------------------------------------------
mod pool;
mod set;
mod set_layout;
//-----------------------------------------------------------------------------
pub mod uniform;
//-----------------------------------------------------------------------------
pub use pool::*;
pub use set::*;
pub use set_layout::*;
//-----------------------------------------------------------------------------

use ash::vk;

#[derive(Debug, Clone, Copy)]
pub struct SetLayoutBinding {
    pub descriptor_type: vk::DescriptorType,
    pub count: u32,
    pub state_flags: vk::ShaderStageFlags,
}

impl Default for SetLayoutBinding {
    fn default() -> Self {
        return SetLayoutBinding {
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            count: 1,
            state_flags: vk::ShaderStageFlags::ALL_GRAPHICS,
        };
    }
}

//-----------------------------------------------------------------------------
