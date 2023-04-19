use crate::window::Window;


use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle,
};
use std::ffi::{c_char};

pub struct VulkanAPIDescription {
    pub app_name: String,
    pub engine_name: String,
}

pub trait VulkanAPIExt {
    fn get_required_instance_extensions(&self, window: &Window) -> &[*const c_char];
    fn create_window_surface(
        &self,
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &Window,
    ) -> Result<Surface, ash::vk::Result>;
}

pub struct Vulkan {}

impl Vulkan {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl VulkanAPIExt for Vulkan {
    fn get_required_instance_extensions(&self, window: &Window) -> &[*const c_char] {
        ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap()
    }

    fn create_window_surface(
        &self,
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &Window,
    ) -> Result<Surface, ash::vk::Result> {
        let surface_khr = unsafe {
            match ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            ) {
                Ok(s) => s,
                Err(e) => return Err(e),
            }
        };

        let surface_fn = ash::extensions::khr::Surface::new(entry, instance);

        Ok(Surface {
            surface: surface_khr,
            surface_fn,
        })
    }
}

pub struct Surface {
    surface: ash::vk::SurfaceKHR,
    surface_fn: ash::extensions::khr::Surface,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_fn.destroy_surface(self.surface, None);
        }
    }
}
