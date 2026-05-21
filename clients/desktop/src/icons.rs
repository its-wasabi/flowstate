use egui::{Image, TextureOptions, include_image};

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
pub fn delete() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/delete.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn panel_open() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/panel-open.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn panel_close() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/panel-close.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn settings() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/settings.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}

#[inline]
pub fn add() -> Image<'static> {
    Image::new(include_image!("../../assets/icons/add.svg"))
        .maintain_aspect_ratio(true)
        .texture_options(TextureOptions::NEAREST)
}
