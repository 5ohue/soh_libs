//-----------------------------------------------------------------------------
mod buffer;
//-----------------------------------------------------------------------------
pub use buffer::Buffer;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

pub fn copy_buffer(
    device: &crate::Device,
    pool: &crate::cmd::Pool,
    src: &Buffer,
    dst: &Buffer,
) -> Result<()> {
    /*
     * Create transfer command buffer
     */
    let cmd_buf = pool.allocate_buffer(crate::cmd::BufferLevel::Primary)?;

    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    let copy_info = vk::BufferCopy {
        size: u64::min(src.size(), dst.size()),
        ..Default::default()
    };

    unsafe {
        device.begin_command_buffer(*cmd_buf, &begin_info)?;
        device.cmd_copy_buffer(*cmd_buf, **src, **dst, std::slice::from_ref(&copy_info));
        device.end_command_buffer(*cmd_buf)?;
    }

    cmd_buf.submit_and_wait()?;

    unsafe {
        device.free_command_buffers(**pool, std::slice::from_ref(&cmd_buf));
    }

    return Ok(());
}

//-----------------------------------------------------------------------------
