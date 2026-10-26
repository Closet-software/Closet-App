use eframe::egui;
use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    time::Duration,
};
use tokio::runtime::Runtime;

/// Runs HTTP requests on a Tokio runtime and sends their results back to the UI thread.
pub struct MarkdownFetcher {
    runtime: Runtime,
    tx: Sender<Result<String, String>>,
    rx: Receiver<Result<String, String>>,
}

impl Default for MarkdownFetcher {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            tx,
            rx,
        }
    }
}

impl MarkdownFetcher {
    /// Starts fetching a URL without blocking egui's UI thread.
    pub fn fetch(&self, url: String, ctx: egui::Context) {
        let tx = self.tx.clone();

        self.runtime.spawn(async move {
            let result: Result<String, String> = async {
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(15))
                    .build()
                    .map_err(|err| err.to_string())?;

                let response = client
                    .get(url)
                    .send()
                    .await
                    .map_err(|err| err.to_string())?
                    .error_for_status()
                    .map_err(|err| err.to_string())?;

                response.text().await.map_err(|err| err.to_string())
            }
            .await;

            // The UI thread receives the result on its next frame.
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }

    /// Returns a completed request result, if one is ready.
    pub fn try_recv(&self) -> Result<Result<String, String>, TryRecvError> {
        self.rx.try_recv()
    }
}
