use egui::{Image, TextureOptions, include_image};

#[inline]
pub fn up() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/up.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn down() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/down.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn left() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/left.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn right() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/right.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn plus() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/plus.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn minus() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/minus.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn trash() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/trash.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}
