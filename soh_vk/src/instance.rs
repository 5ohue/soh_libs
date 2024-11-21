use anyhow::{anyhow, Result};
use ash::{vk, Entry};
use std::ffi::CStr;

pub struct Instance {
    instance: ash::Instance,
    entry: Entry,

    // EXT, KHR instances
    instance_debug_utils: ash::ext::debug_utils::Instance,
    instance_surface: ash::khr::surface::Instance,
}

/// Instance reference stored inside the dependant types (which is the logical device mainly)
pub type InstanceRef = std::rc::Rc<Instance>;

// Getters
impl Instance {
    pub fn entry(&self) -> &Entry {
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

// Constructor, destructor
impl Instance {
    pub fn new(
        app_info: &vk::ApplicationInfo,
        window: &sdl2::video::Window,
    ) -> Result<InstanceRef> {
        #[cfg(feature = "log")]
        soh_log::log_info!("Creating instance");

        // TODO: make it more cross platform
        let entry = unsafe { Entry::load_from("/usr/lib/libvulkan.so")? };

        let required_extensions = Self::get_sdl2_extensions(window)?;
        let required_layers = Self::get_validation_layers(&entry)?;

        // Log stuff
        #[cfg(feature = "log")]
        {
            soh_log::log_info!("Required {} extensions", required_extensions.len());
            for &required_ext in required_extensions.iter() {
                let r_name = unsafe { CStr::from_ptr(required_ext) };

                soh_log::log_info!("    {:?}", r_name);
            }

            soh_log::log_info!("Required {} layers", required_layers.len());
            for &required_layer in required_layers.iter() {
                let r_layer = unsafe { CStr::from_ptr(required_layer) };

                soh_log::log_info!("    {:?}", r_layer);
            }
        }

        let mut create_info = vk::InstanceCreateInfo::default()
            .application_info(app_info)
            .enabled_layer_names(&required_layers)
            .enabled_extension_names(&required_extensions);

        let mut opt_debug_utils_create_info = crate::debug::Messenger::create_info();
        if let Some(ref mut debug_utils_create_info) = opt_debug_utils_create_info {
            #[cfg(feature = "log")]
            soh_log::log_debug!("Using validation layers to debug instance creation!");
            create_info = create_info.push_next(debug_utils_create_info);
        }

        let supported_extensions = unsafe { entry.enumerate_instance_extension_properties(None)? };
        for &required_ext in required_extensions.iter() {
            let r_name = unsafe { CStr::from_ptr(required_ext) };
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

// Drop
impl Drop for Instance {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        soh_log::log_info!("Destroying instance");

        unsafe { self.instance.destroy_instance(None) };
    }
}

// Specific implementation
impl Instance {
    fn get_validation_layers(entry: &Entry) -> Result<Vec<*const i8>> {
        const VALIDATION_LAYERS: &[&std::ffi::CStr] = &[c"VK_LAYER_KHRONOS_validation"];

        if !Self::are_validation_layers_enabled() {
            return Ok(vec![]);
        }

        // Get available layers
        let available_layers = unsafe { entry.enumerate_instance_layer_properties()? };

        // Check if our validation layers are available
        let mut res = Vec::new();

        for &r_name in VALIDATION_LAYERS {
            let mut found = false;
            for available_layer in available_layers.iter() {
                let a_name = available_layer.layer_name_as_c_str()?;

                if r_name == a_name {
                    found = true;
                    break;
                }
            }

            if found {
                res.push(r_name.as_ptr());
            }
        }

        return Ok(res);
    }

    /// Gets the extensions needed to create a vk surface
    fn get_sdl2_extensions(window: &sdl2::video::Window) -> Result<Vec<*const i8>> {
        use sdl2::sys::SDL_bool::SDL_TRUE;

        // Get count
        let mut extension_count = 0;
        let res = unsafe {
            sdl2::sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut extension_count,
                std::ptr::null_mut(),
            )
        };

        anyhow::ensure!(
            res == SDL_TRUE,
            "Failed to get the SDL2 instance extension count ({})",
            sdl2::get_error()
        );

        // Get extensions
        let mut extensions = vec![std::ptr::null(); extension_count as usize];
        let res = unsafe {
            sdl2::sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut extension_count,
                extensions.as_mut_ptr(),
            )
        };

        // Get validation layer extension
        if Self::are_validation_layers_enabled() {
            // Using `vk::EXT_DEBUG_UTILS_NAME` directly sounds dangerous
            // (dangling pointer maybe??? `vk::EXT_DEBUG_UTILS_NAME` is `const`, not `static`)

            static EXTENSION_NAME: &CStr = vk::EXT_DEBUG_UTILS_NAME;
            extensions.push(EXTENSION_NAME.as_ptr());
        }

        anyhow::ensure!(
            res == SDL_TRUE,
            "Failed to get the SDL2 instance extensions ({})",
            sdl2::get_error()
        );

        Ok(extensions)
    }
}

// Deref
impl std::ops::Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        return &self.instance;
    }
}
