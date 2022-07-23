use eframe::{ egui::Vec2, run_native, NativeOptions };
use portfolio::PortfolioApp;

const APP_NAME: &str = "Portfolio";

fn main() {
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));

    let  portfolio = PortfolioApp::default();

    run_native(
        APP_NAME,
        win_option,
        Box::new(|cc| Box::new(portfolio.init(cc))),
    );
}
