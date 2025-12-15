//-----------------------------------------------------------------------------
// Implementation details
mod imp;
//-----------------------------------------------------------------------------

use anyhow::Result;
use ash::vk;

//-----------------------------------------------------------------------------

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
pub type MessengerCallback = fn(crate::debug::CallbackArgs<'_>) -> bool;
//-----------------------------------------------------------------------------
/// This functions sets up the debug messenger callback. This function must be
/// called before calling `[DebugMessenger::new]`.
pub fn setup_messenger(callback: MessengerCallback) {
    imp::setup(callback);
}

//-----------------------------------------------------------------------------

pub struct Messenger {
    instance: crate::InstanceRef,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

//-----------------------------------------------------------------------------
// Constructor
impl Messenger {
    pub fn new(instance: &crate::InstanceRef) -> Result<Self> {
        use soh_log::LogError;

        anyhow::ensure!(
            crate::Instance::are_validation_layers_enabled(),
            "Cannot create debug messenger! Validation layers are not enabled"
        );

        let instance_debug = instance.instance_debug_utils();

        let create_info = Self::create_info()
            .expect_log("`setup_debug_messenger` must be called before `DebugMessenger::new()`");

        let messenger = unsafe { instance_debug.create_debug_utils_messenger(&create_info, None)? };

        return Ok(Messenger {
            instance: instance.clone(),
            debug_messenger: messenger,
        });
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Messenger {
    pub(crate) fn create_info() -> Option<vk::DebugUtilsMessengerCreateInfoEXT<'static>> {
        let data = imp::get()?;

        // mut casting is OK here because in data isn't mutated in debug callback
        let data_ptr = (data as *const MessengerCallback).cast_mut().cast();

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
// Drop
impl Drop for Messenger {
    fn drop(&mut self) {
        unsafe {
            let instance = self.instance.instance_debug_utils();
            instance.destroy_debug_utils_messenger(self.debug_messenger, None);
        }
    }
}

//-----------------------------------------------------------------------------
