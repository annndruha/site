# Quick Commands Reference

Copy and paste these commands directly instead of using scripts.

## Check All Factor Pages
```bash
# Ruby server check
curl -s http://localhost:4567/codebase | grep -c "<title>"

# Rust server check  
curl -s http://localhost:3001/codebase | grep -c "<title>"

# Check all factors (run individually)
for f in codebase dependencies config backing-services build-release-run processes port-binding concurrency disposability dev-prod-parity logs admin-processes; do echo -n "$f: "; curl -s http://localhost:3001/$f | grep -c "breadcrumb"; done
```

## Verify Design Elements
```bash
# Check for breadcrumbs
curl -s http://localhost:3001/codebase | grep -c "breadcrumb"

# Check for numeral icons
curl -s http://localhost:3001/codebase | grep -c "icon-numeral"

# Check for mega menu
curl -s http://localhost:3001/ | grep -c "mega-menu"

# Count navigation links
curl -s http://localhost:3001/ | grep -o 'href="/[^"]*"' | sort -u | wc -l
```

## Server Management
```bash
# Check if servers running
lsof -i :4567  # Ruby
lsof -i :3001  # Rust

# Start Ruby server
ruby web.rb -p 4567

# Start Rust server  
/home/ykatz/Code/Heroku/12factor/rust_prototype/target/release/twelve_factor

# Kill servers by port
kill $(lsof -t -i:4567)  # Kill Ruby
kill $(lsof -t -i:3001)  # Kill Rust
```

## Development Tasks
```bash
# Build Rust (from rust_prototype dir)
cargo build --release

# Find content files
find content -name "*.md" | grep -v "toc\|who\|intro\|background"

# List routes in Ruby
grep -E "get '.*' do" web.rb | sed "s/.*'\(.*\)'.*/\1/" | sort

# List routes in Rust
grep -E '\.route\("' rust_prototype/src/main.rs | sed 's/.*"\(.*\)".*/\1/' | sort

# Check SCSS status
ls -la rust_prototype/public/assets/css/layout.css
stat rust_prototype/public/assets/css/layout.css

# Count SCSS files
find rust_prototype/assets/scss -name "*.scss" | wc -l
```

## Quick Diffs
```bash
# Save page outputs for comparison
curl -s http://localhost:4567/codebase -o ruby_codebase.html
curl -s http://localhost:3001/codebase -o rust_codebase.html

# Then use Read tool to compare

# Quick structure check
echo "Ruby:"; curl -s http://localhost:4567/ | wc -l
echo "Rust:"; curl -s http://localhost:3001/ | wc -l
```

## Testing Specific Elements
```bash
# Test hero section
curl -s http://localhost:3001/ | grep -A5 "pageblock--hero"

# Test footer
curl -s http://localhost:3001/ | grep -A5 "site-footer"

# Test factor navigation
curl -s http://localhost:3001/codebase | grep -E "(prev|next)-factor"

# Test blog grid
curl -s http://localhost:3001/blog | grep -c "blog-post"
```

## Common Patterns
```bash
# Instead of: command1 && command2
# Use: command1; command2

# Instead of: command > file.txt  
# Use: command -o file.txt (for curl)
# Or: command | tee file.txt (if you need stdout too)

# Instead of: cd dir && command
# Use absolute paths: /full/path/to/command

# Instead of complex scripts, chain simple commands:
curl -s http://localhost:3001/ | grep -c "twelve-factor"
```

## Directory Navigation
```bash
# Key paths to remember
ls /home/ykatz/Code/Heroku/12factor/rust_prototype/templates/
ls /home/ykatz/Code/Heroku/12factor/content/en/
ls /home/ykatz/Code/Heroku/12factor/blog/
```