use anyhow::{anyhow, Result};
use ash::{vk, Entry};

pub struct Instance {
    instance: ash::Instance,
    is_destroyed: bool,
}

impl Instance {
    pub fn new(app_info: &vk::ApplicationInfo, window: &sdl2::video::Window) -> Result<Instance> {
        // Get extensions needed to create a vk surface
        let required_extensions = unsafe {
            let mut extension_count = 0;
            let res = sdl2::sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut extension_count,
                std::ptr::null_mut(),
            );

            if res != sdl2::sys::SDL_bool::SDL_TRUE {
                return Err(anyhow!(
                    "Failed to get the SDL2 instance extension count ({})",
                    sdl2::get_error()
                ));
            }

            let mut extensions = vec![std::ptr::null(); extension_count as usize];
            let res = sdl2::sys::SDL_Vulkan_GetInstanceExtensions(
                window.raw(),
                &mut extension_count,
                extensions.as_mut_ptr(),
            );

            if res != sdl2::sys::SDL_bool::SDL_TRUE {
                return Err(anyhow!(
                    "Failed to get the SDL2 instance extensions ({})",
                    sdl2::get_error()
                ));
            }

            extensions
        };

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&required_extensions);

        let entry = Entry::linked();

        let supported_extensions = unsafe { entry.enumerate_instance_extension_properties(None)? };
        for &required_ext in required_extensions.iter() {
            use std::ffi::CStr;

            let r_name = unsafe { CStr::from_ptr(required_ext) };
            let mut found = false;

            for supported_ext in supported_extensions.iter() {
                let s_name = supported_ext.extension_name_as_c_str()?;

                if s_name == r_name {
                    found = true;
                    break;
                }
            }

            let ext_name = r_name.to_string_lossy();
            println!("{ext_name}");
            if !found {
                let ext_name = r_name.to_string_lossy();
                return Err(anyhow!("Extension {ext_name} not supported!"));
            }
        }

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        drop(required_extensions);

        return Ok(Instance {
            instance,
            is_destroyed: false,
        });
    }

    pub unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
        self.is_destroyed = true;
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        assert!(
            self.is_destroyed,
            "Vulkan instance must be manually destroyed!"
        );
    }
}

impl super::ToVK for Instance {
    type TypeVK = ash::Instance;

    fn to_vk(&self) -> &Self::TypeVK {
        return &self.instance;
    }
}
