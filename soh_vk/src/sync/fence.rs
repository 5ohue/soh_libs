use anyhow::Result;
use ash::vk;

pub struct Fence {
    device: crate::DeviceRef,
    fence: vk::Fence,
}

// Constructor, destructor
impl Fence {
    pub fn new(device: &crate::DeviceRef, signaled: bool) -> Result<Self> {
        let create_info = vk::FenceCreateInfo::default().flags(if signaled {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::default()
        });

        let fence = unsafe { device.create_fence(&create_info, None)? };
        return Ok(Fence {
            device: device.clone(),
            fence,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_fence(self.fence, None);
        }
    }
}

// Specific implementation
impl Fence {
    pub fn wait(&self) {
        unsafe {
            let _ = self.device.wait_for_fences(&[self.fence], true, u64::MAX);
        }
    }

    pub fn reset(&self) {
        unsafe {
            let _ = self.device.reset_fences(&[self.fence]);
        }
    }
}

// Deref
impl std::ops::Deref for Fence {
    type Target = vk::Fence;

    fn deref(&self) -> &Self::Target {
        return &self.fence;
    }
}
