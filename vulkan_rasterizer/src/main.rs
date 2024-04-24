#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use vulkano::swapchain::{self, SwapchainPresentInfo};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use log::info;
use std::sync::Arc;

pub mod core;
use core::rhi;

fn main() -> Result<()> {
    // Set log level
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Vulkan Rasterizer (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap());

    // App
    let mut app = App::create(&window, &event_loop)?;
    let mut destroying = false;
    let mut window_resized = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying =>
            {
                app.render(&window);
            }
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                { app.destroy(); }
            },
            // Resize the window.
            Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                // Resize the swapchain and recreate the framebuffers.
                window_resized = true;
            }
            _ => {}
        }
    });

}




#[derive(Clone)]
struct App {
    pub renderer: rhi::Renderer,
    pub recreate_swapchain: bool
}

impl App {

    /// Creates our Vulkan app.
    fn create(window: &Arc<Window>, event_loop: &EventLoop<()>) -> Result<Self> {
        use rhi::RHIDevice;
        use rhi::Renderer;

        Ok(Self {renderer: Renderer::new(RHIDevice::new(window, event_loop), window), recreate_swapchain: false})
    }

    /// Renders a frame for our Vulkan app.
    fn render(&mut self, window: &Arc<Window>) {
        use vulkano::swapchain;
        use vulkano::swapchain::SwapchainCreateInfo;
        use vulkano::sync::*;
        use vulkano::{Validated, VulkanError};

        if self.recreate_swapchain {
            self.recreate_swapchain = false;

            let new_dimensions = window.inner_size();

            let (new_swapchain, new_images) = self.renderer.device.swapchain.recreate(SwapchainCreateInfo {
                image_extent: new_dimensions.into(),
                ..self.renderer.device.swapchain.create_info()
            }).expect("Failed to recreate swapchain");
            //swapchain = new_swapchain;
        }


        // rhi::Renderer::render_fractal(device.device.clone(), &device.queue);
        self.renderer.render_base(window);

        let (image_i, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.renderer.device.swapchain.clone(), None)
            .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next swapchain image {e}"),
            };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        let execution = vulkano::sync::now(self.renderer.device.device.clone())
            .join(acquire_future)
            .then_execute(self.renderer.device.queue.clone(), self.renderer.command_buffers[image_i as usize].clone())
            .unwrap()
            .then_swapchain_present(
                self.renderer.device.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.renderer.device.swapchain.clone(), image_i))
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            Ok(future) => {
                future.wait(None).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to flush future: {:?}", e);
            }
        }
    }

    /// Destroys our Vulkan app.
    fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
