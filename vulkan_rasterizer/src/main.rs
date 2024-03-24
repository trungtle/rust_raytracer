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

                let (mut swapchain, images) = Swapchain::new(
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

struct Renderer {}

impl Renderer {
    pub fn create_buffer(device: Arc<vulkano::device::Device>) {
        use vulkano::memory::allocator::StandardMemoryAllocator;
        use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
        use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};

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

    pub fn init_resources(device: Arc<vulkano::device::Device>, queue: Arc<vulkano::device::Queue>) {
        use vulkano::memory::allocator::StandardMemoryAllocator;
        use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
        use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};

        use vulkano::pipeline::compute::ComputePipelineCreateInfo;
        use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
        use vulkano::pipeline::{ComputePipeline, PipelineLayout, PipelineShaderStageCreateInfo};
        use vulkano::pipeline::Pipeline;
        use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
        use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;

        use vulkano::command_buffer::allocator::{
            StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
        };
        use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
        use vulkano::pipeline::PipelineBindPoint;

        use vulkano::image::{ImageCreateInfo, ImageType, ImageUsage, Image};
        use vulkano::format::Format;
        use vulkano::command_buffer::ClearColorImageInfo;
        use vulkano::format::ClearColorValue;

        use vulkano::command_buffer::CopyImageToBufferInfo;
        use vulkano::sync::{self, GpuFuture};


        // Create shader
        let shader = cs::load(device.clone()).expect("failed to create shader module");

        let cs = shader.entry_point("main").unwrap();
        let stage = PipelineShaderStageCreateInfo::new(cs);
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap();

        let compute_pipeline = ComputePipeline::new(
            device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage, layout),
        )
        .expect("failed to create compute pipeline");


        // Create buffer
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let data_iter = 0..65536u32;
        let data_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data_iter,
        )
        .expect("failed to create buffer");

        // Create descriptors set
        let descriptor_set_allocator =
        StandardDescriptorSetAllocator::new(device.clone(), Default::default());
        let pipeline_layout = compute_pipeline.layout();
        let descriptor_set_layouts = pipeline_layout.set_layouts();

        let descriptor_set_layout_index = 0;
        let descriptor_set_layout = descriptor_set_layouts
            .get(descriptor_set_layout_index)
            .unwrap();
        let descriptor_set = PersistentDescriptorSet::new(
            &descriptor_set_allocator,
            descriptor_set_layout.clone(),
            [WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
            [],
        )
        .unwrap();

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

        let work_group_counts = [1024, 1, 1];

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
            .unwrap();

        let command_buffer = command_buffer_builder.build().unwrap();

        // let future = vulkano::sync::now(device.clone())
        //     .then_execute(queue.clone(), command_buffer)
        //     .unwrap()
        //     .then_signal_fence_and_flush()
        //     .unwrap();

        // future.wait(None).unwrap();

        // let content = data_buffer.read().unwrap();
        // for (n, val) in content.iter().enumerate() {
        //     assert_eq!(*val, n as u32 * 12);
        // }

        // let image = Image::new(
        //     memory_allocator.clone(),
        //     ImageCreateInfo {
        //         image_type: ImageType::Dim2d,
        //         format: Format::R8G8B8A8_UNORM,
        //         extent: [1024, 1024, 1],
        //         usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
        //         ..Default::default()
        //     },
        //     AllocationCreateInfo {
        //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
        //         ..Default::default()
        //     },
        // )
        // .unwrap();

        // let mut builder = AutoCommandBufferBuilder::primary(
        //     &command_buffer_allocator,
        //     queue.queue_family_index(),
        //     CommandBufferUsage::OneTimeSubmit,
        // )
        // .unwrap();

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

        // // Clear the image with blue
        // builder
        // .clear_color_image(ClearColorImageInfo {
        //     clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
        //     ..ClearColorImageInfo::image(image.clone())
        // })
        // .unwrap()
        // .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
        //     image.clone(),
        //     buf.clone(),
        // ))
        // .unwrap();

        // let command_buffer = builder.build().unwrap();

        // let future = sync::now(device.clone())
        //     .then_execute(queue.clone(), command_buffer)
        //     .unwrap()
        //     .then_signal_fence_and_flush()
        //     .unwrap();

        // future.wait(None).unwrap();

        use image::{ImageBuffer, Rgba};

        let buffer_content = buf.read().unwrap();
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
        image.save("image.png").unwrap();

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

        use vulkano::image::view::ImageView;

        let view = ImageView::new_default(image.clone()).unwrap();

        let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            &descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::image_view(0, view.clone())], // 0 is the binding
            [],
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

        let mut builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .bind_pipeline_compute(compute_pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                0,
                set,
            )
            .unwrap()
            .dispatch([1024 / 8, 1024 / 8, 1])
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                image.clone(),
                buf.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

        future.wait(None).unwrap();

        let buffer_content = buf.read().unwrap();
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
        image.save("image.png").unwrap();

        println!("Everything succeeded!");
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
