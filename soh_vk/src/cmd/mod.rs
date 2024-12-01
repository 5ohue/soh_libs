//-----------------------------------------------------------------------------
// https://vkguide.dev/docs/extra-chapter/multithreading/
//-----------------------------------------------------------------------------
mod buffer;
mod pool;
//-----------------------------------------------------------------------------
pub use buffer::*;
pub use pool::*;
//-----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferLevel {
    Primary,
    Secondary,
}

impl From<BufferLevel> for ash::vk::CommandBufferLevel {
    fn from(value: BufferLevel) -> Self {
        match value {
            BufferLevel::Primary => ash::vk::CommandBufferLevel::PRIMARY,
            BufferLevel::Secondary => ash::vk::CommandBufferLevel::SECONDARY,
        }
    }
}

//-----------------------------------------------------------------------------
