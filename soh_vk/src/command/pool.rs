use anyhow::{anyhow, Result};
use ash::vk;

pub struct Pool {
    device: crate::DeviceRef,

    command_pool: vk::CommandPool,
    queue_family_index: u32,
}

// Getters
impl Pool {
    pub fn command_pool(&self) -> vk::CommandPool {
        return self.command_pool;
    }

    pub fn queue_family_index(&self) -> u32 {
        return self.queue_family_index;
    }
}

// Constructor, destructor
impl Pool {
    pub fn new(device: &crate::DeviceRef, queue_family_index: u32) -> Result<Self> {
        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index);

        let command_pool = unsafe { device.create_command_pool(&create_info, None)? };

        return Ok(Pool {
            device: device.clone(),
            command_pool,
            queue_family_index,
        });
    }

    pub fn new_graphics(device: &crate::DeviceRef) -> Result<Self> {
        let graphics_family = device.physical().queue_family_indices().graphics_family;

        return Self::new(device, graphics_family);
    }

    pub fn new_transfer(device: &crate::DeviceRef) -> Result<Self> {
        let transfer_family = device.physical().queue_family_indices().transfer_family;

        return Self::new(device, transfer_family);
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
        }
    }
}

// Specific implementation
impl Pool {
    pub fn allocate_buffer(&self, level: super::BufferLevel) -> Result<super::Buffer> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(**self)
            .level(level.into())
            .command_buffer_count(1);

        let command_buffers = unsafe { self.device.allocate_command_buffers(&alloc_info)? };

        let Some(&command_buffer) = command_buffers.first() else {
            return Err(anyhow!("No command buffers were allocated"));
        };

        return Ok(super::Buffer::from_handle(
            self.device.clone(),
            command_buffer,
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

        let command_buffers = unsafe { self.device.allocate_command_buffers(&alloc_info)? };

        anyhow::ensure!(
            command_buffers.len() == count as usize,
            "Number of allocated buffers doesn't match the requested count!"
        );

        let res = command_buffers
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

// Deref
impl std::ops::Deref for Pool {
    type Target = vk::CommandPool;

    fn deref(&self) -> &Self::Target {
        return &self.command_pool;
    }
}
