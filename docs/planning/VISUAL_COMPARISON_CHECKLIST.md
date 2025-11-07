# Visual Comparison Checklist

## How to Use
Open these URLs side-by-side in your browser:
- **Design**: http://localhost:8000/[page].html
- **Rust Implementation**: http://localhost:3002/[route]

## Home Page Comparison
- **Design**: http://localhost:8000/home-page.html
- **Implementation**: http://localhost:3002/

### Check These Elements:
- [ ] Purple hero section with gradient
- [ ] "Welcome to Nushell" heading (should be "The Twelve-Factor App")
- [ ] 5 diamond icons in introduction
- [ ] 12 factor grid with numeral icons (01-12)
- [ ] Each factor has hover effect
- [ ] Community section with correct styling
- [ ] Blog feed section with 3 featured posts
- [ ] Footer with correct purple background

### Known Issues:
- Text content may differ (using actual factor content vs lorem ipsum)

## Factor Page Comparison (Use Codebase as Example)
- **Design**: http://localhost:8000/interior-factor-page.html
- **Implementation**: http://localhost:3002/codebase

### Check These Elements:
- [ ] Breadcrumb: "App Methodology /"
- [ ] Large factor title
- [ ] Subtitle text
- [ ] Large numeral icon on right (01 for codebase)
- [ ] Left sidebar with all 12 factors
- [ ] Active factor highlighted in sidebar
- [ ] Main content area with proper typography
- [ ] Previous/Next navigation at bottom
- [ ] CTA section before footer

## Blog Listing Comparison
- **Design**: http://localhost:8000/blog-landing-page.html
- **Implementation**: http://localhost:3002/blog

### Check These Elements:
- [ ] Hero section with "Blog" title
- [ ] Featured Posts section with larger cards
- [ ] All Posts section with grid layout
- [ ] Post cards have images (if available)
- [ ] Author avatars on posts
- [ ] Proper spacing and grid alignment

## Blog Detail Comparison
- **Design**: http://localhost:8000/blog-detail-page.html
- **Implementation**: http://localhost:3002/blog/open-source-announcement

### Check These Elements:
- [ ] Hero with post title
- [ ] Author and date information
- [ ] Clean typography for content
- [ ] Author bio section (if data available)
- [ ] Back to Blog navigation

## Community Page Comparison
- **Design**: http://localhost:8000/community-page.html
- **Implementation**: http://localhost:3002/community

### Check These Elements:
- [ ] Page layout matches design
- [ ] Maintainer information displayed
- [ ] Meeting schedule (if implemented)
- [ ] Proper styling and spacing

## Global Elements to Verify

### Header/Navigation
- [ ] Logo switches between black and white versions
- [ ] Mega menu dropdown works
- [ ] Mobile menu button visible on small screens
- [ ] Language selector shows "EN"

### Typography
- [ ] BentonSans font loading correctly
- [ ] Heading sizes match design
- [ ] Proper line height and spacing

### Colors
- [ ] Primary purple: #79589F
- [ ] Dark purple: #430098  
- [ ] Yellow: #FDC500
- [ ] Proper contrast ratios

### Responsive Behavior
- [ ] Test at mobile width (< 768px)
- [ ] Test at tablet width (768px - 1024px)
- [ ] Test at desktop width (> 1024px)

## Quick Visual Test Commands
```bash
# Check if key CSS files are loaded
curl -s http://localhost:3002/ | grep -o 'href="[^"]*\.css"' | sort -u

# Check if fonts are referenced
curl -s http://localhost:3002/assets/css/layout.css | grep -i "benton"

# Check for responsive meta tag
curl -s http://localhost:3002/ | grep "viewport"
```

## Priority Fixes
After comparison, list any critical visual differences here:
1. 
2. 
3.