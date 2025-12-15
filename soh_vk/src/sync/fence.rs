//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Fence {
    device: crate::DeviceRef,
    fence: vk::Fence,
}

//-----------------------------------------------------------------------------
// Constructor
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
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Fence {
    pub fn wait(&self) {
        unsafe {
            let _ = self
                .device
                .wait_for_fences(std::slice::from_ref(self), true, u64::MAX);
        }
    }

    pub fn is_signaled(&self) -> bool {
        unsafe {
            return self.device.get_fence_status(self.fence).unwrap();
        }
    }

    pub fn reset(&self) {
        unsafe {
            let _ = self.device.reset_fences(std::slice::from_ref(self));
        }
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(self.fence, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Fence {
    type Target = vk::Fence;

    fn deref(&self) -> &Self::Target {
        return &self.fence;
    }
}

//-----------------------------------------------------------------------------
