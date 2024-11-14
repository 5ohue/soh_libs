//-----------------------------------------------------------------------------
pub mod physical;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk::{self, Handle};

pub struct Device {
    // Keep instance from being destoyed
    instance: crate::InstanceRef,

    physical: physical::Device,
    logical: ash::Device,

    // EXT, KHR devices
    device_swapchain: ash::khr::swapchain::Device,
}

/// Device reference stored inside other vulkan types
///
/// (This is needed because the vulkan handles are implicitly bound to a specific device. Therefore
/// it's redundant to have to provide devices everywhere)
pub type DeviceRef = std::rc::Rc<Device>;

// Getters
impl Device {
    pub fn instance(&self) -> &crate::Instance {
        return &self.instance;
    }
    pub fn physical(&self) -> &physical::Device {
        return &self.physical;
    }
    pub fn device_swapchain(&self) -> &ash::khr::swapchain::Device {
        return &self.device_swapchain;
    }
}

// Constructor, destructor
impl Device {
    pub fn new(instance: &crate::InstanceRef, surface: &vk::SurfaceKHR) -> Result<DeviceRef> {
        #[cfg(feature = "log")]
        soh_log::log_info!("Creating logical device");

        let physical = physical::Device::pick_device(instance, surface)?;

        // Make a `vkDeviceQueueCreateInfo` for each unique queue
        let queue_create_infos = physical
            .queue_family_indices()
            .get_unique_indices()
            .iter()
            .map(|&idx| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(idx)
                    .queue_priorities(&[1.0])
            })
            .collect::<Vec<_>>();

        let swapchain_extension_name = ash::khr::swapchain::NAME;
        let extensions = [swapchain_extension_name.as_ptr()];

        let device_features = vk::PhysicalDeviceFeatures::default()
            .depth_clamp(true)
            .fill_mode_non_solid(true) // For lines
            .wide_lines(true); // For wide lines

        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&extensions);

        let device = unsafe { instance.create_device(*physical, &create_info, None)? };

        let device_swapchain = ash::khr::swapchain::Device::new(instance, &device);

        return Ok(DeviceRef::new(Device {
            instance: instance.clone(),
            physical,
            logical: device,
            device_swapchain,
        }));
    }
}

// Drop
impl Drop for Device {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        soh_log::log_info!(
            "Destroying logical device \"{}\"",
            self.physical.info().name
        );

        unsafe { self.logical.destroy_device(None) };
    }
}

// Specific implementation
impl Device {
    pub fn wait_idle(&self) {
        unsafe {
            let _ = self.logical.device_wait_idle();
        }
    }

    /// Get a queue handle for specified queue family index
    pub fn get_queue(&self, queue_family_index: u32) -> vk::Queue {
        return unsafe { self.logical.get_device_queue(queue_family_index, 0) };
    }

    pub fn get_graphics_queue_family(&self) -> vk::Queue {
        if let Some(idx) = self.physical.queue_family_indices().graphics_family {
            return self.get_queue(idx);
        }

        return vk::Queue::null();
    }

    pub fn get_present_queue_family(&self) -> vk::Queue {
        if let Some(idx) = self.physical.queue_family_indices().present_family {
            return self.get_queue(idx);
        }

        return vk::Queue::null();
    }
}

// Deref
impl std::ops::Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        assert!(
            !self.logical.handle().is_null(),
            "Trying to use a logical device which is VK_NULL_HANDLE"
        );

        return &self.logical;
    }
}

//-----------------------------------------------------------------------------
