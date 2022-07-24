use crate::portfolio::*;
use eframe::{
    egui::{
        self, Ui,
    },
    epaint,
};


pub const WIDTH: f32 = 500.0;
pub const MIN_HEIGHT: f32 = 100.0;

pub const INNER_MARGIN: f32 = 20.0;
pub const OUTER_MARGIN: f32 = 10.0;
pub const TOTAL_CARD_SIZE: f32 = WIDTH + 2.0 * INNER_MARGIN + 2.0 * OUTER_MARGIN;

pub const RADIUS: f32 = 1.0;

pub struct ProjectCard {
    pub image_path: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub url_text: String,
}

impl Default for ProjectCard {
    fn default() -> Self {
        return Self {
            image_path: String::from(""),
            title: String::from(""),
            description: String::from(""),
            url: String::from(""),
            url_text: String::from(""),
        };
    }
}

impl Clone for ProjectCard {
    fn clone(&self) -> Self {
        return Self {
            image_path: self.image_path.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            url_text: self.url_text.clone(),
        }
    }
}

impl ProjectCard {
    pub fn init(&self) {
        
    }

    pub fn create_card_frame() -> egui::Frame {
        let inner_margin: egui::style::Margin = egui::style::Margin::same(INNER_MARGIN);
        let outer_margin: egui::style::Margin = egui::style::Margin::same(OUTER_MARGIN);

        let rounding: epaint::Rounding = epaint::Rounding {
            ne: RADIUS, nw: RADIUS, se: RADIUS, sw: RADIUS
        };

        let shadow: epaint::Shadow = epaint::Shadow::small_dark();
        // let stroke: epaint::Stroke = epaint::Stroke::new(0.5, Color32::LIGHT_GRAY);

        return egui::Frame {
            inner_margin: inner_margin,
            outer_margin: outer_margin,
            rounding: rounding,
            fill: DARK,
            shadow: shadow,
            // stroke: stroke,
            ..Default::default()
        };
    }

    pub fn render(&self, ui: &mut Ui) {
        ProjectCard::create_card_frame().show(ui, |ui| {
            ui.set_width(WIDTH);
            ui.set_min_height(MIN_HEIGHT);

            // ui.image(texture_id, size)

            ui.vertical(|ui| {
                ui.heading(&self.title);
                ui.label(&self.description);
            });
        });
    }
}
