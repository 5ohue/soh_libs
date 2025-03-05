//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Pool {
    device: crate::DeviceRef,

    cmd_pool: vk::CommandPool,
    queue_family_index: u32,
}

//-----------------------------------------------------------------------------
// Getters
impl Pool {
    pub fn queue_family_index(&self) -> u32 {
        return self.queue_family_index;
    }
}

//-----------------------------------------------------------------------------
// Constructor, destructor
impl Pool {
    /// Creates a command pool that is used to do graphics operations
    pub fn new_graphics(device: &crate::DeviceRef) -> Result<Self> {
        let graphics_family = device.physical().queue_family_indices().graphics_family;

        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(graphics_family);

        let cmd_pool = unsafe { device.create_command_pool(&create_info, None)? };

        return Ok(Pool {
            device: device.clone(),
            cmd_pool,
            queue_family_index: graphics_family,
        });
    }

    /// Creates a command pool that is used to do data transfers
    pub fn new_transfer(device: &crate::DeviceRef) -> Result<Self> {
        let transfer_family = device.physical().queue_family_indices().transfer_family;

        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(transfer_family);

        let cmd_pool = unsafe { device.create_command_pool(&create_info, None)? };

        return Ok(Pool {
            device: device.clone(),
            cmd_pool,
            queue_family_index: transfer_family,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_command_pool(self.cmd_pool, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Pool {
    pub fn allocate_buffer(&self, level: super::BufferLevel) -> Result<super::Buffer> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(**self)
            .level(level.into())
            .command_buffer_count(1);

        let cmd_buffers = unsafe { self.device.allocate_command_buffers(&alloc_info)? };

        let Some(&cmd_buffer) = cmd_buffers.first() else {
            anyhow::bail!("No command buffers were allocated");
        };

        return Ok(super::Buffer::from_handle(
            self.device.clone(),
            cmd_buffer,
            level,
            self.queue_family_index,
        ));
    }

    pub fn allocate_buffers(
        &self,
        level: super::BufferLevel,
        count: u32,
    ) -> Result<Vec<super::Buffer>> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(**self)
            .level(level.into())
            .command_buffer_count(count);

        let cmd_buffers = unsafe { self.device.allocate_command_buffers(&alloc_info)? };

        anyhow::ensure!(
            cmd_buffers.len() == count as usize,
            "Number of allocated buffers doesn't match the requested count"
        );

        let res = cmd_buffers
            .iter()
            .map(|vk_buf| {
                super::Buffer::from_handle(
                    self.device.clone(),
                    *vk_buf,
                    level,
                    self.queue_family_index,
                )
            })
            .collect::<Vec<_>>();

        return Ok(res);
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Pool {
    type Target = vk::CommandPool;

    fn deref(&self) -> &Self::Target {
        return &self.cmd_pool;
    }
}

//-----------------------------------------------------------------------------
