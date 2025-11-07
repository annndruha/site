use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityConfig {
    pub discord_url: String,
    pub github_repo: String,
    pub open_sourced_year: u16,
    pub original_year: u16,
    pub proposed_factors: Vec<ProposedFactor>,
    pub active_discussions: Vec<ActiveDiscussion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedFactor {
    pub name: String,
    pub description: String,
    pub discussion_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDiscussion {
    pub factor_slug: String,
    pub issue_number: u32,
    pub issue_url: String,
    pub participant_count: Option<u32>,
    pub last_activity: Option<String>,
    pub status: Option<DiscussionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscussionStatus {
    Active,
    Resolved,
    Implementing,
}

impl Default for CommunityConfig {
    fn default() -> Self {
        Self {
            discord_url: "https://discord.gg/QDAJZjqhhx".to_string(),
            github_repo: "https://github.com/twelve-factor/twelve-factor".to_string(),
            open_sourced_year: 2024,
            original_year: 2011,
            proposed_factors: vec![
                ProposedFactor {
                    name: "Workload Identity".to_string(),
                    description: "Secure, platform-provided identity for services".to_string(),
                    discussion_url: Some("https://github.com/twelve-factor/twelve-factor/issues/46".to_string()),
                },
                ProposedFactor {
                    name: "API Design".to_string(),
                    description: "Consistent interface patterns".to_string(),
                    discussion_url: None,
                },
                ProposedFactor {
                    name: "Observability".to_string(),
                    description: "Beyond just logs".to_string(),
                    discussion_url: None,
                },
            ],
            active_discussions: vec![], // Populated dynamically from GitHub API
        }
    }
}

impl CommunityConfig {
    pub fn load() -> Self {
        // In the future, this could load from a TOML/YAML file
        // For now, use defaults
        Self::default()
    }
    
    pub fn get_discussion_for_factor(&self, factor_slug: &str) -> Option<&ActiveDiscussion> {
        self.active_discussions.iter()
            .find(|d| d.factor_slug == factor_slug)
    }
}