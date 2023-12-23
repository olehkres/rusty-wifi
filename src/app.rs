use std::fmt;

use eframe::egui::{self, Button};
use egui_plot::{Line, Plot, PlotBounds};

use rusty_wifi::WiFi;

#[derive(Default)]
pub enum Band {
    #[default]
    G2,
    G5,
}

impl Band {
    pub fn toggle(&mut self) {
        match self {
            Band::G2 => *self = Band::G5,
            Band::G5 => *self = Band::G2,
        }
    }
}

impl fmt::Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Band::G2 => {
                write!(f, "2 GHz")
            }
            Band::G5 => {
                write!(f, "5 GHz")
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    wifis: Vec<(WiFi, Vec<[f64; 2]>)>,
    band: Band,
    zoom: bool,
}

impl App {
    fn wifi_points(w: &WiFi) -> Vec<[f64; 2]> {
        let start = (w.channel() - w.bandwidth() / 20) * 30;
        let end = (w.channel() + w.bandwidth() / 20) * 30;

        (start..=end)
            .map(|x| {
                let x = x as f64 / 30.0;
                let a = *w.bandwidth() as f64 * -0.05;
                let b = *w.signal() as f64 / 10.0;
                let c = *w.channel() as f64;
                let y: f64 = a * b * ((x - c) * (x - c)) + b;
                [x, y]
            })
            .collect::<Vec<[f64; 2]>>()
    }

    fn rescan(&mut self) {
        self.wifis = WiFi::scan()
            .into_iter()
            .map(|w| {
                let wifi_points = App::wifi_points(&w);
                (w, wifi_points)
            })
            .collect();
    }

    pub fn new() -> Self {
        let mut app = App {
            ..Default::default()
        };
        app.rescan();
        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let band_button = ui.add_enabled(!self.zoom, Button::new(format!("{}", self.band)));
            if band_button.clicked() {
                self.band.toggle();
            }
            ui.toggle_value(&mut self.zoom, "Zoom");
            let rescan_button = ui.add(Button::new("Rescan"));
            if rescan_button.clicked() {
                self.rescan();
            }

            let plot = Plot::new("wifi_plot")
                .allow_zoom(self.zoom)
                .allow_scroll(false)
                .allow_drag(false);

            plot.show(ui, |plot_ui| {
                // Show only 2GHz bounds.
                let bounds = match self.band {
                    // Plot for 2 GHZ
                    Band::G2 => PlotBounds::from_min_max([0.0, 0.0], [17.0, 11.0]),
                    // Plot for 5 GHZ
                    Band::G5 => PlotBounds::from_min_max([32.0, 0.0], [177.0, 11.0]),
                };

                if !self.zoom {
                    plot_ui.set_plot_bounds(bounds);
                }

                for (wifi, points) in self.wifis.iter() {
                    plot_ui.line(Line::new(points.clone()).name(format!(
                        "SSID: {}\n\
                        BSSID: {}",
                        wifi.ssid().clone(),
                        wifi.bssid()
                    )));
                }
            });
        });
    }
}
