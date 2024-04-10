#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use vulkano::sync::GpuFuture;
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
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying =>
                { app.render(&window) }.unwrap(),
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                { app.destroy(); }
            }
            _ => {}
        }
    });
}

mod rhi {
    pub mod vulkan {
        use anyhow::Result;
        use log::info;
        use std::sync::Arc;
        use winit::window::Window;
        use winit::event_loop::EventLoop;
        use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
        use vulkano::device::DeviceExtensions;
        use vulkano::VulkanLibrary;
        use vulkano::instance::{Instance, InstanceCreateInfo};
        use vulkano::swapchain::Surface;

        #[derive(Clone, Debug)]
        pub struct RHIDevice {
            pub device: Arc<vulkano::device::Device>,
            pub queue: Arc<vulkano::device::Queue>,
        }

        impl RHIDevice {
            fn select_physical_device(
                instance: &Arc<Instance>,
                surface: &Arc<Surface>,
                device_extensions: &DeviceExtensions,
            ) -> (Arc<PhysicalDevice>, u32) {
                use vulkano::device::QueueFlags;

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


            pub fn initialize(window: &Arc<Window>, event_loop: &EventLoop<()>) -> Result<Self> {
                use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
                use vulkano::image::ImageUsage;
                use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};

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

                let (swapchain, images) = Swapchain::new(
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

                Ok(Self {device, queue})
            }
        }
    }
}


struct Renderer {}

impl Renderer {
    // An example of create buffer
    pub fn create_buffer(device: Arc<vulkano::device::Device>) {
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

    pub fn render_fractal(device: Arc<vulkano::device::Device>, queue: Arc<vulkano::device::Queue>) {
        use image::*;
        use vulkano::buffer::*;
        use vulkano::command_buffer::*;
        use vulkano::command_buffer::allocator::*;
        use vulkano::descriptor_set::allocator::*;
        use vulkano::descriptor_set::*;
        use vulkano::format::*;
        use vulkano::image::*;
        use vulkano::image::view::*;
        use vulkano::memory::allocator::*;
        use vulkano::pipeline::*;
        use vulkano::pipeline::layout::*;
        use vulkano::pipeline::compute::*;

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

    pub fn render_base(device: Arc<vulkano::device::Device>, queue: Arc<vulkano::device::Queue>) {
        use vulkano::buffer::*;
        use vulkano::command_buffer::*;
        use vulkano::command_buffer::allocator::*;
        use vulkano::descriptor_set::*;
        use vulkano::descriptor_set::allocator::*;
        use vulkano::format::*;
        use vulkano::image::*;
        use vulkano::image::view::*;
        use vulkano::memory::allocator::*;
        use vulkano::pipeline::*;
        use vulkano::pipeline::graphics::*;
        use vulkano::pipeline::graphics::color_blend::*;
        use vulkano::pipeline::graphics::input_assembly::*;
        use vulkano::pipeline::graphics::multisample::*;
        use vulkano::pipeline::graphics::rasterization::*;
        use vulkano::pipeline::graphics::vertex_input::*;
        use vulkano::pipeline::graphics::viewport::*;
        use vulkano::pipeline::layout::*;
        use vulkano::render_pass::*;

        // ---- Setup resources ----
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        #[derive(BufferContents, Vertex)]
        #[repr(C)]
        struct VertexPositionOnly {
            #[format(R32G32_SFLOAT)]
            position: [f32; 2],
        }

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

        let vs = vs::load(device.clone()).expect("failed to create shader module");
        let fs = fs::load(device.clone()).expect("failed to create shader module");

        // Renderpass
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: Format::R8G8B8A8_UNORM,
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
        .unwrap();

        // Framebuffer
        let framebuffer = Framebuffer::new(
            render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![view.clone()],
                ..Default::default()
            },
        )
        .unwrap();

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };

        let pipeline = {
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
                    // This graphics pipeline object concerns the first pass of the render pass.
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )
            .unwrap()
        };

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
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        );

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        // --- Draw --- //
        command_buffer_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
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
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, out_buf.clone()))
            .unwrap();

        let command_buffer = command_buffer_builder.build().unwrap();

        let future = vulkano::sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let buffer_content = out_buf.read().unwrap();
        let image = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
        image.save("image_triangle.png").unwrap();

        info!("Render base triangle");
    }

    pub fn init_resources(device: Arc<vulkano::device::Device>, queue: Arc<vulkano::device::Queue>) {
    }

}

#[derive(Clone, Debug)]
struct App {}

impl App {
    /// Creates our Vulkan app.
    fn create(window: &Arc<Window>, event_loop: &EventLoop<()>) -> Result<Self> {
        let device = rhi::vulkan::RHIDevice::initialize(window, event_loop);

        if let Ok(device) = device {

            Renderer::create_buffer(device.device.clone());
            Renderer::init_resources(device.device.clone(), device.queue.clone());
            Renderer::render_fractal(device.device.clone(), device.queue.clone());
            Renderer::render_base(device.device.clone(), device.queue.clone());

            info!("Everything succeeded!");

        }

        Ok(Self {})
    }

    /// Renders a frame for our Vulkan app.
    fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
