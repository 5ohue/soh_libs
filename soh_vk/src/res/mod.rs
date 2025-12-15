//-----------------------------------------------------------------------------
mod buffer;
mod image;
mod memory;
//-----------------------------------------------------------------------------
pub use buffer::*;
pub use image::*;
pub use memory::*;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

//-----------------------------------------------------------------------------

pub fn copy_buffer(
    device: &crate::Device,
    transfer_pool: &crate::cmd::Pool,
    src: &Buffer,
    dst: &Buffer,
    size: u64,
) -> Result<()> {
    assert!(size <= src.size());
    assert!(size <= dst.size());

    /*
     * Create transfer command buffer
     */
    let cmd_buf = transfer_pool.allocate_buffer(crate::cmd::BufferLevel::Primary)?;

    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    let copy_info = vk::BufferCopy {
        size,
        ..Default::default()
    };

    unsafe {
        device.begin_command_buffer(*cmd_buf, &begin_info)?;
        device.cmd_copy_buffer(*cmd_buf, **src, **dst, std::slice::from_ref(&copy_info));
        device.end_command_buffer(*cmd_buf)?;
    }

    cmd_buf.submit_and_wait()?;

    unsafe {
        device.free_command_buffers(**transfer_pool, std::slice::from_ref(&cmd_buf));
    }

    return Ok(());
}

//-----------------------------------------------------------------------------
/// Get the pixel size in bytes for a particular format
pub fn format_size(format: vk::Format) -> u64 {
    match format {
        vk::Format::R8G8B8A8_UNORM => 4,
        vk::Format::R8G8B8A8_SRGB => 4,
        vk::Format::B8G8R8A8_UNORM => 4,
        vk::Format::B8G8R8A8_SRGB => 4,
        vk::Format::R32G32B32A32_SFLOAT => 16,
        _ => panic!("Unsupported format"),
    }
}

//-----------------------------------------------------------------------------
