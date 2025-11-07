use std::time::SystemTime;
use std::process::Command;
use reqwest;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

pub const TWELVE_FACTOR_PORT: u16 = 12012;

#[derive(Debug)]
pub struct ServerStatus {
    pub pid: Option<u32>,
    pub start_time: Option<SystemTime>,
    pub is_our_server: bool,
    pub css_outdated: bool,
    pub verification_results: VerificationResults,
}

#[derive(Debug, Default)]
pub struct VerificationResults {
    pub css_compiled: bool,
    pub css_has_yellow_borders: bool,
    pub css_has_numeral_icons: bool,
    pub css_has_text_indent: bool,
    pub server_serving_latest: bool,
    pub html_has_factor_cards: bool,
    pub html_has_external_links: bool,
    pub overall_confidence: ConfidenceLevel,
}

#[derive(Debug, PartialEq)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

impl Default for ConfidenceLevel {
    fn default() -> Self {
        ConfidenceLevel::Low
    }
}

pub struct ServerManager;

impl ServerManager {
    /// Check if port 12012 is in use and by what
    pub async fn check_port() -> Result<ServerStatus, Box<dyn std::error::Error>> {
        let mut status = ServerStatus {
            pid: None,
            start_time: None,
            is_our_server: false,
            css_outdated: false,
            verification_results: VerificationResults::default(),
        };

        // Simple and direct: try to bind to the port
        match std::net::TcpListener::bind(format!("127.0.0.1:{}", TWELVE_FACTOR_PORT)) {
            Ok(listener) => {
                // Port is available, close the listener immediately
                drop(listener);
                // status.pid remains None, indicating port is free
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                // Port is in use
                // We can't easily get the PID without platform-specific code
                // Use a sentinel value to indicate "unknown PID but port is taken"
                status.pid = Some(999999);
                
                // Check if it's our twelve-factor server by trying to hit the health endpoint
                status.is_our_server = Self::verify_our_server().await?;
            }
            Err(e) => {
                // Some other error (permissions, etc.)
                return Err(Box::new(e));
            }
        }

        // Check if CSS is outdated
        status.css_outdated = Self::check_css_outdated()?;

        // Run verification checks
        if status.is_our_server {
            status.verification_results = Self::run_verifications().await?;
        }

        Ok(status)
    }

    /// Verify if the server on port 12012 is our twelve-factor server
    async fn verify_our_server() -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()?;

        match client.get(&format!("http://localhost:{}/", TWELVE_FACTOR_PORT)).send().await {
            Ok(response) => {
                let text = response.text().await?;
                Ok(text.contains("Twelve-Factor") || text.contains("twelve-factor"))
            }
            Err(_) => Ok(false),
        }
    }

    /// Check if SCSS files are newer than compiled CSS
    fn check_css_outdated() -> Result<bool, Box<dyn std::error::Error>> {
        let css_path = "public/assets/css/layout.css";
        let scss_dir = "assets/scss";

        if !Path::new(css_path).exists() {
            return Ok(true); // CSS doesn't exist, needs compilation
        }

        let css_modified = fs::metadata(css_path)?.modified()?;

        // Check all SCSS files
        for entry in fs::read_dir(scss_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("scss") {
                let scss_modified = entry.metadata()?.modified()?;
                if scss_modified > css_modified {
                    return Ok(true);
                }
            }
        }

        // Check subdirectories
        for subdir in &["components", "features", "layout", "pageblocks", "utilities"] {
            let subdir_path = format!("{}/{}", scss_dir, subdir);
            if Path::new(&subdir_path).exists() {
                for entry in fs::read_dir(&subdir_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.extension().and_then(|s| s.to_str()) == Some("scss") {
                        let scss_modified = entry.metadata()?.modified()?;
                        if scss_modified > css_modified {
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    /// Run all verification checks
    async fn run_verifications() -> Result<VerificationResults, Box<dyn std::error::Error>> {
        let mut results = VerificationResults::default();

        // 1. Check CSS content
        if let Ok(css_content) = fs::read_to_string("public/assets/css/layout.css") {
            results.css_compiled = true;
            results.css_has_yellow_borders = css_content.contains("border-bottom: 3px solid #FDC500");
            results.css_has_numeral_icons = css_content.contains("icon-numeral-01.svg");
            results.css_has_text_indent = css_content.contains("text-indent: -9999px");
        }

        // 2. Check HTTP response
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;

        if let Ok(response) = client.get(&format!("http://localhost:{}/assets/css/layout.css", TWELVE_FACTOR_PORT)).send().await {
            if let Ok(served_css) = response.text().await {
                // Calculate hash of served CSS
                let mut hasher = Sha256::new();
                hasher.update(&served_css);
                let served_hash = format!("{:x}", hasher.finalize());

                // Calculate hash of file CSS
                if let Ok(file_css) = fs::read_to_string("public/assets/css/layout.css") {
                    let mut hasher = Sha256::new();
                    hasher.update(&file_css);
                    let file_hash = format!("{:x}", hasher.finalize());

                    results.server_serving_latest = served_hash == file_hash;
                }
            }
        }

        // 3. Check DOM structure
        if let Ok(response) = client.get(&format!("http://localhost:{}/", TWELVE_FACTOR_PORT)).send().await {
            if let Ok(html) = response.text().await {
                results.html_has_factor_cards = html.contains(r#"class="factor-card""#);
                results.html_has_external_links = html.contains(r#"target="_blank""#);
            }
        }

        // Calculate overall confidence
        let checks_passed = [
            results.css_compiled,
            results.css_has_yellow_borders,
            results.css_has_numeral_icons,
            results.css_has_text_indent,
            results.server_serving_latest,
            results.html_has_factor_cards,
            results.html_has_external_links,
        ].iter().filter(|&&x| x).count();

        results.overall_confidence = match checks_passed {
            7 => ConfidenceLevel::High,
            5..=6 => ConfidenceLevel::Medium,
            _ => ConfidenceLevel::Low,
        };

        Ok(results)
    }

    /// Kill the process on port 12012
    pub fn kill_existing_server(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("kill")
            .arg(pid.to_string())
            .output()?;
        
        // Wait a moment for the process to die
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        Ok(())
    }

    /// Display status with nice formatting
    pub fn display_status(status: &ServerStatus) {
        println!("🔍 Checking port {}...", TWELVE_FACTOR_PORT);
        
        if let Some(pid) = status.pid {
            let pid_display = if pid == 999999 { 
                "unknown".to_string() 
            } else { 
                pid.to_string() 
            };
            println!("   ✓ Found {} server (PID: {})", 
                if status.is_our_server { "twelve-factor" } else { "another" },
                pid_display
            );
            
            if status.css_outdated {
                println!("   ✗ Your SCSS changes won't be visible (CSS compiled before your edits)");
            } else {
                println!("   ✓ CSS is up to date");
            }
            
            if status.is_our_server {
                Self::display_verification_results(&status.verification_results);
            }
        } else {
            println!("   ✓ Port {} is available", TWELVE_FACTOR_PORT);
        }
    }

    fn display_verification_results(results: &VerificationResults) {
        if results.overall_confidence == ConfidenceLevel::High {
            println!("\n   🎯 Verification passed:");
            println!("      ✓ All CSS changes compiled");
            println!("      ✓ Yellow borders present");
            println!("      ✓ Numeral icons configured");
            println!("      ✓ Server serving latest version");
        } else {
            println!("\n   ⚠️  Verification issues:");
            if !results.css_has_yellow_borders {
                println!("      ✗ Yellow borders not found in CSS");
            }
            if !results.css_has_numeral_icons {
                println!("      ✗ Numeral icons not found in CSS");
            }
            if !results.server_serving_latest {
                println!("      ✗ Server may be serving old CSS");
            }
        }
    }
}