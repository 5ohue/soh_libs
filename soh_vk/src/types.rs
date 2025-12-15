//-----------------------------------------------------------------------------
use ash::vk;
//-----------------------------------------------------------------------------
// Format
pub type Format = vk::Format;
//-----------------------------------------------------------------------------
// For buffers and images
pub type BufferUsageFlags = vk::BufferUsageFlags;
pub type MemoryPropertyFlags = vk::MemoryPropertyFlags;
pub type ImageLayout = vk::ImageLayout;
//-----------------------------------------------------------------------------
// Other types
#[derive(Debug, Clone, Copy)]
pub enum QueueType {
    Graphics,
    Present,
    Transfer,
}
//-----------------------------------------------------------------------------
