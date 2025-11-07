use grass::OutputStyle;
use std::path::Path;
use tokio::fs;

#[derive(Clone)]
pub struct AssetCompiler {
    scss_input_dir: String,
    css_output_dir: String,
}

impl AssetCompiler {
    pub fn new(scss_input_dir: &str, css_output_dir: &str) -> Self {
        Self {
            scss_input_dir: scss_input_dir.to_string(),
            css_output_dir: css_output_dir.to_string(),
        }
    }

    pub async fn compile_scss(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure output directory exists
        fs::create_dir_all(&self.css_output_dir).await?;

        // Compile main layout.scss file
        let scss_path = format!("{}/layout.scss", self.scss_input_dir);
        let css_output_path = format!("{}/layout.css", self.css_output_dir);

        if Path::new(&scss_path).exists() {
            tracing::info!("Compiling SCSS: {} -> {}", scss_path, css_output_path);
            
            let css = grass::from_path(
                &scss_path,
                &grass::Options::default()
                    .style(OutputStyle::Compressed)
                    .load_path(&self.scss_input_dir)
            )?;
            
            fs::write(&css_output_path, css).await?;
            tracing::info!("SCSS compilation completed");
        } else {
            tracing::warn!("SCSS file not found: {}", scss_path);
        }

        Ok(())
    }

    pub async fn compile_scss_dev(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Development version with source maps and expanded output
        fs::create_dir_all(&self.css_output_dir).await?;

        let scss_path = format!("{}/layout.scss", self.scss_input_dir);
        let css_output_path = format!("{}/layout.css", self.css_output_dir);

        if Path::new(&scss_path).exists() {
            tracing::info!("Compiling SCSS (dev): {} -> {}", scss_path, css_output_path);
            
            let css = grass::from_path(
                &scss_path,
                &grass::Options::default()
                    .style(OutputStyle::Expanded)
                    .load_path(&self.scss_input_dir)
            )?;
            
            fs::write(&css_output_path, css).await?;
            tracing::info!("SCSS compilation (dev) completed");
        }

        Ok(())
    }

    #[cfg(debug_assertions)]
    pub async fn watch_and_compile(&self) -> Result<(), Box<dyn std::error::Error>> {
        use notify::{Watcher, RecursiveMode, Event, EventKind};
        use std::sync::mpsc;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        
        watcher.watch(Path::new(&self.scss_input_dir), RecursiveMode::Recursive)?;
        
        tracing::info!("Watching SCSS files for changes in {}", self.scss_input_dir);

        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(Event { kind: EventKind::Modify(_), .. })) => {
                    tracing::info!("SCSS file changed, recompiling...");
                    if let Err(e) = self.compile_scss_dev().await {
                        tracing::error!("SCSS compilation failed: {}", e);
                    }
                }
                Ok(Ok(_)) => {} // Other file events, ignore
                Ok(Err(e)) => tracing::error!("Watch error: {}", e),
                Err(_) => {} // Timeout, continue watching
            }
        }
    }
}