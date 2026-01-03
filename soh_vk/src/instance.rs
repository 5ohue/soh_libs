//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
use std::ffi::CStr;
//-----------------------------------------------------------------------------

pub struct Instance {
    instance: ash::Instance,
    entry: ash::Entry,

    // EXT, KHR instances
    instance_debug_utils: ash::ext::debug_utils::Instance,
    instance_surface: ash::khr::surface::Instance,
}

//-----------------------------------------------------------------------------
/// Instance reference stored inside the dependant types (which is the logical device mainly)
pub type InstanceRef = std::rc::Rc<Instance>;
//-----------------------------------------------------------------------------
// Getters
impl Instance {
    pub fn entry(&self) -> &ash::Entry {
        return &self.entry;
    }

    pub fn instance_debug_utils(&self) -> &ash::ext::debug_utils::Instance {
        return &self.instance_debug_utils;
    }
    pub fn instance_surface(&self) -> &ash::khr::surface::Instance {
        return &self.instance_surface;
    }

    pub fn are_validation_layers_enabled() -> bool {
        // Only enable validation layers in a debug build
        return cfg!(debug_assertions) == true;
    }
}

//-----------------------------------------------------------------------------
// Constructor
impl Instance {
    pub fn new(
        app_info: &vk::ApplicationInfo,
        surface_platform: crate::wsi::Platform,
    ) -> Result<InstanceRef> {
        soh_log::log_info!("Creating instance");

        /*
         * Load the vulkan library
         */
        let entry = unsafe { ash::Entry::load()? };

        /*
         * Get the required extensions and layers
         */
        let required_extensions = Self::get_extensions(surface_platform);
        let required_layers = Self::get_validation_layers(&entry)?;

        // Log stuff
        {
            soh_log::log_info!("Required {} extensions", required_extensions.len());

            for &required_ext in required_extensions.iter() {
                soh_log::log_info!("    {:?}", required_ext);
            }

            soh_log::log_info!("Required {} layers", required_layers.len());
            for &required_layer in required_layers.iter() {
                soh_log::log_info!("    {:?}", required_layer);
            }
        }

        /*
         * Check if required extensions are supported
         */
        let supported_extensions = unsafe { entry.enumerate_instance_extension_properties(None)? };
        for &r_name in required_extensions.iter() {
            let mut found = false;

            for supported_ext in supported_extensions.iter() {
                let s_name = supported_ext.extension_name_as_c_str()?;

                if s_name == r_name {
                    found = true;
                    break;
                }
            }

            anyhow::ensure!(found, "Extension {:?} not supported!", r_name);
        }

        /*
         * Create Vec<*const i8> for the create info struct
         */
        let ptr_required_layers = Self::cstr_to_ptr(&required_layers);
        let ptr_required_extensions: Vec<*const i8> = Self::cstr_to_ptr(&required_extensions);

        /*
         * Create instance
         */
        let mut create_info = vk::InstanceCreateInfo::default()
            .application_info(app_info)
            .enabled_layer_names(&ptr_required_layers)
            .enabled_extension_names(&ptr_required_extensions);

        // Use debug messenger if it is used
        let mut opt_debug_utils_create_info = crate::debug::Messenger::create_info();
        if let Some(ref mut debug_utils_create_info) = opt_debug_utils_create_info {
            soh_log::log_debug!("Using validation layers to debug instance creation!");
            create_info = create_info.push_next(debug_utils_create_info);
        }

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let instance_debug_utils = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let instance_surface = ash::khr::surface::Instance::new(&entry, &instance);

        return Ok(InstanceRef::new(Instance {
            instance,
            entry,

            instance_debug_utils,
            instance_surface,
        }));
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Instance {
    fn get_extensions(surface_platform: crate::wsi::Platform) -> Vec<&'static CStr> {
        /*
         * Require the VK_KHR_surface
         */
        let mut extensions = vec![ash::khr::surface::NAME];

        /*
         * Require platform specific extension
         */
        match surface_platform {
            crate::wsi::Platform::Win32 => {
                extensions.push(ash::khr::win32_surface::NAME);
            }
            crate::wsi::Platform::X11 => {
                extensions.push(ash::khr::xlib_surface::NAME);
                extensions.push(ash::khr::xcb_surface::NAME);
            }
            crate::wsi::Platform::Wayland => {
                extensions.push(ash::khr::wayland_surface::NAME);
            }
            crate::wsi::Platform::MacOS => {
                extensions.push(ash::mvk::macos_surface::NAME);
            }
        }

        /*
         * Require validation layer extension
         */
        if Self::are_validation_layers_enabled() {
            extensions.push(ash::ext::debug_utils::NAME);
        }

        return extensions;
    }

    fn get_validation_layers(entry: &ash::Entry) -> Result<Vec<&'static CStr>> {
        static REQUIRED_VALIDATION_LAYERS: &[&CStr] = &[c"VK_LAYER_KHRONOS_validation"];

        if !Self::are_validation_layers_enabled() {
            return Ok(vec![]);
        }

        /*
         * Get available layers
         */
        let available_layers = unsafe { entry.enumerate_instance_layer_properties()? };

        return Ok(REQUIRED_VALIDATION_LAYERS
            .iter()
            .filter_map(|&r_name| {
                /*
                 * Check if the required validation layer is available
                 */
                let mut found = false;

                for available_layer in available_layers.iter() {
                    let a_name = available_layer.layer_name_as_c_str().unwrap();

                    if r_name == a_name {
                        found = true;
                        break;
                    }
                }

                if found {
                    return Some(r_name);
                }
                return None;
            })
            .collect());
    }

    fn cstr_to_ptr(arr: &[&CStr]) -> Vec<*const i8> {
        return arr.iter().map(|&cstr| cstr.as_ptr()).collect();
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Instance {
    fn drop(&mut self) {
        soh_log::log_info!("Destroying instance");

        unsafe { self.instance.destroy_instance(None) };
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        return &self.instance;
    }
}

//-----------------------------------------------------------------------------
