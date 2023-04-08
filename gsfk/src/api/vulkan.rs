use std::ffi::CString;
use ash::{Entry, Instance};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WindowsDisplayHandle};
use crate::{APIDescription};

pub struct Vulkan {
    entry: Entry,
    instance: Instance,
    surface_khr: ash::vk::SurfaceKHR,
    surface_fn: ash::extensions::khr::Surface
}

impl Vulkan {
    pub(crate) fn new(handle: RawWindowHandle,desc: APIDescription) -> Self {
        let entry = ash::Entry::linked();
        let app_name = CString::new("Vulkan Application").unwrap();
        let engine_name = CString::new("Prism").unwrap();
        let app_info = ash::vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .api_version(ash::vk::make_api_version(0,1,2,1))
            .build();

        let instance_extensions = ash_window::enumerate_required_extensions(RawDisplayHandle::Windows(WindowsDisplayHandle::empty())).unwrap();

        let instance_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extensions)
            .build();

        let instance = unsafe { entry.create_instance(&instance_info,None).unwrap() };

        let surface_khr = unsafe {
            ash_window::create_surface(&entry, &instance, RawDisplayHandle::Windows(WindowsDisplayHandle::empty()), handle, None).unwrap()
        };

        let surface_fn = ash::extensions::khr::Surface::new(&entry,&instance);

        Self {
            entry,
            instance,
            surface_khr,
            surface_fn,
        }
    }
}