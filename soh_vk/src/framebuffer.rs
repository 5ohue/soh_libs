//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

pub struct Framebuffer {
    device: crate::DeviceRef,

    extent: vk::Extent2D,

    image_views: Vec<vk::ImageView>,
    framebuffer: vk::Framebuffer,
}

// Getters
impl Framebuffer {
    pub fn extent(&self) -> vk::Extent2D {
        return self.extent;
    }
}

// Constructor, destructor
impl Framebuffer {
    /// Creates an array of framebuffers for each of the images in the swapchain
    pub fn new_from_swapchain(
        device: &crate::DeviceRef,
        swapchain: &crate::Swapchain,
        render_pass: &crate::RenderPass,
    ) -> Result<Vec<Self>> {
        let image_views =
            Self::create_image_views(device, &swapchain.get_images()?, swapchain.image_format())?;

        let extent = swapchain.extent();

        let mut create_info = vk::FramebufferCreateInfo::default()
            .render_pass(**render_pass)
            .width(extent.width)
            .height(extent.height)
            .layers(1);

        let framebuffers = image_views
            .iter()
            .map(|image_view| {
                create_info = create_info.attachments(std::slice::from_ref(image_view));

                let framebuffer = unsafe { device.create_framebuffer(&create_info, None).unwrap() };

                return Framebuffer {
                    device: device.clone(),
                    extent,
                    image_views: vec![*image_view],
                    framebuffer,
                };
            })
            .collect::<Vec<_>>();

        return Ok(framebuffers);
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_framebuffer(self.framebuffer, None);

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
    type Target = vk::Framebuffer;

    fn deref(&self) -> &Self::Target {
        return &self.framebuffer;
    }
}

//-----------------------------------------------------------------------------
