const TEXTURE_OPTIONS: egui::TextureOptions = egui::TextureOptions {
    magnification: egui::TextureFilter::Nearest,
    minification: egui::TextureFilter::Linear,
    wrap_mode: egui::TextureWrapMode::ClampToEdge,
    mipmap_mode: Some(egui::TextureFilter::Nearest),
};

const BIG: f32 = 24.0;
const MID: f32 = 18.0;
const SMALL: f32 = 16.0;

pub enum IconSize {
    Big,
    Mid,
    Small,
}

impl IconSize {
    const fn size_v2(self) -> egui::Vec2 {
        match self {
            Self::Big => egui::Vec2::new(BIG, BIG),
            Self::Mid => egui::Vec2::new(MID, MID),
            Self::Small => egui::Vec2::new(SMALL, SMALL),
        }
    }
}

#[inline]
pub fn add(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/add.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn delete(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/delete.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down_open(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down-open.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down_close(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down-close.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn down(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/down.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn left(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/left.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn right(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/right.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn minus(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/minus.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}

#[inline]
pub fn plus(size: IconSize) -> egui::Image<'static> {
    egui::Image::new(egui::include_image!("../../assets/icons/plus.svg"))
        .fit_to_exact_size(size.size_v2())
        .texture_options(TEXTURE_OPTIONS)
}
