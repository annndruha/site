# Tera Deep Dive - Template Engine Analysis

## What is Tera?

Tera is a template engine for Rust inspired by Jinja2 (Python) and Django templates. It's designed to be powerful, fast, and easy to use, with a focus on runtime flexibility rather than compile-time validation.

**Key Philosophy**: Templates are data-driven documents that should be editable by non-programmers, with logic kept simple and declarative.

## Architecture & Design

### Template Loading & Compilation
```rust
// Initialize Tera with glob patterns
let tera = Tera::new("templates/**/*")?;

// Or with explicit paths
let mut tera = Tera::default();
tera.add_template_file("templates/base.html", Some("base"))?;
tera.add_template_file("templates/home.html", Some("home"))?;

// Templates are parsed once at startup, then cached
```

### Runtime Process
1. **Parse**: Templates parsed into AST at startup
2. **Cache**: Parsed templates stored in memory
3. **Render**: Context data merged with template at request time
4. **Output**: HTML string returned

### Hot Reloading (Development)
```rust
let mut tera = Tera::new("templates/**/*")?;
tera.autoescape_on(vec![".html"]);

// In development, can reload templates
if cfg!(debug_assertions) {
    tera.full_reload()?;
}
```

## Syntax Deep Dive

### Variable Output
```html
<!-- ERB -->
<h1><%= page_title %></h1>
<p>Welcome, <%= user.name %>!</p>

<!-- Tera -->
<h1>{{ page_title }}</h1>
<p>Welcome, {{ user.name }}!</p>
```

### Control Structures
```html
<!-- ERB Conditionals -->
<% if user.admin? %>
  <div class="admin-panel">Admin content</div>
<% elsif user.member? %>
  <div class="member-panel">Member content</div>  
<% else %>
  <div class="guest-panel">Please log in</div>
<% end %>

<!-- Tera Conditionals -->
{% if user.admin %}
  <div class="admin-panel">Admin content</div>
{% elif user.member %}
  <div class="member-panel">Member content</div>
{% else %}
  <div class="guest-panel">Please log in</div>
{% endif %}
```

### Loops and Iteration
```html
<!-- ERB -->
<ul>
<% factors.each_with_index do |factor, index| %>
  <li class="factor-<%= index + 1 %>">
    <a href="/<%= factor.slug %>"><%= factor.title %></a>
  </li>
<% end %>
</ul>

<!-- Tera -->
<ul>
{% for factor in factors %}
  <li class="factor-{{ loop.index }}">
    <a href="/{{ factor.slug }}">{{ factor.title }}</a>
  </li>
{% endfor %}
</ul>
```

### Template Inheritance
```html
<!-- base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Default Title{% endblock title %}</title>
    {% block extra_head %}{% endblock extra_head %}
</head>
<body>
    <header>{% block header %}{% endblock header %}</header>
    <main>{% block content %}{% endblock content %}</main>
    <footer>{% block footer %}{% endblock footer %}</footer>
</body>
</html>

<!-- home.html -->
{% extends "base.html" %}

{% block title %}The Twelve-Factor App{% endblock title %}

{% block content %}
<h1>{{ intro.title }}</h1>
<div>{{ intro.content | safe }}</div>
{% endblock content %}
```

### Filters (Data Transformation)
```html
<!-- Built-in filters -->
{{ name | upper }}                    <!-- JOHN -->
{{ content | safe }}                  <!-- Render HTML without escaping -->
{{ date | date(format="%Y-%m-%d") }}  <!-- 2024-05-30 -->
{{ items | length }}                  <!-- 5 -->
{{ text | truncate(length=100) }}     <!-- Truncate to 100 chars -->

<!-- Chain filters -->
{{ user.bio | markdown | truncate(length=200) | safe }}

<!-- Custom filters -->
{{ factor_name | factor_url }}        <!-- Custom filter for URL generation -->
```

### Macros (Reusable Components)
```html
<!-- Define macro -->
{% macro render_factor(factor, current_locale) %}
<div class="factor-card">
    <h3>{{ factor.title }}</h3>
    <p>{{ factor.description }}</p>
    <a href="/{{ current_locale }}/{{ factor.slug }}">Read more</a>
</div>
{% endmacro render_factor %}

<!-- Use macro -->
{% for factor in factors %}
    {{ render_factor(factor=factor, current_locale=locale) }}
{% endfor %}
```

## Integration with Axum

### Basic Setup
```rust
use axum::{extract::State, response::Html, routing::get, Router};
use tera::{Context, Tera};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    tera: Tera,
}

async fn home(State(state): State<Arc<AppState>>) -> Result<Html<String>, String> {
    let mut context = Context::new();
    context.insert("title", "The Twelve-Factor App");
    context.insert("factors", &get_factors());
    
    match state.tera.render("home.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(e) => Err(format!("Template error: {}", e)),
    }
}

async fn main() {
    let tera = Tera::new("templates/**/*").expect("Failed to parse templates");
    let app_state = AppState { tera };
    
    let app = Router::new()
        .route("/", get(home))
        .with_state(Arc::new(app_state));
    
    // ... server setup
}
```

### Advanced Context Building
```rust
// Complex context for factor pages
async fn factor_page(
    Path(factor_slug): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, String> {
    let mut context = Context::new();
    
    // Load factor data
    let factor = load_factor(&factor_slug)?;
    context.insert("factor", &factor);
    
    // Add navigation context
    let factors = get_all_factors();
    let current_index = factors.iter().position(|f| f.slug == factor_slug);
    
    if let Some(index) = current_index {
        if index > 0 {
            context.insert("prev_factor", &factors[index - 1]);
        }
        if index < factors.len() - 1 {
            context.insert("next_factor", &factors[index + 1]);
        }
    }
    
    // Add i18n context
    context.insert("locale", "en");
    context.insert("translations", &get_translations("en"));
    
    // Render template
    state.tera.render("factor.html", &context)
        .map(Html)
        .map_err(|e| format!("Template error: {}", e))
}
```

## Custom Filters for 12factor

### Factor URL Generation
```rust
use tera::{Filter, Result as TeraResult, Value};

struct FactorUrlFilter;

impl Filter for FactorUrlFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let factor_name = value.as_str().ok_or("Expected string")?;
        let locale = args.get("locale")
            .and_then(|v| v.as_str())
            .unwrap_or("en");
        
        let url = if locale == "en" {
            format!("/{}", factor_name)
        } else {
            format!("/{}/{}", locale, factor_name)
        };
        
        Ok(Value::String(url))
    }
}

// Register the filter
tera.register_filter("factor_url", FactorUrlFilter);
```

### Markdown Processing
```rust
struct MarkdownFilter;

impl Filter for MarkdownFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
        let text = value.as_str().ok_or("Expected string")?;
        let html = markdown_to_html(text, &ComrakOptions::default());
        Ok(Value::String(html))
    }
}

// Usage in templates
{{ content | markdown | safe }}
```

## Error Handling & Debugging

### Template Error Types
```rust
// Common Tera errors and handling
match tera.render("template.html", &context) {
    Ok(html) => Ok(Html(html)),
    Err(tera::Error::TemplateNotFound { name }) => {
        Err(format!("Template '{}' not found", name))
    },
    Err(tera::Error::VariableNotFound { name }) => {
        Err(format!("Variable '{}' not found in context", name))
    },
    Err(tera::Error::FilterNotFound { name }) => {
        Err(format!("Filter '{}' not found", name))
    },
    Err(e) => Err(format!("Template error: {}", e)),
}
```

### Development Debugging
```html
<!-- Debug context in templates -->
{% if debug %}
<pre>{{ __tera_context | json_encode(pretty=true) }}</pre>
{% endif %}

<!-- Conditional debugging -->
{% if factor %}
  <p>Factor loaded: {{ factor.title }}</p>
{% else %}
  <p>ERROR: No factor in context</p>
{% endif %}
```

## Performance Characteristics

### Memory Usage
- **Template Storage**: Parsed templates cached in memory
- **Context Creation**: New context per request (lightweight)
- **Rendering**: String concatenation and replacement

### Benchmarks (Approximate)
- **Template Parse**: ~1ms for complex template (one-time)
- **Render Simple**: ~10μs (variables only)
- **Render Complex**: ~100μs (loops, conditionals, filters)
- **Memory**: ~50KB per cached template

### Optimization Strategies
```rust
// Pre-compile commonly used contexts
lazy_static! {
    static ref BASE_CONTEXT: Context = {
        let mut ctx = Context::new();
        ctx.insert("site_name", "The Twelve-Factor App");
        ctx.insert("current_year", "2024");
        ctx
    };
}

// Reuse base context
let mut context = BASE_CONTEXT.clone();
context.insert("page_specific_data", &data);
```

## Real 12factor Implementation Example

### Factor Page Template
```html
<!-- templates/factor.html -->
{% extends "base.html" %}

{% block title %}{{ factor.title }} - The Twelve-Factor App{% endblock title %}

{% block content %}
<nav class="factor-nav">
    {% if prev_factor %}
    <a href="{{ prev_factor.slug | factor_url(locale=locale) }}" class="prev">
        ← {{ prev_factor.title }}
    </a>
    {% endif %}
    
    {% if next_factor %}
    <a href="{{ next_factor.slug | factor_url(locale=locale) }}" class="next">
        {{ next_factor.title }} →
    </a>
    {% endif %}
</nav>

<article class="factor-content">
    <h1>{{ factor.number }}. {{ factor.title }}</h1>
    <h2>{{ factor.subtitle }}</h2>
    
    <div class="content">
        {{ factor.content | markdown | safe }}
    </div>
</article>

<aside class="factor-sidebar">
    <h3>All Factors</h3>
    <ol>
    {% for f in all_factors %}
        <li{% if f.slug == factor.slug %} class="current"{% endif %}>
            <a href="{{ f.slug | factor_url(locale=locale) }}">{{ f.title }}</a>
        </li>
    {% endfor %}
    </ol>
</aside>
{% endblock content %}
```

### Rust Handler
```rust
async fn factor_page(
    Path((locale, factor_slug)): Path<(String, String)>,
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, AppError> {
    // Load factor content
    let factor = state.content.get_factor(&locale, &factor_slug)
        .ok_or(AppError::NotFound)?;
    
    // Build context
    let mut context = Context::new();
    context.insert("factor", &factor);
    context.insert("locale", &locale);
    context.insert("all_factors", &state.content.get_all_factors(&locale));
    
    // Add navigation
    if let Some((prev, next)) = state.content.get_factor_navigation(&locale, &factor_slug) {
        if let Some(prev) = prev {
            context.insert("prev_factor", &prev);
        }
        if let Some(next) = next {
            context.insert("next_factor", &next);
        }
    }
    
    // Render
    state.tera.render("factor.html", &context)
        .map(Html)
        .map_err(AppError::Template)
}
```

## Development Workflow

### Hot Reload Setup
```rust
#[cfg(debug_assertions)]
async fn reload_templates(State(state): State<Arc<AppState>>) -> &'static str {
    if let Err(e) = state.tera.full_reload() {
        eprintln!("Template reload failed: {}", e);
        "Reload failed"
    } else {
        "Templates reloaded"
    }
}

// Add reload endpoint in development
#[cfg(debug_assertions)]
let app = app.route("/__reload", get(reload_templates));
```

### Asset Integration
```rust
// Integrate with Sass compilation
struct AssetContext {
    css_hash: String,
    js_hash: String,
}

let mut context = Context::new();
context.insert("assets", &AssetContext {
    css_hash: calculate_css_hash(),
    js_hash: calculate_js_hash(),
});

// In templates
<link rel="stylesheet" href="/assets/style.{{ assets.css_hash }}.css">
```

## Pros and Cons Summary

### Advantages
- ✅ **Familiar syntax** for Ruby developers
- ✅ **Hot reload** for fast development
- ✅ **Runtime flexibility** for dynamic content
- ✅ **Good error messages** with line numbers
- ✅ **Rich filter ecosystem** for data transformation
- ✅ **Template inheritance** for DRY layouts

### Disadvantages
- ❌ **Runtime overhead** (parsing + rendering per request)
- ❌ **No compile-time validation** (errors found at runtime)
- ❌ **Larger binary** (includes template engine)
- ❌ **Memory usage** (templates cached in RAM)

### Best Fit For
- Documentation sites with moderate traffic
- Projects where developer velocity matters
- Teams with Ruby/Python template experience
- Applications needing runtime template flexibility

Tera strikes a balance between power and simplicity, making it ideal for projects where maintainer experience is prioritized over maximum performance.