//-----------------------------------------------------------------------------
// Implementation details
mod imp;
//-----------------------------------------------------------------------------

use anyhow::{anyhow, Result};
use ash::vk;

pub enum MsgSeverity {
    Verbose,
    Info,
    Warning,
    Error,
}

pub enum MsgType {
    General,
    Validation,
    Performance,
}

//-----------------------------------------------------------------------------

pub struct CallbackArgs<'a> {
    pub message_severity: MsgSeverity,
    pub message_type: MsgType,
    pub message_str: &'a str,
}

//-----------------------------------------------------------------------------
/// This functions sets up the debug messenger callback. This function must be
/// called before calling `[DebugMessenger::new]`.
pub fn setup_messenger<F>(callback: F)
where
    F: Fn(&CallbackArgs<'_>) -> bool + Send + Sync + 'static,
{
    imp::MessengerData::setup(callback);
}

//-----------------------------------------------------------------------------
/// Debug messenger
pub struct Messenger {
    instance: crate::InstanceRef,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

// Constructor, destructor
impl Messenger {
    pub fn new(instance: &crate::InstanceRef) -> Result<Self> {
        if !crate::Instance::are_validation_layers_enabled() {
            return Err(anyhow!(
                "Cannot create debug messenger! Validation layers are not enabled"
            ));
        }

        #[cfg(feature = "log")]
        use soh_log::LogError;

        let instance_debug = instance.instance_debug_utils();

        #[cfg(feature = "log")]
        let create_info = Self::create_info()
            .expect_log("`setup_debug_messenger` must be called before `DebugMessenger::new()`");
        #[cfg(not(feature = "log"))]
        let create_info = Self::create_info()
            .expect("`setup_debug_messenger` must be called before `DebugMessenger::new()`");

        let messenger = unsafe { instance_debug.create_debug_utils_messenger(&create_info, None)? };

        return Ok(Messenger {
            instance: instance.clone(),
            debug_messenger: messenger,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            let instance = self.instance.instance_debug_utils();
            instance.destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}

// Specific implementation
impl Messenger {
    pub(crate) fn create_info() -> Option<vk::DebugUtilsMessengerCreateInfoEXT<'static>> {
        let data = imp::MessengerData::get()?;

        // mut casting is OK here because in data isn't mutated in debug callback
        let data_ptr = (data as *const imp::MessengerData).cast_mut().cast();

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
            .pfn_user_callback(Some(imp::debug_messenger_callback))
            .user_data(data_ptr);

        return Some(create_info);
    }
}

//-----------------------------------------------------------------------------
