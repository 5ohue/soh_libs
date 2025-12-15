//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Semaphore {
    device: crate::DeviceRef,
    semaphore: vk::Semaphore,
}

//-----------------------------------------------------------------------------
// Constructor
impl Semaphore {
    pub fn new(device: &crate::DeviceRef) -> Result<Self> {
        let create_info = vk::SemaphoreCreateInfo::default();

        let semaphore = unsafe { device.create_semaphore(&create_info, None)? };
        return Ok(Semaphore {
            device: device.clone(),
            semaphore,
        });
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(**self, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Semaphore {
    type Target = vk::Semaphore;

    fn deref(&self) -> &Self::Target {
        return &self.semaphore;
    }
}

//-----------------------------------------------------------------------------
