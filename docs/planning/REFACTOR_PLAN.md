# 12Factor Redesign Refactor Plan

## Current Status
The refactor is approximately 60% complete. The core structure is in place but needs cleanup and completion.

## Phase 1: Asset Cleanup (Priority: High)
- [ ] Consolidate duplicate asset directories
  - Move all assets from `public/resources/` to `public/assets/`
  - Remove `12factor FINAL/` after confirming all assets are migrated
  - Update all template references to use consistent paths
- [ ] Set up SCSS compilation
  - Move SCSS files to a proper source directory
  - Add a simple build script or use Ruby-based Sass compilation
  - Generate minified CSS files

## Phase 2: Fix Critical Functionality (Priority: High)
- [ ] Fix JavaScript loading
  - Update all script tags to reference correct paths
  - Remove WordPress-specific scripts
  - Ensure Bootstrap and custom JS load properly
- [ ] Complete mega menu
  - Generate factor links dynamically from TOC constant
  - Add proper active states
  - Fix mobile menu functionality
- [ ] Fix broken links
  - Update hardcoded href="#" links
  - Ensure all factor pages load correctly
  - Fix language switcher URLs

## Phase 3: Complete Blog Integration (Priority: Medium)
- [ ] Finish blog listing page
  - Implement pagination
  - Add category filtering
  - Style blog cards to match design
- [ ] Complete blog post template
  - Add author bio section
  - Implement related posts
  - Add social sharing buttons
- [ ] RSS feed generation

## Phase 4: Polish and Optimization (Priority: Low)
- [ ] Implement proper caching headers
- [ ] Add favicon and meta tags
- [ ] Optimize images
- [ ] Add 404 page
- [ ] Test all language versions
- [ ] Add analytics integration

## Technical Debt to Address
1. Remove old CSS files (`public/css/screen.css`, `public/css/mobile.css`)
2. Document the new asset pipeline
3. Add development instructions for SCSS compilation
4. Create a deployment checklist

## Immediate Next Steps
1. Start with Phase 1 - consolidate assets
2. Fix JavaScript paths to get interactive features working
3. Test each factor page to ensure content renders correctly

## Phase 5: Rust Migration (Priority: Future)
After completing the Ruby refactor and fully understanding the codebase:
- [ ] Document all application behavior and requirements
- [ ] Choose appropriate Rust web framework (Axum/Actix-web)
- [ ] Select templating engine (Tera/Askama/Maud)
- [ ] Implement core routing and content serving
- [ ] Port Markdown processing and front matter parsing
- [ ] Implement i18n/l10n support
- [ ] Create single binary deployment
- [ ] Configure Heroku deployment with Rust buildpack

### Benefits of Rust Migration
- Single binary deployment (no Ruby version management)
- Better performance for static content serving
- Simplified deployment to various platforms
- No runtime dependencies
- Type safety for content structure

### Maintainer Experience Goals
The Rust version MUST improve maintainability for Ruby developers:
- **No Rust knowledge needed for common tasks**:
  - Content updates remain in Markdown files
  - Templates use familiar HTML/Tera syntax (similar to ERB)
  - CSS/JS changes don't require recompilation
  - Clear error messages guide to the right files
- **Easier deployment**: Single binary vs complex Ruby environment
- **Better ergonomics**: No more Ruby version issues, bundler problems, or dependency conflicts
- **Clear documentation**: Step-by-step guides for all common maintenance tasks

### Heroku Deployment Requirements
- [ ] Research and test Rust buildpack options
- [ ] Use inline buildpack configuration in app.json
- [ ] Ensure deployment remains simple (`git push heroku main`)
- [ ] Document rollback procedures
- [ ] Test on heroku-24 stack for future compatibility

### Prerequisites
- Complete current Ruby refactor first
- Document all edge cases and behaviors
- Create comprehensive test cases
- Map out all content transformations
- Write migration guide for maintainers

## Estimated Timeline
- Phase 1: 2-3 hours
- Phase 2: 3-4 hours  
- Phase 3: 2-3 hours
- Phase 4: 2 hours
- Phase 5: 15-20 hours (after other phases complete)

Total: ~10-12 hours for Ruby refactor, then 15-20 hours for Rust migration