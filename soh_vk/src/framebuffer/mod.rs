//-----------------------------------------------------------------------------
pub mod render_pass;
//-----------------------------------------------------------------------------
pub use render_pass::RenderPass;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

pub struct Framebuffer {
    extent: vk::Extent2D,

    image_views: Vec<vk::ImageView>,
    render_pass: RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
}

// Getters
impl Framebuffer {
    pub fn extent(&self) -> vk::Extent2D {
        return self.extent;
    }
    pub fn render_pass(&self) -> &RenderPass {
        return &self.render_pass;
    }
}

// Constructor, destructor
impl Framebuffer {
    pub fn new_from_swapchain(
        device: &crate::Device,
        swapchain: &crate::Swapchain,
    ) -> Result<Self> {
        let image_views = Self::create_image_views(device, swapchain)?;
        let render_pass = RenderPass::new(device, swapchain.image_format())?;

        let extent = swapchain.extent();

        let framebuffers = image_views
            .iter()
            .filter_map(|&image_view| {
                let attachments = &[image_view];

                let create_info = vk::FramebufferCreateInfo::default()
                    .render_pass(*render_pass)
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe { device.create_framebuffer(&create_info, None).ok() }
            })
            .collect::<Vec<_>>();

        let framebuffer = Framebuffer {
            extent,
            image_views,
            render_pass,
            framebuffers,
        };

        let num_of_image_views = framebuffer.image_views.len();
        let num_of_framebuffers = framebuffer.framebuffers.len();
        anyhow::ensure!(
            num_of_image_views == num_of_framebuffers,
            "The number of framebuffers doesn't match the number of image view: {} != {}",
            num_of_image_views,
            num_of_framebuffers
        );

        return Ok(framebuffer);
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();

        unsafe {
            self.render_pass.destroy(device);

            for &image_view in self.image_views.iter() {
                device.destroy_image_view(image_view, None);
            }

            for &framebuffer in self.framebuffers.iter() {
                device.destroy_framebuffer(framebuffer, None);
            }
        }
    }
}

// Specific implementation
impl Framebuffer {
    fn create_image_views(
        device: &crate::Device,
        swapchain: &crate::Swapchain,
    ) -> Result<Vec<vk::ImageView>> {
        let images = swapchain.get_images()?;

        let mut res = Vec::new();

        for &image in images.iter() {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain.image_format())
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            res.push(unsafe { device.create_image_view(&create_info, None)? })
        }

        return Ok(res);
    }
}

// Deref
impl std::ops::Deref for Framebuffer {
    type Target = [vk::Framebuffer];

    fn deref(&self) -> &Self::Target {
        return &self.framebuffers;
    }
}

//-----------------------------------------------------------------------------
