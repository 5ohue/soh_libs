use anyhow::{anyhow, Result};
use ash::vk;

pub struct Buffer {
    device: crate::DeviceRef,

    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

// Getters
impl Buffer {
    pub fn buffer(&self) -> vk::Buffer {
        return self.buffer;
    }
    pub fn memory(&self) -> vk::DeviceMemory {
        return self.memory;
    }
}

// Constructor, destructor
impl Buffer {
    pub fn new(
        device: &crate::DeviceRef,
        buffer_size: u64,
        usage: crate::BufferUsageFlags,
        properties: crate::MemoryPropertyFlags,
    ) -> Result<Self> {
        /*
         * Create the buffer
         */
        let create_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
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
         * Bind memory
         */
        unsafe {
            device.bind_buffer_memory(buffer, memory, 0)?;
        }

        return Ok(Buffer {
            device: device.clone(),
            buffer,
            memory,
        });
    }

    pub fn new_vertex_buffer<T>(device: &crate::DeviceRef, data: &[T]) -> Result<Self>
    where
        T: crate::Vertex,
    {
        let buffer_size = size_of_val(data) as u64;

        /*
         * Create and allocate buffer
         */
        let res = Self::new(
            device,
            buffer_size,
            crate::BufferUsageFlags::VERTEX_BUFFER,
            crate::MemoryPropertyFlags::HOST_VISIBLE | crate::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        /*
         * Map the memory
         */
        let data_ptr =
            unsafe { device.map_memory(res.memory, 0, buffer_size, vk::MemoryMapFlags::empty())? };

        /*
         * Write the data to the mapped memory
         */
        let src_slice: &[i8] =
            unsafe { std::slice::from_raw_parts(data.as_ptr().cast(), buffer_size as usize) };
        let dst_slice: &mut [i8] =
            unsafe { std::slice::from_raw_parts_mut(data_ptr.cast(), buffer_size as usize) };

        assert!(src_slice.len() == dst_slice.len());

        for (&src, dst) in src_slice.iter().zip(dst_slice.iter_mut()) {
            *dst = src;
        }

        /*
         * Unmap
         */
        unsafe {
            device.unmap_memory(res.memory);
        }

        return Ok(res);
    }

    pub fn destroy(&self) {
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
