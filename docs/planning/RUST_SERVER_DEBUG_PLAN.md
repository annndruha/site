# Rust Server Debug and Remediation Plan for Sonnet

## Problem Statement
The Rust server at `rust_prototype/` is not starting when running `cargo run --release`. The yellow design elements (Phase 1 of Vish's feedback) have been implemented in SCSS but cannot be viewed because the server won't start.

## ROOT CAUSE FOUND
The `main()` function is missing from `/home/ykatz/Code/Heroku/12factor/rust_prototype/src/main.rs`. The file ends at line 477 with just the health check handler, but there's no `#[tokio::main] async fn main()` function to actually start the server!

## Investigation Steps

### 1. Check Basic Prerequisites
```bash
# Check if we're in the right directory
pwd
ls rust_prototype/

# Check Rust/Cargo installation
which cargo
cargo --version

# Check if Cargo.toml exists and is valid
cat rust_prototype/Cargo.toml
```

### 2. Check for Compilation Errors
```bash
# Try to build without running
cd rust_prototype
cargo build --release

# If that fails, try debug build for more verbose errors
cargo build

# Check for any error logs
ls *.log
cat ../rust_server.log 2>/dev/null || echo "No log file"
```

### 3. Check Port Availability
```bash
# Check if port 3001 is already in use
lsof -i :3001 || echo "Port 3001 is free"
ss -tln | grep 3001 || echo "Port 3001 not listening"

# Check if any other process is blocking
ps aux | grep -E "cargo|rust_prototype" | grep -v grep
```

### 4. Check Dependencies and Lock File
```bash
# Check if Cargo.lock exists and is valid
ls rust_prototype/Cargo.lock

# Try cleaning and rebuilding
cd rust_prototype
cargo clean
cargo build --release
```

### 5. Check for Runtime Configuration Issues
```bash
# Check for required environment variables
grep -r "env::" rust_prototype/src/ || echo "No env vars required"

# Check the main.rs for the actual port binding
grep -A5 -B5 "bind\|listen\|serve" rust_prototype/src/main.rs

# Check if there's a configuration file
find rust_prototype -name "*.toml" -o -name "*.yaml" -o -name "*.json" | grep -v target
```

### 6. Try Alternative Start Methods
```bash
# Method 1: Direct cargo run with explicit output
cd rust_prototype
cargo run --release 2>&1 | tee ../rust_debug.log

# Method 2: Build then run binary directly
cargo build --release
./target/release/twelve-factor 2>&1 | tee ../rust_debug.log

# Method 3: Try with RUST_BACKTRACE for more info
RUST_BACKTRACE=1 cargo run --release 2>&1 | tee ../rust_debug.log
```

### 7. Check SCSS Compilation
Since the yellow design elements are in SCSS:
```bash
# Check if compiled CSS exists
ls rust_prototype/public/assets/css/layout.css

# Check if SCSS files are being watched/compiled
grep -r "grass\|scss" rust_prototype/src/

# Manually compile SCSS if needed
# (grass is the Rust SCSS compiler used in the project)
```

## Remediation Steps

### If Port is Blocked:
1. Kill any existing processes on port 3001
2. Use a different port by modifying the bind address in main.rs

### If Compilation Fails:
1. Check error messages for missing dependencies
2. Run `cargo update` to update dependencies
3. Check Rust version compatibility with `rustc --version`

### If Runtime Fails:
1. Check for missing assets or templates
2. Verify all symlinks are intact (content/, blog/, public/)
3. Check file permissions

### If SCSS Not Compiling:
1. Verify grass (SCSS compiler) is in dependencies
2. Check if assets are being served from the right path
3. Manually compile SCSS and check output

## Quick Test Plan
Once server starts:
1. Open http://localhost:3001
2. Check browser console for 404s on CSS/assets
3. Use browser inspector to verify:
   - Yellow borders on header/footer
   - External link icons loading
   - Numeral SVGs replacing Roman numerals

## Fallback Options

### Option 1: Use Ruby Server Temporarily
```bash
# Copy the compiled CSS to Ruby's public directory
cp rust_prototype/public/assets/css/layout.css public/assets/css/
# Start Ruby server and test
ruby web.rb
```

### Option 2: Static File Server
```bash
# Use Python's simple HTTP server in the Rust public directory
cd rust_prototype/public
python3 -m http.server 8000
# Open http://localhost:8000/test_index.html
```

### Option 3: Debug with Minimal Example
Create a minimal Rust server to isolate the issue:
```rust
// minimal_server.rs
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    
    println!("Starting server on 0.0.0.0:3001");
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## IMMEDIATE FIX REQUIRED

Add the missing `main()` function to the end of `/home/ykatz/Code/Heroku/12factor/rust_prototype/src/main.rs`:

```rust
#[tokio::main]
async fn main() {
    // Change to the rust_prototype directory to ensure relative paths work
    if std::env::current_dir().unwrap().file_name().unwrap() != "rust_prototype" {
        std::env::set_current_dir("rust_prototype").expect("Failed to change to rust_prototype directory");
    }
    
    // Initialize components
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template error: {:?}", e);
            std::process::exit(1);
        }
    };
    
    let markdown_options = ComrakOptions {
        extension: comrak::ComrakExtensionOptions {
            autolink: true,
            table: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let asset_compiler = assets::AssetCompiler::new();
    let blog_data = Arc::new(blog::BlogData::load());
    let community_config = Arc::new(config::CommunityConfig::load());
    
    let state = Arc::new(AppState {
        tera,
        markdown_options,
        asset_compiler,
        blog_data,
        community_config,
    });
    
    // Build the router
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health_check))
        .route("/blog", get(blog))
        .route("/blog/:slug", get(blog_post))
        .route("/community", get(community))
        .route("/:factor", get(factor))
        .route("/:locale/:factor", get(localized_factor))
        .nest_service("/assets", ServeDir::new("public/assets"))
        .nest_service("/resources", ServeDir::new("public/resources"))
        .with_state(state);
    
    // Start the server
    let addr = "0.0.0.0:3001";
    println!("Starting server on {}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Success Criteria
1. Rust server starts and listens on port 3001
2. Homepage loads with yellow design elements visible
3. All Phase 1 items from DESIGN_RESTORATION_PLAN.md are working

## Notes for Sonnet
- The main function is MISSING from the file - this is why it won't start
- Add the function above to the END of main.rs (after line 477)
- The yellow design implementation is complete in SCSS, we just need the server running
- Priority is getting Vish's feedback implemented and visible
- Once the main function is added, the server should start normally

## Important: Follow the "Why?" Protocol

When the user asks "Why?" at any point:
1. **STOP** current action immediately
2. **EXPLAIN** reasoning clearly
3. **WAIT** for direction
4. **DON'T** proceed with assumptions

Example scenarios:
- If adding the main function and user asks "Why?" → Explain that the entry point is missing
- If modifying any file and user asks "Why?" → Explain the specific reason for that change
- If running any command and user asks "Why?" → Explain what the command does and why it's needed

This protocol is documented in CLAUDE.md under "Quick Decision Guide" and is critical for maintaining user trust.