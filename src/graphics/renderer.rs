use std::fmt::{self, Display};

use wgpu::{
    include_spirv,
    util::{BufferInitDescriptor, DeviceExt},
    Adapter,
    BackendBit,
    Buffer,
    BufferUsage,
    Color as Colour,
    CommandEncoderDescriptor,
    Device,
    DeviceDescriptor,
    Features,
    FragmentState,
    Instance,
    Limits,
    LoadOp,
    MultisampleState,
    Operations,
    PipelineLayout,
    PipelineLayoutDescriptor,
    PowerPreference,
    PresentMode,
    PrimitiveState,
    Queue,
    RenderPassColorAttachment as RenderPassColourAttachment,
    RenderPassDescriptor,
    RenderPipeline,
    RequestAdapterOptions,
    RequestDeviceError,
    Surface,
    SwapChain,
    SwapChainDescriptor,
    SwapChainError,
    SwapChainFormat,
    TextureUsage,
    VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::graphics::{Pipeline, Vertex};

#[derive(Debug, Clone)]
pub enum RendererError {
    NoPreferredSwapChainFormat,
    NoAvailableGraphicsAdapter,
    NoAvailableGraphicsDevice(RequestDeviceError),
}

impl Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RendererError::*;

        match self {
            NoPreferredSwapChainFormat => write!(f, "No preferred swap chain format"),
            NoAvailableGraphicsAdapter => write!(f, "Unable to request a graphics adapter"),
            NoAvailableGraphicsDevice(device_error) => {
                write!(f, "Unable to request graphics device, {}", device_error)
            },
        }
    }
}

pub struct Renderer {
    pub(in crate::graphics) size:                  PhysicalSize<u32>,
    pub(in crate::graphics) instance:              Instance,
    pub(in crate::graphics) surface:               Surface,
    pub(in crate::graphics) adapter:               Adapter,
    pub(in crate::graphics) device:                Device,
    pub(in crate::graphics) queue:                 Queue,
    pub(in crate::graphics) swap_chain:            SwapChain,
    pub(in crate::graphics) swap_chain_descriptor: SwapChainDescriptor,
    pub(in crate::graphics) swap_chain_format:     SwapChainFormat,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference:   PowerPreference::LowPower,
                compatible_surface: Some(&surface),
            })
            .await;

        if let Some(adapter) = adapter {
            let device_request = adapter
                .request_device(
                    &DeviceDescriptor {
                        label:    None,
                        features: Features::empty(),
                        limits:   Limits::default(),
                    },
                    None,
                )
                .await;

            match device_request {
                Ok((device, queue)) => {
                    if let Some(swap_chain_format) =
                        adapter.get_swap_chain_preferred_format(&surface)
                    {
                        let swap_chain_descriptor = SwapChainDescriptor {
                            usage:        TextureUsage::RENDER_ATTACHMENT,
                            format:       swap_chain_format,
                            width:        size.width,
                            height:       size.height,
                            present_mode: PresentMode::Mailbox,
                        };

                        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

                        Ok(Self {
                            size,
                            instance,
                            surface,
                            adapter,
                            device,
                            queue,
                            swap_chain,
                            swap_chain_descriptor,
                            swap_chain_format,
                        })
                    } else {
                        Err(RendererError::NoPreferredSwapChainFormat)
                    }
                },

                Err(device_error) => Err(RendererError::NoAvailableGraphicsDevice(device_error)),
            }
        } else {
            Err(RendererError::NoAvailableGraphicsAdapter)
        }
    }

    pub fn create_buffer(&self, vertices: &[Vertex]) -> Buffer {
        let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label:    None,
            contents: bytemuck::cast_slice(vertices),
            usage:    BufferUsage::VERTEX,
        });

        vertex_buffer
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.swap_chain_descriptor.width = new_size.width;
        self.swap_chain_descriptor.height = new_size.height;

        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
    }

    pub fn recreate_swap_chain(&mut self) {
        self.resize(self.size);
    }

    pub fn render(&mut self, pipeline: &Pipeline, buffer: &Buffer) -> Result<(), SwapChainError> {
        self.swap_chain.get_current_frame().map(|frame| {
            let frame = frame.output;

            let mut encoder = self
                .device
                .create_command_encoder(&CommandEncoderDescriptor { label: None });

            {
                let colour_attachments = &[RenderPassColourAttachment {
                    view:           &frame.view,
                    resolve_target: None,
                    ops:            Operations {
                        load:  LoadOp::Clear(Colour::GREEN),
                        store: true,
                    },
                }];

                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label:                    None,
                    color_attachments:        colour_attachments,
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(pipeline.render_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..3, 0..1);
            }

            self.queue.submit(std::iter::once(encoder.finish()));
        })
    }
}
