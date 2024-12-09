use anyhow::Result;

pub struct Buffer {
    buffer: crate::Buffer,
    data_ptr: *mut std::ffi::c_void,
}

// Getters
impl Buffer {
    pub fn buffer(&self) -> &crate::Buffer {
        return &self.buffer;
    }
}

// Constructor, destructor
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
        let data_ptr = buffer.map()?;

        return Ok(Buffer { buffer, data_ptr });
    }

    pub fn free(&self) {
        self.buffer.free();
    }
}

// Specific implementation
impl Buffer {
    pub fn write<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Copy,
    {
        let buffer_size = size_of_val(data) as u64;

        anyhow::ensure!(
            self.buffer.size() >= buffer_size,
            "Buffer memory is smaller than the data that is being written to it"
        );

        unsafe { crate::Buffer::write_memory(self.data_ptr, data) };

        return Ok(());
    }
}
