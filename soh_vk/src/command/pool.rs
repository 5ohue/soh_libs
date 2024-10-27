use anyhow::{anyhow, Result};
use ash::vk;

#[repr(transparent)]
pub struct Pool {
    command_pool: vk::CommandPool,
}

// Constructor, destructor
impl Pool {
    pub fn new(device: &crate::Device) -> Result<Self> {
        let Some(graphics_family) = device.physical().queue_family_indices().graphics_family else {
            return Err(anyhow!(
                "No graphics queue family available to create command pool"
            ));
        };

        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(graphics_family);

        let command_pool = unsafe { device.create_command_pool(&create_info, None)? };

        return Ok(Pool { command_pool });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}

// Deref
impl std::ops::Deref for Pool {
    type Target = vk::CommandPool;

    fn deref(&self) -> &Self::Target {
        return &self.command_pool;
    }
}
