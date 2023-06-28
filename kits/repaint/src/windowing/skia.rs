
use std::{rc::Rc, cell::RefCell, ffi::CString, num::NonZeroU32};

use gl::types::GLint;
use glutin::{config::ConfigTemplateBuilder, prelude::PossiblyCurrentContextGlSurfaceAccessor};
use glutin_winit::DisplayBuilder;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use repaint::Canvas;
use repaint_with_skia_safe::SkiaCanvas;
use skia_safe::{gpu::{gl::FramebufferInfo, BackendRenderTarget}, ColorType};
use winit::{window::{Window as WinitWindow, WindowBuilder}, dpi::PhysicalSize, event::WindowEvent, event_loop::{ControlFlow, EventLoop as WinitEventLoop}};
use glutin::{prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor}, context::{ContextAttributesBuilder, ContextApi, NotCurrentContext}, display::GetGlDisplay, surface::{SurfaceAttributesBuilder, WindowSurface, GlSurface, SurfaceAttributes}};

use glutin::config::Config as GlutinConfig;
use glutin::surface::Surface as GlutinSurface;
use glutin::context::PossiblyCurrentContext;

mod renderer;

use crate::SPainter;

use super::{ReLoop, ReWindow};

pub struct SkiaWindow {
    env: SkiaGlEnv,
    to_repaint: bool,
}

impl SkiaWindow {
    pub fn new_no_register(re_loop: &mut ReLoop) -> Self {
        let (window, gl_config) = create_gl_window(
            WindowBuilder::new()
                .with_title("rust-skia-gl-window")
                .with_transparent(true), // TODO https://github.com/rust-windowing/winit/issues/538
            &re_loop.event_loop,
            ConfigTemplateBuilder::new()
                .with_alpha_size(8),
        );
        println!("Picked a config with {} samples", gl_config.num_samples());

        let window = window.expect("Could not create window with OpenGL context");

        // TODO remove this
        //window.set_enable(false);
        //window.set_inner_size(winit::dpi::Size::new(winit::dpi::LogicalSize::new(
        //    1024.0 / 4.0, 1024.0 / 4.0,
        //)));

        let env = SkiaGlEnv::new(window, gl_config);

        Self {
            env,
            to_repaint: false,
        }
    }

    pub fn new(re_loop: &mut ReLoop) -> Rc<RefCell<Self>> {
        let s = Self::new_no_register(re_loop);
        re_loop.register_window(s)
    }

    pub fn request_redraw(&mut self) {
        self.to_repaint = true;
        self.instance().request_redraw();
    }

    pub fn resized(&mut self, physical_size: &PhysicalSize<u32>) {
        self.env.resized(*physical_size);
    }

    pub fn paint_with_skia_surface<T>(&mut self, f: impl FnOnce(&mut skia_safe::Surface) -> T) -> T {
        self.env.gl_context.make_current(&self.env.gl_surface).unwrap();
        let r = f(&mut self.env.surface);
        self.env.gr_context.flush_and_submit();
        self.env.gl_surface.swap_buffers(&self.env.gl_context).unwrap();
        r
    }

    pub fn paint_with_skia_canvas<T>(&mut self, f: impl FnOnce(&mut skia_safe::Canvas) -> T) -> T {
        self.paint_with_skia_surface(|surface| f(surface.canvas()))
    }

    pub fn paint_with_skia_painter<T>(&mut self, f: impl FnOnce(&mut SPainter) -> T) -> T {
        self.paint_with_skia_surface(|surface| {
            let w = surface.width() as f64;
            let h = surface.height() as f64;
            let mut canvas = SkiaCanvas::new(surface.canvas(), w, h);
            let mut painter = canvas.painter().unwrap();
            f(&mut painter)
        })
    }
}

impl ReWindow for SkiaWindow {
    fn instance(&self) -> &WinitWindow {
        &self.env.window
    }

    fn handle_event(&mut self, event: &WindowEvent) -> Option<ControlFlow> {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.resized(physical_size);
            },
            _ => {}
        }
        None
    }

    fn main_events_cleared(&mut self) {
        if self.to_repaint {
            self.instance().request_redraw();
        }
    }

    fn draw(&mut self) {
        self.to_repaint = false;
    }
}



// Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
// `DirectContext`.
//
// https://github.com/rust-skia/rust-skia/issues/476
struct SkiaGlEnv {
    surface: skia_safe::Surface,
    gl_surface: GlutinSurface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
    window: WinitWindow,
}

impl Drop for SkiaGlEnv {
    fn drop(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }
}

impl SkiaGlEnv {
    fn new(
        mut window: WinitWindow,
        gl_config: GlutinConfig,
    ) -> Self {
        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });

        let (
            not_current_gl_context,
            gl_surface
        ) = make_not_current_gl_context_and_surface(
            &window,
            &gl_config
        );

        let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current when setting up skia renderer");

        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };
    
            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let surface = create_surface(
            &mut window,
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        Self {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            fb_info,
            num_samples,
            stencil_size,
            window,
        }
    }

    fn resized(&mut self, physical_size: PhysicalSize<u32>) {
        self.surface = create_surface(
            &mut self.window,
            self.fb_info,
            &mut self.gr_context,
            self.num_samples,
            self.stencil_size,
        );
        /* First resize the opengl drawable */
        let (width, height): (u32, u32) = physical_size.into();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );
    }
}

fn create_gl_window(
    winit_window_builder: WindowBuilder,
    el: &WinitEventLoop<()>,
    template: ConfigTemplateBuilder,
) -> (Option<WinitWindow>, GlutinConfig) {
    let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));

    let (window, gl_config) = display_builder
        .build(&el, template, |configs| {
            // Find the config with the minimum number of samples. Usually Skia takes care of
            // anti-aliasing and may not be able to create appropriate Surfaces for samples > 0.
            // See https://github.com/rust-skia/rust-skia/issues/782
            // And https://github.com/rust-skia/rust-skia/issues/764
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    (window, gl_config)
}

fn make_not_current_gl_context_and_surface(
    window: &WinitWindow,
    gl_config: &GlutinConfig,
) -> (NotCurrentContext, GlutinSurface<WindowSurface>) {
    let raw_window_handle = window.raw_window_handle();

    let surface_attributes = make_surface_attributes(
        &window,
        raw_window_handle
    );

    let gl_surface = create_gl_surface(
        &gl_config,
        &surface_attributes,
    );

    let not_current_gl_context = make_not_current_gl_context(
        raw_window_handle,
        &gl_config,
    );

    (not_current_gl_context, gl_surface)
}

fn make_surface_attributes(
    window: &WinitWindow,
    raw_window_handle: RawWindowHandle,
) -> SurfaceAttributes<WindowSurface> {
    let (width, height) = window.inner_size().into();

    SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    )
}

fn create_gl_surface(
    gl_config: &GlutinConfig,
    surface_attributes: &SurfaceAttributes<WindowSurface>,
) -> GlutinSurface<WindowSurface> {
    unsafe {
        gl_config
            .display()
            .create_window_surface(gl_config, surface_attributes)
            .expect("Could not create gl window surface")
    }
}

fn make_not_current_gl_context(
    raw_window_handle: RawWindowHandle,
    gl_config: &GlutinConfig,
) -> NotCurrentContext {
    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };

    not_current_gl_context
}

fn create_surface(
    window: &mut WinitWindow,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> skia_safe::Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        BackendRenderTarget::new_gl(size, num_samples, stencil_size, fb_info);

    use skia_safe::gpu::SurfaceOrigin;

    skia_safe::Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}