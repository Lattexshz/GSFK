use gsfk::api::vulkan::{VulkanAPIExt};
use gsfk::window::Window;
use gsfk::WindowImplementation;
use std::ffi::CString;

fn main() {
    let (window, vulkan) = Window::new_with_vulkan("Vulkan Window!", 500, 500);
    let vk = vulkan.get_api();

    let app_name = CString::new("Vulkan Window").unwrap();
    let engine_name = CString::new("My Vulkan engine").unwrap();
    let app_info = ash::vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .engine_name(&engine_name)
        .api_version(ash::vk::make_api_version(0, 1, 2, 1))
        .build();

    let entry = ash::Entry::linked();

    let required_extensions = vk.get_required_instance_extensions(&window);

    let instance_info = ash::vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&required_extensions)
        .build();

    let instance = unsafe { entry.create_instance(&instance_info, None).unwrap() };

    let _surface = vk
        .create_window_surface(&entry, &instance, &window)
        .unwrap();

    window.show();
    window.run(|_event, _flow| {})
}
