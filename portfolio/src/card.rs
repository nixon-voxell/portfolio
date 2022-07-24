use crate::portfolio::*;
use eframe::{
    egui::{
        self,
    },
    epaint,
};


const INNER_MARGIN: f32 = 20.0;
const OUTER_MARGIN: f32 = 10.0;

const RADIUS: f32 = 1.0;

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