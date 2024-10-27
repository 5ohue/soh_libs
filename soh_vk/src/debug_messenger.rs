use anyhow::{anyhow, Result};
use ash::vk;

#[repr(transparent)]
pub struct DebugMessenger {
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

// Constructor, destructor
impl DebugMessenger {
    /// # Safety
    /// `user_data` should live up until `destroy()` is called
    pub unsafe fn new<T>(
        instance: &crate::Instance,
        callback: vk::PFN_vkDebugUtilsMessengerCallbackEXT,
        user_data: Option<&mut T>,
    ) -> Result<Self> {
        if !crate::Instance::are_validation_layers_enabled() {
            return Err(anyhow!(
                "Cannot create debug messenger! Validation layers are not enabled"
            ));
        }

        let instance = instance.instance_debug_utils();

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(callback)
            .user_data(if let Some(data) = user_data {
                (data as *mut T).cast()
            } else {
                std::ptr::null_mut()
            });

        let messenger = unsafe { instance.create_debug_utils_messenger(&create_info, None)? };

        return Ok(DebugMessenger {
            debug_messenger: messenger,
        });
    }

    pub fn destroy(&self, instance: &crate::Instance) {
        instance.assert_not_destroyed();
        unsafe {
            let instance = instance.instance_debug_utils();
            instance.destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}

// Deref
impl std::ops::Deref for DebugMessenger {
    type Target = vk::DebugUtilsMessengerEXT;

    fn deref(&self) -> &Self::Target {
        return &self.debug_messenger;
    }
}
