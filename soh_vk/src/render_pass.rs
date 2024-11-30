//-----------------------------------------------------------------------------
// https://developer.samsung.com/galaxy-gamedev/resources/articles/renderpasses.html#Using-a-VkRenderPass
//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct RenderPass {
    device: crate::DeviceRef,

    render_pass: vk::RenderPass,
}

//-----------------------------------------------------------------------------
#[derive(Clone, Copy)]
pub struct Attachment {
    pub format: crate::Format,
    pub num_of_samples: u8,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub initial_layout: crate::ImageLayout,
    pub final_layout: crate::ImageLayout,
}

pub type LoadOp = vk::AttachmentLoadOp;
pub type StoreOp = vk::AttachmentStoreOp;

impl From<Attachment> for vk::AttachmentDescription {
    fn from(value: Attachment) -> Self {
        let samples = match value.num_of_samples {
            1 => vk::SampleCountFlags::TYPE_1,
            2 => vk::SampleCountFlags::TYPE_2,
            4 => vk::SampleCountFlags::TYPE_4,
            8 => vk::SampleCountFlags::TYPE_8,
            16 => vk::SampleCountFlags::TYPE_16,
            32 => vk::SampleCountFlags::TYPE_32,
            64 => vk::SampleCountFlags::TYPE_64,
            _ => {
                panic!("The sample count for attachment must be a power of two");
            }
        };

        return vk::AttachmentDescription::default()
            .format(value.format)
            .samples(samples)
            .load_op(value.load_op)
            .store_op(value.store_op)
            .stencil_load_op(value.stencil_load_op)
            .stencil_store_op(value.stencil_store_op)
            .initial_layout(value.initial_layout)
            .final_layout(value.final_layout);
    }
}

impl Default for Attachment {
    fn default() -> Self {
        return Attachment {
            format: crate::Format::default(),
            num_of_samples: 1,
            load_op: LoadOp::DONT_CARE,
            store_op: StoreOp::DONT_CARE,
            stencil_load_op: LoadOp::DONT_CARE,
            stencil_store_op: StoreOp::DONT_CARE,
            initial_layout: crate::ImageLayout::UNDEFINED,
            final_layout: crate::ImageLayout::UNDEFINED,
        };
    }
}

//-----------------------------------------------------------------------------

// Constructor, destructor
impl RenderPass {
    /// Create render pass with only one color attachment with specified format
    pub fn new_simple(device: &crate::DeviceRef, format: crate::Format) -> Result<Self> {
        let color_attachments = &[Attachment {
            format,
            load_op: LoadOp::CLEAR,
            store_op: StoreOp::STORE,
            initial_layout: crate::ImageLayout::UNDEFINED,
            final_layout: crate::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        }];

        return Self::new(device, color_attachments);
    }

    pub fn new(device: &crate::DeviceRef, color_attachments: &[Attachment]) -> Result<Self> {
        /*
         * Declare all of the attachments in the render pass
         * (attachment is a render target and corresponds to an image view in
         * the framebuffer)
         */
        let color_attachments = color_attachments
            .iter()
            .map(|attachment| (*attachment).into())
            .collect::<Vec<_>>();
        /*
         * Declare all the references to the attachments
         * (used by subpasses)
         */
        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL); // Layout DURING the subpass

        /*
         * Declare the subpasses
         */
        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref));

        /*
         * Dependencies between subpasses
         */
        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        /*
         * Create render pass
         */
        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(&color_attachments)
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(std::slice::from_ref(&dependency));

        let render_pass = unsafe { device.create_render_pass(&create_info, None)? };

        return Ok(RenderPass {
            device: device.clone(),
            render_pass,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_render_pass(self.render_pass, None);
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

//-----------------------------------------------------------------------------
