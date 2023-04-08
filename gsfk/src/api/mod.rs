pub mod vulkan;

use raw_window_handle::RawWindowHandle;
use crate::{API, APIDescription};
use vulkan::Vulkan;

// pub fn choose_api<T>(desc: APIDescription, handle: RawWindowHandle) -> API<T> {
//     match handle {
//         RawWindowHandle::UiKit(_) => {
//             todo!()
//         }
//         RawWindowHandle::AppKit(_) => {
//             todo!()
//         }
//         RawWindowHandle::Orbital(_) => {
//             todo!()
//         }
//         RawWindowHandle::Xlib(_) => {
//             todo!()
//         }
//         RawWindowHandle::Xcb(_) => {
//             todo!()
//         }
//         RawWindowHandle::Wayland(_) => {
//             todo!()
//         }
//         RawWindowHandle::Drm(_) => {
//             todo!()
//         }
//         RawWindowHandle::Gbm(_) => {
//             todo!()
//         }
//         RawWindowHandle::Win32(_) => {
//             match desc.api {
//                 APIKind::Direct2D => {
//                     todo!()
//                 }
//                 APIKind::OpenGL => {
//                     todo!()
//                 }
//                 APIKind::Vulkan => {
//                     API {
//                         context: Vulkan {},
//                     }
//                 }
//             }
//         }
//         RawWindowHandle::WinRt(_) => {
//             todo!()
//         }
//         RawWindowHandle::Web(_) => {
//             todo!()
//         }
//         RawWindowHandle::AndroidNdk(_) => {
//             todo!()
//         }
//         RawWindowHandle::Haiku(_) => {
//             todo!()
//         }
//         _ => {}
//     }
// }