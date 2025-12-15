//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk::{self, Handle};
//-----------------------------------------------------------------------------

pub struct Device {
    // Used to query swapchain info
    instance: crate::InstanceRef,

    physical_device: vk::PhysicalDevice,

    info: PhysicalDeviceInfo,
}

#[derive(Debug)]
pub struct PhysicalDeviceInfo {
    pub name: String,
    pub memory_props: vk::PhysicalDeviceMemoryProperties,
    pub device_props: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,

    pub queue_family_indices: QueueFamilyIndices,
}

//-----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct QueueFamilyIndices {
    pub graphics_family: u32,
    pub present_family: u32,
    pub transfer_family: u32,
}

#[derive(Debug)]
pub struct SwapchainSupportInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,

    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

//-----------------------------------------------------------------------------
// Getters
impl Device {
    pub fn info(&self) -> &PhysicalDeviceInfo {
        return &self.info;
    }
    pub fn gpu_name(&self) -> &str {
        return &self.info.name;
    }
    pub fn queue_family_idx(&self, ty: crate::QueueType) -> u32 {
        let indices = &self.info.queue_family_indices;

        return match ty {
            crate::QueueType::Graphics => indices.graphics_family,
            crate::QueueType::Present => indices.present_family,
            crate::QueueType::Transfer => indices.transfer_family,
        };
    }
    pub fn queue_family_indices(&self) -> &QueueFamilyIndices {
        return &self.info.queue_family_indices;
    }
}

// Constructor, destructor
impl Device {
    pub fn pick_device(instance: &crate::InstanceRef, surface: &vk::SurfaceKHR) -> Result<Self> {
        /*
         * Enumerate available GPUs
         */
        let devices = unsafe { instance.enumerate_physical_devices()? };

        {
            soh_log::log_info!("Available {} devices:", devices.len());

            devices.iter().enumerate().for_each(|(idx, &device)| {
                soh_log::log_info!(
                    "    Device {}: \"{}\"",
                    idx,
                    PhysicalDeviceInfo::query_gpu_name(instance, device).unwrap()
                );
            })
        }

        /*
         * Loop over all devices and choose the suitable one
         */
        let suitable_devices = devices
            .iter()
            .enumerate()
            .filter(|(_idx, &device)| {
                assert!(!device.is_null());
                return Self::is_device_suitable(instance, device, surface);
            })
            .collect::<Vec<_>>();

        // Throw error if no suitable devices
        anyhow::ensure!(
            !suitable_devices.is_empty(),
            "Coudn't find suitable physical device"
        );

        {
            soh_log::log_info!("Found {} suitable devices:", suitable_devices.len());

            suitable_devices.iter().for_each(|(idx, &device)| {
                soh_log::log_info!(
                    "    Device {}: \"{}\"",
                    idx,
                    PhysicalDeviceInfo::query_gpu_name(instance, device).unwrap()
                );
            })
        }

        let selected_device = suitable_devices[0];

        /*
         * Query gpu info:
         * This shouldn't panic because this function was already called for this device before
         */
        let gpu_info =
            PhysicalDeviceInfo::query_info(instance, *selected_device.1, surface).unwrap();

        {
            soh_log::log_info!("Choose GPU {}", selected_device.0);
            soh_log::log_debug!("GPU Info: \"{:#?}\"", gpu_info);
            soh_log::log_debug!(
                "Number of queues: {}",
                gpu_info.queue_family_indices.get_unique_indices().len()
            );
        }

        return Ok(Device {
            instance: instance.clone(),
            physical_device: *selected_device.1,
            info: gpu_info,
        });
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Device {
    #[inline(always)]
    pub fn query_swapchain_support_info(
        &self,
        surface: &vk::SurfaceKHR,
    ) -> Result<SwapchainSupportInfo> {
        return PhysicalDeviceInfo::query_swapchain_support_info(
            &self.instance,
            self.physical_device,
            surface,
        );
    }

    /// Find the index for the physical device memory type that supports the given properties
    ///
    /// * `type_filter`: the vk::MemoryRequirements::memory_type_bits field
    /// * `properties`: the requested memory property flags
    pub(crate) fn find_memory_type(
        &self,
        type_filter: u32,
        properties: crate::MemoryPropertyFlags,
    ) -> Option<u32> {
        for i in 0..self.info.memory_props.memory_type_count {
            let memory_type_supported = type_filter & (1 << i) != 0;
            let properties_supported =
                (self.info.memory_props.memory_types[i as usize].property_flags & properties)
                    == properties;

            if memory_type_supported && properties_supported {
                return Some(i);
            }
        }

        return None;
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

        let (Ok(_), Ok(swapchain_support)) = (
            PhysicalDeviceInfo::query_info(instance, physical_device, surface),
            PhysicalDeviceInfo::query_swapchain_support_info(instance, physical_device, surface),
        ) else {
            let gpu_name = PhysicalDeviceInfo::query_gpu_name(instance, physical_device).unwrap();

            soh_log::log_warning!("Failed to get information about device \"{}\"!", gpu_name);

            // If failed to get info, the device is probably not suitable
            return false;
        };

        let extensions_supported = check_device_extension_support(instance, physical_device);

        let swapchain_adequate =
            !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty();

        return extensions_supported && swapchain_adequate;
    }
}

//-----------------------------------------------------------------------------

impl PhysicalDeviceInfo {
    fn query_info(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> Result<Self> {
        return Ok(PhysicalDeviceInfo {
            name: Self::query_gpu_name(instance, physical_device)?,
            memory_props: Self::query_memory_properties(instance, physical_device),
            device_props: Self::query_device_properties(instance, physical_device),
            features: Self::query_device_features(instance, physical_device),

            queue_family_indices: Self::find_queue_families(instance, physical_device, surface)?,
        });
    }

    fn query_gpu_name(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Result<String> {
        let device_props = Self::query_device_properties(instance, physical_device);
        let gpu_name = device_props
            .device_name_as_c_str()?
            .to_string_lossy()
            .to_string();

        return Ok(gpu_name);
    }

    fn query_memory_properties(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        return unsafe { instance.get_physical_device_memory_properties(physical_device) };
    }

    fn query_device_properties(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        return unsafe { instance.get_physical_device_properties(physical_device) };
    }

    fn query_device_features(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        return unsafe { instance.get_physical_device_features(physical_device) };
    }

    fn find_queue_families(
        instance: &crate::Instance,
        physical_device: vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
    ) -> Result<QueueFamilyIndices> {
        /*
         * Declare optional queue type
         */
        #[derive(Clone, Copy, Debug)]
        pub struct OptionalQueueFamilyIndices {
            pub graphics_family: Option<u32>,
            pub present_family: Option<u32>,
            pub transfer_family: Option<u32>,
        }

        impl OptionalQueueFamilyIndices {
            fn is_complete(&self) -> bool {
                return self.graphics_family.is_some()
                    && self.present_family.is_some()
                    && self.transfer_family.is_some();
            }
        }

        /*
         * Create empty queues
         */
        let mut res = OptionalQueueFamilyIndices {
            graphics_family: None,
            present_family: None,
            transfer_family: None,
        };

        /*
         * Get queue data
         */
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let instance = instance.instance_surface();

        /*
         * Iterate over queues and find the appropriate queue indices
         */
        for (i, qf) in queue_families.iter().enumerate() {
            if qf.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                res.graphics_family = Some(i as u32);
            } else if qf.queue_flags.intersects(vk::QueueFlags::TRANSFER) {
                res.transfer_family = Some(i as u32);
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

        anyhow::ensure!(
            res.is_complete(),
            "The queue family indices are not complete!"
        );

        return Ok(QueueFamilyIndices {
            graphics_family: res.graphics_family.unwrap(),
            present_family: res.present_family.unwrap(),
            transfer_family: res.transfer_family.unwrap(),
        });
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
        return [
            self.graphics_family,
            self.present_family,
            self.transfer_family,
        ]
        .iter()
        .copied()
        .collect();
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Device {
    type Target = vk::PhysicalDevice;

    fn deref(&self) -> &Self::Target {
        return &self.physical_device;
    }
}

//-----------------------------------------------------------------------------
