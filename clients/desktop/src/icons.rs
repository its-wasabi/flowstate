const TEXTURE_OPTIONS: egui::TextureOptions = egui::TextureOptions {
    magnification: egui::TextureFilter::Nearest,
    minification: egui::TextureFilter::Linear,
    wrap_mode: egui::TextureWrapMode::ClampToEdge,
    mipmap_mode: Some(egui::TextureFilter::Nearest),
};

#[inline]
pub fn add() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/add.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn delete() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/delete.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down_open() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down-open.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down_close() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down-close.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn left() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/left.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn right() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/right.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn up() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/up.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn minus() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/minus.svg"))
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn plus() -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/plus.svg"))
        .texture_options(TEXTURE_OPTIONS)
}
