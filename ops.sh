#!/bin/bash
# ops.sh - Common operations helper script for 12factor development
# Usage: bash ops.sh [command]

set -e

PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"
PORT=12012  # 12factor on port 12012!

case "$1" in
  "check-pages")
    # Check all factor pages
    echo "=== Checking Factor Pages ==="
    for factor in codebase dependencies config backing-services build-release-run processes port-binding concurrency disposability dev-prod-parity logs admin-processes; do
      printf "%-20s " "$factor:"
      title=$(curl -s "http://localhost:$PORT/$factor" 2>/dev/null | grep -o '<title>.*</title>' | sed 's/<[^>]*>//g' || echo "")
      
      if [ -n "$title" ]; then
        echo "✓"
      else
        echo "✗ (not responding)"
      fi
    done
    
    echo -e "\n=== Checking Other Pages ==="
    for page in "" "blog" "community"; do
      if [ -z "$page" ]; then
        display="home"
        url="http://localhost:$PORT/"
      else
        display="$page"
        url="http://localhost:$PORT/$page"
      fi
      
      printf "%-20s " "$display:"
      if curl -s -f -o /dev/null "$url"; then
        echo "✓"
      else
        echo "✗"
      fi
    done
    ;;
    
  "verify-design")
    # Check for key design elements
    echo "=== Verifying Design Elements ==="
    
    # Check home page
    echo -n "Home page diamond icons: "
    if curl -s "http://localhost:$PORT/" | grep -q "icon-diamond"; then
      echo "✓"
    else
      echo "✗"
    fi
    
    echo -n "Yellow accent colors: "
    if curl -s "http://localhost:$PORT/assets/css/layout.css" | grep -q "#ffd42d\|#ffd700"; then
      echo "✓"
    else
      echo "✗"
    fi
    
    echo -n "Navigation external link indicator: "
    if curl -s "http://localhost:$PORT/" | grep -q "glyph-external-link"; then
      echo "✓"
    else
      echo "✗"
    fi
    ;;
    
  "page-diff")
    # Compare a specific page between old and new implementation
    page=${2:-""}
    if [ -z "$page" ]; then
      echo "Usage: bash ops.sh page-diff [page-path]"
      echo "Example: bash ops.sh page-diff codebase"
      exit 1
    fi
    
    echo "=== Comparing $page ==="
    
    # Create temp files
    temp_file="/tmp/12factor_page_$$"
    
    # Get page content
    curl -s "http://localhost:$PORT/$page" > "${temp_file}.html"
    
    # Extract key elements
    echo -e "\nTitle:"
    grep -o '<title>.*</title>' "${temp_file}.html" | sed 's/<[^>]*>//g' || echo "Not found"
    
    echo -e "\nMain heading:"
    grep -o '<h1[^>]*>.*</h1>' "${temp_file}.html" | head -1 | sed 's/<[^>]*>//g' || echo "Not found"
    
    echo -e "\nLinks count:"
    grep -o '<a[^>]*>' "${temp_file}.html" | wc -l
    
    # Cleanup
    rm -f "${temp_file}.html"
    ;;
    
  "compile-scss")
    # Compile SCSS files
    echo "=== Compiling SCSS ==="
    cd "$PROJECT_ROOT/rust_prototype"
    
    if command -v grass &> /dev/null; then
      grass assets/scss/layout.scss public/assets/css/layout.css --load-path=assets/scss --style=expanded
      echo "✓ SCSS compiled successfully"
    else
      echo "✗ grass not installed. Installing..."
      cargo install grass
      grass assets/scss/layout.scss public/assets/css/layout.css --load-path=assets/scss --style=expanded
      echo "✓ SCSS compiled successfully"
    fi
    ;;
    
  "check-locale")
    # Check localization support
    locale=${2:-"es"}
    echo "=== Checking Locale Support ($locale) ==="
    
    # Check if locale directory exists
    if [ -d "$PROJECT_ROOT/content/$locale" ]; then
      echo "✓ Content directory exists"
    else
      echo "✗ Content directory missing"
    fi
    
    # Check a factor page in locale
    echo -n "Factor page in $locale: "
    if curl -s -f -o /dev/null "http://localhost:$PORT/$locale/codebase"; then
      echo "✓"
    else
      echo "✗"
    fi
    
    # Check if UI is translated
    echo -n "UI translations: "
    if [ -f "$PROJECT_ROOT/locales/$locale.yml" ]; then
      echo "✓ Translation file exists (but may not be loaded)"
    else
      echo "✗ Translation file missing"
    fi
    ;;
    
  *)
    echo "Usage: bash ops.sh [command]"
    echo ""
    echo "Commands:"
    echo "  check-pages    - Check all pages are responding"
    echo "  verify-design  - Verify key design elements are present"
    echo "  page-diff      - Compare a specific page"
    echo "  compile-scss   - Compile SCSS to CSS"
    echo "  check-locale   - Check localization support"
    exit 1
    ;;
esac