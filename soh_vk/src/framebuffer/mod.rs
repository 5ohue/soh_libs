//-----------------------------------------------------------------------------
pub mod render_pass;
//-----------------------------------------------------------------------------
pub use render_pass::RenderPass;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

pub struct Framebuffer {
    device: crate::DeviceRef,

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
        device: &crate::DeviceRef,
        swapchain: &crate::Swapchain,
    ) -> Result<Self> {
        let image_views =
            Self::create_image_views(device, &swapchain.get_images()?, swapchain.image_format())?;
        let render_pass = RenderPass::new(device, swapchain.image_format())?;

        let extent = swapchain.extent();

        let framebuffers = image_views
            .iter()
            .filter_map(|&image_view| {
                let create_info = vk::FramebufferCreateInfo::default()
                    .render_pass(*render_pass)
                    .attachments(std::slice::from_ref(&image_view))
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe { device.create_framebuffer(&create_info, None).ok() }
            })
            .collect::<Vec<_>>();

        let framebuffer = Framebuffer {
            device: device.clone(),
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

    pub fn destroy(&self) {
        unsafe {
            self.render_pass.destroy();

            for &framebuffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            for &image_view in self.image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }
        }
    }
}

// Specific implementation
impl Framebuffer {
    pub fn get_viewport_scissor(&self) -> (vk::Viewport, vk::Rect2D) {
        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: self.extent.width as f32,
            height: self.extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.extent,
        };

        return (viewport, scissor);
    }

    fn create_image_views(
        device: &crate::Device,
        images: &[vk::Image],
        format: vk::Format,
    ) -> Result<Vec<vk::ImageView>> {
        let mut res = Vec::new();

        for &image in images.iter() {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
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
