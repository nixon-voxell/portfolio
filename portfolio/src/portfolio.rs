use eframe:: {
    egui:: {
        self,
        Button, Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink,
        Label, Layout, RichText, Separator, TextStyle, TopBottomPanel, Ui,
    },
    CreationContext, epaint::FontId,
};

pub const PADDING: f32 = 10.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);


pub struct ProjectCard {
    pub image_path: String,
    pub title: String,
    pub description: String,
}

impl Default for ProjectCard {
    fn default() -> Self {
        return Self {
            image_path: String::from(""),
            title: String::from(""),
            description: String::from(""),
        };
    }
}

impl ProjectCard {
    pub fn init(&self) {
        
    }

    pub fn render(&self) {
        
    }
}

pub struct PortfolioApp {
    pub project_cards: Vec<ProjectCard>,
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

impl PortfolioApp {
    pub fn init(self, cc: &CreationContext) -> Self {
        self.configure_fonts(&cc.egui_ctx);
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
            (TextStyle::Heading, FontId::new(40.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(20.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(18.0, FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
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
}

pub(crate) fn render_footer(ctx: &Context) {
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

pub(crate) fn render_header(ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.vertical_centered(|ui| {
        ui.heading("PORTFOLIO");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(10.0);
    ui.add(sep);
    ui.add_space(PADDING);
}

pub(crate) fn render_about(ui: &mut Ui) {
    ui.heading("About Me");
    ui.add_space(PADDING);

    ui.label("Hi, my name is Nixon. I specialized in Graphics Programming.");

    ui.separator();
}