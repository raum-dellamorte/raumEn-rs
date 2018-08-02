
//! Taking the bloat out of main.
#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal", feature = "gl"))]
pub mod display {
  use back;
  // use back::Backend;
  use hal;
  use hal::window::Extent2D;
  use hal::{format as f, image as i, };
  use hal::{Instance, };
  use winit;
  
  /// Window size
  pub const DIMS: Extent2D = Extent2D { width: 1024, height: 768 };
  /// Don't know what this is for yet.
  pub const ENTRY_NAME: &str = "main";
  /// Color Range for images
  pub const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
  };
  
  /// Make needed variables for window generation non gl
  #[cfg(not(feature = "gl"))]
  pub fn make_non_gl_backend(title: &str) -> (
      winit::EventsLoop, winit::Window,
      Box<Instance<Backend=back::Backend>>, 
      Box<Vec<hal::Adapter<back::Backend>>>, 
      Box<back::window::Surface>) {
    let wb = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::from_physical(winit::dpi::PhysicalSize {
          width: DIMS.width as _,
          height: DIMS.height as _,
        }, 1.0))
          .with_title(title.to_string());
    let events_loop = winit::EventsLoop::new();
    let window = wb.build(&events_loop).unwrap();
    let instance = Box::new(back::Instance::create(&format!("{} gfx-rs", title), 1));
    let surface = Box::new(instance.create_surface(&window));
    let adapters = Box::new(instance.enumerate_adapters());
    (events_loop, window, instance, adapters, surface)
  }
  #[cfg(feature = "gl")]
  pub fn make_gl_backend(title: &str) -> (
      Box<Vec<hal::Adapter<back::Backend>>>, 
      Box<back::window::Surface>
    ) {
    let mut events_loop = winit::EventsLoop::new();
    let window = {
      let builder =
        back::config_context(back::glutin::ContextBuilder::new(), ColorFormat::SELF, None)
          .with_vsync(true);
      back::glutin::GlWindow::new(wb, builder, &events_loop).unwrap()
    };
    
    let surface = Box::new(back::Surface::from_window(window));
    let adapters = Box::new(surface.enumerate_adapters());
    (adapters, surface)
  }
}
