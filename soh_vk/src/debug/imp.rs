//-----------------------------------------------------------------------------
use super::MessengerCallback;
use ash::vk;
//-----------------------------------------------------------------------------
static SINGLETON: std::sync::OnceLock<MessengerCallback> = std::sync::OnceLock::new();
//-----------------------------------------------------------------------------

pub fn setup(callback: MessengerCallback) {
    SINGLETON.get_or_init(|| {
        return callback;
    });
}

pub fn get() -> Option<&'static MessengerCallback> {
    return SINGLETON.get();
}

//-----------------------------------------------------------------------------

pub extern "system" fn debug_messenger_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    /*
     * Get callback data
     */
    if p_callback_data.is_null() {
        return vk::FALSE;
    }

    let callback_data = unsafe { &*p_callback_data };

    /*
     * Get message ( as &str )
     */
    let Some(msg) = (unsafe { callback_data.message_as_c_str() }) else {
        return vk::FALSE;
    };

    let Ok(message_str) = msg.to_str() else {
        let utf8_err = "Failed to convert validation layer message: UTF8 error";

        soh_log::log_error!("{utf8_err}");

        return vk::FALSE;
    };

    /*
     * Compile args
     */
    let args = super::CallbackArgs {
        message_severity: message_severity.into(),
        message_type: message_type.into(),
        message_str,
    };

    /*
     * Get the user's callback and call it
     */
    let callback: &MessengerCallback = unsafe { &*(p_user_data.cast()) };

    return callback(args).into();
}

//-----------------------------------------------------------------------------

impl From<vk::DebugUtilsMessageSeverityFlagsEXT> for super::MsgSeverity {
    fn from(value: vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        match value {
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => super::MsgSeverity::Verbose,
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => super::MsgSeverity::Info,
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => super::MsgSeverity::Warning,
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => super::MsgSeverity::Error,
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<vk::DebugUtilsMessageTypeFlagsEXT> for super::MsgType {
    fn from(value: vk::DebugUtilsMessageTypeFlagsEXT) -> Self {
        match value {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => super::MsgType::General,
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => super::MsgType::Validation,
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => super::MsgType::Performance,
            _ => {
                unreachable!()
            }
        }
    }
}

//-----------------------------------------------------------------------------
