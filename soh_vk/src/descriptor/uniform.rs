//-----------------------------------------------------------------------------
use anyhow::Result;
//-----------------------------------------------------------------------------

pub struct Buffer {
    buffer: crate::Buffer,
}

//-----------------------------------------------------------------------------
// Getters
impl Buffer {
    pub fn buffer(&self) -> &crate::Buffer {
        return &self.buffer;
    }
}

//-----------------------------------------------------------------------------
// Constructor
impl Buffer {
    pub fn new(device: &crate::DeviceRef, size: u64) -> Result<Self> {
        /*
         * Create buffer
         */
        let mut buffer = crate::Buffer::new(
            device,
            size,
            crate::BufferUsageFlags::UNIFORM_BUFFER,
            crate::MemoryPropertyFlags::HOST_VISIBLE | crate::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        /*
         * Map the memory ( to use "persistent mapping" )
         */
        buffer.memory_mut().map()?;

        return Ok(Buffer { buffer });
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Buffer {
    pub fn write<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Copy,
    {
        return self.buffer.memory_mut().write(data);
    }
}

//-----------------------------------------------------------------------------
