//use raw_window_handle::{Win32WindowHandle, WindowsDisplayHandle, HasRawWindowHandle, HasRawDisplayHandle};
use regui::{component::{FunctionsCache, GetFromCache}, decl_function_component, function_component::State};
use regui_nwg::{NwgNode, components::ExternCanvas};

use native_windows_gui as nwg;
//use wgpu::InstanceDescriptor;
//use winapi::um::winuser::{GetWindow, GWL_HINSTANCE};

decl_function_component!(pub WgpuCanvas wgpu_canvas(()) -> NwgNode<nwg::ControlHandle>);

fn wgpu_canvas(props: &(), cache: &FunctionsCache, state: &mut State) -> NwgNode<nwg::ControlHandle> {

    let on_created = {
        move |canvas: &nwg::ExternCanvas| {
            //pollster::block_on(async {
                //let (width, height) = canvas.size();
//
                //let instance = wgpu::Instance::new(InstanceDescriptor::default());
//
                //let surface = unsafe {
                //    //let window = canvas.handle;
                //    //let hinstance = nwg::window_helper::get_window_long(canvas.handle.hwnd().unwrap(), nwg::HwndLong::HInstance).unwrap();
                //    let hwnd = canvas.handle.hwnd().unwrap();
                //    let hinstance = unsafe {
                //        GetWindow(hwnd, GWL_HINSTANCE as _)
                //    };
                //    let mut handle = Win32WindowHandle::empty();
                //    handle.hwnd = hwnd as _;
                //    handle.hinstance = hinstance as _;
                //    let handle = raw_window_handle::RawWindowHandle::Win32(handle);
                //    let display_handle = raw_window_handle::RawDisplayHandle::Windows(WindowsDisplayHandle::empty());
                //    struct MyCanvas {
                //        handle: raw_window_handle::RawWindowHandle,
                //        display_handle: raw_window_handle::RawDisplayHandle,
                //    };
                //    let my_canvas = MyCanvas {
                //        handle,
                //        display_handle,
                //    };
                //    unsafe impl HasRawWindowHandle for MyCanvas {
                //        fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
                //            self.handle
                //        }
                //    }
                //    unsafe impl HasRawDisplayHandle for MyCanvas {
                //        fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
                //            self.display_handle
                //        }
                //    }
                //    instance.create_surface(&my_canvas)
                //};

                //let surface = surface.unwrap();

                println!("OK");
            //});
        }
    };


    let canvas_node = ExternCanvas::builder()
        .position(250, 50)
        .size(50, 50)
        .on_event(|e| { println!("E: {:?}", e) })
        .on_created(on_created)
        .get(cache);

    canvas_node
}