use crate::card;
use eframe::{
    egui::{
        self,
        Context, Ui,
        FontFamily, FontData, FontDefinitions, 
        Label, RichText, Button, TextStyle, Hyperlink,
        Color32, Layout, Separator, TopBottomPanel,
    },
    CreationContext,
    epaint,
};

pub struct PortfolioApp {
    pub project_cards: Vec<card::ProjectCard>,
    pub dark_mode: bool,
}

impl Default for PortfolioApp {
    fn default() -> Self {
        return Self {
            project_cards: Default::default(),
            dark_mode: true,
        };
    }
}

pub const V_PADDING: f32 = 20.0;
pub const H_PADDING: f32 = 50.0;

pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const DARK: Color32 = Color32::from_rgb(15, 15, 15);
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);


impl PortfolioApp {
    pub fn init(mut self, cc: &CreationContext) -> Self {
        self.configure_fonts(&cc.egui_ctx);

        self.project_cards = Vec::new();

        for i in 0..100 {
            self.project_cards.push(card::ProjectCard::default());
            self.project_cards[i].title = String::from(format!("Title #{}", i));
            self.project_cards[i].description = String::from(format!("This is a description for 'Title #{}'", i));
        }
        self.project_cards[0].description = String::from("This is a super long description described to test out the features of long text handling. More strings, more strings, and more strings!!! I just need a little more xd.");
        return self;
    }

    pub fn configure_fonts(&self, ctx: &Context) {
        // let mut font_def = FontDefinitions::default();
        // font_def.font_data.insert(
        //     "MesloLGS".to_string(),
        //     FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        // );

        // font_def
        //     .families
        //     .get_mut(&FontFamily::Proportional)
        //     .unwrap()
        //     .insert(0, "MesloLGS".to_string());

        // ctx.set_fonts(font_def);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, epaint::FontId::new(40.0, FontFamily::Proportional)),
            (TextStyle::Body, epaint::FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Monospace, epaint::FontId::new(20.0, FontFamily::Monospace)),
            (TextStyle::Button, epaint::FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Small, epaint::FontId::new(18.0, FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);
    }

    pub fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.add(Label::new(
                        RichText::new("📰").text_style(egui::TextStyle::Heading),
                    ));
                });

                // controls
                ui.with_layout(Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if !cfg!(target_arch = "wasm32") {
                        let close_btn = ui.add(Button::new(
                            RichText::new("❌").text_style(egui::TextStyle::Body),
                        ));
                        if close_btn.clicked() {
                            frame.quit();
                        }
                    }

                    // theme button
                    let theme_btn = ui.add(Button::new(
                        RichText::new({
                            if self.dark_mode { "🌞" }
                            else { "🌙" }
                        })
                        .text_style(egui::TextStyle::Body),
                    ));

                    if theme_btn.clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    pub fn render_footer(&self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.add(Label::new(
                    RichText::new("Made in Rust, exported to WASM.").small()
                ));
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Made with egui").small(),
                    "https://github.com/emilk/egui",
                ));
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("nixon-voxell/portfolio").small(),
                    "https://github.com/nixon-voxell/portfolio",
                ));
                ui.add_space(10.0);
            })
        });
    }

    pub fn render_header(&self, ui: &mut Ui) {
        ui.add_space(V_PADDING);
        ui.vertical_centered(|ui| {
            ui.heading("PORTFOLIO");
        });
        ui.add_space(V_PADDING);
        let sep = Separator::default().spacing(10.0);
        ui.add(sep);
        ui.add_space(V_PADDING);
    }

    pub fn render_about(&self, ui: &mut Ui) {
        ui.heading("About Me");
        ui.add_space(V_PADDING);

        ui.label("Hi, my name is Nixon and I specialized in Computer Graphics!");

        ui.separator();
        ui.add_space(V_PADDING);
    }

    pub fn render_projects(&self, ui: &mut Ui) {
        ui.heading("Projects");

        let project_cards_count: usize = self.project_cards.len();
        let mut card_idx: usize = 0;

        while card_idx < project_cards_count {
            // fist width must be at least the same length, or else, nothing will be rendered for this row
            let mut ui_available_width = f32::max(ui.available_width(), card::TOTAL_CARD_SIZE);

            ui.horizontal(|ui| {
                while card_idx < project_cards_count && ui_available_width >= card::TOTAL_CARD_SIZE {
                    self.project_cards[card_idx].render(ui);
                    card_idx += 1;
                    ui_available_width = ui.available_width();
                }
            });
        }
    }
}
