use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(alias = "github")]
    pub github_username: Option<String>,
    pub maintainer: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub author_id: String,
    pub author: Option<Author>,
    pub featured: bool,
    pub content_html: String,
    pub excerpt: String,
    pub image_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlogConfig {
    posts: Vec<BlogPostConfig>,
}

#[derive(Debug, Deserialize)]
struct BlogPostConfig {
    slug: String,
}

#[derive(Debug, Deserialize)]
struct PostFrontMatter {
    title: String,
    date: String,
    author: String,
    image: Option<String>,
}

pub struct BlogData {
    authors: HashMap<String, Author>,
    posts: Vec<BlogPost>,
    featured_posts: Vec<BlogPost>,
}

impl BlogData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Load authors
        let authors_content = fs::read_to_string("blog/authors.toml")?;
        let authors: HashMap<String, Author> = toml::from_str(&authors_content)?;

        // Load blog config for featured posts
        let config_content = fs::read_to_string("blog/blog.toml")?;
        let config: BlogConfig = toml::from_str(&config_content)?;
        let featured_slugs: Vec<String> = config.posts.iter().take(3).map(|p| p.slug.clone()).collect();

        // Load all blog posts
        let mut posts = Vec::new();
        let blog_dir = Path::new("blog");
        
        for entry in fs::read_dir(blog_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    // Skip special files
                    if filename == "blog" || filename == "community" || filename == "maintainers" || filename == "posts" {
                        continue;
                    }
                    
                    let content = fs::read_to_string(&path)?;
                    
                    // Check if file has front matter (starts with ---)
                    if content.trim().starts_with("---") {
                        // Parse front matter manually
                        if let Some((front_matter_yaml, body)) = parse_front_matter(&content) {
                            if let Ok(front_matter) = serde_yaml::from_str::<PostFrontMatter>(&front_matter_yaml) {
                                let slug = filename.to_string();
                                
                                // Get author
                                let author = authors.get(&front_matter.author).cloned();
                                
                                // Convert markdown to HTML
                                let markdown_options = comrak::ComrakOptions::default();
                                let content_html = comrak::markdown_to_html(&body, &markdown_options);
                                
                                // Extract excerpt (first paragraph or up to <!--more-->)
                                let excerpt = extract_excerpt(&body);
                                
                                let post = BlogPost {
                                    slug: slug.clone(),
                                    title: front_matter.title,
                                    date: front_matter.date,
                                    author_id: front_matter.author,
                                    author,
                                    featured: featured_slugs.contains(&slug),
                                    content_html,
                                    excerpt,
                                    image_path: front_matter.image,
                                };
                                
                                posts.push(post);
                            }
                        }
                    }
                }
            }
        }
        
        // Sort posts by date (newest first)
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        
        // Get featured posts
        let featured_posts: Vec<BlogPost> = posts.iter()
            .filter(|p| p.featured)
            .cloned()
            .collect();
        
        Ok(BlogData {
            authors,
            posts,
            featured_posts,
        })
    }
    
    pub fn get_post(&self, slug: &str) -> Option<&BlogPost> {
        self.posts.iter().find(|p| p.slug == slug)
    }
    
    pub fn get_featured_posts(&self) -> &[BlogPost] {
        &self.featured_posts
    }
    
    pub fn get_all_posts(&self) -> &[BlogPost] {
        &self.posts
    }
}

fn parse_front_matter(content: &str) -> Option<(String, String)> {
    let content = content.trim();
    if !content.starts_with("---") {
        return None;
    }
    
    // Find the end of front matter
    let lines: Vec<&str> = content.lines().collect();
    let mut end_index = 0;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_index = i;
            break;
        }
    }
    
    if end_index == 0 {
        return None;
    }
    
    // Extract front matter and body
    let front_matter = lines[1..end_index].join("\n");
    let body = lines[(end_index + 1)..].join("\n");
    
    Some((front_matter, body))
}

fn extract_excerpt(content: &str) -> String {
    // Look for <!-- END_EXCERPT --> marker
    if let Some(excerpt_pos) = content.find("<!-- END_EXCERPT -->") {
        let excerpt = &content[..excerpt_pos];
        let markdown_options = comrak::ComrakOptions::default();
        return comrak::markdown_to_html(excerpt.trim(), &markdown_options);
    }
    
    // Look for <!--more--> marker
    if let Some(more_pos) = content.find("<!--more-->") {
        let excerpt = &content[..more_pos];
        let markdown_options = comrak::ComrakOptions::default();
        return comrak::markdown_to_html(excerpt.trim(), &markdown_options);
    }
    
    // Otherwise, take first paragraph
    let paragraphs: Vec<&str> = content.split("\n\n").collect();
    if let Some(first_para) = paragraphs.first() {
        let markdown_options = comrak::ComrakOptions::default();
        return comrak::markdown_to_html(first_para.trim(), &markdown_options);
    }
    
    String::new()
}