mod portfolio;

use eframe::{
    egui::{ self, CentralPanel, ScrollArea, Spinner, Visuals, Window, Layout, Separator, },
    App,
};
use portfolio::{render_footer, render_header, render_about};
pub use portfolio::{ PortfolioApp, ProjectCard, PADDING };

impl App for PortfolioApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        ctx.set_debug_on_hover(false);

        if self.dark_mode { ctx.set_visuals(Visuals::dark()); }
        else { ctx.set_visuals(Visuals::light()); }

        self.render_top_panel(ctx, frame);
        render_footer(ctx);

        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
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

                const ABOUT_PADDING: f32 = 100.0;
                let available_width: f32 = ui.available_width();
                ui.set_max_width(available_width - ABOUT_PADDING);

                ui.horizontal(|ui| {
                    // left padding
                    ui.add_space(ABOUT_PADDING);

                    ui.vertical(|ui| {
                        render_about(ui);
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
