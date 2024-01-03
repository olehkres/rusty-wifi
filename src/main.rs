mod app;
mod error;
mod wifi;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Wifi",
        options,
        Box::new(|_cc| Box::new(app::App::new())),
    )
}
