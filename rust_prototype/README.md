# 12factor Rust Prototype

This is an experimental Rust implementation of the 12factor.net website, aimed at improving deployment and maintenance ergonomics while maintaining full compatibility with the existing content and workflow.

## Goals

1. **Improve maintainer experience** - No Ruby version management, single binary deployment
2. **Maintain simplicity** - Content editors shouldn't need to know Rust
3. **Heroku compatibility** - Must deploy easily to Heroku
4. **Performance** - Faster page loads and lower resource usage

## Current Status

- ✅ Basic routing for home and factor pages
- ✅ Markdown rendering with Comrak
- ✅ Template system with Tera
- ✅ Static file serving
- 🚧 Blog functionality
- 🚧 Full design implementation
- 📋 Heroku deployment configuration

## For Maintainers

**You don't need to know Rust to maintain this site!**

- Content updates: Edit Markdown files in `/content/` as usual
- Template changes: Edit HTML templates in `/views/` (Tera syntax is similar to ERB)
- CSS/JS updates: Modify files in `/public/` without recompilation
- Deployment: Still just `git push heroku main` (once configured)

## Development

```bash
# Run locally (after initial setup)
cargo run

# The server runs on http://localhost:3000
```

## Why Rust?

The Ruby version has served well, but we've experienced:
- Ruby version management complexity
- Bundler dependency issues  
- Deployment environment inconsistencies
- Performance limitations with many concurrent users

Rust provides:
- Single binary with no runtime dependencies
- 10-50x performance improvement
- Tiny memory footprint
- Guaranteed memory safety
- Excellent error messages

## Architecture

The Rust version maintains the same architecture as the Ruby version:
- Content in Markdown files
- Templates for rendering
- Simple routing
- No database

The only difference is the runtime - everything else stays familiar.