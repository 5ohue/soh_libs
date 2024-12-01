//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk::{self, Handle};
//-----------------------------------------------------------------------------

pub struct Buffer {
    device: crate::DeviceRef,

    cmd_buffer: vk::CommandBuffer,
    level: super::BufferLevel,
    queue_family_index: u32,
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Buffer {
    pub fn reset(&self) -> Result<()> {
        unsafe {
            self.device
                .reset_command_buffer(**self, vk::CommandBufferResetFlags::default())?;
        }
        return Ok(());
    }

    /**************************************************************************
     *                          Recording functions                           *
     **************************************************************************/

    pub fn begin(&self, flags: vk::CommandBufferUsageFlags) -> Result<()> {
        let begin_info = vk::CommandBufferBeginInfo::default().flags(flags);

        unsafe { self.device.begin_command_buffer(**self, &begin_info)? };

        return Ok(());
    }

    pub fn end(&self) -> Result<()> {
        unsafe {
            self.device.end_command_buffer(**self)?;
        }
        return Ok(());
    }

    //-------------------------------------------------------------------------

    pub fn begin_render_pass(
        &self,
        framebuffer: &crate::Framebuffer,
        render_pass: &crate::RenderPass,
    ) {
        static CLEAR_VALUE: vk::ClearValue = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };

        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(**render_pass)
            .framebuffer(**framebuffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: framebuffer.extent(),
            })
            .clear_values(std::slice::from_ref(&CLEAR_VALUE));

        unsafe {
            self.device.cmd_begin_render_pass(
                **self,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }
    }

    pub fn end_render_pass(&self) {
        unsafe {
            self.device.cmd_end_render_pass(**self);
        }
    }

    //-------------------------------------------------------------------------

    pub fn set_fb_viewport_scissor(&self, framebuffer: &crate::Framebuffer) {
        let (viewport, scissor) = framebuffer.get_viewport_scissor();

        unsafe {
            self.device
                .cmd_set_viewport(**self, 0, std::slice::from_ref(&viewport));
            self.device
                .cmd_set_scissor(**self, 0, std::slice::from_ref(&scissor));
        }
    }

    //-------------------------------------------------------------------------

    pub fn bind_pipeline(&self, graphics_pipeline: &crate::Pipeline) {
        unsafe {
            self.device.cmd_bind_pipeline(
                **self,
                vk::PipelineBindPoint::GRAPHICS,
                **graphics_pipeline,
            );
        }
    }

    pub fn bind_vertex_buffer(&self, vertex_buffer: &crate::vertex::Buffer) {
        unsafe {
            self.device.cmd_bind_vertex_buffers(
                **self,
                0,
                &[vertex_buffer.buffer().buffer()],
                &[0],
            );
        }
    }

    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device.cmd_draw(
                **self,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
        }
    }

    /**************************************************************************
     *                            Submit functions                            *
     **************************************************************************/

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
        let queue = self.get_queue_handle();

        // This means that the pipeline is going to wait for the color attachment to be available
        // ( so that GPU can run vertex shader before the image is available for example )
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

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

    pub fn submit_and_wait(&self) -> Result<()> {
        let queue = self.get_queue_handle();

        let submit_info = vk::SubmitInfo::default().command_buffers(std::slice::from_ref(self));

        unsafe {
            self.device.queue_submit(
                queue,
                std::slice::from_ref(&submit_info),
                vk::Fence::null(),
            )?;
            self.device.queue_wait_idle(queue)?;
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
            cmd_buffer: buffer,
            level,
            queue_family_index,
        };
    }

    fn get_queue_handle(&self) -> vk::Queue {
        let queue = self.device.get_queue(self.queue_family_index);

        // Cannot submit to null queue
        debug_assert!(!queue.is_null());
        // Only submit primary buffers
        debug_assert_eq!(self.level, super::BufferLevel::Primary);

        return queue;
    }
}

//-----------------------------------------------------------------------------

// Deref
impl std::ops::Deref for Buffer {
    type Target = vk::CommandBuffer;

    fn deref(&self) -> &Self::Target {
        return &self.cmd_buffer;
    }
}

//-----------------------------------------------------------------------------
