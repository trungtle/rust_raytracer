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

mod rhi {
    pub mod vulkan {
        use image::*;
        use log::info;
        use std::sync::Arc;
        use vulkano::buffer::*;
        use vulkano::command_buffer::*;
        use vulkano::command_buffer::allocator::*;
        use vulkano::descriptor_set::allocator::*;
        use vulkano::descriptor_set::*;
        use vulkano::device::*;
        use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
        use vulkano::format::*;
        use vulkano::image::*;
        use vulkano::image::view::*;
        use vulkano::instance::{Instance, InstanceCreateInfo};
        use vulkano::memory::allocator::*;
        use vulkano::pipeline::*;
        use vulkano::pipeline::compute::*;
        use vulkano::pipeline::graphics::*;
        use vulkano::pipeline::graphics::color_blend::*;
        use vulkano::pipeline::graphics::input_assembly::*;
        use vulkano::pipeline::graphics::multisample::*;
        use vulkano::pipeline::graphics::rasterization::*;
        use vulkano::pipeline::graphics::vertex_input::*;
        use vulkano::pipeline::graphics::viewport::*;
        use vulkano::pipeline::layout::*;
        use vulkano::render_pass::*;
        use vulkano::shader::*;
        use vulkano::swapchain::*;
        use vulkano::sync::GpuFuture;
        use vulkano::VulkanLibrary;
        use winit::window::Window;
        use winit::event_loop::EventLoop;

        #[derive(BufferContents, Vertex)]
        #[repr(C)]
        struct VertexPositionOnly {
            #[format(R32G32_SFLOAT)]
            position: [f32; 2],
        }


        #[derive(Clone, Debug)]
        pub struct RHIDevice {
            pub device: Arc<Device>,
            pub queue: Arc<Queue>,
            pub swapchain: Arc<Swapchain>,
            pub swapchain_images: Vec<Arc<Image>>,
        }

        impl RHIDevice {
            fn select_physical_device(
                instance: &Arc<Instance>,
                surface: &Arc<Surface>,
                device_extensions: &DeviceExtensions,
            ) -> (Arc<PhysicalDevice>, u32) {
                instance
                    .enumerate_physical_devices()
                    .expect("could not enumerate devices")
                    .filter(|p| p.supported_extensions().contains(&device_extensions))
                    .filter_map(|p| {
                        p.queue_family_properties()
                            .iter()
                            .enumerate()
                            // Find the first first queue family that is suitable.
                            // If none is found, `None` is returned to `filter_map`,
                            // which disqualifies this physical device.
                            .position(|(i, q)| {
                                q.queue_flags.contains(QueueFlags::GRAPHICS)
                                    && p.surface_support(i as u32, &surface).unwrap_or(false)
                            })
                            .map(|q| (p, q as u32))
                    })
                    .min_by_key(|(p, _)| match p.properties().device_type {
                        PhysicalDeviceType::DiscreteGpu => 0,
                        PhysicalDeviceType::IntegratedGpu => 1,
                        PhysicalDeviceType::VirtualGpu => 2,
                        PhysicalDeviceType::Cpu => 3,

                        // Note that there exists `PhysicalDeviceType::Other`, however,
                        // `PhysicalDeviceType` is a non-exhaustive enum. Thus, one should
                        // match wildcard `_` to catch all unknown device types.
                        _ => 4,
                    })
                    .expect("no device available")
            }


            pub fn new(window: &Arc<Window>, event_loop: &EventLoop<()>) -> Arc<RHIDevice> {
                let device_extensions = DeviceExtensions {
                    khr_swapchain: true,
                    ..DeviceExtensions::empty()
                };

                let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
                let required_extensions = Surface::required_extensions(&event_loop);
                let instance = Instance::new(library, InstanceCreateInfo {
                    enabled_extensions: required_extensions,
                    ..Default::default()
                }).expect("failed to create instance");

                let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

                let (physical_device, queue_family_index) = RHIDevice::select_physical_device(
                    &instance,
                    &surface,
                    &device_extensions,
                );

                let version = physical_device.api_version();
                info!("Found physical device supported Vulkan version {:?}", version);

                for family in physical_device.queue_family_properties() {
                    info!("Found a queue family with {:?} queue(s)", family.queue_count);
                }

                let (device, mut queues) = Device::new(
                    physical_device.clone(),
                    DeviceCreateInfo {
                        queue_create_infos: vec![QueueCreateInfo {
                            queue_family_index,
                            ..Default::default()
                        }],
                        enabled_extensions: device_extensions,
                        ..Default::default()
                    },
                )
                .expect("failed to create device");

                info!("Created device - Vulkan {:?} with extensions {:?}", device.api_version(), device.enabled_extensions());

                let queue = queues.next().unwrap();

                // Create swapchain
                let caps = physical_device
                    .surface_capabilities(&surface, Default::default())
                    .expect("failed to get surface capabilities");

                let dimensions = window.inner_size();
                let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
                let image_format =  physical_device
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0;

                let (swapchain, swapchain_images) = Swapchain::new(
                    device.clone(),
                    surface.clone(),
                    SwapchainCreateInfo {
                        min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
                        image_format,
                        image_extent: dimensions.into(),
                        image_usage: ImageUsage::COLOR_ATTACHMENT, // What the images are going to be used for
                        composite_alpha,
                        ..Default::default()
                    },
                )
                .unwrap();

                info!("Created swapchain with {:?} images", swapchain_images.len());

                Arc::from(Self {device, queue, swapchain, swapchain_images})
            }
        }


        #[derive(Clone)]
        pub struct Renderer {
            pub device: Arc<RHIDevice>,
            pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
        }

        impl Renderer {
            pub fn new(device: Arc<RHIDevice>, window: &Arc<Window>) -> Self {
                // ---- Setup resources ----
                let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.device.clone()));

                // Base triangle
                let vertex1 = VertexPositionOnly { position: [-0.5, -0.5] };
                let vertex2 = VertexPositionOnly { position: [ 0.0,  0.5] };
                let vertex3 = VertexPositionOnly { position: [ 0.5, -0.25] };

                // Create vertex buffer from host data
                let vertex_buffer = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::VERTEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    vec![vertex1, vertex2, vertex3],
                )
                .unwrap();

                // Image used to draw the triangle renders
                let image = vulkano::image::Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_UNORM,
                        extent: [1024, 1024, 1],
                        usage: ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    },
                )
                .unwrap();
                let view = ImageView::new_default(image.clone()).unwrap();

                // Output buffer for saving into file
                let out_buf = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_DST,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST
                            | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                        ..Default::default()
                    },
                    (0..1024 * 1024 * 4).map(|_| 0u8),
                )
                .expect("failed to create buffer");

                mod vs {
                    vulkano_shaders::shader!{
                        ty: "vertex",
                        src: r"
                            #version 460

                            layout(location = 0) in vec2 position;

                            void main() {
                                gl_Position = vec4(position, 0.0, 1.0);
                            }
                        ",
                    }
                }

                mod fs {
                    vulkano_shaders::shader!{
                        ty: "fragment",
                        src: "
                            #version 460

                            layout(location = 0) out vec4 f_color;

                            void main() {
                                f_color = vec4(1.0, 0.0, 0.0, 1.0);
                            }
                        ",
                    }
                }

                let vs = vs::load(device.device.clone()).expect("failed to create shader module");
                let fs = fs::load(device.device.clone()).expect("failed to create shader module");

                let render_pass = Renderer::get_render_pass(device.device.clone(), &device.swapchain);

                let framebuffer = Renderer::get_framebuffers(&device.swapchain_images, render_pass.clone());

                // Let viewport be fullscreen
                let viewport = Viewport {
                    offset: [0.0, 0.0],
                    extent: window.inner_size().into(),
                    depth_range: 0.0..=1.0,
                };

                let pipeline = Renderer::get_graphics_pipeline(device.device.clone(), vs, fs, render_pass.clone(), viewport.clone());

                // NOTE: Uncomment if our vs and fs has descriptor set
                // Allocate descriptors set
                // let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone(), Default::default());
                // let pipeline_layout = pipeline.layout();
                // let descriptor_set_layouts = pipeline_layout.set_layouts();
                // // Write descriptor set
                // let descriptor_set_layout_index = 0;
                // let descriptor_set_layout = descriptor_set_layouts
                //     .get(descriptor_set_layout_index)
                //     .unwrap();
                // let descriptor_set = PersistentDescriptorSet::new(
                //     &descriptor_set_allocator,
                //     descriptor_set_layout.clone(),
                //     [WriteDescriptorSet::image_view(0, view.clone())], // 0 is the binding
                //     [],
                // )
                // .unwrap();

                let command_buffer_allocator = StandardCommandBufferAllocator::new(
                    device.device.clone(),
                    StandardCommandBufferAllocatorCreateInfo::default(),
                );

                // --- Draw --- //
                let command_buffers = Renderer::get_command_buffers(
                    &command_buffer_allocator,
                    &device.queue,
                    &pipeline,
                    &framebuffer,
                    &vertex_buffer,
                    viewport.clone()
                );

                Self {
                    device,
                    command_buffers,
                }
            }

            // An example of create buffer
            pub fn create_buffer(device: Arc<Device>) {
                use vulkano::buffer::*;
                use vulkano::memory::allocator::*;

                let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

                let data: i32 = 12;
                let buffer = Buffer::from_data(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    data,
                )
                .expect("failed to create buffer");
                info!("Create buffer data");

                let source_content: Vec<i32> = (0..64).collect();
                let source = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    source_content
                ).expect("failed to create source buffer");
                info!("Create source buffer");

                let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
                let destination = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_DST,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                    ..Default::default()
                    }, destination_content
                ).expect("failed to create destination buffer");
                info!("Create destination buffer");
            }

            pub fn render_fractal(device: Arc<Device>, queue: &Arc<Queue>) {

                // Create shader

                mod cs {
                    vulkano_shaders::shader!{
                        ty: "compute",
                        src: r"
                        #version 460

                        layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

                        layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;

                        void main() {
                            vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));
                            vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);

                            vec2 z = vec2(0.0, 0.0);
                            float i;
                            for (i = 0.0; i < 1.0; i += 0.005) {
                                z = vec2(
                                    z.x * z.x - z.y * z.y + c.x,
                                    z.y * z.x + z.x * z.y + c.y
                                );

                                if (length(z) > 4.0) {
                                    break;
                                }
                            }

                            vec4 to_write = vec4(vec3(i), 1.0);
                            imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
                        }
                        ",
                    }
                }

                let shader = cs::load(device.clone()).expect("failed to create shader module");

                let cs = shader.entry_point("main").unwrap();
                let stage = PipelineShaderStageCreateInfo::new(cs);
                let pipeline_layout = PipelineLayout::new(
                    device.clone(),
                    PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                        .into_pipeline_layout_create_info(device.clone())
                        .unwrap(),
                )
                .unwrap();

                let compute_pipeline = ComputePipeline::new(
                    device.clone(),
                    None,
                    ComputePipelineCreateInfo::stage_layout(stage, pipeline_layout),
                )
                .expect("failed to create compute pipeline");

                let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

                // Create command buffer
                let command_buffer_allocator = StandardCommandBufferAllocator::new(
                    device.clone(),
                    StandardCommandBufferAllocatorCreateInfo::default(),
                );

                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    &command_buffer_allocator,
                    queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                let image = vulkano::image::Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_UNORM,
                        extent: [1024, 1024, 1],
                        usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    },
                )
                .unwrap();

                let buf = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_DST,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST
                            | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                        ..Default::default()
                    },
                    (0..1024 * 1024 * 4).map(|_| 0u8),
                )
                .expect("failed to create buffer");

                // Clear the image with blue
                command_buffer_builder
                    .clear_color_image(ClearColorImageInfo {
                        clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
                        ..ClearColorImageInfo::image(image.clone())
                    })
                    .unwrap()
                    .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                        image.clone(),
                        buf.clone(),
                    ))
                    .unwrap();

                // Create fractal
                let image = Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_UNORM,
                        extent: [1024, 1024, 1],
                        usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    },
                )
                .unwrap();

                // Allocate descriptors set
                let descriptor_set_allocator =
                StandardDescriptorSetAllocator::new(device.clone(), Default::default());
                let pipeline_layout = compute_pipeline.layout();
                let descriptor_set_layouts = pipeline_layout.set_layouts();

                // Write descriptor set
                let descriptor_set_layout_index = 0;
                let descriptor_set_layout = descriptor_set_layouts
                    .get(descriptor_set_layout_index)
                    .unwrap();
                let descriptor_set = PersistentDescriptorSet::new(
                    &descriptor_set_allocator,
                    descriptor_set_layout.clone(),
                    [WriteDescriptorSet::image_view(0, ImageView::new_default(image.clone()).unwrap())], // 0 is the binding
                    [],
                )
                .unwrap();

                let work_group_counts = [1024 / 8, 1024 / 8, 1];

                command_buffer_builder
                    .bind_pipeline_compute(compute_pipeline.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        PipelineBindPoint::Compute,
                        compute_pipeline.layout().clone(),
                        descriptor_set_layout_index as u32,
                        descriptor_set,
                    )
                    .unwrap()
                    .dispatch(work_group_counts)
                    .unwrap()
                    .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                        image.clone(),
                        buf.clone(),
                    ))
                    .unwrap();

                let command_buffer = command_buffer_builder.build().unwrap();

                let future = vulkano::sync::now(device.clone())
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    .then_signal_fence_and_flush()
                    .unwrap();

                future.wait(None).unwrap();

                let buffer_content = buf.read().unwrap();
                let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
                image.save("image_fractal.png").unwrap();
            }

            pub fn render_base(&mut self, window: &Arc<Window>) {
                // ---- Setup resources ----
                let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(self.device.device.clone()));

                // Base triangle
                let vertex1 = VertexPositionOnly { position: [-0.5, -0.5] };
                let vertex2 = VertexPositionOnly { position: [ 0.0,  0.5] };
                let vertex3 = VertexPositionOnly { position: [ 0.5, -0.25] };

                // Create vertex buffer from host data
                let vertex_buffer = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::VERTEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    vec![vertex1, vertex2, vertex3],
                )
                .unwrap();

                // Image used to draw the triangle renders
                let image = vulkano::image::Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_UNORM,
                        extent: [1024, 1024, 1],
                        usage: ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    },
                )
                .unwrap();
                let view = ImageView::new_default(image.clone()).unwrap();

                // Output buffer for saving into file
                let out_buf = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_DST,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST
                            | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                        ..Default::default()
                    },
                    (0..1024 * 1024 * 4).map(|_| 0u8),
                )
                .expect("failed to create buffer");

                mod vs {
                    vulkano_shaders::shader!{
                        ty: "vertex",
                        src: r"
                            #version 460

                            layout(location = 0) in vec2 position;

                            void main() {
                                gl_Position = vec4(position, 0.0, 1.0);
                            }
                        ",
                    }
                }

                mod fs {
                    vulkano_shaders::shader!{
                        ty: "fragment",
                        src: "
                            #version 460

                            layout(location = 0) out vec4 f_color;

                            void main() {
                                f_color = vec4(1.0, 0.0, 0.0, 1.0);
                            }
                        ",
                    }
                }

                let vs = vs::load(self.device.device.clone()).expect("failed to create shader module");
                let fs = fs::load(self.device.device.clone()).expect("failed to create shader module");

                // Renderpass
                let render_pass = Renderer::get_render_pass(self.device.device.clone(), &self.device.swapchain);

                // Framebuffer
                let framebuffer = Renderer::get_framebuffers(&self.device.swapchain_images, render_pass.clone());

                // Let viewport be fullscreen
                let viewport = Viewport {
                    offset: [0.0, 0.0],
                    extent: window.inner_size().into(),
                    depth_range: 0.0..=1.0,
                };

                let pipeline = Renderer::get_graphics_pipeline(self.device.device.clone(), vs, fs, render_pass.clone(), viewport.clone());

                // NOTE: Uncomment if our vs and fs has descriptor set
                // Allocate descriptors set
                // let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone(), Default::default());
                // let pipeline_layout = pipeline.layout();
                // let descriptor_set_layouts = pipeline_layout.set_layouts();
                // // Write descriptor set
                // let descriptor_set_layout_index = 0;
                // let descriptor_set_layout = descriptor_set_layouts
                //     .get(descriptor_set_layout_index)
                //     .unwrap();
                // let descriptor_set = PersistentDescriptorSet::new(
                //     &descriptor_set_allocator,
                //     descriptor_set_layout.clone(),
                //     [WriteDescriptorSet::image_view(0, view.clone())], // 0 is the binding
                //     [],
                // )
                // .unwrap();

                let command_buffer_allocator = StandardCommandBufferAllocator::new(
                    self.device.device.clone(),
                    StandardCommandBufferAllocatorCreateInfo::default(),
                );

                // --- Draw --- //
                self.command_buffers = Renderer::get_command_buffers(
                    &command_buffer_allocator,
                    &self.device.queue.clone(),
                    &pipeline,
                    &framebuffer,
                    &vertex_buffer,
                    viewport.clone()
                );
            }

            pub fn init_resources(device: Arc<Device>, queue: &Arc<Queue>) {
            }

            fn get_render_pass(
                device: Arc<vulkano::device::Device>,
                swapchain: &Arc<Swapchain>) -> Arc<RenderPass>
            {
                vulkano::single_pass_renderpass!(
                    device.clone(),
                    attachments: {
                        color: {
                            format: swapchain.image_format(),
                            samples: 1,
                            load_op: Clear,
                            store_op: Store,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {},
                    },
                )
                .unwrap()
            }

            fn get_framebuffers(
                images: &[Arc<Image>],
                render_pass: Arc<RenderPass>
            ) -> Vec<Arc<Framebuffer>>
            {
                images.iter().map(|image| {
                    let view = ImageView::new_default(image.clone()).unwrap();
                    Framebuffer::new(
                        render_pass.clone(),
                        FramebufferCreateInfo {
                            attachments: vec![view.clone()],
                            ..Default::default()
                        },
                    ).unwrap()
                }).collect::<Vec<_>>()
            }

            fn get_graphics_pipeline(
                device: Arc<Device>,
                vs: Arc<ShaderModule>,
                fs: Arc<ShaderModule>,
                render_pass: Arc<RenderPass>,
                viewport: Viewport
            ) -> Arc<GraphicsPipeline>
            {
                let vs = vs.entry_point("main").unwrap();
                let fs = fs.entry_point("main").unwrap();

                let vertex_input_state = VertexPositionOnly::per_vertex()
                    .definition(&vs.info().input_interface)
                    .unwrap();

                let stages = [
                    PipelineShaderStageCreateInfo::new(vs),
                    PipelineShaderStageCreateInfo::new(fs),
                ];

                let layout = PipelineLayout::new(
                    device.clone(),
                    PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                        .into_pipeline_layout_create_info(device.clone())
                        .unwrap(),
                )
                .unwrap();

                let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

                GraphicsPipeline::new(
                    device.clone(),
                    None,
                    GraphicsPipelineCreateInfo {
                        // The stages of our pipeline, we have vertex and fragment stages.
                        stages: stages.into_iter().collect(),
                        // Describes the layout of the vertex input and how should it behave.
                        vertex_input_state: Some(vertex_input_state),
                        // Indicate the type of the primitives (the default is a list of triangles).
                        input_assembly_state: Some(InputAssemblyState::default()),
                        // Set the fixed viewport.
                        viewport_state: Some(ViewportState {
                            viewports: [viewport].into_iter().collect(),
                            ..Default::default()
                        }),
                        // Ignore these for now.
                        rasterization_state: Some(RasterizationState::default()),
                        multisample_state: Some(MultisampleState::default()),
                        color_blend_state: Some(ColorBlendState::with_attachment_states(
                            subpass.num_color_attachments(),
                            ColorBlendAttachmentState::default(),
                        )),
                        // NOTE: Set dynamic state if we want to resize viewports
                        // dynamic_state: dynamic_state.into_iter().collect(),
                        // This graphics pipeline object concerns the first pass of the render pass.
                        subpass: Some(subpass.into()),
                        ..GraphicsPipelineCreateInfo::layout(layout)
                    },
                )
                .unwrap()
            }

            fn get_command_buffers(
                command_buffer_allocator: &StandardCommandBufferAllocator,
                queue: &Arc<Queue>,
                pipeline: &Arc<GraphicsPipeline>,
                framebuffers: &Vec<Arc<Framebuffer>>,
                vertex_buffer: &Subbuffer<[VertexPositionOnly]>,
                viewport: Viewport
            ) -> Vec<Arc<PrimaryAutoCommandBuffer>>
            {
                framebuffers.iter().map(|framebuffer| {
                    let mut builder = AutoCommandBufferBuilder::primary(
                        command_buffer_allocator,
                        queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    )
                    .unwrap();

                    // --- Draw --- //
                    builder
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                            },
                            SubpassBeginInfo {
                                contents: SubpassContents::Inline,
                                ..Default::default()
                            },
                        )
                        .unwrap()
                        .bind_pipeline_graphics(pipeline.clone())
                        .unwrap()
                        // NOTE: Uncomment if our vs and fs has descriptor set
                        // .bind_descriptor_sets(
                        //     PipelineBindPoint::Graphics,
                        //     pipeline.layout().clone(),
                        //     descriptor_set_layout_index as u32,
                        //     descriptor_set,
                        // )
                        // .unwrap()
                        .bind_vertex_buffers(0, vertex_buffer.clone())
                        .unwrap()
                        .draw(3, 1, 0, 0)
                        .unwrap()
                        .end_render_pass(SubpassEndInfo::default())
                        .unwrap();

                    // NOTE: Set dynamic state if we want to resize viewports
                    // let viewports = smallvec::SmallVec::<[Viewport; 2]>::from([viewport.clone(), viewport.clone()]);
                    // builder.set_viewport(0, viewports).unwrap();

                    builder.build().unwrap()
                }).collect()
            }
        }

    }
}



#[derive(Clone)]
struct App {
    pub renderer: rhi::vulkan::Renderer,
    pub recreate_swapchain: bool
}

impl App {

    /// Creates our Vulkan app.
    fn create(window: &Arc<Window>, event_loop: &EventLoop<()>) -> Result<Self> {
        use rhi::vulkan::RHIDevice;
        use rhi::vulkan::Renderer;

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


        // rhi::vulkan::Renderer::render_fractal(device.device.clone(), &device.queue);
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
