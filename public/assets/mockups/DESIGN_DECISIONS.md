# Design Decisions for 12factor.net

This document captures all design decisions made during the 12factor.net redesign process. It serves as a reference for implementation and future design work.

## Design Principles

### Core Values
- **Minimal but thoughtful** - Every element should have purpose
- **Documentation clarity first** - Never sacrifice readability for aesthetics  
- **Living document feel** - Show activity and freshness where appropriate
- **Nice design elements** - Small touches of yellow and polish (per Vish's feedback)
- **Consistency across pages** - Unified experience throughout

## Color System

### Primary Colors
- **Yellow (#FDC500)** - Primary accent for importance, activity, and visual polish
- **Purple (#79589F)** - Secondary accent for Heroku brand connection and hover states
- **Gray (#444)** - Text color for high contrast (9.73:1 ratio)
- **Medium Gray (#666)** - Secondary text (5.74:1 ratio) 

### Color Usage Guidelines
1. **Yellow** is used for:
   - Header/footer borders (3px)
   - Hover borders on cards
   - Activity indicators (dots)
   - External link icons
   - Current/active states

2. **Purple** is used for:
   - Text color on hover
   - Heroku brand references
   - Interactive feedback

3. **Contrast Requirements**:
   - All text must meet WCAG AA (4.5:1 minimum)
   - Never use opacity on text colors
   - Activity indicators use color shifts, not opacity

## Page-Specific Decisions

### Homepage
- **Factor cards**: Include numeral icons with yellow border on hover
- **Activity indicators**: Option C (Fading) - Shows time stamps that fade after 7 days
  - 0-3 days: #444 text with yellow dot
  - 4-7 days: #666 text with faded dot  
  - 8+ days: Hidden completely
- **Grid layout**: Responsive with clear visual hierarchy

### Factor Pages  
- **Decision**: Selective Yellow (Option B) - *Updated 2025-06-04*
- **Rationale**: Balances brand connection with documentation focus
- **Implementation**: 
  - Yellow accents only on interactive elements (CTAs, hover states)
  - Breadcrumb links turn yellow on hover
  - "Help evolve this factor" section with yellow button
  - Content links remain purple for consistency
  - Black headings (not purple) to maintain readability
  - Sidebar active state: light gray with yellow left border
- **Key principle**: Yellow guides action, not reading

### Blog Pages
- **Decision**: Minimal styling aligned with homepage values
- **Implementation**: Card-based layout with subtle yellow accents
- **Features**: Show dates/authors for "living document" feel

### Community Page
- **Current state**: May need purple hero toned down
- **Goal**: Align with minimal aesthetic while keeping activity elements

## Accessibility Standards

See [Accessibility Guide](/docs/ACCESSIBILITY_GUIDE.md) for comprehensive standards including:
- Contrast requirements and color usage
- Animation guidelines
- Keyboard navigation patterns
- Testing procedures

### Key Principle for This Project
**"APCA for design, WCAG 2.1 for validation"** - Use perceptual accuracy during design, validate against legal standards.

## Component Patterns

### Navigation
- Yellow external link indicators
- Purple text on hover
- Smooth transitions (150ms)

### Cards
- 2px transparent border default
- Yellow border on hover
- Subtle shadow on hover
- Transform: translateY(-2px)

### Buttons
- Export button: Green (#0cce6b) for actions
- Yellow for current/selected states
- Always include hover states

## Implementation Notes

### CSS Architecture
- Use CSS custom properties for colors
- Maintain consistent spacing scale
- Mobile-first responsive approach

### JavaScript Enhancements
- Progressive enhancement only
- Activity data fetched async
- Graceful fallbacks

### Performance
- Minimize reflows/repaints
- Use transform for animations
- Lazy load where appropriate

## Mockup System

### File Organization
- Single `review.html` for all design reviews
- No separate mockup files - everything in one place
- Export decisions to formatted prompts

### Review Workflow
1. Visual options presented in mockup
2. User selects preferred option
3. Export decision to Claude Code
4. Implementation proceeds

### Mockup Tool Design

The unified mockup tool itself follows these design principles:

#### Layout Structure
- **Split view**: 280px sidebar + flexible main content
- **Fixed header**: With 2px yellow bottom border + preview URL
- **No double scrolling**: Only main content scrolls
- **Preview URL**: Always visible in header, yellow link color for consistency

#### Visual Hierarchy
- **Yellow (#FDC500)**: Current/active states, selections, emphasis
- **Green (#0cce6b)**: Actions (export button), success states
- **Gray scale**: Background (#f5f5f5), borders (#e0e0e0), text (#333, #666)

#### Option Card Alignment (Modern CSS Approach)
- **Vertical alignment principle**: All option cards maintain perfect section alignment using CSS Subgrid
- **CSS Grid with Subgrid**: Parent grid defines rows, children use `grid-template-rows: subgrid`
- **No fixed heights needed**: Subgrid automatically aligns sections across all cards
- **Fallback approach**: Table-based layout when Subgrid support is limited

#### Header Elements
- **Title**: "12factor Design Review" - clear purpose
- **Preview URL**: Right-aligned, yellow link color, opens in new tab
- **Minimal height**: Keeps vertical space for content
- **Responsive**: URL hides on mobile to save space
- **Container Queries**: Component-based responsive design (not viewport-based)
- **Rationale**: Native CSS solution eliminates JavaScript and fixed heights
- **Mobile behavior**: Container query at 900px switches to single column
- **Browser support**: 75% global (all modern browsers) with graceful fallback
- **Implementation**: 
  ```css
  .options-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: auto auto auto auto; /* Header + 3 sections */
  }
  .option-card {
    display: grid;
    grid-template-rows: subgrid;
    grid-row: span 4;
  }
  ```

#### Interactive Elements
- **Status indicators**: ✓ completed, ⏳ pending, 🎯 current (moved inside headers)
- **Selection pattern**: Radio buttons with yellow highlight when selected
- **Export button**: Green with hover state, copies to clipboard
- **Visual feedback**: 3-second success message after export

#### Typography
- **Base size**: 14px body, 0.85rem lists, 0.9rem content
- **Headers**: 1.3rem main title, 1.2rem question headers
- **Font stack**: system-ui, -apple-system, sans-serif

#### Accessibility
- **ARIA labels**: Selection indicators marked as radio buttons
- **Color contrast**: All text meets WCAG AA standards
- **Keyboard support**: Tab navigation through options

### Change Log
- 2025-06-03: Initial unified mockup design created
- 2025-06-03: Moved bullseye emoji inside question box header
- 2025-06-03: Fixed Option C wrapping and restored vertical alignment principle
- 2025-06-06: Implemented community page minimal redesign

## Community Page Design (June 6, 2025)

### Context
The enhanced community page design had critical accessibility failures:
- Yellow buttons on purple background (2.3:1 contrast - FAILS WCAG)
- White outline buttons on purple gradient (~3:1 - borderline)
- Glassmorphism effects creating unpredictable contrast
- Broken legacy sections at the bottom
- Inconsistent with our minimal design approach

### Decision: Minimal Redesign
Implemented Option C - Minimal approach:
- **White hero section** with yellow bottom border (clean, accessible)
- **Yellow primary button** with black text (9.5:1 contrast)
- **Purple outline secondary button** (4.82:1 contrast)
- **Clean stat cards** without glassmorphism
- **Removal of broken legacy sections**
- **Consistent hover states** with yellow accents

### Rationale
- Resolves all accessibility violations
- Maintains brand consistency with minimal approach
- Reduces visual complexity for better focus
- Aligns with documentation-first philosophy
- Keeps yellow as accent rather than dominant color

### Implementation
- Created `_community-minimal.scss` to replace enhanced version
- Reused existing HTML structure for easy rollback
- Maintained all functionality while improving accessibility

## Future Considerations

### Pending Decisions
- Blog page design approach (rich vs minimal)
- Blog post detail page styling
- Search functionality design

### Maintenance
- Keep this document updated with new decisions
- Reference from CLAUDE.md for consistency
- Use for onboarding new contributors