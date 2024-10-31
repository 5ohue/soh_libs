use anyhow::{anyhow, Result};
use ash::vk;

pub struct Device {
    physical_device: vk::PhysicalDevice,

    info: PhysicalDeviceInfo,
}

#[derive(Debug)]
pub struct PhysicalDeviceInfo {
    pub name: String,

    pub queue_family_indices: QueueFamilyIndices,
    pub swapchain_support: SwapchainSupportInfo,
}

#[derive(Clone, Copy, Debug)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

#[derive(Debug)]
pub struct SwapchainSupportInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,

    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

// Getters
impl Device {
    pub fn info(&self) -> &PhysicalDeviceInfo {
        return &self.info;
    }
    pub fn gpu_name(&self) -> &str {
        return &self.info.name;
    }
    pub fn queue_family_indices(&self) -> &QueueFamilyIndices {
        return &self.info.queue_family_indices;
    }
    pub fn swapchain_support_info(&self) -> &SwapchainSupportInfo {
        return &self.info.swapchain_support;
    }
}

// Constructor, destructor
impl Device {
    pub fn new(instance: &crate::Instance, surface: &vk::SurfaceKHR) -> Result<Self> {
        let devices = unsafe { instance.enumerate_physical_devices()? };

        // Loop over all devices and choose the suitable one
        let mut selected_device = vk::PhysicalDevice::null();
        for &available_device in devices.iter() {
            if Self::is_device_suitable(instance, available_device, surface) {
                selected_device = available_device;
                break;
            }
        }

        // Throw error if no device was chosen
        if selected_device == vk::PhysicalDevice::null() {
            return Err(anyhow!("Coudn't find suitable physical device"));
        }

        return Ok(Device {
            physical_device: selected_device,
            info: PhysicalDeviceInfo::get_info(instance, selected_device, surface).unwrap(), // This shouldn't panic because this function was already called for this
                                                                                             // device before
        });
    }

    #[inline(always)]
    pub fn update_swapchain_support_info(
        &mut self,
        instance: &crate::Instance,
        surface: &vk::SurfaceKHR,
    ) -> Result<&SwapchainSupportInfo> {
        return self
            .info
            .update_swapchain_support_info(instance, self.physical_device, surface);
    }

    fn is_device_suitable(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> bool {
        fn check_device_extension_support(
            instance: &crate::Instance,
            physical_device: vk::PhysicalDevice,
        ) -> bool {
            const DEVICE_EXTENSIONS: &[&std::ffi::CStr] = &[ash::khr::swapchain::NAME];

            let available_extensions =
                unsafe { instance.enumerate_device_extension_properties(physical_device) }
                    .expect("Failed to enumerate device extension properties");

            for &required_extension_name in DEVICE_EXTENSIONS.iter() {
                let mut found = false;

                for available_extension in available_extensions.iter() {
                    let available_extension_name =
                        available_extension.extension_name_as_c_str().unwrap();

                    if required_extension_name == available_extension_name {
                        found = true;
                        break;
                    }
                }

                if !found {
                    return false;
                }
            }

            return true;
        }

        let Ok(device_info) = PhysicalDeviceInfo::get_info(instance, physical_device, surface)
        else {
            #[cfg(feature = "log")]
            {
                let gpu_name =
                    PhysicalDeviceInfo::query_gpu_name(instance, physical_device).unwrap();

                soh_log::log_warning!("Failed to get information about device \"{}\"!", gpu_name);
            }

            // If failed to get info, the device is probably not suitable
            return false;
        };

        let extensions_supported = check_device_extension_support(instance, physical_device);

        let swapchain_adequate = !device_info.swapchain_support.formats.is_empty()
            && !device_info.swapchain_support.present_modes.is_empty();

        return device_info.queue_family_indices.is_complete()
            && extensions_supported
            && swapchain_adequate;
    }
}

// Specific implementation
impl PhysicalDeviceInfo {
    fn get_info(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> Result<Self> {
        return Ok(PhysicalDeviceInfo {
            name: Self::query_gpu_name(instance, physical_device)?,

            queue_family_indices: Self::find_queue_families(instance, physical_device, surface),
            swapchain_support: Self::query_swapchain_support_info(
                instance,
                physical_device,
                surface,
            )?,
        });
    }

    #[inline(always)]
    fn update_swapchain_support_info(
        &mut self,
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> Result<&SwapchainSupportInfo> {
        self.swapchain_support =
            Self::query_swapchain_support_info(instance, physical_device, surface)?;

        return Ok(&self.swapchain_support);
    }

    fn query_gpu_name(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Result<String> {
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };

        return Ok(properties
            .device_name_as_c_str()?
            .to_string_lossy()
            .to_string());
    }

    fn find_queue_families(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> QueueFamilyIndices {
        let mut res = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };

        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let instance = instance.instance_surface();

        for (i, qf) in queue_families.iter().enumerate() {
            if qf.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                res.graphics_family = Some(i as u32);
            }

            let present_supported = unsafe {
                instance
                    .get_physical_device_surface_support(physical_device, i as u32, *surface)
                    .unwrap()
            };

            if present_supported {
                res.present_family = Some(i as u32);
            }

            if res.is_complete() {
                break;
            }
        }

        return res;
    }

    fn query_swapchain_support_info(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> Result<SwapchainSupportInfo> {
        let instance = instance.instance_surface();

        let capabilities = unsafe {
            instance.get_physical_device_surface_capabilities(physical_device, *surface)?
        };

        let formats =
            unsafe { instance.get_physical_device_surface_formats(physical_device, *surface)? };

        let present_modes = unsafe {
            instance.get_physical_device_surface_present_modes(physical_device, *surface)?
        };

        return Ok(SwapchainSupportInfo {
            capabilities,
            formats,
            present_modes,
        });
    }
}

impl QueueFamilyIndices {
    /// Return a set of all unique indices
    pub fn get_unique_indices(&self) -> std::collections::HashSet<u32> {
        // Abuse `Option::iter` method
        return self
            .graphics_family
            .iter()
            .chain(self.present_family.iter())
            .copied()
            .collect();
    }

    fn is_complete(&self) -> bool {
        return self.graphics_family.is_some() && self.present_family.is_some();
    }
}

// Deref
impl std::ops::Deref for Device {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        return &self.physical_device;
    }
}
