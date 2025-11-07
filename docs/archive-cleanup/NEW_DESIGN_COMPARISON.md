# New Design Implementation Guide

## Overview
The "12factor FINAL" directory contains the new UI/UX design mockups that should be implemented in both Ruby and Rust versions. These designs represent a significant visual refresh of the twelve-factor website.

## Design Files Access
A Python HTTP server is running on port 8000 to serve the design files:
- Base URL: http://localhost:8000/

## Key Design Pages

### 1. Home Page
- **Design**: http://localhost:8000/home-page.html
- **Current Ruby**: http://localhost:4567/
- **Current Rust**: http://localhost:3001/
- **Key Features**:
  - Modern hero section with gradient background
  - Diamond icons for each factor
  - Clean typography with BentonSans font
  - Purple/yellow color scheme

### 2. Factor Pages (Interior)
- **Design**: http://localhost:8000/interior-factor-page.html
- **Current Ruby**: http://localhost:4567/codebase
- **Current Rust**: http://localhost:3001/codebase
- **Key Features**:
  - Breadcrumb navigation
  - Large factor title with numeral
  - Clean content area
  - Previous/Next navigation

### 3. Blog Landing
- **Design**: http://localhost:8000/blog-landing-page.html
- **Current Ruby**: http://localhost:4567/blog
- **Current Rust**: http://localhost:3001/blog
- **Key Features**:
  - Featured posts section
  - Grid layout for all posts
  - Author images and metadata
  - Post teasers with images

### 4. Blog Detail
- **Design**: http://localhost:8000/blog-detail-page.html
- **Current Ruby**: http://localhost:4567/blog/open-source-announcement
- **Current Rust**: http://localhost:3001/blog/open-source-announcement
- **Key Features**:
  - Hero section with title
  - Author bio section
  - Clean typography for content
  - Related posts (if applicable)

### 5. Community Page
- **Design**: http://localhost:8000/community-page.html
- **Current Ruby**: http://localhost:4567/community
- **Current Rust**: http://localhost:3001/community
- **Key Features**:
  - Calendar integration
  - Maintainer profiles with images
  - Meeting information
  - Community links

## Design Assets

### SCSS Structure
The new design's SCSS files are already copied to:
- `/rust_prototype/assets/scss/`

Key directories:
- `layout/` - Header, footer, mega-menu
- `pageblocks/` - Hero, content blocks, CTAs
- `features/` - Blog, factor pages specific styles
- `components/` - Breadcrumbs, tabs, etc.

### Images and Icons
- Diamond icons: `/wp-content/themes/orbit-media/resources/images/Icons/icon-diamond-*.svg`
- Numeral icons: `/wp-content/themes/orbit-media/resources/images/Icons/icon-numeral-*.svg`
- Logos: `/wp-content/themes/orbit-media/resources/images/Logos/`
- Patterns: `tile-pattern-light.jpg`, `tile-pattern-transparent.png`

### Fonts
- BentonSans-Book.otf
- BentonSans-Medium.otf
- BentonSans-Regular.otf

## Implementation Strategy

### Phase 1: Visual Alignment
1. Update base template with new header/footer design
2. Implement mega-menu navigation
3. Apply new typography and color scheme
4. Add background patterns and gradients

### Phase 2: Home Page Redesign
1. Implement hero section with gradient
2. Add diamond icons to factor list
3. Update content sections with new styling
4. Apply responsive grid layouts

### Phase 3: Factor Pages Update
1. Add breadcrumb navigation
2. Implement large numeral design
3. Update prev/next navigation styling
4. Apply new content typography

### Phase 4: Blog Redesign
1. Implement featured posts section
2. Update post teaser cards
3. Add author images to posts
4. Implement blog detail hero section

### Phase 5: Community Page
1. Update maintainer profile cards
2. Implement calendar section (if needed)
3. Apply new layout structure

## Color Palette
- Primary Purple: (check design)
- Secondary Yellow: (check design)
- Dark backgrounds
- Light gray sections

## Typography
- Headers: BentonSans Medium
- Body: BentonSans Book
- Code: Monospace (unchanged)

## Next Steps
1. Take screenshots of each design page for reference
2. Create detailed comparison between current and target designs
3. Prioritize which design elements to implement first
4. Update templates incrementally to match new design

## Notes
- The new design is significantly more modern and polished
- Maintains the core twelve-factor content structure
- Adds visual interest with icons, gradients, and patterns
- Improves readability with better typography and spacing