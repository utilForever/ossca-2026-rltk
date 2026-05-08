use bevy::{
    asset::UntypedHandle,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

#[derive(Resource)]
pub(crate) struct ImagesToLoad(pub(crate) Vec<UntypedHandle>);

pub(crate) fn fix_images(mut fonts: ResMut<ImagesToLoad>, mut images: ResMut<Assets<Image>>) {
    if fonts.0.is_empty() {
        return;
    }

    for (id, img) in images.iter_mut() {
        if let Some(i) = fonts.0.iter().position(|h| h.id() == id.untyped()) {
            let mut sampler = ImageSamplerDescriptor::nearest();
            sampler.set_address_mode(ImageAddressMode::ClampToEdge);
            img.sampler = ImageSampler::Descriptor(sampler);
            fonts.0.swap_remove(i);
        }
    }
}
