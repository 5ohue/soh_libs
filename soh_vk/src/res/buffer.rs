//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Buffer {
    device: crate::DeviceRef,

    buffer: vk::Buffer,
    usage: crate::BufferUsageFlags,

    memory: super::Memory,
}

//-----------------------------------------------------------------------------
// Getters
impl Buffer {
    pub fn buffer(&self) -> vk::Buffer {
        return self.buffer;
    }
    pub fn memory(&self) -> &super::Memory {
        return &self.memory;
    }
    pub fn size(&self) -> u64 {
        return self.memory.size();
    }
    pub fn memory_mut(&mut self) -> &mut super::Memory {
        return &mut self.memory;
    }
    pub fn usage(&self) -> crate::BufferUsageFlags {
        return self.usage;
    }
}

//-----------------------------------------------------------------------------
// Constructors
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

        /*
         * Allocate memory
         */
        let memory = super::Memory::alloc(device, memory_requirements, properties)?;

        /*
         * Bind allocted memory to buffer
         */
        unsafe {
            device.bind_buffer_memory(buffer, *memory, 0)?;
        }

        return Ok(Buffer {
            device: device.clone(),
            buffer,
            memory,
            usage,
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
        let mut buffer = Self::new(
            device,
            buffer_size,
            usage,
            crate::MemoryPropertyFlags::HOST_VISIBLE | crate::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        buffer.memory_mut().map_and_write(data)?;

        return Ok(buffer);
    }

    /// Create a buffer using a staging buffer.
    /// This method creates a host-visible temporary staging buffer, copies the data into it, and
    /// then transfers the data to a device-local ( faster to use by GPU ) buffer.
    pub fn new_staged<T>(
        device: &crate::DeviceRef,
        transfer_pool: &crate::cmd::Pool,
        data: &[T],
        usage: crate::BufferUsageFlags,
    ) -> Result<Self>
    where
        T: Copy,
    {
        let buffer_size = size_of_val(data) as u64;

        /*
         * Create staging buffer (a host visible buffer with data written to it)
         */
        let staging_buffer = Self::new_mapped(device, data, crate::BufferUsageFlags::TRANSFER_SRC)?;

        /*
         * Create the result buffer (device local)
         */
        let buffer = Self::new(
            device,
            buffer_size,
            usage | crate::BufferUsageFlags::TRANSFER_DST,
            crate::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        /*
         * Copy from staging buffer to the result
         */
        super::copy_buffer(device, transfer_pool, &staging_buffer, &buffer, buffer_size)?;

        return Ok(buffer);
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_buffer(self.buffer, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::Buffer;

    fn deref(&self) -> &Self::Target {
        return &self.buffer;
    }
}

//-----------------------------------------------------------------------------
