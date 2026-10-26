use eframe::egui;
use egui_commonmark;
use tokio;

use std::sync::mpsc::{self, Receiver, Sender};

const CHANGELOG_URL: &str = "";

struct markdown {
    runtime: Runtime,
    tx: Sender<Result<String, String>>,
    rx: Receiver<Result<String, String>>,

    markdown: Option<String>,
    loading: bool,
    error: Options<String>,

    cache: egui_commonmark::CommonMarkCache,
}

impl Default for markdown {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            runtime: Runtime::new().expect("Failed to create tokio runtime"),
            tx,
            rx,
            markdown: None,
            loading: False,
            error: None,
            cachy: egui_commonmark::CommonMarkCache::default(),
        }
    }

    fn fetch(&mut self, ctx: egui::contect) {
        self.loading = true;
        self.error = None;

        let tx = self.tx.clone();

        self.runtime.spawn(async move {
            let result: Result<String, String> = async {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|e| e.to_string())?;

                let text = client
                    .get(CHANGELOG_URL)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .error_for_status()
                    .map_err(|e| e.to_string())?
                    .text()
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(text)
            };
            let _ = tx.send(result);

            ctx.request_repaint();
        })
    }
}
