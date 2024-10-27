use anyhow::Result;
use ash::vk;

#[repr(transparent)]
pub struct RenderPass {
    render_pass: vk::RenderPass,
}

// Constructor, destructor
impl RenderPass {
    pub fn new(device: &crate::Device, format: vk::Format) -> Result<Self> {
        // Declare all of the attachments in the render pass
        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        // Declare all the references to the attachments
        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let color_attachment_refs = &[color_attachment_ref];

        // Declare the subpasses
        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachment_refs);

        // Dependencies
        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let color_attachments = &[color_attachment];
        let subpasses = &[subpass];
        let dependencies = &[dependency];

        // Create render pass
        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(color_attachments)
            .subpasses(subpasses)
            .dependencies(dependencies);

        let render_pass = unsafe { device.create_render_pass(&create_info, None)? };

        return Ok(RenderPass { render_pass });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            device.destroy_render_pass(self.render_pass, None);
        }
    }
}

// Deref
impl std::ops::Deref for RenderPass {
    type Target = vk::RenderPass;

    fn deref(&self) -> &Self::Target {
        return &self.render_pass;
    }
}
