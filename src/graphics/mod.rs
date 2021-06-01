use anyhow::*;
use raw_window_handle::HasRawWindowHandle;
use wgpu::{CommandBuffer, SwapChainError, SwapChainTexture};
use winit::dpi::PhysicalSize;

pub struct Pipeline {}

pub struct Swapchain {
    pub size: PhysicalSize<u32>,
    swap_chain: wgpu::SwapChain,
}

impl Swapchain {
    #[inline]
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn get_current_frame(&self) -> Result<SwapChainTexture, SwapChainError> {
        Ok(self.swap_chain.get_current_frame()?.output)
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8UnormSrgb
    }
    fn descriptor(width: u32, height: u32) -> wgpu::SwapChainDescriptor {
        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
}
pub struct Device {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Device {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, surface: wgpu::Surface) -> Self {
        Self {
            device,
            queue,
            surface,
        }
    }

    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn wgpu_device_mut(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Command Encoder"),
            })
    }
    pub fn create_swap_chain(&self, width: u32, height: u32) -> wgpu::SwapChain {
        let desc = Swapchain::descriptor(width, height);
        self.device.create_swap_chain(&self.surface, &desc)
    }
    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.queue.submit(command_buffers);
    }
}
pub struct GraphicSystem {
    pub device: Device,
}

impl GraphicSystem {
    pub async fn new<W>(window: &W) -> Result<Self>
    where
        W: HasRawWindowHandle,
    {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        Ok(Self {
            device: Device::new(device, queue, surface),
        })
    }

    pub fn swap_chain(&self, size: PhysicalSize<u32>) -> Swapchain {
        Swapchain {
            size,
            swap_chain: self.device.create_swap_chain(size.width, size.height),
        }
    }
    pub fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_encoder()
    }

    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.device.submit(command_buffers);
    }
}
