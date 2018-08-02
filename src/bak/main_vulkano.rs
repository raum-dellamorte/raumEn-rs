#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![feature(attr_literals)]
#![feature(custom_derive)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate nom;

//#[macro_use]
//extern crate glium;
//extern crate glutin;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;
extern crate image;
extern crate time;

//use glutin::Event;

//pub mod camera;
//pub mod entities;
//pub mod fbo;
//pub mod input;
pub mod model;
pub mod shaders;
//pub mod timer;
pub mod util;

use shaders::compute;

//use camera::Camera;
//use entities::entity::Entity;
//use entities::mobs::Mob;
//use fbo::FboWithDepth;
//use input::Handler;
//use model::import::load_obj;
//use model::mesh::{Mesh, MeshBuffers};
//use util::rmatrix::Matrix4f;
//use util::rvector::Vector3f;
use util::rvertex::Vertex2D;

//use timer::Timer;

//use glium::DisplayBuild;

use winit::EventsLoop;
use winit::WindowBuilder;
use vulkano_win::VkSurfaceBuild;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::command_buffer::CommandBuffer;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;
use image::ImageBuffer;
use image::Rgba;

use std::iter;
use std::mem;
use std::sync::Arc;

fn main() {
  // Setup Instance
  let instance = {
    let extensions = vulkano_win::required_extensions();
    Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
  };
  
  // Setup Physical Device
  let physical = PhysicalDevice::enumerate(&instance)
      .next().expect("no device available");
  // For debug
  println!("Using device: {} (type: {:?})", physical.name(), physical.ty());
  
  // Create Window
  let mut events_loop = EventsLoop::new();
  let window = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
  let mut dimensions = {
    let (width, height) = window.window().get_inner_size_pixels().unwrap();
    [width, height]
  };
  // Setup Device
  let queue = physical.queue_families().find(|&q| {
    q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
  }).expect("couldn't find a graphical queue family");
  let (device, mut queues) = {
    let device_ext = DeviceExtensions {
      khr_swapchain: true,
      .. DeviceExtensions::none()
    };
    Device::new(physical, physical.supported_features(), &device_ext,
                [(queue, 0.5)].iter().cloned()).expect("failed to create device")
  };
  // Setup Queue
  let queue = queues.next().unwrap();
  // Setup Swapchain
  let (mut swapchain, mut images) = {
    let caps = window.surface().capabilities(physical)
                     .expect("failed to get surface capabilities");
    // We choose the dimensions of the swapchain to match the current dimensions of the window.
    // If `caps.current_extent` is `None`, this means that the window size will be determined
    // by the dimensions of the swapchain, in which case we just use the width and height defined above.
    //let dimensions = caps.current_extent.unwrap_or([width, height]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    Swapchain::new(device.clone(), window.surface().clone(), caps.min_image_count, format,
                   dimensions, 1, caps.supported_usage_flags, &queue,
                   SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                   None).expect("failed to create swapchain")
  };
  // Pre Main Loop Setup
  let vertex_buffer = {
    CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), [
      Vertex2D { position: [-0.5, -0.25], tex_coords: [0.0, 0.0] },
      Vertex2D { position: [0.0, 0.5], tex_coords: [0.0, 0.0] },
      Vertex2D { position: [0.25, -0.1], tex_coords: [0.0, 0.0] }
    ].iter().cloned()).expect("failed to create buffer")
  };
  let vs = shaders::model::model_vert::Shader::load(device.clone()).expect("failed to create shader module");
  let fs = shaders::model::model_frag::Shader::load(device.clone()).expect("failed to create shader module");
  
  // The next step is to create a *render pass*, which is an object that describes where the
  // output of the graphics pipeline will go. It describes the layout of the images
  // where the colors, depth and/or stencil information will be written.
  let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                // `load: Clear` means that we ask the GPU to clear the content of this
                // attachment at the start of the drawing.
                load: Clear,
                // `store: Store` means that we ask the GPU to store the output of the draw
                // in the actual image. We could also ask it to discard the result.
                store: Store,
                // `format: <ty>` indicates the type of the format of the image. This has to
                // be one of the types of the `vulkano::format` module (or alternatively one
                // of your structs that implements the `FormatDesc` trait). Here we use the
                // generic `vulkano::format::Format` enum because we don't know the format in
                // advance.
                format: swapchain.format(),
                // TODO:
                samples: 1,
            }
        },
        pass: {
            // We use the attachment named `color` as the one and only color attachment.
            color: [color],
            // No depth-stencil attachment is indicated with empty brackets.
            depth_stencil: {}
        }
    ).unwrap());
  
  let pipeline = Arc::new(GraphicsPipeline::start()
      // We need to indicate the layout of the vertices.
      // The type `SingleBufferDefinition` actually contains a template parameter corresponding
      // to the type of each vertex. But in this code it is automatically inferred.
      .vertex_input_single_buffer()
      // A Vulkan shader can in theory contain multiple entry points, so we have to specify
      // which one. The `main` word of `main_entry_point` actually corresponds to the name of
      // the entry point.
      .vertex_shader(vs.main_entry_point(), ())
      // The content of the vertex buffer describes a list of triangles.
      .triangle_list()
      // Use a resizable viewport set to draw over the entire window
      .viewports_dynamic_scissors_irrelevant(1)
      // See `vertex_shader`.
      .fragment_shader(fs.main_entry_point(), ())
      // We have to indicate which subpass of which render pass this pipeline is going to be used
      // in. The pipeline will only be usable from this particular subpass.
      .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
      // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
      .build(device.clone())
      .unwrap());
  
  // The render pass we created above only describes the layout of our framebuffers. Before we
  // can draw we also need to create the actual framebuffers.
  //
  // Since we need to draw to multiple images, we are going to create a different framebuffer for
  // each image.
  let mut framebuffers: Option<Vec<Arc<vulkano::framebuffer::Framebuffer<_,_>>>> = None;
  
  // Initialization is finally finished!
  
  // Flag for invalidating swapchain
  let mut recreate_swapchain = false;
  
  // In the loop below we are going to submit commands to the GPU. Submitting a command produces
  // an object that implements the `GpuFuture` trait, which holds the resources for as long as
  // they are in use by the GPU.
  //
  // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
  // that, we store the submission of the previous frame here.
  let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;
  
  loop {
    // It is important to call this function from time to time, otherwise resources will keep
    // accumulating and you will eventually reach an out of memory error.
    // Calling this function polls various fences in order to determine what the GPU has
    // already processed, and frees the resources that are no longer needed.
    previous_frame_end.cleanup_finished();
    
    // If the swapchain needs to be recreated, recreate it
    if recreate_swapchain {
      dimensions = {
        let (new_width, new_height) = window.window().get_inner_size_pixels().unwrap();
        [new_width, new_height]
      };
      let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
        Ok(r) => r,
        // This error tends to happen when the user is manually resizing the window.
        // Simply restarting the loop is the easiest way to fix this issue.
        Err(SwapchainCreationError::UnsupportedDimensions) => {
          continue;
        },
        Err(err) => panic!("{:?}", err)
      };
      mem::replace(&mut swapchain, new_swapchain);
      mem::replace(&mut images, new_images);
      framebuffers = None;
      recreate_swapchain = false;
    }
    
    // Because framebuffers contains an Arc on the old swapchain, we need to
    // recreate framebuffers as well.
    if framebuffers.is_none() {
      let new_framebuffers = Some(images.iter().map(|image| {
        Arc::new(Framebuffer::start(render_pass.clone())
            .add(image.clone()).unwrap()
            .build().unwrap())
      }).collect::<Vec<_>>());
      mem::replace(&mut framebuffers, new_framebuffers);
    }
    
    // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
    // no image is available (which happens if you submit draw commands too quickly), then the
    // function will block.
    // This operation returns the index of the image that we are allowed to draw upon.
    //
    // This function can block if no image is available. The parameter is an optional timeout
    // after which the function call will return an error.
    let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
      Ok(r) => r,
      Err(AcquireError::OutOfDate) => {
        recreate_swapchain = true;
        continue;
      },
      Err(err) => panic!("{:?}", err)
    };
    
    // In order to draw, we have to build a *command buffer*. The command buffer object holds
    // the list of commands that are going to be executed.
    //
    // Building a command buffer is an expensive operation (usually a few hundred
    // microseconds), but it is known to be a hot path in the driver and is expected to be
    // optimized.
    //
    // Note that we have to pass a queue family when we create the command buffer. The command
    // buffer will only be executable on that given queue family.
    let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
        // Before we can draw, we have to *enter a render pass*. There are two methods to do
        // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
        // not covered here.
        //
        // The third parameter builds the list of values to clear the attachments with. The API
        // is similar to the list of attachments when building the framebuffers, except that
        // only the attachments that use `load: Clear` appear in the list.
        .begin_render_pass(framebuffers.as_ref().unwrap()[image_num].clone(), false,
            vec![[0.0, 0.0, 1.0, 1.0].into()])
        .unwrap()
        // We are now inside the first subpass of the render pass. We add a draw command.
        //
        // The last two parameters contain the list of resources to pass to the shaders.
        // Since we used an `EmptyPipeline` object, the objects have to be `()`.
        .draw(pipeline.clone(),
              DynamicState {
                line_width: None,
                // TODO: Find a way to do this without having to dynamically allocate a Vec every frame.
                viewports: Some(vec![Viewport {
                  origin: [0.0, 0.0],
                  dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                  depth_range: 0.0 .. 1.0,
                }]),
                scissors: None,
              },
              vertex_buffer.clone(), (), ())
        .unwrap()
        
        // We leave the render pass by calling `draw_end`. Note that if we had multiple
        // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
        // next subpass.
        .end_render_pass()
        .unwrap()
        // Finish building the command buffer by calling `build`.
        .build().unwrap();
    
    let future = previous_frame_end.join(acquire_future)
                                   .then_execute(queue.clone(), command_buffer).unwrap()
        
        // The color output is now expected to contain our triangle. But in order to show it on
        // the screen, we have to *present* the image by calling `present`.
        //
        // This function does not actually present the image immediately. Instead it submits a
        // present command at the end of the queue. This means that it will only be presented once
        // the GPU has finished executing the command buffer that draws the triangle.
                                   .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                                   .then_signal_fence_and_flush().unwrap();
    previous_frame_end = Box::new(future) as Box<_>;
    
    // Note that in more complex programs it is likely that one of `acquire_next_image`,
    // `command_buffer::submit`, or `present` will block for some time. This happens when the
    // GPU's queue is full and the driver has to wait until the GPU finished some work.
    //
    // Unfortunately the Vulkan API doesn't provide any way to not wait or to detect when a
    // wait would happen. Blocking may be the desired behavior, but if you don't want to
    // block you should spawn a separate thread dedicated to submissions.
    
    // Handling the window events in order to close the program when the user wants to close
    // it.
    let mut done = false;
    events_loop.poll_events(|ev| {
      match ev {
        winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
        winit::Event::WindowEvent { event: winit::WindowEvent::Resized(_, _), .. } => recreate_swapchain = true,
        _ => ()
      }
    });
    if done { return; }
  }
  
//  let mut cam = Camera::create(
//    glium::glutin::WindowBuilder::new()
//      .with_title(format!("RaumEn Test"))
//      .with_dimensions(1024, 760)
//      .with_depth_buffer(24)
//      .build_glium().unwrap()
//  );
  
  //use model::import::test_nom;
  //test_nom();
  
//  let mut timer = Timer::new();
//
//  let mut entity = Entity::new("spaceship");
//  entity.load_model_defaults(&cam.display);
//  let mut focus = Mob::new("spaceship");
//  focus.entity.load_model_defaults(&cam.display).marker.pos.from_isize(0, 0, -20);
//  let mut handler = Handler::new();
//
//  let fbuffer = FboWithDepth::new_default(&cam);
//
//  loop {
//    use glium::Surface;
//
//    timer.tick();
//    cam.load_target();
//
//    // Calc Movement
//    entity.marker.inc_yrot(0.01_f32);
//    //focus.entity.marker.inc_yrot(-0.01_f32);
//    focus.move_mob(&mut handler, timer.delta);
//    cam.calc_pos(&focus.entity.marker);
//
//    // Draw!
//    let mut fbo = fbuffer.fb(&cam);
//    cam.draw_entity_surface(&mut entity, &mut fbo);
//    cam.draw_entity_surface(&mut focus.entity, &mut fbo);
//    {cam.target.as_mut().unwrap().clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);}
//    cam.draw_entity(&mut entity);
//    cam.draw_entity(&mut focus.entity);
//
//
//    // Finish!
//    cam.finish();
//
//    // listing the events produced by the window and waiting to be received
//    for event in cam.display.poll_events() {
//      match event {
//        glium::glutin::Event::Closed => return,   // the window has been closed by the user
//        ev => handler.event(&ev)
//      }
//    }
//  }
}
