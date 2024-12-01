use anyhow::{anyhow, Result};
use ash::vk;

pub struct Buffer {
    device: crate::DeviceRef,

    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    size: u64,

    usage: crate::BufferUsageFlags,
    properties: crate::MemoryPropertyFlags,
}

// Getters
impl Buffer {
    pub fn buffer(&self) -> vk::Buffer {
        return self.buffer;
    }
    pub fn memory(&self) -> vk::DeviceMemory {
        return self.memory;
    }
    pub fn size(&self) -> u64 {
        return self.size;
    }
    pub fn usage(&self) -> crate::BufferUsageFlags {
        return self.usage;
    }
    pub fn properties(&self) -> crate::MemoryPropertyFlags {
        return self.properties;
    }
}

// Constructor, destructor
impl Buffer {
    pub fn new(
        device: &crate::DeviceRef,
        size: u64,
        usage: crate::BufferUsageFlags,
        properties: crate::MemoryPropertyFlags,
    ) -> Result<Self> {
        /*
         * Create the buffer
         */
        let create_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&create_info, None)? };

        /*
         * Get memory requirements
         */
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        // Find which GPU memory type to use for allocation
        let Some(memory_type_index) = device
            .physical()
            .find_memory_type(memory_requirements.memory_type_bits, properties)
        else {
            return Err(anyhow!("Failed to find memory type"));
        };

        /*
         * Allocate memory
         */
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.allocate_memory(&alloc_info, None)? };

        /*
         * Bind allocted memory to buffer
         */
        unsafe {
            device.bind_buffer_memory(buffer, memory, 0)?;
        }

        return Ok(Buffer {
            device: device.clone(),
            buffer,
            memory,
            size,
            usage,
            properties,
        });
    }

    /// Create the buffer with data by mapping buffer and writing to it
    /// (making it HOST_VISIBLE)
    ///
    /// * `device`: logical device to use to create buffer
    /// * `data`: data to write to the buffer
    pub fn new_mapped<T>(
        device: &crate::DeviceRef,
        data: &[T],
        usage: crate::BufferUsageFlags,
    ) -> Result<Self>
    where
        T: Copy,
    {
        let buffer_size = size_of_val(data) as u64;

        /*
         * Create and allocate buffer
         */
        let buffer = Self::new(
            device,
            buffer_size,
            usage,
            crate::MemoryPropertyFlags::HOST_VISIBLE | crate::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        /*
         * Map the memory
         */
        let data_ptr = unsafe {
            device.map_memory(buffer.memory, 0, buffer_size, vk::MemoryMapFlags::empty())?
        };

        /*
         * Write the data to the mapped memory
         */
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().cast(), data_ptr, buffer_size as usize);
        }

        /*
         * Unmap
         */
        unsafe {
            device.unmap_memory(buffer.memory);
        }

        return Ok(buffer);
    }

    pub fn new_staged<T>(
        device: &crate::DeviceRef,
        pool: &crate::cmd::Pool,
        data: &[T],
        usage: crate::BufferUsageFlags,
    ) -> Result<Self>
    where
        T: Copy,
    {
        /*
         * Create staging buffer (a host visible buffer with data written to it)
         */
        let staging_buffer = Self::new_mapped(device, data, crate::BufferUsageFlags::TRANSFER_SRC)?;

        /*
         * Create the result buffer (device local)
         */
        let buffer = Self::new(
            device,
            staging_buffer.size(),
            usage | crate::BufferUsageFlags::TRANSFER_DST,
            crate::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        /*
         * Copy from staging buffer to the result
         */
        super::copy_buffer(device, pool, &staging_buffer, &buffer)?;

        /*
         * Free the staging buffer
         */
        staging_buffer.free();

        return Ok(buffer);
    }

    pub fn free(&self) {
        unsafe {
            self.device.free_memory(self.memory, None);
            self.device.destroy_buffer(self.buffer, None);
        }
    }
}

// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::Buffer;

    fn deref(&self) -> &Self::Target {
        return &self.buffer;
    }
}
