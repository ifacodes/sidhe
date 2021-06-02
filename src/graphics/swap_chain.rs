pub struct Swapchain {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub swap_chain: wgpu::SwapChain,
}

impl Swapchain {
    #[inline]
    #[allow(dead_code)]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainTexture, wgpu::SwapChainError> {
        Ok(self.swap_chain.get_current_frame()?.output)
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8UnormSrgb
    }
    pub fn descriptor(width: u32, height: u32) -> wgpu::SwapChainDescriptor {
        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
}
