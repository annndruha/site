#!/bin/bash
# Simple SCSS compilation script using grass

cd rust_prototype

# Check if grass CLI is available
if ! command -v grass &> /dev/null; then
    echo "Installing grass CLI..."
    cargo binstall grass
fi

# Compile SCSS
echo "Compiling SCSS..."
grass assets/scss/layout.scss public/assets/css/layout.css --load-path=assets/scss --style=expanded

echo "SCSS compilation complete!"