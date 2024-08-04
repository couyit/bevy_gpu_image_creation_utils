use bevy::{
    math::UVec2,
    render::{
        render_resource::{SamplerDescriptor, Texture, TextureViewDescriptor},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

pub fn create_gpu_image_from_texture(device: &RenderDevice, texture: Texture) -> GpuImage {
    let texture_view = texture.create_view(&TextureViewDescriptor::default());
    let texture_format = texture.format();
    let sampler = device.create_sampler(&SamplerDescriptor::default());
    let size = UVec2::new(texture.size().width, texture.size().height);
    let mip_level_count = texture.mip_level_count();

    GpuImage {
        texture,
        texture_view,
        texture_format,
        sampler,
        size,
        mip_level_count,
    }
}
