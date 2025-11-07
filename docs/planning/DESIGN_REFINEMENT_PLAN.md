# Design Refinement Plan: Documentation-First Modern Design

## Overview
Extract the positive visual elements from the new design while removing all marketing-oriented features that undermine 12factor.net's purpose as technical documentation.

## Core Principle
**Keep the polish, lose the pitch.**

## What We Keep from New Design
- Modern color palette (purple #79589F, yellow #FDC500)
- Professional BentonSans typography
- Responsive Bootstrap 5 grid system
- Clean, consistent visual language
- Improved spacing and readability

## What We Remove
- All CTA (Call-to-Action) sections
- "Conversion-focused" language
- Marketing copy ("Join the Movement", "Build Cloud Native Apps")
- Diamond icons and decorative images
- Hero sections with placeholder content
- Complex mega-menu navigation
- Gradient backgrounds and patterns
- Redundant navigation elements

## Implementation Plan

### Phase 1: Strip Problematic Elements (30 minutes)

#### 1.1 Remove All CTAs
```html
<!-- DELETE these sections from all templates -->
<section class="pageblock--oms-cta">
  <h2>Build Cloud Native Apps</h2>
  <p>Join the Twelve-Factor community...</p>
</section>
```

#### 1.2 Eliminate Marketing Language
- Replace "Join the Twelve-Factor Movement" → "The Twelve-Factor App"
- Remove "Explore Community" → Simple "Community" link
- Delete "conversion-focused" headers → Descriptive titles

#### 1.3 Remove Decorative Elements
- Delete all diamond icon sections
- Remove hero background images
- Eliminate tile pattern backgrounds
- Remove gradient overlays

### Phase 2: Restructure for Developer UX (2 hours)

#### 2.1 Home Page Redesign
```
┌─────────────────────────────────────┐
│ Header (minimal)                    │
├─────────────────────────────────────┤
│ The Twelve-Factor App               │
│ [Introduction paragraph]            │
├─────────────────────────────────────┤
│ I. Codebase                        │
│ One codebase tracked in revision... │
│                                     │
│ II. Dependencies                    │
│ Explicitly declare and isolate...   │
│                                     │
│ [... all 12 factors visible]        │
├─────────────────────────────────────┤
│ Footer (minimal)                    │
└─────────────────────────────────────┘
```

#### 2.2 Factor Page Simplification
```
┌─────────────────────────────────────┐
│ Home > Codebase                     │
├─────────────────────────────────────┤
│ I. Codebase                         │
│ One codebase tracked in revision... │
├─────────────────────────────────────┤
│ [Factor content]                    │
│                                     │
│ [Clean, readable text]              │
│                                     │
├─────────────────────────────────────┤
│ ← Previous | Next →                 │
└─────────────────────────────────────┘
```

#### 2.3 Navigation Streamlining
- Simple horizontal navigation bar
- No mega-menu dropdowns
- No redundant left sidebar
- Clear, minimal breadcrumbs

### Phase 3: Retain Visual Improvements (1 hour)

#### 3.1 Typography Guidelines
```scss
// Keep modern font stack
$font-family-sans: 'BentonSans', -apple-system, BlinkMacSystemFont, sans-serif;

// Maintain good readability
body {
  font-size: 18px;
  line-height: 1.6;
  color: #333;
}

h1, h2, h3 {
  font-family: $font-family-sans;
  font-weight: 500;
}
```

#### 3.2 Color Usage
```scss
// Primary accent - use sparingly
$primary: #79589F;  // Links, important headers

// Secondary - minimal usage
$secondary: #FDC500;  // Hover states only

// Clean backgrounds
$bg-white: #FFFFFF;
$bg-light: #F8F8F8;  // Subtle sections

// No gradients, no complex backgrounds
```

#### 3.3 Layout Principles
- Maximum content width: 800px (optimal reading)
- Generous whitespace
- No decorative containers
- Focus on content hierarchy

## File-by-File Changes

### Templates to Modify

#### `home.html`
- Remove hero section
- Delete diamond icons section
- Show factors list immediately
- Remove blog feed section
- Delete all CTAs

#### `factor.html`
- Simplify breadcrumbs
- Remove numeral icon
- Delete left sidebar
- Remove bottom CTA
- Simplify prev/next navigation

#### `base.html`
- Simplify header navigation
- Remove mega-menu code
- Clean up footer
- Remove marketing scripts

### SCSS Files to Clean

#### `layout.scss`
- Remove gradient mixins
- Delete hero styles
- Remove CTA styles
- Simplify navigation styles

#### `pageblocks/`
- Delete `_cta.scss`
- Remove `_hero.scss` backgrounds
- Simplify `_content.scss`

## Success Criteria

### User Experience
- [ ] Factors visible on home page without scrolling
- [ ] Clean URLs suitable for documentation references
- [ ] Fast page loads (<1 second)
- [ ] No marketing language anywhere
- [ ] Professional, modern appearance

### Technical
- [ ] Reduced CSS file size by >50%
- [ ] Removed all decorative images
- [ ] Simplified HTML structure
- [ ] Maintained responsive design
- [ ] Improved accessibility

### Content First
- [ ] Introduction immediately visible
- [ ] All 12 factors listed clearly
- [ ] No distractions from core content
- [ ] Easy to bookmark/share specific sections

## Timeline

1. **Immediate (30 min)**: Remove CTAs and marketing copy
2. **Short-term (2 hours)**: Restructure templates
3. **Polish (1 hour)**: Fine-tune typography and spacing

Total estimated time: 3.5 hours

## Expected Outcome

A modern, professional design that:
- Respects developers' time and intelligence
- Provides quick access to information
- Maintains visual consistency
- Loads fast and works everywhere
- Serves as authoritative documentation

## Alternative References

Good documentation design examples:
- MDN Web Docs - Clean, technical, fast
- Rust Book - Simple, readable, focused
- Go Documentation - Minimal, scannable
- Vue.js Docs - Modern but content-first

## Next Steps

1. Get approval for this plan
2. Create backup of current templates
3. Implement Phase 1 (remove problematic elements)
4. Test simplified design
5. Implement Phase 2 (restructure)
6. Fine-tune Phase 3 (visual polish)
7. Document final design decisions

This balanced approach preserves the visual improvements while creating a design that actually serves the needs of developers using 12factor.net as reference documentation.