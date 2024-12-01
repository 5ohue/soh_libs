use anyhow::Result;
use ash::vk;

pub struct Buffer {
    buffer: crate::Buffer,

    num_of_indexes: usize,
    index_type: vk::IndexType,
}

// Getters
impl Buffer {
    pub fn buffer(&self) -> &crate::Buffer {
        return &self.buffer;
    }
    pub fn num_of_indexes(&self) -> usize {
        return self.num_of_indexes;
    }
    pub fn index_type(&self) -> vk::IndexType {
        return self.index_type;
    }
}

// Constructor, destructor
impl Buffer {
    pub fn new_u16(context: &crate::VulkanContext, indexes: &[u16]) -> Result<Buffer> {
        let buffer = crate::Buffer::new_staged(
            context.device(),
            unsafe { context.cmd_pool_transfer() },
            indexes,
            crate::BufferUsageFlags::INDEX_BUFFER,
        )?;

        return Ok(Buffer {
            buffer,
            num_of_indexes: indexes.len(),
            index_type: vk::IndexType::UINT16,
        });
    }

    pub fn new_u32(context: &crate::VulkanContext, indexes: &[u32]) -> Result<Buffer> {
        let buffer = crate::Buffer::new_staged(
            context.device(),
            unsafe { context.cmd_pool_transfer() },
            indexes,
            crate::BufferUsageFlags::INDEX_BUFFER,
        )?;

        return Ok(Buffer {
            buffer,
            num_of_indexes: indexes.len(),
            index_type: vk::IndexType::UINT32,
        });
    }

    pub fn free(&self) {
        self.buffer.free();
    }
}
