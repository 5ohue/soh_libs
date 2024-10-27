use anyhow::{anyhow, Result};
use ash::vk;

#[repr(transparent)]
pub struct Buffer {
    command_buffer: vk::CommandBuffer,
}

// Constructor, destructor
impl Buffer {
    pub fn new(device: &crate::Device, command_pool: &super::Pool) -> Result<Self> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(**command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffers = unsafe { device.allocate_command_buffers(&alloc_info)? };

        let Some(&command_buffer) = command_buffers.first() else {
            return Err(anyhow!("No command buffers were allocated"));
        };

        return Ok(Buffer { command_buffer });
    }
}

// Specific implementation
impl Buffer {
    pub fn reset(&self, device: &crate::Device) -> Result<()> {
        unsafe {
            device.reset_command_buffer(
                self.command_buffer,
                vk::CommandBufferResetFlags::default(),
            )?;
        }
        return Ok(());
    }

    pub fn record(
        &self,
        image_index: usize,
        device: &crate::Device,
        framebuffer: &crate::Framebuffer,
        graphics_pipeline: &crate::Pipeline,
    ) -> Result<()> {
        let extent = framebuffer.extent();

        // Begin command buffer
        let begin_info = vk::CommandBufferBeginInfo::default();
        unsafe { device.begin_command_buffer(**self, &begin_info)? };

        // Starting a render pass
        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];
        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(**framebuffer.render_pass())
            .framebuffer(framebuffer[image_index])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent,
            })
            .clear_values(&clear_values);
        unsafe {
            device.cmd_begin_render_pass(
                self.command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }

        // Bind the pipeline
        unsafe {
            device.cmd_bind_pipeline(
                self.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                **graphics_pipeline,
            );
        }

        // Dynamic state
        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        unsafe {
            device.cmd_set_viewport(self.command_buffer, 0, &[viewport]);
        }
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        };
        unsafe {
            device.cmd_set_scissor(self.command_buffer, 0, &[scissor]);
        }

        // Actually draw
        unsafe {
            device.cmd_draw(self.command_buffer, 3, 1, 0, 0);
        }

        // End the render pass
        unsafe {
            device.cmd_end_render_pass(self.command_buffer);
            device.end_command_buffer(self.command_buffer)?;
        }

        return Ok(());
    }

    pub fn submit(
        &self,
        device: &crate::Device,
        wait_semaphore: &crate::sync::Semaphore,
        signal_semaphore: &crate::sync::Semaphore,
        fence: &crate::sync::Fence,
    ) -> Result<()> {
        let queue = device.get_graphics_queue_family();

        anyhow::ensure!(queue != vk::Queue::null(), "No graphics queue available");

        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let wait_semaphores = &[**wait_semaphore];
        let signal_semaphores = &[**signal_semaphore];
        let command_buffers = &[**self];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(wait_semaphores)
            .signal_semaphores(signal_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers);

        unsafe {
            device.queue_submit(device.get_graphics_queue_family(), &[submit_info], **fence)?;
        }

        return Ok(());
    }
}

// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::CommandBuffer;

    fn deref(&self) -> &Self::Target {
        return &self.command_buffer;
    }
}
