//-----------------------------------------------------------------------------
use ash::vk;
// Format ---------------------------------------------------------------------

pub type Format = vk::Format;

// For buffers (and images) ---------------------------------------------------

pub type BufferUsageFlags = vk::BufferUsageFlags;
pub type ImageUsageFlags = vk::ImageUsageFlags;
pub type MemoryPropertyFlags = vk::MemoryPropertyFlags;
pub type ImageLayout = vk::ImageLayout;

// For Pipelines --------------------------------------------------------------

pub type PrimitiveTopology = vk::PrimitiveTopology;
pub type PipelineStageFlags = vk::PipelineStageFlags;

// Other types ----------------------------------------------------------------

pub enum QueueType {
    Graphics,
    Present,
    Transfer,
}

//-----------------------------------------------------------------------------
