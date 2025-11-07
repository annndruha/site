use axum::{
    extract::{Path, State},
    response::{Html, Json},
    routing::get,
    Router,
};
use comrak::{markdown_to_html, ComrakOptions};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

mod assets;
mod blog;
mod config;
mod server_manager;

// The 12 factors
static FACTORS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "codebase",
        "dependencies",
        "config",
        "backing-services",
        "build-release-run",
        "processes",
        "port-binding",
        "concurrency",
        "disposability",
        "dev-prod-parity",
        "logs",
        "admin-processes",
    ]
});

// Supported locales
static LOCALES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "en", "cs", "de", "el", "es", "fr", "it", "ja", "ko", "pl", "pt_br", "ru", "sk", "th",
        "tr", "uk", "vi", "zh_cn",
    ]
});

#[derive(Clone)]
struct AppState {
    tera: Tera,
    markdown_options: ComrakOptions,
    asset_compiler: assets::AssetCompiler,
    blog_data: Arc<blog::BlogData>,
    community_config: Arc<config::CommunityConfig>,
}

#[derive(Serialize)]
struct FactorCard {
    number: String,
    slug: String,
    title: String,
    subtitle: String,
    discussion_url: Option<String>,
    discussion_text: Option<String>,
    participant_count: Option<u32>,
    last_activity: Option<String>,
    is_active: bool,
}

#[tokio::main]
async fn main() {
    // Check for --check or --launch flags
    let args: Vec<String> = std::env::args().collect();
    let check_only = args.contains(&"--check".to_string());
    let launch_mode = args.contains(&"--launch".to_string());
    
    tracing_subscriber::fmt::init();
    
    // Check server status on port 12012
    match server_manager::ServerManager::check_port().await {
        Ok(status) => {
            server_manager::ServerManager::display_status(&status);
            
            if let Some(pid) = status.pid {
                if status.is_our_server && (status.css_outdated || status.verification_results.overall_confidence != server_manager::ConfidenceLevel::High) {
                    println!("\n🔄 Restarting to apply changes...");
                    server_manager::ServerManager::kill_existing_server(pid).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                } else if !status.is_our_server {
                    eprintln!("\n❌ Port {} is in use by another process", server_manager::TWELVE_FACTOR_PORT);
                    eprintln!("   Please stop that process first");
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️  Warning: Could not check port status: {}", e);
        }
    }
    
    // If --check flag was passed, just report status and exit
    if check_only {
        std::process::exit(0);
    }
    
    // If --launch flag was passed, spawn server in background and exit
    if launch_mode {
        // Check if server is already running
        if let Ok(status) = server_manager::ServerManager::check_port().await {
            if status.pid.is_some() && status.is_our_server {
                println!("✅ Server already running on port {}", server_manager::TWELVE_FACTOR_PORT);
                std::process::exit(0);
            }
        }
        
        println!("🚀 Launching server on port {}...", server_manager::TWELVE_FACTOR_PORT);
        
        // Get the executable path
        let exe_path = std::env::current_exe().expect("Failed to get executable path");
        
        // Spawn the server without --launch flag
        let mut cmd = std::process::Command::new(&exe_path);
        cmd.stdout(std::process::Stdio::null())
           .stderr(std::process::Stdio::null())
           .stdin(std::process::Stdio::null());
        
        // On Unix, we can properly daemonize
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                cmd.pre_exec(|| {
                    // Create new session
                    libc::setsid();
                    Ok(())
                });
            }
        }
        
        match cmd.spawn() {
            Ok(_) => {
                // Wait a moment for server to start
                std::thread::sleep(std::time::Duration::from_millis(2000));
                println!("✅ Server launched successfully: http://localhost:{}", server_manager::TWELVE_FACTOR_PORT);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("❌ Failed to launch server: {}", e);
                std::process::exit(1);
            }
        }
    }

    // We run from the repository root, as specified in Procfile
    tracing::info!("Working directory: {:?}", std::env::current_dir().unwrap());

    // Initialize Tera templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            tracing::info!("Templates loaded successfully");
            t
        },
        Err(e) => {
            let error_msg = format!("Template parsing error: {}", e);
            std::fs::write("template_error.log", &error_msg).ok();
            eprintln!("{}", error_msg);
            std::process::exit(1);
        }
    };

    // Configure markdown parser
    let mut markdown_options = ComrakOptions::default();
    markdown_options.extension.autolink = true;
    markdown_options.extension.table = true;
    markdown_options.render.hardbreaks = false;

    // Set up asset compilation
    let asset_compiler = assets::AssetCompiler::new("assets/scss", "public/assets/css");
    
    // Compile SCSS on startup
    if let Err(e) = asset_compiler.compile_scss_dev().await {
        let error_msg = format!("Failed to compile SCSS: {}", e);
        std::fs::write("scss_error.log", &error_msg).ok();
        eprintln!("{}", error_msg);
        std::process::exit(1);
    }

    // Load blog data
    let blog_data = match blog::BlogData::load() {
        Ok(data) => Arc::new(data),
        Err(e) => {
            let error_msg = format!("Failed to load blog data: {}", e);
            std::fs::write("blog_error.log", &error_msg).ok();
            eprintln!("{}", error_msg);
            std::process::exit(1);
        }
    };

    // Load community config
    let community_config = Arc::new(config::CommunityConfig::load());

    let app_state = AppState {
        tera,
        markdown_options,
        asset_compiler,
        blog_data,
        community_config,
    };

    // Build the router
    let app = Router::new()
        // Health check endpoints
        .route("/health", get(|| async { "OK" }))
        .route("/health/detailed", get(health_check))
        // Home page
        .route("/", get(home))
        // Blog routes (more specific, so they go first)
        .route("/blog", get(blog))
        .route("/blog/:post", get(blog_post))
        // Community page
        .route("/community", get(community))
        // Factor pages - these will handle both /:factor and /:locale patterns
        .route("/:path", get(factor_or_locale))
        .route("/:locale/:factor", get(factor_locale))
        // Static files
        .nest_service("/assets", ServeDir::new("public/assets"))
        .nest_service("/images", ServeDir::new("public/images"))
        .nest_service("/resources", ServeDir::new("public/resources"))
        // State
        .with_state(Arc::new(app_state));

    // Use PORT env var if available (for Heroku), otherwise use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| server_manager::TWELVE_FACTOR_PORT.to_string());
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    
    println!("\n✅ Server ready: http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn home(State(state): State<Arc<AppState>>) -> Html<String> {
    render_home(&state, "en").await
}

async fn factor_or_locale(
    Path(path): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    // Check if it's a locale
    if LOCALES.contains(&path.as_str()) {
        render_home(&state, &path).await
    } else if FACTORS.contains(&path.as_str()) {
        // It's a factor
        render_factor(&state, "en", &path).await
    } else {
        Html("404 - Not Found".to_string())
    }
}

async fn render_home(state: &AppState, locale: &str) -> Html<String> {
    let mut context = Context::new();
    context.insert("locale", locale);
    context.insert("factors", &*FACTORS);
    
    // Load featured posts for home page
    let featured_posts = state.blog_data.get_featured_posts();
    context.insert("featured_posts", &featured_posts);
    
    // Get latest blog post
    let all_posts = state.blog_data.get_all_posts();
    if let Some(latest_post) = all_posts.first() {
        context.insert("latest_post", latest_post);
    }
    
    // Build factor cards with metadata
    let factor_cards = build_factor_cards(&state.community_config);
    context.insert("factor_cards", &factor_cards);
    
    // Add community config to context
    context.insert("community_config", &*state.community_config);
    
    // Load content sections
    let intro = load_markdown_content(locale, "intro", &state.markdown_options);
    let background = load_markdown_content(locale, "background", &state.markdown_options);
    let who = load_markdown_content(locale, "who", &state.markdown_options);
    let toc = load_markdown_content(locale, "toc", &state.markdown_options);
    
    context.insert("intro", &intro);
    context.insert("background", &background);
    context.insert("who", &who);
    context.insert("toc", &toc);
    
    match state.tera.render("home_community.html", &context) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("Template error: {}", e)),
    }
}


async fn factor_locale(
    Path((locale, factor)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    render_factor(&state, &locale, &factor).await
}

async fn render_factor(state: &AppState, locale: &str, factor: &str) -> Html<String> {
    // Validate factor
    if !FACTORS.contains(&factor.as_ref()) {
        return Html("404 - Factor not found".to_string());
    }
    
    let mut context = Context::new();
    context.insert("locale", locale);
    context.insert("factor", factor);
    context.insert("factor_slug", factor);
    
    // Load factor content
    let content = load_markdown_content(locale, factor, &state.markdown_options);
    context.insert("content", &content);
    
    // Get factor metadata
    let factor_number = FACTORS.iter().position(|&f| f == factor).unwrap_or(0) + 1;
    let factor_title = get_factor_display_title(factor);
    let factor_subtitle = get_factor_subtitle(factor);
    
    context.insert("factor_number", &factor_number);
    context.insert("factor_title", &factor_title);
    context.insert("factor_subtitle", &factor_subtitle);
    context.insert("factor_slug", &factor);
    context.insert("factor_roman", &get_roman_numeral(factor_number));
    
    // Add community/evolution data
    context.insert("github_repo", &state.community_config.github_repo);
    context.insert("original_year", &state.community_config.original_year);
    
    // Check if there's an active discussion for this factor
    if let Some(discussion) = state.community_config.get_discussion_for_factor(factor) {
        context.insert("discussion_url", &discussion.issue_url);
        // Note: participant_count will be populated by JavaScript
    }
    
    // Add navigation
    if let Some(pos) = FACTORS.iter().position(|&f| f == factor) {
        if pos > 0 {
            let prev_slug = FACTORS[pos - 1];
            let prev_title = get_factor_display_title(prev_slug);
            context.insert("prev_factor", &serde_json::json!({
                "slug": prev_slug,
                "title": prev_title
            }));
        }
        if pos < FACTORS.len() - 1 {
            let next_slug = FACTORS[pos + 1];
            let next_title = get_factor_display_title(next_slug);
            context.insert("next_factor", &serde_json::json!({
                "slug": next_slug,
                "title": next_title
            }));
        }
    }
    
    match state.tera.render("factor.html", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            eprintln!("Factor template error: {:?}", e);
            eprintln!("Context: {:?}", context);
            Html(format!("Template error: {}", e))
        }
    }
}

async fn blog(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    
    // Add featured posts
    context.insert("featured_posts", state.blog_data.get_featured_posts());
    
    // Add all posts
    context.insert("all_posts", state.blog_data.get_all_posts());
    
    match state.tera.render("blog_listing.html", &context) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("Error rendering blog listing: {}", e)),
    }
}

async fn blog_post(
    Path(post_slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    if let Some(post) = state.blog_data.get_post(&post_slug) {
        let mut context = Context::new();
        context.insert("post", post);
        
        match state.tera.render("blog_post.html", &context) {
            Ok(html) => Html(html),
            Err(e) => Html(format!("Error rendering blog post: {}", e)),
        }
    } else {
        Html(format!("<h1>Post not found: {}</h1>", post_slug))
    }
}

async fn community(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    
    // Load community content from blog directory
    let community_path = "blog/community.md";
    let community_markdown = std::fs::read_to_string(community_path)
        .unwrap_or_else(|_| "Community content not found.".to_string());
    let community_content = markdown_to_html(&community_markdown, &state.markdown_options);
    context.insert("community_content", &community_content);
    
    // Load maintainers content from blog directory
    let maintainers_path = "blog/maintainers.md";
    let maintainers_markdown = std::fs::read_to_string(maintainers_path)
        .unwrap_or_else(|_| "Maintainers content not found.".to_string());
    let maintainers_content = markdown_to_html(&maintainers_markdown, &state.markdown_options);
    context.insert("maintainers_content", &maintainers_content);
    
    match state.tera.render("community_enhanced.html", &context) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("Error rendering community page: {}", e)),
    }
}

fn load_markdown_content(locale: &str, name: &str, options: &ComrakOptions) -> String {
    let path = format!("content/{}/{}.md", locale, name);
    
    match std::fs::read_to_string(&path) {
        Ok(content) => markdown_to_html(&content, options),
        Err(_) => {
            // Fallback to English
            let fallback_path = format!("content/en/{}.md", name);
            match std::fs::read_to_string(&fallback_path) {
                Ok(content) => markdown_to_html(&content, options),
                Err(_) => format!("<p>Content not found: {}</p>", name),
            }
        }
    }
}

fn get_factor_title(factor: &str) -> String {
    match factor {
        "codebase" => "I. Codebase".to_string(),
        "dependencies" => "II. Dependencies".to_string(),
        "config" => "III. Config".to_string(),
        "backing-services" => "IV. Backing services".to_string(),
        "build-release-run" => "V. Build, release, run".to_string(),
        "processes" => "VI. Processes".to_string(),
        "port-binding" => "VII. Port binding".to_string(),
        "concurrency" => "VIII. Concurrency".to_string(),
        "disposability" => "IX. Disposability".to_string(),
        "dev-prod-parity" => "X. Dev/prod parity".to_string(),
        "logs" => "XI. Logs".to_string(),
        "admin-processes" => "XII. Admin processes".to_string(),
        _ => factor.to_string(),
    }
}

fn get_factor_display_title(factor: &str) -> String {
    match factor {
        "codebase" => "Codebase".to_string(),
        "dependencies" => "Dependencies".to_string(),
        "config" => "Config".to_string(),
        "backing-services" => "Backing Services".to_string(),
        "build-release-run" => "Build, Release, Run".to_string(),
        "processes" => "Processes".to_string(),
        "port-binding" => "Port Binding".to_string(),
        "concurrency" => "Concurrency".to_string(),
        "disposability" => "Disposability".to_string(),
        "dev-prod-parity" => "Dev/Prod Parity".to_string(),
        "logs" => "Logs".to_string(),
        "admin-processes" => "Admin Processes".to_string(),
        _ => factor.to_string(),
    }
}

fn get_factor_subtitle(factor: &str) -> String {
    match factor {
        "codebase" => "One codebase tracked in revision control, many deploys".to_string(),
        "dependencies" => "Explicitly declare and isolate dependencies".to_string(),
        "config" => "Store config in the environment".to_string(),
        "backing-services" => "Treat backing services as attached resources".to_string(),
        "build-release-run" => "Strictly separate build and run stages".to_string(),
        "processes" => "Execute the app as one or more stateless processes".to_string(),
        "port-binding" => "Export services via port binding".to_string(),
        "concurrency" => "Scale out via the process model".to_string(),
        "disposability" => "Maximize robustness with fast startup and graceful shutdown".to_string(),
        "dev-prod-parity" => "Keep development, staging, and production as similar as possible".to_string(),
        "logs" => "Treat logs as event streams".to_string(),
        "admin-processes" => "Run admin/management tasks as one-off processes".to_string(),
        _ => "".to_string(),
    }
}

fn get_roman_numeral(number: usize) -> &'static str {
    match number {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "XI",
        12 => "XII",
        _ => "",
    }
}

fn build_factor_cards(community_config: &config::CommunityConfig) -> Vec<FactorCard> {
    FACTORS.iter().enumerate().map(|(index, factor)| {
        let number = index + 1;
        let title = get_factor_display_title(factor);
        let subtitle = get_factor_subtitle(factor);
        
        // Get discussion info from config
        let discussion_url = community_config.get_discussion_for_factor(factor)
            .map(|d| d.issue_url.clone());
        
        FactorCard {
            number: get_roman_numeral(number).to_string(),
            slug: factor.to_string(),
            title,
            subtitle,
            discussion_url,
            discussion_text: None, // Will be populated by JavaScript
            participant_count: None,
            last_activity: None,
            is_active: false,
        }
    }).collect()
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
    version: &'static str,
    uptime_seconds: u64,
}

async fn health_check() -> Json<HealthStatus> {
    static START_TIME: Lazy<std::time::Instant> = Lazy::new(std::time::Instant::now);
    
    Json(HealthStatus {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: START_TIME.elapsed().as_secs(),
    })
}

