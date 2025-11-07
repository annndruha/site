use grass::OutputStyle;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scss_input_dir = "assets/scss";
    let css_output_dir = "public/assets/css";

    // Ensure output directory exists
    fs::create_dir_all(css_output_dir)?;

    // Compile main layout.scss file
    let scss_path = format!("{}/layout.scss", scss_input_dir);
    let css_output_path = format!("{}/layout.css", css_output_dir);

    if Path::new(&scss_path).exists() {
        println!("Compiling SCSS: {} -> {}", scss_path, css_output_path);

        let css = grass::from_path(
            &scss_path,
            &grass::Options::default()
                .style(OutputStyle::Expanded)
                .load_path(scss_input_dir),
        )?;

        fs::write(&css_output_path, css)?;
        println!("✅ SCSS compilation completed successfully!");
        println!("   Output: {}", css_output_path);
    } else {
        eprintln!("❌ SCSS file not found: {}", scss_path);
        std::process::exit(1);
    }

    Ok(())
}
