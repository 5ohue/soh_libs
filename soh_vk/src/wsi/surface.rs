//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Surface {
    instance: crate::InstanceRef,
    surface: vk::SurfaceKHR,
}

//-----------------------------------------------------------------------------
// Constructor, destructor
impl Surface {
    pub fn new(instance: &crate::InstanceRef, window: &winit::window::Window) -> Result<Surface> {
        // Helper function
        fn get_ptr<T>(opt_ptr: Option<std::ptr::NonNull<T>>) -> *mut T {
            return match opt_ptr {
                Some(ptr) => ptr.as_ptr(),
                None => std::ptr::null_mut(),
            };
        }

        use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
        use winit::raw_window_handle::{RawDisplayHandle, RawWindowHandle};

        /*
         * Get the raw window and display handle
         */
        let h_win = window.window_handle()?;
        let h_disp = window.display_handle()?;

        /*
         * Create surface
         */
        let surface = match (h_win.as_raw(), h_disp.as_raw()) {
            /*
             * X11 (Xcb)
             */
            (RawWindowHandle::Xcb(h_win), RawDisplayHandle::Xcb(h_disp)) => {
                let instance = ash::khr::xcb_surface::Instance::new(instance.entry(), instance);

                let create_info = vk::XcbSurfaceCreateInfoKHR::default()
                    .window(h_win.window.into())
                    .connection(get_ptr(h_disp.connection));

                unsafe { instance.create_xcb_surface(&create_info, None) }
            }
            /*
             * X11 (xlib)
             */
            (RawWindowHandle::Xlib(h_win), RawDisplayHandle::Xlib(h_disp)) => {
                let instance = ash::khr::xlib_surface::Instance::new(instance.entry(), instance);

                let create_info = vk::XlibSurfaceCreateInfoKHR::default()
                    .window(h_win.window as _)
                    .dpy(get_ptr(h_disp.display));

                unsafe { instance.create_xlib_surface(&create_info, None) }
            }
            /*
             * Wayland
             */
            (RawWindowHandle::Wayland(h_win), RawDisplayHandle::Wayland(h_disp)) => {
                let instance = ash::khr::wayland_surface::Instance::new(instance.entry(), instance);

                let create_into = vk::WaylandSurfaceCreateInfoKHR::default()
                    .surface(h_win.surface.as_ptr())
                    .display(h_disp.display.as_ptr());

                unsafe { instance.create_wayland_surface(&create_into, None) }
            }
            /*
             * Windows
             */
            (RawWindowHandle::Win32(h_win), RawDisplayHandle::Windows(_h_disp)) => {
                let instance = ash::khr::win32_surface::Instance::new(instance.entry(), instance);

                let create_info = vk::Win32SurfaceCreateInfoKHR::default()
                    .hwnd(h_win.hwnd.into())
                    .hinstance(match h_win.hinstance {
                        Some(hinstance) => hinstance.into(),
                        None => 0,
                    });

                unsafe { instance.create_win32_surface(&create_info, None) }
            }
            /*
             * Anything else
             */
            (h_win, h_disp) => {
                anyhow::bail!(
                    "Unsupported window and display handle type: {:#?}, {:?}",
                    h_win,
                    h_disp
                );
            }
        }?;

        return Ok(Surface {
            instance: instance.clone(),
            surface,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.instance
                .instance_surface()
                .destroy_surface(self.surface, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Surface {
    type Target = vk::SurfaceKHR;

    fn deref(&self) -> &Self::Target {
        return &self.surface;
    }
}

//-----------------------------------------------------------------------------
