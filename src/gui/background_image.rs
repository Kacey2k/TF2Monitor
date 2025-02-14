use eframe::egui::{include_image, Color32, ImageSource};

pub struct ImageDescription {
    pub author: String,
    pub url: String,
}

const BACKGROUND_IMAGE_BYTES: ImageSource = include_image!("../../images/scout_slingshot.jpg");

pub fn add_background_image(ui: &mut eframe::egui::Ui) -> ImageDescription {
    let image = eframe::egui::Image::new(BACKGROUND_IMAGE_BYTES)
        // .maintain_aspect_ratio(true)
        .bg_fill(Color32::from_rgb(32, 32, 128))
        .tint(eframe::egui::Color32::from_rgb(80, 80, 80));

    // let rect_vec2 = ui.max_rect().size();
    let rect_vec2 = ui.ctx().screen_rect().size();
    // println!("rect_vec2: {:?}", rect_vec2);
    let rect = eframe::egui::Rect::from_min_size(Default::default(), rect_vec2);
    image.paint_at(ui, rect);

    ImageDescription {
        author: "die salo".to_string(),
        url: "https://www.instagram.com/p/CyrMyRos2xQ/?img_index=1".to_string(),
    }
}
