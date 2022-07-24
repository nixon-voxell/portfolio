mod portfolio;
mod card;

use eframe::{
    egui::{ self, CentralPanel, ScrollArea, Visuals, },
    App,
};
pub use portfolio::{ PortfolioApp, ProjectCard, H_PADDING };

impl App for PortfolioApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        ctx.set_debug_on_hover(false);

        if self.dark_mode { ctx.set_visuals(Visuals::dark()); }
        else { ctx.set_visuals(Visuals::light()); }

        self.render_top_panel(ctx, frame);
        self.render_footer(ctx);

        CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ScrollArea::vertical().show(ui, |ui| {

                // Window::new("About").show(ctx,|ui| {
                //     ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);
                //     ui.horizontal(|ui| {
                //         ui.label("About");
                //         ui.label("About");
                //     })
                // });

                let spacing = ui.spacing_mut();
                spacing.item_spacing = egui::vec2(40.0, 20.0);

                let available_width: f32 = ui.available_width();
                ui.set_max_width(available_width - H_PADDING);

                ui.horizontal(|ui| {
                    // left padding
                    ui.add_space(H_PADDING);

                    ui.vertical(|ui| {
                        self.render_about(ui);
                        self.render_projects(ui);
                    });
                });
            });
        });
    }
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_web(canvas_id: &str) {
    let portfolio = PortfolioApp::default();
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();
    eframe::start_web(canvas_id, web_options, Box::new(|cc| Box::new(portfolio.init(cc))))
        .expect("Failed to launch portfolio");
}
