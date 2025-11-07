# Accessibility Guide for 12factor.net

> **TL;DR**: APCA for design decisions, WCAG 2.1 for validation. Yellow (#FDC500) needs black text or secondary indicators.

## Jump to What You Need

- [Can I use yellow for...?](#yellow-usage-quick-reference)
- [Color combinations that work](#approved-color-combinations)
- [Animation checklist](#animation-requirements)
- [Keyboard navigation patterns](#keyboard-patterns)
- [Testing requirements](#testing-checklist)

---

## Yellow Usage Quick Reference

### ✅ YES - Yellow works here

**Yellow background with black text**
```
Recipe: background:#FDC500, color:#212529, padding:12px 24px, border-radius:6px
Contrast: 9.5:1 (WCAG AAA)
```
[IMAGE PLACEHOLDER: Yellow button with black text]

**Yellow accent with dark text**
```
Recipe: border-left:3px solid #FDC500, background:#FFFBEB, color:#212529
Usage: CTA sections, important callouts
```
[IMAGE PLACEHOLDER: Yellow accent callout box]

**Yellow hover with secondary indicator**
```
Recipe: color:#6c757d → #FDC500 on hover, text-decoration:underline
Usage: Breadcrumb links, navigation
```
[IMAGE PLACEHOLDER: Link hover state comparison]

### ❌ NO - Yellow fails here

**Yellow text on white**
```
Recipe: color:#FDC500, background:#FFFFFF
Contrast: 1.94:1 (FAILS WCAG)
```
[IMAGE PLACEHOLDER: Yellow text failing contrast]

**Yellow border as sole indicator**
```
Recipe: border:2px solid #FDC500, no other changes
Problem: Insufficient contrast for UI components
```
[IMAGE PLACEHOLDER: Yellow border only example]

---

## Approved Color Combinations

### Primary Combinations

| Background | Text | Contrast | Use Case |
|------------|------|----------|----------|
| #FFFFFF | #212529 | 15.3:1 ✅ | Body text |
| #FFFFFF | #444444 | 9.73:1 ✅ | Primary content |
| #FFFFFF | #666666 | 5.74:1 ✅ | Secondary content |
| #FFFFFF | #79589F | 4.82:1 ✅ | Links |
| #FDC500 | #212529 | 9.5:1 ✅ | CTAs, buttons |
| #79589F | #FFFFFF | 4.82:1 ✅ | Purple buttons |

### Never Use

| Background | Text | Contrast | Why it fails |
|------------|------|----------|--------------|
| #FFFFFF | #FDC500 | 1.94:1 ❌ | Below all standards |
| #FDC500 | #FFFFFF | 1.94:1 ❌ | Below all standards |
| #FFFBEB | #FDC500 | 1.87:1 ❌ | Even worse on tinted bg |

---

## Animation Requirements

### Required CSS for Every Project

```css
/* Non-negotiable - must be in your CSS */
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

### Animation Performance Rules

✅ **Animate only these properties**:
- `transform` (translate, scale, rotate)
- `opacity`
- `color`
- `background-color`

❌ **Never animate these** (causes reflow/repaint):
- `width`, `height`
- `margin`, `padding`
- `top`, `left`, `right`, `bottom`
- `font-size`

### Duration Limits

| Animation Type | Maximum Duration | Example |
|----------------|------------------|---------|
| Micro-interactions | 250ms | Hover, focus |
| Page transitions | 400ms | Route changes |
| Loading animations | ∞ with controls | Must have pause |

---

## Keyboard Patterns

### Focus Indicators

**Minimum requirements**:
```css
:focus {
  outline: 2px solid #000000;
  outline-offset: 2px;
}
```

**Yellow focus (needs secondary indicator)**:
```css
:focus {
  outline: 2px solid #FDC500;  /* Low contrast */
  box-shadow: 0 0 0 4px rgba(253, 197, 0, 0.2);  /* Secondary */
  background: #FFFBEB;  /* Tertiary indicator */
}
```

### Skip Links

```html
<!-- Must be first focusable element -->
<a href="#main" class="skip-link">Skip to main content</a>

<!-- CSS -->
.skip-link {
  position: absolute;
  left: -9999px;
}
.skip-link:focus {
  left: 50%;
  transform: translateX(-50%);
  top: 10px;
  z-index: 9999;
  /* High contrast styling */
  background: #000;
  color: #FFF;
  padding: 12px 24px;
}
```

---

## Testing Checklist

### Quick Manual Tests

- [ ] **Keyboard only**: Can you reach everything with Tab?
- [ ] **Screen reader**: Do all images have alt text?
- [ ] **Zoom to 200%**: Does layout remain usable?
- [ ] **Reduce motion**: Do animations stop?
- [ ] **Color contrast**: Run axe DevTools scan

### Automated Testing

```bash
# Install
npm install --save-dev @axe-core/playwright

# Run in tests
const { injectAxe, checkA11y } = require('axe-playwright');
await injectAxe(page);
await checkA11y(page);
```

### Browser Testing

1. **Chrome DevTools**:
   - Inspect → Color picker shows contrast
   - Lighthouse → Accessibility audit

2. **Firefox**:
   - Accessibility Inspector
   - Right click → Inspect Accessibility Properties

---

## Core Principles

### "APCA for Design, WCAG 2.1 for Validation"

**Why both?**
- APCA: More accurate for human perception (especially yellow/orange)
- WCAG 2.1: Legal standard, what automated tools check
- Using both ensures good UX and compliance

### Yellow Philosophy

> Yellow (#FDC500) should enhance, not be the sole conveyor of information.

**Always provide secondary indicators**:
1. Text that meets contrast
2. Icons or shapes
3. Underlines or borders
4. Background changes

---

## Implementation Recipes

### Accessible Yellow Button

```html
<button class="btn-primary">
  Join Discussion
</button>
```

```css
.btn-primary {
  /* Yellow background, black text = 9.5:1 contrast */
  background: #FDC500;
  color: #212529;
  border: 2px solid #FDC500;
  padding: 12px 24px;
  font-weight: 600;
  
  /* Hover adds secondary indicators */
  &:hover {
    background: #FFD633;  /* Lighter yellow */
    transform: translateY(-1px);  /* Movement */
    box-shadow: 0 4px 8px rgba(253, 197, 0, 0.3);  /* Shadow */
  }
  
  /* Focus has multiple indicators */
  &:focus {
    outline: 2px solid #000;
    outline-offset: 2px;
  }
}
```

### Accessible Activity Indicator

```html
<!-- Text provides info, dot is decorative -->
<span class="activity">
  <span class="activity-dot" aria-hidden="true"></span>
  <span class="activity-text">Updated 2 days ago</span>
</span>
```

```css
.activity-dot {
  width: 6px;
  height: 6px;
  background: #FDC500;  /* Decorative, so contrast doesn't matter */
  border-radius: 50%;
}

.activity-text {
  color: #444;  /* Text has proper contrast */
}
```

---

## Quick Reference Card

```
YELLOW (#FDC500):
├── On white: 1.94:1 ❌
├── With black text: 9.5:1 ✅
└── As decoration: OK if not sole indicator

PURPLE (#79589F):
├── On white: 4.82:1 ✅
└── Good for links and headings

ANIMATION:
├── Always honor prefers-reduced-motion
├── Max 250ms for interactions
└── Only transform & opacity

TESTING:
├── Keyboard: Tab through everything
├── Contrast: Use axe DevTools
└── Motion: Check OS reduce motion
└── Zoom: Test at 200%
└── Screen reader: NVDA/JAWS/VoiceOver
```

---

*Last updated: 2025-06-04*  
*Principle: Universal design benefits everyone*