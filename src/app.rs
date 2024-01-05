use std::fs::File;
use std::io::{BufReader, Write};

use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use native_dialog::FileDialog;

use crate::error::AppError;
use crate::wifi::{AccessPoint, Band};

mod back;
mod front;

use self::back::AccessPointGUI;

pub struct App {
    aps: Arc<Mutex<Vec<AccessPointGUI>>>,
    band: Band,
    zoom: bool,
    scan_task: JoinHandle<()>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            aps: Arc::new(Mutex::new(vec![])),
            band: Band::G2,
            zoom: false,
            scan_task: tokio::spawn(async {}),
        }
    }
}

impl App {
    /// Creates a new [`App`] instance.
    pub fn new() -> Self {
        App {
            aps: Arc::new(Mutex::new(AccessPointGUI::scan())),
            ..Default::default()
        }
    }

    /// Calls scan function of WiFi API and updates Access Points on [`App`].
    /// Note: this replaces previosly scaned Access Points with the new one without saving.
    fn rescan(&mut self) {
        if !self.scan_task.is_finished() {
            return;
        }

        let wifis = Arc::clone(&self.aps);
        self.scan_task = tokio::spawn(async move {
            let scan = AccessPointGUI::scan();
            let mut aps_lock = wifis.lock().await;
            *aps_lock = scan;
        });
    }

    /// Opens native file dialog to select json formatted Access Points list.
    fn open_file_spawn(&mut self) {
        let aps_p = Arc::clone(&self.aps);

        tokio::spawn(async move {
            match App::open_file().await {
                Ok(Some(aps)) => {
                    let mut aps_lock = aps_p.lock().await;
                    *aps_lock = aps;
                }
                // If user closed window than no message needs to be displayed
                Ok(None) => (),
                Err(e) => e.show(),
            }
        });
    }

    async fn open_file() -> Result<Option<Vec<AccessPointGUI>>, AppError> {
        let file_join = tokio::spawn(async move {
            FileDialog::new()
                .add_filter("json", &["json"])
                .set_location("~")
                .show_open_single_file()
        });

        if let Some(path) = file_join.await?? {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let aps: Vec<AccessPoint> = serde_json::from_reader(reader)?;
            let aps: Vec<AccessPointGUI> = aps.into_iter().map(AccessPointGUI::new).collect();

            return Ok(Some(aps));
        };

        Ok(None)
    }

    /// Opens native file dialog to select path and name for file
    /// of json formatted Access Points list.
    fn save_file_spawn(&self) {
        let aps_p = Arc::clone(&self.aps);

        tokio::spawn(async move {
            match Self::save_file(aps_p).await {
                Ok(()) => (),
                Err(e) => e.show(),
            }
        });
    }

    async fn save_file<'a>(aps_p: Arc<Mutex<Vec<AccessPointGUI>>>) -> Result<(), AppError> {
        let file_join = tokio::spawn(async move {
            FileDialog::new()
                .add_filter("json", &["json"])
                .set_location("~")
                .show_save_single_file()
        });

        let serder_aps = tokio::spawn(async move {
            let aps_lock = aps_p.lock().await;
            let aps_raw: Vec<&AccessPoint> = aps_lock.iter().map(|w| w.raw()).collect();
            Result::<Vec<u8>, AppError>::Ok(serde_json::to_vec(&aps_raw)?)
        });

        let path = file_join.await??;

        if let Some(path) = path {
            let mut file = File::create(path)?;
            file.write_all(&serder_aps.await??)?;
        };

        Ok(())
    }
}
