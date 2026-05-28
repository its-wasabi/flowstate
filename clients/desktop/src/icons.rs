const TEXTURE_OPTIONS: egui::TextureOptions = egui::TextureOptions {
    magnification: egui::TextureFilter::Nearest,
    minification: egui::TextureFilter::Linear,
    wrap_mode: egui::TextureWrapMode::ClampToEdge,
    mipmap_mode: Some(egui::TextureFilter::Nearest),
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Add,
    Delete,
    DownOpen,
    DownClose,
    Down,
    Left,
    Right,
    Minus,
    Plus,
}

impl Icon {
    const fn image_bytes(self) -> egui::ImageSource<'static> {
        match self {
            Self::Add => egui::include_image!("../../assets/icons/add.svg"),
            Self::Delete => egui::include_image!("../../assets/icons/delete.svg"),
            Self::DownOpen => egui::include_image!("../../assets/icons/down-open.svg"),
            Self::DownClose => egui::include_image!("../../assets/icons/down-close.svg"),
            Self::Down => egui::include_image!("../../assets/icons/down.svg"),
            Self::Left => egui::include_image!("../../assets/icons/left.svg"),
            Self::Right => egui::include_image!("../../assets/icons/right.svg"),
            Self::Minus => egui::include_image!("../../assets/icons/minus.svg"),
            Self::Plus => egui::include_image!("../../assets/icons/plus.svg"),
        }
    }

    pub fn image(self, size: impl Into<egui::Vec2>) -> egui::Image<'static> {
        egui::Image::new(self.image_bytes())
            .fit_to_exact_size(size.into())
            .texture_options(TEXTURE_OPTIONS)
    }
}
