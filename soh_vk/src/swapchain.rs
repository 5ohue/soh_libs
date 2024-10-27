use anyhow::Result;
use ash::vk;

pub struct Swapchain {
    swapchain: vk::SwapchainKHR,

    device: ash::khr::swapchain::Device,

    image_format: vk::Format,
    extent: vk::Extent2D,
}

// Getters
impl Swapchain {
    pub fn image_format(&self) -> vk::Format {
        return self.image_format;
    }
    pub fn extent(&self) -> vk::Extent2D {
        return self.extent;
    }
}

// Constructor, destructor
impl Swapchain {
    pub fn new(
        device: &crate::Device,
        surface: &crate::Surface,
        window_size: (u32, u32),
    ) -> Result<Self> {
        #[cfg(feature = "log")]
        soh_log::log_debug!("Creating swapchain for window size {:?}", window_size);

        let swapchain_support = device.physical().swapchain_support_info();
        let queue_family_info = device.physical().queue_family_indices();

        let queue_family_indices = queue_family_info
            .get_unique_indices()
            .into_iter()
            .collect::<Vec<_>>();

        let device = device.device_swapchain().clone();

        let surface_format = Self::choose_swapchain_format(&swapchain_support.formats);
        let present_mode = Self::choose_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = Self::choose_swap_extent(&swapchain_support.capabilities, window_size);

        let image_count = if swapchain_support.capabilities.max_image_count == 0 {
            swapchain_support.capabilities.min_image_count + 1
        } else {
            u32::min(
                swapchain_support.capabilities.min_image_count + 1,
                swapchain_support.capabilities.max_image_count,
            )
        };

        let mut create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(**surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .queue_family_indices(&queue_family_indices);

        if let &crate::physical::QueueFamilyIndices {
            graphics_family: Some(gf),
            present_family: Some(pf),
            ..
        } = queue_family_info
        {
            if gf != pf {
                create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            } else {
                create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            }
        }

        let create_info = create_info
            .pre_transform(swapchain_support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let swapchain = unsafe { device.create_swapchain(&create_info, None)? };

        return Ok(Swapchain {
            swapchain,
            device,
            image_format: surface_format.format,
            extent,
        });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            self.device.destroy_swapchain(**self, None);
        }
    }
}

// Specific implementation
impl Swapchain {
    pub fn acquire_next_image(
        &self,
        signal_semaphore: Option<&crate::sync::Semaphore>,
        fence: Option<&crate::sync::Fence>,
    ) -> Result<(u32, bool), vk::Result> {
        let semaphore = if let Some(s) = signal_semaphore {
            **s
        } else {
            vk::Semaphore::null()
        };

        let fence = if let Some(f) = fence {
            **f
        } else {
            vk::Fence::null()
        };

        return unsafe {
            self.device
                .acquire_next_image(**self, u64::MAX, semaphore, fence)
        };
    }

    pub fn present_image(
        &self,
        device: &crate::Device,
        wait_semaphore: &crate::sync::Semaphore,
        image_index: u32,
    ) -> Result<()> {
        let wait_semaphores = &[**wait_semaphore];
        let swapchains = &[**self];
        let image_indices = &[image_index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(wait_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        unsafe {
            self.device
                .queue_present(device.get_present_queue_family(), &present_info)?
        };

        return Ok(());
    }

    pub fn get_images(&self) -> Result<Vec<vk::Image>> {
        return unsafe { Ok(self.device.get_swapchain_images(**self)?) };
    }

    fn choose_swapchain_format(available_formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        #[cfg(feature = "log")]
        soh_log::log_debug!("Available formats: {:#?}", available_formats);

        for &available_format in available_formats.iter() {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return available_format;
            }
        }

        #[cfg(feature = "log")]
        soh_log::log_warning!("Couldn't find desired surface format! Defaulting to {:?}", available_formats[0]);

        return available_formats[0];
    }

    fn choose_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        for &available_present_mode in present_modes.iter() {
            if available_present_mode == vk::PresentModeKHR::MAILBOX {
                return available_present_mode;
            }
        }

        return vk::PresentModeKHR::FIFO;
    }

    fn choose_swap_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window_size: (u32, u32),
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            return capabilities.current_extent;
        }

        return vk::Extent2D {
            width: window_size.0.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ),
            height: window_size.1.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ),
        };
    }
}

// Deref
impl std::ops::Deref for Swapchain {
    type Target = vk::SwapchainKHR;

    fn deref(&self) -> &Self::Target {
        return &self.swapchain;
    }
}
