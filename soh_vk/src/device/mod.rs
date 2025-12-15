//-----------------------------------------------------------------------------
pub mod physical;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk::{self, Handle};

//-----------------------------------------------------------------------------

pub struct Device {
    // Keep instance from being destoyed
    instance: crate::InstanceRef,

    physical: physical::Device,
    logical: ash::Device,

    // EXT, KHR devices
    device_swapchain: ash::khr::swapchain::Device,

    // Queues
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    transfer_queue: vk::Queue,
}

//-----------------------------------------------------------------------------
/// Device reference stored inside other vulkan types
///
/// (This is needed because the vulkan handles are implicitly bound to a specific device. Therefore
/// it's redundant to have to provide devices everywhere)
pub type DeviceRef = std::rc::Rc<Device>;
//-----------------------------------------------------------------------------
// According to the vulkan documentation this should be OK
unsafe impl Sync for Device {}
//-----------------------------------------------------------------------------
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

    pub fn graphics_queue(&self) -> vk::Queue {
        return self.graphics_queue;
    }
    pub fn present_queue(&self) -> vk::Queue {
        return self.present_queue;
    }
    pub fn transfer_queue(&self) -> vk::Queue {
        return self.transfer_queue;
    }
}

//-----------------------------------------------------------------------------
// Constructor, destructor
impl Device {
    pub fn new(instance: &crate::InstanceRef, surface: &vk::SurfaceKHR) -> Result<DeviceRef> {
        soh_log::log_info!("Creating logical device");

        /*
         * Pick logical device
         */
        let physical = physical::Device::pick_device(instance, surface)?;

        /*
         * Create queues:
         * Make a `vkDeviceQueueCreateInfo` for each unique queue
         */
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

        /*
         * Specify extensions
         */
        let swapchain_extension_name = ash::khr::swapchain::NAME;
        let extensions = [swapchain_extension_name.as_ptr()];

        let device_features = vk::PhysicalDeviceFeatures::default()
            .depth_clamp(true)
            .fill_mode_non_solid(true) // For lines
            .wide_lines(true); // For wide lines

        /*
         * Create logical device
         */
        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&extensions);

        let device = unsafe { instance.create_device(*physical, &create_info, None)? };

        let device_swapchain = ash::khr::swapchain::Device::new(instance, &device);

        /*
         * Get queues
         */
        let graphics_queue = Self::__get_queue(
            &device,
            physical.queue_family_idx(crate::QueueType::Graphics),
        );
        let present_queue = Self::__get_queue(
            &device,
            physical.queue_family_idx(crate::QueueType::Present),
        );
        let transfer_queue = Self::__get_queue(
            &device,
            physical.queue_family_idx(crate::QueueType::Transfer),
        );

        return Ok(DeviceRef::new(Device {
            instance: instance.clone(),
            physical,
            logical: device,
            device_swapchain,
            graphics_queue,
            present_queue,
            transfer_queue,
        }));
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Device {
    fn drop(&mut self) {
        soh_log::log_info!(
            "Destroying logical device \"{}\"",
            self.physical.info().name
        );

        unsafe { self.logical.destroy_device(None) };
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Device {
    pub fn wait_idle(&self) {
        unsafe {
            let _ = self.logical.device_wait_idle();
        }
    }

    /// Get a queue handle for specified queue family index
    pub fn get_queue(&self, queue_family_index: u32) -> vk::Queue {
        return Self::__get_queue(&self.logical, queue_family_index);
    }

    fn __get_queue(device: &ash::Device, queue_family_index: u32) -> vk::Queue {
        return unsafe { device.get_device_queue(queue_family_index, 0) };
    }
}

//-----------------------------------------------------------------------------
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
