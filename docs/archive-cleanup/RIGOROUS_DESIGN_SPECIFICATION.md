# Rigorous Design Specification: 12factor.net Documentation-First Redesign

## 1. Design Philosophy

### 1.1 Core Principles
1. **Content Supremacy**: Every design decision must improve access to content
2. **Developer Respect**: Assume intelligence, value time, avoid persuasion
3. **Performance First**: <1s page load, <100KB CSS, zero decorative images
4. **Accessibility**: WCAG 2.1 AA compliance minimum

### 1.2 Anti-Patterns (Explicitly Forbidden)
- Marketing language of any kind
- Call-to-action buttons beyond simple navigation
- Decorative images that don't enhance comprehension
- Animations or transitions that delay content access
- Multi-level navigation for <20 pages
- Any element that requires JavaScript to access content

## 2. Information Architecture

### 2.1 Site Structure
```
12factor.net/
├── / (Home - Introduction + Factor List)
├── /[factor-name] (12 individual factor pages)
├── /community (Single page with links)
├── /blog (Optional - list only if active)
└── /[locale]/[any-above] (Internationalization)
```

### 2.2 Home Page Structure (Strict Order)
```
1. Header (50px max height)
   - Logo (text or simple SVG, <5KB)
   - Language selector (if applicable)
   - GitHub link (icon only)

2. Primary Content (above fold)
   - H1: "The Twelve-Factor App"
   - Lead paragraph (methodology introduction, <100 words)
   - Ordered list of 12 factors with:
     - Number + Name as link
     - Single-line description
     - No icons, no grid, simple list

3. Secondary Content (below fold)
   - Background (collapsible or anchor link)
   - Authors/Credits (if required)

4. Footer (100px max height)
   - Copyright
   - GitHub link (text)
   - Community link (text)
   - No CTAs, no newsletter signup
```

### 2.3 Factor Page Structure
```
1. Minimal Breadcrumb
   - "Home > [Factor Name]" only
   - No decorative separators

2. Factor Header
   - H1: "[Roman Numeral]. [Factor Name]"
   - Subtitle: One-line description
   - No hero sections, no images

3. Content
   - Markdown-rendered HTML
   - Clean typography
   - Code blocks with syntax highlighting
   - No interruptions or CTAs

4. Navigation
   - Previous factor (left aligned)
   - Next factor (right aligned)
   - Text only, no buttons
```

## 3. Visual Design Specifications

### 3.1 Typography (Strict)
```css
/* Base typography */
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 18px;
  line-height: 1.7;
  color: #24292e;
}

/* Headers - no custom fonts unless already loaded */
h1 { font-size: 2.5rem; margin-bottom: 0.5rem; }
h2 { font-size: 1.875rem; margin-top: 3rem; }
h3 { font-size: 1.5rem; margin-top: 2rem; }

/* Constrain line length for readability */
.content {
  max-width: 740px;
  margin: 0 auto;
}
```

### 3.2 Color Palette (Minimal)
```css
:root {
  --color-text: #24292e;
  --color-link: #0366d6;
  --color-link-hover: #0056b3;
  --color-border: #e1e4e8;
  --color-bg-secondary: #f6f8fa;
  --color-code-bg: #f3f4f6;
}

/* Purple accent ONLY if it passes contrast requirements */
/* --color-link: #79589F; Must be AA compliant */
```

### 3.3 Spacing System
```css
/* Consistent spacing scale */
--space-1: 0.25rem;  /* 4px */
--space-2: 0.5rem;   /* 8px */
--space-3: 1rem;     /* 16px */
--space-4: 1.5rem;   /* 24px */
--space-5: 2rem;     /* 32px */
--space-6: 3rem;     /* 48px */
--space-7: 4rem;     /* 64px */
```

### 3.4 Component Specifications

#### Navigation
```css
nav {
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-3) 0;
}

nav a {
  color: var(--color-text);
  text-decoration: none;
  padding: var(--space-2) var(--space-3);
}

/* No dropdowns, no mega-menu */
```

#### Factor List (Home)
```css
.factor-list {
  list-style: none;
  padding: 0;
}

.factor-list li {
  padding: var(--space-3) 0;
  border-bottom: 1px solid var(--color-border);
}

.factor-list a {
  font-weight: 600;
  text-decoration: none;
}

.factor-list .description {
  color: #586069;
  margin-top: var(--space-1);
}
```

## 4. Performance Requirements

### 4.1 Page Weight Budget
- HTML: <10KB per page (gzipped)
- CSS: <50KB total (gzipped)
- JavaScript: <10KB if any (gzipped)
- Images: NONE except logo
- Fonts: System fonts only OR existing BentonSans if <50KB

### 4.2 Loading Metrics
- First Contentful Paint: <1s
- Time to Interactive: <1.5s
- Lighthouse Score: >95

### 4.3 Forbidden Elements
- Background images
- Icon fonts (use inline SVG if needed)
- External trackers/analytics
- Social media widgets
- Third-party comment systems

## 5. Accessibility Requirements

### 5.1 Mandatory
- Semantic HTML5 elements
- Proper heading hierarchy
- Link text must be descriptive
- Color contrast ratio ≥4.5:1 for normal text
- Color contrast ratio ≥3:1 for large text
- Keyboard navigation for all interactive elements
- Skip to main content link

### 5.2 ARIA Usage
- Minimal ARIA - prefer semantic HTML
- Breadcrumb navigation: `aria-label="Breadcrumb"`
- Main content: `<main role="main">`
- Navigation: `<nav aria-label="Primary">`

## 6. Implementation Checklist

### 6.1 Must Remove
- [ ] All sections with class containing "cta"
- [ ] All sections with class containing "hero"
- [ ] All img tags except logo
- [ ] All marketing copy (grep for "Join", "Build", "Explore")
- [ ] All gradient CSS
- [ ] All animation/transition CSS
- [ ] Mega-menu JavaScript and HTML
- [ ] Diamond icon references
- [ ] Blog feed on home (unless specifically requested)

### 6.2 Must Implement
- [ ] Simple ordered list of factors on home
- [ ] Single-column readable content
- [ ] Consistent spacing scale
- [ ] Proper semantic markup
- [ ] Print stylesheet for factor pages
- [ ] Clear focus states for keyboard navigation

### 6.3 Must Test
- [ ] Load time <1s on 3G connection
- [ ] All content accessible without JavaScript
- [ ] Keyboard navigation works throughout
- [ ] Screen reader announces content properly
- [ ] Links work without surrounding context
- [ ] Print layout is readable

## 7. Content Guidelines

### 7.1 Language Tone
- Technical, not conversational
- Descriptive, not persuasive  
- Neutral, not enthusiastic
- Informative, not marketing

### 7.2 Forbidden Phrases
- "Join the movement"
- "Build better apps"
- "Get started today"
- "Explore our community"
- "Take action"
- Any exclamation marks

### 7.3 Acceptable CTAs
- "View on GitHub"
- "Community"
- "Next: [Factor Name]"
- "[Factor Number]. [Factor Name]"

## 8. Quality Metrics

### 8.1 Success Criteria
1. Developer can find specific factor in <5 seconds
2. Factor content loads in <1 second
3. No scrolling required to see factor list
4. URLs are clean and shareable
5. Content is primary visual element
6. Zero marketing language
7. Passes WAVE accessibility checker

### 8.2 Failure Criteria
1. Any element that delays content access
2. Any persuasive language
3. Any decorative image
4. Multi-level navigation
5. JavaScript required for content
6. Below AA accessibility rating

## 9. Reference Implementations

### 9.1 Good Examples
- **MDN Web Docs**: Clean, fast, developer-focused
- **Python Docs**: Simple, scannable, authoritative  
- **man pages**: Ultimate content-first design
- **PlainJS**: Minimal, functional, fast

### 9.2 What We're NOT Building
- Product landing pages
- Marketing websites
- SaaS application dashboards
- Corporate documentation portals

## 10. Maintenance Principles

### 10.1 Future Changes Must
- Improve content accessibility
- Reduce page weight
- Increase readability
- Enhance performance

### 10.2 Future Changes Must NOT
- Add marketing elements
- Increase visual complexity
- Require JavaScript for content
- Add decorative elements
- Introduce animations

This specification provides clear, measurable criteria for implementing a documentation-first design that serves developers effectively while maintaining modern, professional aesthetics.