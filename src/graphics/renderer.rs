use std::fmt::{self, Display};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter,
    BackendBit,
    BufferUsage,
    Color as Colour,
    CommandEncoderDescriptor,
    Device,
    DeviceDescriptor,
    Features,
    Instance,
    Limits,
    LoadOp,
    Operations,
    PowerPreference,
    PresentMode,
    Queue,
    RenderPassColorAttachment as RenderPassColourAttachment,
    RenderPassDescriptor,
    RequestAdapterOptions,
    RequestDeviceError,
    Surface,
    SwapChain,
    SwapChainDescriptor,
    SwapChainError,
    TextureFormat,
    TextureUsage,
};
use winit::{dpi::PhysicalSize, event::WindowEvent, event_loop::ControlFlow, window::Window};

use crate::graphics::{Buffer, Pipeline, Vertex};

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
    pub(in crate::graphics) swap_chain_format:     TextureFormat,
}

impl Renderer {
    pub async fn new(
        window: &Window,
        power_preference: PowerPreference,
    ) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference,
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

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(new_size) => self.resize(*new_size),
            _ => {},
        }
    }

    pub fn create_buffer(&self, vertices: &[Vertex]) -> Buffer {
        let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label:    None,
            contents: bytemuck::cast_slice(vertices),
            usage:    BufferUsage::VERTEX,
        });

        Buffer {
            buffer: vertex_buffer,
            size:   vertices.len() as u32,
        }
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

    #[must_use]
    pub fn begin_frame(&mut self) -> Result<RenderFrame, ControlFlow> {
        match self.swap_chain.get_current_frame() {
            Ok(frame) => {
                let encoder = self
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor { label: None });

                Ok(RenderFrame { frame, encoder })
            },

            Err(swap_chain_error) => {
                let mut critical_failure = false;

                match swap_chain_error {
                    SwapChainError::Lost => {
                        log::warn!("{}", swap_chain_error);
                        self.recreate_swap_chain();
                    },

                    SwapChainError::OutOfMemory => {
                        log::error!("{}", swap_chain_error);
                        critical_failure = true;
                    },

                    _ => log::error!("{}", swap_chain_error),
                }

                if critical_failure {
                    Err(ControlFlow::Exit)
                } else {
                    Err(ControlFlow::Poll)
                }
            },
        }
    }

    // NOTE:
    // It would have been good to do this though a drop impl on the RenderFrame, but that would
    // require storing a `&'a mut` reference to the renderer which makes winit `event_loop.run` sad
    // because of the `move` keyword.
    pub fn finish_frame(&mut self, frame: RenderFrame) {
        self.queue.submit(std::iter::once(frame.encoder.finish()));
    }
}
