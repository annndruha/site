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