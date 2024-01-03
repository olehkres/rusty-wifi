use crate::app::App;

use eframe::egui::{self, Button, Ui};
use egui_plot::{Line, Plot, PlotBounds};

use crate::wifi::Band;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_top_panel(ui);
            self.show_plot(ui);
        });
    }
}

impl App {
    fn show_top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Note: we grayout band button if zoom enabled.
            let band_button = ui.add_enabled(!self.zoom, Button::new(format!("{}", self.band)));
            if band_button.clicked() {
                self.band.toggle();
            }
            ui.toggle_value(&mut self.zoom, "Zoom");
            let rescan_button = ui.add(Button::new("Rescan"));
            if rescan_button.clicked() {
                self.rescan();
            }
            if ui.button("Save").clicked() {
                self.save_file();
            }
            if ui.button("Open").clicked() {
                self.open_file();
            }
        });
    }

    fn show_plot(&mut self, ui: &mut Ui) {
        let plot = Plot::new("wifi_plot")
            .label_formatter(|name, _| name.to_owned())
            .allow_zoom(self.zoom)
            .allow_scroll(false)
            .allow_drag(false);

        plot.show(ui, |plot_ui| {
            let bounds = match self.band {
                Band::G2 => PlotBounds::from_min_max([0.0, 0.0], [17.0, 11.0]),
                Band::G5 => PlotBounds::from_min_max([32.0, 0.0], [177.0, 11.0]),
            };

            // Zoom and bounds are in conflict!
            if !self.zoom {
                plot_ui.set_plot_bounds(bounds);
            }

            if let Ok(wifis) = &self.aps.try_lock() {
                for w in wifis.iter() {
                    plot_ui.line(Line::new(w.points().clone()).name(format!(
                        "SSID: {}\n\
                        BSID: {}\n\
                        CHAN: {}\n\
                        SIGNAL: {}",
                        w.raw().ssid(),
                        w.raw().bssid(),
                        w.raw().channel(),
                        w.raw().signal()
                    )));
                }
            };
        });
    }
}
