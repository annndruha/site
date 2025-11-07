# Template Engine Evaluation for Rust Migration

## Current Requirements (Based on Detailed Analysis)

### Template Complexity Assessment
- **7 ERB templates** to convert: layout, home, factor, blog, post, community, login
- **Complex logic patterns**:
  - I18n integration (`I18n.t(:translation)`)
  - Conditional rendering (`<% if ENV['GOOGLE_TAG_MANAGER_ACCOUNT'] %>`)
  - Dynamic navigation (prev/next factors, language switching)
  - Blog metadata rendering (author bios, publication dates)
  - Partial includes (`<%= partial %>`)

### Ruby ERB Patterns We Need to Support
```erb
<!-- I18n -->
<title>The Twelve-Factor App <%= I18n.t(:translation) %></title>

<!-- Environment variables -->
<% if ENV['GOOGLE_TAG_MANAGER_ACCOUNT'] %>
  <!-- Google Tag Manager -->
<% end %>

<!-- Dynamic URLs -->
<link rel="canonical" href="<%= request.url %>">
<%= alternate_links %>

<!-- Content rendering -->
<%= yield %>

<!-- Iteration -->
<% factors.each do |factor| %>
  <li><%= factor.name %></li>
<% end %>
```

## Template Engine Options

### 1. Tera (Current Choice)
**What it is**: Jinja2-inspired template engine

**Syntax Comparison**:
```html
<!-- ERB -->
<title>The Twelve-Factor App <%= I18n.t(:translation) %></title>
<% if logged_in? %>
  <p>Welcome!</p>
<% end %>

<!-- Tera -->
<title>The Twelve-Factor App {{ translation }}</title>
{% if logged_in %}
  <p>Welcome!</p>
{% endif %}
```

**Pros**:
- ✅ Very similar to ERB syntax
- ✅ Runtime template compilation (easier development)
- ✅ Good error messages with line numbers
- ✅ Template inheritance and macros
- ✅ Built-in filters and functions
- ✅ Template hot-reloading in development

**Cons**:
- ❌ Runtime overhead (templates parsed at request time)
- ❌ Larger binary size (template engine + templates)
- ❌ No compile-time template validation
- ❌ Potential runtime template errors

**Migration Effort**: **Low** - Very straightforward ERB → Tera conversion

### 2. Askama
**What it is**: Compile-time template engine with Jinja2 syntax

**Syntax Comparison**:
```rust
// Template file: home.html
<title>The Twelve-Factor App {{ translation }}</title>
{% if logged_in %}
  <p>Welcome!</p>
{% endif %}

// Rust code:
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    translation: String,
    logged_in: bool,
}
```

**Pros**:
- ✅ Compile-time template validation (catch errors at build)
- ✅ Zero runtime overhead (templates compiled to Rust code)
- ✅ Smaller binary size (no template parser)
- ✅ Type safety (template variables must match struct fields)
- ✅ Excellent performance
- ✅ Similar syntax to ERB/Tera

**Cons**:
- ❌ Requires Rust structs for every template
- ❌ No hot-reloading (must recompile to see changes)
- ❌ More complex for dynamic data
- ❌ Maintainers need to understand Rust structs

**Migration Effort**: **Medium** - Need to create Rust structs for all templates

### 3. Maud
**What it is**: Rust macro for writing HTML

**Syntax Comparison**:
```rust
// ERB
<div class="container">
  <h1><%= title %></h1>
  <% if show_content %>
    <p><%= content %></p>
  <% end %>
</div>

// Maud
html! {
    div.container {
        h1 { (title) }
        @if show_content {
            p { (content) }
        }
    }
}
```

**Pros**:
- ✅ Compile-time validation and performance
- ✅ Type safety throughout
- ✅ No separate template files
- ✅ Excellent IDE support
- ✅ Very fast compilation and runtime

**Cons**:
- ❌ Completely different syntax from ERB
- ❌ Requires Rust knowledge to edit templates
- ❌ No designer-friendly HTML files
- ❌ Large migration effort

**Migration Effort**: **High** - Complete rewrite of all templates

### 4. Handlebars
**What it is**: Logic-less template engine

**Syntax Comparison**:
```html
<!-- ERB -->
<% if user.admin? %>
  <p>Admin content</p>
<% else %>
  <p>Regular content</p>
<% end %>

<!-- Handlebars -->
{{#if user.admin}}
  <p>Admin content</p>
{{else}}
  <p>Regular content</p>
{{/if}}
```

**Pros**:
- ✅ Simple, logic-less design
- ✅ Runtime template compilation
- ✅ JSON data integration
- ✅ Hot-reloading possible

**Cons**:
- ❌ Limited logic capabilities
- ❌ Different syntax from ERB
- ❌ May require preprocessing for complex data
- ❌ Runtime overhead

**Migration Effort**: **Medium** - Syntax differences and logic limitations

## Build System Integration Analysis

### Development Workflow Requirements
1. **Hot reloading** - Template changes visible without restart
2. **Asset watching** - Sass/JS recompilation on changes
3. **Error reporting** - Clear messages pointing to template files
4. **Source maps** - Debug asset compilation issues

### Integration Complexity

**Tera + Asset Pipeline**:
```rust
// Development server with hot reload
let tera = Tera::new("templates/**/*")?;
// Sass compilation
let sass_output = grass::from_path("styles.scss")?;
// Template rendering
let html = tera.render("home.html", &context)?;
```

**Askama + Asset Pipeline**:
```rust
// Compile-time templates
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate { /* fields */ }

// Build script compiles both templates and assets
fn main() {
    compile_templates();
    compile_sass();
}
```

## Performance Analysis

### Runtime Performance
- **Maud**: Fastest (compile-time, no parsing)
- **Askama**: Fast (compile-time, minimal overhead)
- **Tera**: Moderate (runtime parsing, caching)
- **Handlebars**: Moderate (runtime parsing)

### Binary Size Impact
- **Maud**: Smallest (templates compiled to code)
- **Askama**: Small (no template parser)
- **Tera**: Larger (template engine + templates)
- **Handlebars**: Larger (template engine + templates)

### Development Speed
- **Tera**: Fastest iteration (hot reload)
- **Handlebars**: Fast iteration (hot reload)
- **Askama**: Slower (recompile for changes)
- **Maud**: Slower (recompile for changes)

## Maintainer Experience Analysis

### For Ruby Developers

**Template Editing Difficulty**:
1. **Tera**: Very easy (minimal syntax differences)
2. **Handlebars**: Easy (different but simple syntax)
3. **Askama**: Moderate (need to understand struct fields)
4. **Maud**: Hard (requires Rust knowledge)

**Error Understanding**:
1. **Tera**: Good (template line numbers, runtime errors)
2. **Askama**: Excellent (compile-time errors with locations)
3. **Handlebars**: Good (runtime errors)
4. **Maud**: Excellent (compile-time Rust errors)

## Recommendation Matrix

| Criterion | Tera | Askama | Maud | Handlebars |
|-----------|------|--------|------|------------|
| **Migration Effort** | 🟢 Low | 🟡 Medium | 🔴 High | 🟡 Medium |
| **Maintainer Friendly** | 🟢 High | 🟡 Medium | 🔴 Low | 🟢 High |
| **Performance** | 🟡 Good | 🟢 Excellent | 🟢 Excellent | 🟡 Good |
| **Development Speed** | 🟢 Fast | 🟡 Moderate | 🟡 Moderate | 🟢 Fast |
| **Error Handling** | 🟡 Runtime | 🟢 Compile-time | 🟢 Compile-time | 🟡 Runtime |
| **Binary Size** | 🟡 Larger | 🟢 Small | 🟢 Smallest | 🟡 Larger |

## Final Recommendation

### Primary Choice: **Tera**
**Rationale**:
- Lowest migration effort (critical for project success)
- Highest maintainer acceptance (Ruby developers can edit easily)
- Good development workflow (hot reload)
- Acceptable performance for this use case
- Runtime overhead minimal for low-traffic documentation site

### Alternative: **Askama** (If performance is critical)
**Use if**:
- Performance benchmarks show significant issues with Tera
- Team is willing to accept higher maintenance complexity
- Binary size is a major concern

### Not Recommended:
- **Maud**: Too alien for Ruby maintainers
- **Handlebars**: Offers no advantages over Tera for this use case

## Implementation Plan

### Phase 1: Prototype with Tera
1. Convert one template (home.html) to validate approach
2. Implement asset pipeline integration
3. Test hot reload workflow
4. Measure performance

### Phase 2: Full Implementation
1. Convert all ERB templates to Tera
2. Implement i18n integration
3. Add error handling and debugging
4. Create maintainer documentation

### Phase 3: Optimization (Optional)
1. Performance benchmarking
2. Consider Askama migration if needed
3. Asset optimization
4. Caching strategies

The choice prioritizes **project success** (low migration effort, maintainer acceptance) over **technical perfection** (compile-time validation, maximum performance).