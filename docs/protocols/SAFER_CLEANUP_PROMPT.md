# Safer Cleanup Approach for Twelve-Factor Project

## Key Safety Principles
1. **Archive before delete** - Never delete without archiving first
2. **Test after each step** - Verify the app works before proceeding
3. **Keep Rust structure intact** - Don't break the working app
4. **Preview destructive operations** - See what will be affected first

## Phase 1: Preparation and Backup

### Step 1: Create Full Backup
```bash
# Create backup branch
git checkout -b pre-cleanup-backup

# See what we're about to commit
git status

# Commit everything for safety
git add .
git commit -m "Pre-cleanup backup: $(date +%Y%m%d_%H%M%S) - Rust implementation complete"

# Push to remote for extra safety
git push origin pre-cleanup-backup
```

### Step 2: Test Current State
```bash
# Verify Rust app works BEFORE any changes
cd rust_prototype
cargo build
cargo run

# Test a few pages
curl -s http://localhost:12012/ | grep -q "Twelve-Factor" && echo "✓ Home page works"
curl -s http://localhost:12012/community | grep -q "community-hero" && echo "✓ Enhanced community works"

# Stop the server
# Press Ctrl+C
cd ..
```

## Phase 2: Safe Archiving (No Deletions Yet)

### Step 3: Create Archive Structure
```bash
mkdir -p archive/2025-01-migration/ruby-implementation
mkdir -p archive/2025-01-migration/design-assets
mkdir -p archive/2025-01-migration/temporary-files
```

### Step 4: Archive Ruby Implementation
```bash
# Copy (don't move yet) Ruby files
cp -r web.rb lib views Gemfile Gemfile.lock archive/2025-01-migration/ruby-implementation/
cp -r bin config.ru Procfile archive/2025-01-migration/ruby-implementation/
cp test_markdown_rendering.rb archive/2025-01-migration/ruby-implementation/ 2>/dev/null || true
```

### Step 5: Archive Design Assets
```bash
# Copy design assets
cp -r "12factor FINAL" archive/2025-01-migration/design-assets/
cp -r public/assets/mockups archive/2025-01-migration/design-assets/

# Archive screenshots with clear names
mkdir -p archive/2025-01-migration/design-assets/screenshots
for png in new_design_*.png canonical_home.png community_design_*.png; do
  [ -f "$png" ] && cp "$png" archive/2025-01-migration/design-assets/screenshots/
done
```

## Phase 3: Test With Archives

### Step 6: Verify Archives Are Complete
```bash
# Check that archive has everything we need
ls -la archive/2025-01-migration/ruby-implementation/
ls -la archive/2025-01-migration/design-assets/

# Verify Rust app still works (nothing broken yet)
cd rust_prototype && cargo run
# Test again, then Ctrl+C
cd ..
```

## Phase 4: Clean Temporary Files Only

### Step 7: Preview What Will Be Deleted
```bash
# See what log files exist
find . -name "*.log" -type f

# See what PNG files would be deleted (NOT in critical directories)
find . -name "*.png" -type f \
  -not -path "./archive/*" \
  -not -path "./public/assets/images/*" \
  -not -path "./public/images/*" \
  -not -path "./public/resources/images/*" \
  -not -path "./.git/*" \
  -not -path "./content/*"

# Review these lists carefully!
```

### Step 8: Remove Only Confirmed Temporary Files
```bash
# Remove log files
find . -name "*.log" -type f -delete

# Remove PID files
rm -f ruby_server.pid rust_server.pid
rm -f rust_prototype/rust_server.pid

# Remove test outputs
rm -f maruku_output.html redcarpet_output.html
rm -f test_markdown_rust/comrak_output.html test_markdown_rust/pulldown_output.html
```

## Phase 5: Prepare for Heroku Deployment

### Step 9: Create Root-Level Entry Points
Instead of moving the entire Rust structure (which would break it), create entry points:

```bash
# Create a root-level script to run the Rust app
cat > run.sh << 'EOF'
#!/bin/bash
cd rust_prototype && cargo run --release
EOF
chmod +x run.sh

# Create a build script for Heroku
cat > build.sh << 'EOF'
#!/bin/bash
cd rust_prototype && cargo build --release
EOF
chmod +x build.sh

# Create Procfile for Heroku (using Rust app)
echo "web: cd rust_prototype && ./target/release/twelve_factor" > Procfile
```

## Phase 6: Update Documentation

### Step 10: Update README
Update README.md to reflect that this is now primarily a Rust application:
- Remove Ruby setup instructions  
- Add Rust setup instructions
- Document the new structure
- Note that Ruby implementation is archived

### Step 11: Create Migration Record
```bash
cat > archive/2025-01-migration/MIGRATION_NOTES.md << 'EOF'
# Twelve-Factor Migration to Rust

Date: January 2025

## What Changed
- Primary implementation moved from Ruby/Sinatra to Rust/Axum
- Enhanced community page with live GitHub integration
- Added offline support with service workers
- Implemented new design with yellow accents

## Structure
- Rust application remains in `rust_prototype/` for stability
- Ruby implementation archived in `archive/2025-01-migration/ruby-implementation/`
- Entry points created at root level for deployment

## Deployment
- Heroku deployment will use Rust buildpack
- See Procfile and build.sh for configuration
EOF
```

## Phase 7: Final Cleanup

### Step 12: Remove Ruby Files (Only After Full Verification)
```bash
# Test one more time that Rust app works
cd rust_prototype && cargo run
# Visit http://localhost:12012 and test thoroughly
# Ctrl+C when done

# If EVERYTHING works, remove Ruby files
cd ..
rm -rf lib views web.rb
rm -f Gemfile Gemfile.lock
rm -rf bin
```

### Step 13: Commit Final State
```bash
git add .
git commit -m "Complete Rust migration - Ruby implementation archived"
```

## Rollback Plan
If anything goes wrong:
```bash
git checkout pre-cleanup-backup
```

This will restore everything to the pre-cleanup state.

## Important Notes
1. The Rust app stays in `rust_prototype/` to avoid breaking paths
2. We create thin wrappers at root level for deployment
3. Everything is archived before deletion
4. Multiple verification steps ensure nothing breaks
5. Easy rollback if needed

This approach is much safer and preserves the working application structure.