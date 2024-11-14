use anyhow::Result;
use ash::vk;

pub struct Swapchain {
    device: crate::DeviceRef,

    swapchain: vk::SwapchainKHR,

    image_format: crate::Format,
    extent: vk::Extent2D,
}

// Getters
impl Swapchain {
    pub fn image_format(&self) -> crate::Format {
        return self.image_format;
    }
    pub fn extent(&self) -> vk::Extent2D {
        return self.extent;
    }
}

// Constructor, destructor
impl Swapchain {
    pub fn new(
        device: &crate::DeviceRef,
        surface: &crate::Surface,
        window_size: (u32, u32),
    ) -> Result<Self> {
        #[cfg(feature = "log")]
        soh_log::log_debug!("Creating swapchain for window size {:?}", window_size);

        return Self::create_swapchain(device, surface, window_size, None);
    }

    pub fn recreate(&mut self, surface: &crate::Surface, window_size: (u32, u32)) -> Result<()> {
        #[cfg(feature = "log")]
        soh_log::log_debug!("Rereating swapchain for window size {:?}", window_size);

        let new_swapchain = Self::create_swapchain(&self.device, surface, window_size, Some(self))?;
        self.destroy();
        *self = new_swapchain;
        return Ok(());
    }

    pub fn destroy(&self) {
        unsafe {
            self.device
                .device_swapchain()
                .destroy_swapchain(**self, None);
        }
    }

    fn create_swapchain(
        device: &crate::DeviceRef,
        surface: &crate::Surface,
        window_size: (u32, u32),
        old_swapchain: Option<&Self>,
    ) -> Result<Self> {
        let swapchain_support = device.physical().query_swapchain_support_info(surface)?;
        let queue_family_info = device.physical().queue_family_indices();

        #[cfg(feature = "log")]
        soh_log::log_debug!("SwapchainSupport: {:#?}", swapchain_support);

        let queue_family_indices = queue_family_info
            .get_unique_indices()
            .into_iter()
            .collect::<Vec<_>>();

        let device_swapchain = device.device_swapchain();

        let surface_format = Self::choose_swapchain_format(&swapchain_support.formats);
        let present_mode = Self::choose_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = Self::choose_swap_extent(&swapchain_support.capabilities, window_size);

        // #[cfg(feature = "log")]
        // soh_log::log_debug!("Chose extent: {:?}", extent);

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
            .queue_family_indices(&queue_family_indices)
            .old_swapchain(crate::get_opt_handle(old_swapchain));

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

        let swapchain = unsafe { device_swapchain.create_swapchain(&create_info, None)? };

        return Ok(Swapchain {
            device: device.clone(),
            swapchain,
            image_format: surface_format.format,
            extent,
        });
    }
}

// Specific implementation
impl Swapchain {
    pub fn acquire_next_image(
        &self,
        signal_semaphore: Option<&crate::sync::Semaphore>,
        fence: Option<&crate::sync::Fence>,
    ) -> Result<(u32, bool), vk::Result> {
        let semaphore = crate::get_opt_handle(signal_semaphore);
        let fence = crate::get_opt_handle(fence);

        return unsafe {
            self.device
                .device_swapchain()
                .acquire_next_image(**self, u64::MAX, semaphore, fence)
        };
    }

    pub fn present_image(
        &self,
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
                .device_swapchain()
                .queue_present(self.device.get_present_queue_family(), &present_info)?
        };

        return Ok(());
    }

    pub fn get_images(&self) -> Result<Vec<vk::Image>> {
        return unsafe {
            Ok(self
                .device
                .device_swapchain()
                .get_swapchain_images(**self)?)
        };
    }

    fn choose_swapchain_format(available_formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        for &available_format in available_formats.iter() {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return available_format;
            }
        }

        #[cfg(feature = "log")]
        soh_log::log_warning!(
            "Couldn't find desired surface format! Defaulting to {:?}",
            available_formats[0]
        );

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
