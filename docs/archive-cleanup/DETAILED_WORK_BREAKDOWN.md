# Detailed Work Breakdown - Static Binary Migration

## Implementation Constraint
**Key Principle**: Only implement features that already exist in Ruby. New features (like Sass compilation) go directly to Rust - do NOT implement them in Ruby first.

## Ruby Baseline Status
- ✅ **Exists in Ruby**: ERB templates, basic CSS serving, Markdown processing, routing, i18n, TOML parsing
- ❌ **Missing in Ruby**: Sass compilation, asset bundling, hot reload, build pipeline
- 🎯 **Target**: Implement missing features directly in Rust

## 1. Asset Pipeline & Build System

### CSS/SCSS Extraction and Setup
- [ ] **Extract SCSS source files**
  - Copy from `12factor FINAL/wp-content/themes/orbit-media/resources/scss/`
  - Components: breadcrumbs, calendar, new-window-links, pagination, share, skip-to-content, tables, tabs
  - Features: forms, post-detail, post-landing, post-teaser, blog/*
  - Layout: content, footer, header, mega-menu, primary-menu
  - Pageblocks: animations, blurbs, columns, content, cta, hero, image-text, post-feed
  - Utilities: bootstrap-vars, bootstrap, buttons, mixins, overrides, typography, vars

- [ ] **Set up Sass compilation in Rust** (Ruby doesn't have this - implement directly in Rust)
  - Research: grass (Dart Sass) vs other Rust Sass compilers
  - Integrate with build process
  - Configure source maps for development
  - Set up watch mode for development

- [ ] **Font and Asset Management**
  - Copy BentonSans fonts (Book, Medium, Regular)
  - Copy Font Awesome webfonts
  - Copy Bootstrap Icons
  - Set up proper font loading strategy

- [ ] **Image Asset Organization**
  - Icons: 48 various UI icons (carets, numerals, diamonds, social)
  - Logos: Twelve-Factor branding (regular and white versions)
  - Backgrounds and patterns
  - Author images

### JavaScript Setup
- [ ] **Extract and organize JS** (Ruby serves these statically - replicate in Rust)
  - Custom scripts: accessibility.js, custom.js
  - Third-party: FullCalendar, PageBlocks
  - Bootstrap JS integration
  - jQuery dependency management

- [ ] **Build system integration** (New feature - implement in Rust only)
  - Concatenation and minification
  - Source maps for development
  - Asset fingerprinting for cache busting

## 2. Page Template Audit

### HTML Template Sources Available
- [ ] **Reference Templates** (in `12factor FINAL/`)
  - ✅ home-page.html
  - ✅ interior-factor-page.html
  - ✅ blog-landing-page.html
  - ✅ blog-detail-page.html
  - ✅ community-page.html
  - ✅ content-styles-page.html
  - Component demos: blurb-blocks, column-blocks, hero-blocks, image-text-blocks

### Ruby ERB Templates to Convert
- [ ] **Core Templates**
  - ✅ layout.erb → base layout template
  - ✅ home.erb → home page
  - ✅ factor.erb → factor pages
  - ✅ blog.erb → blog listing
  - ✅ post.erb → individual blog posts
  - ✅ community.erb → community page
  - ❓ login.erb → password protection (needed?)

- [ ] **Partials**
  - ✅ _footer.erb → footer component

### Template Conversion Tasks
- [ ] **Convert ERB syntax to Tera**
  - `<%= %>` → `{{ }}`
  - `<% %>` → `{% %}`
  - Ruby helpers → Tera filters/functions
  - I18n integration

- [ ] **Template Logic Migration**
  - Locale switching logic
  - Factor navigation (prev/next)
  - Blog post metadata rendering
  - Author bio integration

## 3. Feature Matrix by Page

### Home Page Features
- [ ] **Header/Navigation**
  - ✅ Twelve-Factor logo
  - ✅ Mega menu with factor categories
  - ✅ Language selector dropdown
  - ✅ GitHub link button
  - ✅ Mobile hamburger menu

- [ ] **Content Sections**
  - ✅ Introduction text
  - ✅ Factor list with numbering
  - ✅ Background section
  - ✅ Who section

- [ ] **Footer**
  - ✅ Social media links
  - ✅ Navigation links
  - ✅ Copyright information

### Factor Pages Features
- [ ] **Navigation**
  - ✅ Breadcrumb navigation
  - ✅ Previous/Next factor links
  - ✅ Language switcher
  - ✅ Back to home link

- [ ] **Content**
  - ✅ Factor title and subtitle
  - ✅ Markdown content rendering
  - ✅ Image support
  - ✅ Code block syntax highlighting (if any)

### Blog Landing Page Features
- [ ] **Layout**
  - ❓ Featured posts section
  - ❓ All posts grid/list
  - ❓ Pagination
  - ❓ Category filtering
  - ❓ Search functionality

### Blog Detail Page Features
- [ ] **Post Content**
  - ✅ Post title and metadata
  - ✅ Author information
  - ✅ Publication date
  - ✅ Content rendering
  - ❓ Tags/categories
  - ❓ Social sharing buttons

- [ ] **Navigation**
  - ❓ Related posts
  - ❓ Previous/Next post navigation

### Community Page Features
- [ ] **Content Sections**
  - ❓ Community information
  - ❓ Maintainer profiles
  - ❓ Calendar integration (FullCalendar)
  - ❓ Meeting information

## 4. Configuration & Data Format Analysis

### TOML Configuration Files
- [ ] **Blog Configuration** (`blog/blog.toml`)
  - Post metadata structure
  - Category definitions
  - Featured post selection
  - Publication workflow

- [ ] **Author Configuration** (`blog/authors.toml`)
  - ✅ Author profiles (brett, yehuda, brian, vish)
  - ✅ Maintainer flags
  - ✅ GitHub links
  - ✅ Bio descriptions

- [ ] **Factor Configuration** (`content/factors.toml`)
  - Factor ordering
  - Metadata per factor
  - Navigation structure

### YAML Front Matter
- [ ] **Blog Posts**
  - Title, date, author
  - Categories, tags
  - Featured status
  - Excerpt markers

- [ ] **Content Pages**
  - Multi-language metadata
  - SEO information
  - Custom fields

### Environment Configuration
- [ ] **Ruby Environment Variables**
  - PASSWORD (optional auth)
  - FORCE_SSL (security)
  - GOOGLE_TAG_MANAGER_ACCOUNT (analytics)

- [ ] **Rust Environment Mapping**
  - Configuration struct
  - Environment variable parsing
  - Default values

## 5. New Refactor Features - Design Decisions

### Mega Menu System
- [ ] **Implementation Approach**
  - ❓ Static generation vs dynamic
  - ❓ Factor categorization system
  - ❓ Multi-level navigation structure
  - ❓ Mobile responsiveness strategy

### Blog System Enhancement
- [ ] **Content Management**
  - ❓ TOML vs front matter for metadata
  - ❓ Featured post selection mechanism
  - ❓ Category/tag system design
  - ❓ Pagination strategy

### Language System Improvements
- [ ] **I18n Architecture**
  - ❓ Locale detection strategy
  - ❓ Fallback mechanisms
  - ❓ URL structure (/en/factor vs /factor?lang=en)
  - ❓ Content organization

### Performance Features
- [ ] **Optimization Strategy**
  - ❓ Static asset caching
  - ❓ Image optimization
  - ❓ Gzip compression
  - ❓ CDN integration

## 6. Implementation Sequencing

### Phase 1: Foundation (Immediate)
1. Set up Sass compilation
2. Convert base layout template
3. Fix CSS loading in Rust prototype
4. Implement basic header/footer

### Phase 2: Core Pages (Next)
1. Complete home page styling
2. Finish factor page templates
3. Implement navigation system
4. Add language switching

### Phase 3: Blog System (After Core)
1. Blog listing page
2. Blog detail template
3. Author integration
4. TOML configuration parsing

### Phase 4: Advanced Features (Final)
1. Community page
2. Calendar integration
3. Search functionality
4. Performance optimizations

## 7. Technical Risks & Dependencies

### Template Engine Decision Point
- ❓ **Tera vs Alternatives** (to be reviewed after this analysis)
  - Template complexity assessment
  - Maintainer learning curve
  - Build system integration
  - Performance considerations

### Build System Complexity
- ❓ **Asset Pipeline** 
  - Sass compilation performance
  - Asset watching and hot reload
  - Development vs production builds

### Deployment Requirements
- ❓ **Heroku Integration**
  - Rust buildpack selection
  - Asset compilation during build
  - Environment variable management
  - SSL and security configuration

## 8. Success Metrics

### Functional Parity
- [ ] All pages render identically to Ruby version
- [ ] All navigation works correctly
- [ ] All content displays properly
- [ ] All languages function correctly

### Performance Improvements
- [ ] Faster page load times
- [ ] Lower memory usage
- [ ] Reduced deployment time
- [ ] Smaller binary size

### Maintainer Experience
- [ ] Content updates require no Rust knowledge
- [ ] Template changes are straightforward
- [ ] Asset updates don't require recompilation
- [ ] Clear error messages guide to correct files

## Next Steps
1. Review template engine decision (Tera vs alternatives)
2. Begin Phase 1 implementation
3. Set up visual regression testing with screenshot tool
4. Create development workflow documentation