use anyhow::Result;
use ash::vk;

pub struct Buffer {
    buffer: vk::Buffer,
}

// Constructor, destructor
impl Buffer {
    pub fn new() -> Result<Self> {
        // let create_info = vk::BufferCreateInfo::default();

        todo!()
    }
}

// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::Buffer;

    fn deref(&self) -> &Self::Target {
        return &self.buffer;
    }
}
