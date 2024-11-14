use anyhow::Result;
use ash::vk::{self, Handle};

pub struct Buffer {
    device: crate::DeviceRef,

    command_buffer: vk::CommandBuffer,
    level: super::BufferLevel,
    queue_family_index: u32,
}

// Specific implementation
impl Buffer {
    pub fn reset(&self) -> Result<()> {
        unsafe {
            self.device.reset_command_buffer(
                self.command_buffer,
                vk::CommandBufferResetFlags::default(),
            )?;
        }
        return Ok(());
    }

    pub fn record(
        &self,
        image_index: usize,
        framebuffer: &crate::Framebuffer,
        graphics_pipeline: &crate::Pipeline,
    ) -> Result<()> {
        /*
         * Begin command buffer
         */
        let begin_info = vk::CommandBufferBeginInfo::default();
        unsafe { self.device.begin_command_buffer(**self, &begin_info)? };

        /*
         * Starting a render pass
         */
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
                extent: framebuffer.extent(),
            })
            .clear_values(&clear_values);
        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }

        /*
         * Bind the pipeline
         */
        unsafe {
            self.device.cmd_bind_pipeline(
                self.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                **graphics_pipeline,
            );
        }

        /*
         * Dynamic state
         */
        let (viewport, scissor) = framebuffer.get_viewport_scissor();
        unsafe {
            self.device
                .cmd_set_viewport(self.command_buffer, 0, std::slice::from_ref(&viewport));
            self.device
                .cmd_set_scissor(self.command_buffer, 0, std::slice::from_ref(&scissor));
        }

        /*
         * Actually draw
         */
        unsafe {
            self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);
        }

        /*
         * End the render pass
         */
        unsafe {
            self.device.cmd_end_render_pass(self.command_buffer);
            self.device.end_command_buffer(self.command_buffer)?;
        }

        return Ok(());
    }

    /// Submit the command buffer to the queue
    ///
    /// * `wait_semaphore`: the semaphore to wait for signal
    /// * `signal_semaphore`: the semaphore which should get signaled once the command is executed
    /// * `fence`: the fence that should be signaled once the execution completes
    pub fn submit(
        &self,
        wait_semaphore: &crate::sync::Semaphore,
        signal_semaphore: &crate::sync::Semaphore,
        fence: Option<&crate::sync::Fence>,
    ) -> Result<()> {
        let queue = self.device.get_queue(self.queue_family_index);

        // Cannot submit to null queue
        debug_assert!(!queue.is_null());
        // Only submit primary buffers
        debug_assert_eq!(self.level, super::BufferLevel::Primary);

        // This means that the pipeline is going to wait for the color attachment to be available
        // ( so that GPU can run vertex shader before the image is available for example )
        let wait_stages = std::slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(std::slice::from_ref(wait_semaphore))
            .signal_semaphores(std::slice::from_ref(signal_semaphore))
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(std::slice::from_ref(self));

        let fence = crate::get_opt_handle(fence);

        unsafe {
            self.device
                .queue_submit(queue, std::slice::from_ref(&submit_info), fence)?;
        }

        return Ok(());
    }

    #[inline(always)]
    pub(super) fn from_handle(
        device: crate::DeviceRef,
        buffer: vk::CommandBuffer,
        level: super::BufferLevel,
        queue_family_index: u32,
    ) -> Self {
        assert!(!buffer.is_null());

        return Buffer {
            device,
            command_buffer: buffer,
            level,
            queue_family_index,
        };
    }
}

// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::CommandBuffer;

    fn deref(&self) -> &Self::Target {
        return &self.command_buffer;
    }
}
