use anyhow::Result;
use ash::vk;

pub struct Buffer {
    buffer: crate::Buffer,

    num_of_vertexes: usize,
    vertex_description: super::VertexDescription,
}

// Getters
impl Buffer {
    pub fn buffer(&self) -> &crate::Buffer {
        return &self.buffer;
    }
    pub fn num_of_vertexes(&self) -> usize {
        return self.num_of_vertexes;
    }
    pub fn vertex_description(&self) -> &super::VertexDescription {
        return &self.vertex_description;
    }
}

// Constructor, destructor
impl Buffer {
    pub fn new<T>(device: &crate::DeviceRef, data: &[T]) -> Result<Self>
    where
        T: super::Vertex,
    {
        let buffer_size = size_of_val(data) as u64;

        /*
         * Create and allocate buffer
         */
        let buffer = crate::Buffer::new(
            device,
            buffer_size,
            crate::BufferUsageFlags::VERTEX_BUFFER,
            crate::MemoryPropertyFlags::HOST_VISIBLE | crate::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        /*
         * Map the memory
         */
        let data_ptr = unsafe {
            device.map_memory(buffer.memory(), 0, buffer_size, vk::MemoryMapFlags::empty())?
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
            device.unmap_memory(buffer.memory());
        }

        return Ok(Buffer {
            buffer,
            num_of_vertexes: data.len(),
            vertex_description: T::get_vertex_description(),
        });
    }

    pub fn destroy(&self) {
        self.buffer.destroy();
    }
}
