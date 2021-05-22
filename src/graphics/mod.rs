use anyhow::*;
use raw_window_handle::HasRawWindowHandle;

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
        self.device.create_swap_chain(
            &self.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            },
        )
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
}
