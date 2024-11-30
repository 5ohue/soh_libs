use anyhow::Result;

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
        let buffer =
            crate::Buffer::new_mapped(device, data, crate::BufferUsageFlags::VERTEX_BUFFER)?;

        return Ok(Buffer {
            buffer,
            num_of_vertexes: data.len(),
            vertex_description: T::get_vertex_description(),
        });
    }

    pub fn free(&self) {
        self.buffer.free();
    }
}
