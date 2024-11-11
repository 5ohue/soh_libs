//-----------------------------------------------------------------------------
pub mod physical;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk::{self, Handle};

pub struct Device {
    physical: physical::Device,
    logical: ash::Device,

    is_destroyed: bool,

    // EXT, KHR devices
    device_swapchain: ash::khr::swapchain::Device,
}

// Getters
impl Device {
    pub fn physical(&self) -> &physical::Device {
        return &self.physical;
    }
    pub fn physical_mut(&mut self) -> &mut physical::Device {
        return &mut self.physical;
    }
    pub fn device_swapchain(&self) -> &ash::khr::swapchain::Device {
        return &self.device_swapchain;
    }

    pub fn is_destroyed(&self) -> bool {
        return self.is_destroyed;
    }
    pub fn assert_not_destroyed(&self) {
        assert!(
            !self.is_destroyed,
            "This function should only be called before the device is destoyed"
        );
    }
}

// Constructor, destructor
impl Device {
    pub fn new(instance: &crate::Instance, surface: &vk::SurfaceKHR) -> Result<Self> {
        let physical = physical::Device::new(instance, surface)?;

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

        return Ok(Device {
            physical,
            logical: device,
            is_destroyed: false,
            device_swapchain,
        });
    }

    pub fn destroy(&mut self, instance: &crate::Instance) {
        instance.assert_not_destroyed();

        self.is_destroyed = true;
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
