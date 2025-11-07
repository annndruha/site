// Community Activity - Real-time GitHub data for a living document
(function() {
  'use strict';
  
  const GITHUB_API_BASE = 'https://api.github.com';
  const REPO = 'twelve-factor/twelve-factor';
  const CACHE_DURATION = 15 * 60 * 1000; // 15 minutes
  const CACHE_KEY = 'twelve-factor-activity';
  
  // We'll discover factor discussions dynamically from GitHub
  
  // Get cached data if fresh
  function getCachedData() {
    try {
      const cached = localStorage.getItem(CACHE_KEY);
      if (!cached) return null;
      
      const data = JSON.parse(cached);
      const age = Date.now() - data.timestamp;
      
      if (age < CACHE_DURATION) {
        return data.issues;
      }
    } catch (e) {
      console.warn('Cache read failed:', e);
    }
    return null;
  }
  
  // Cache the fetched data
  function cacheData(issues) {
    try {
      localStorage.setItem(CACHE_KEY, JSON.stringify({
        timestamp: Date.now(),
        issues: issues
      }));
    } catch (e) {
      console.warn('Cache write failed:', e);
    }
  }
  
  // Format relative time
  function formatRelativeTime(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now - date;
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    
    if (diffDays === 0) {
      const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
      if (diffHours === 0) {
        const diffMins = Math.floor(diffMs / (1000 * 60));
        return diffMins <= 1 ? 'just now' : `${diffMins} minutes ago`;
      }
      return diffHours === 1 ? '1 hour ago' : `${diffHours} hours ago`;
    } else if (diffDays === 1) {
      return 'yesterday';
    } else if (diffDays < 7) {
      return `${diffDays} days ago`;
    } else if (diffDays < 30) {
      const weeks = Math.floor(diffDays / 7);
      return weeks === 1 ? '1 week ago' : `${weeks} weeks ago`;
    } else if (diffDays < 365) {
      const months = Math.floor(diffDays / 30);
      return months === 1 ? '1 month ago' : `${months} months ago`;
    } else {
      const years = Math.floor(diffDays / 365);
      return years === 1 ? '1 year ago' : `${years} years ago`;
    }
  }
  
  // Extract factor name from issue title/labels
  function extractFactor(issue) {
    const factors = ['codebase', 'dependencies', 'config', 'backing-services', 
                    'build-release-run', 'processes', 'port-binding', 'concurrency',
                    'disposability', 'dev-prod-parity', 'logs', 'admin-processes'];
    
    // Check labels first
    for (const label of issue.labels || []) {
      if (label.name.startsWith('factor:')) {
        return label.name.substring(7);
      }
    }
    
    // Check title patterns
    const titleLower = issue.title.toLowerCase();
    for (const factor of factors) {
      if (titleLower.includes(`[${factor}]`) || 
          titleLower.includes(`${factor}:`) ||
          titleLower.includes(`${factor} factor`)) {
        return factor;
      }
    }
    
    return null;
  }
  
  // Fetch all open issues from GitHub
  async function fetchIssueData() {
    const cached = getCachedData();
    if (cached) {
      return cached;
    }
    
    try {
      const response = await fetch(`${GITHUB_API_BASE}/repos/${REPO}/issues?state=open&per_page=100`);
      if (!response.ok) {
        throw new Error(`GitHub API returned ${response.status}`);
      }
      
      const issues = await response.json();
      
      // Filter to only factor-related issues
      const factorIssues = issues
        .map(issue => ({
          ...issue,
          factor: extractFactor(issue)
        }))
        .filter(issue => issue.factor !== null);
      
      if (factorIssues.length > 0) {
        cacheData(factorIssues);
      }
      
      return factorIssues;
    } catch (error) {
      console.error('Failed to fetch GitHub issues:', error);
      return [];
    }
  }
  
  // Update factor page CTA with real data
  async function updateFactorPageCta(factorSlug) {
    try {
      const issues = await fetchIssueData();
      const factorIssue = issues.find(issue => issue.factor === factorSlug);
      
      if (factorIssue) {
        // Update participant count
        const ctaNotes = document.querySelectorAll('.factor-cta .cta-note');
        ctaNotes.forEach(note => {
          if (note.textContent.includes('contributors')) {
            const count = factorIssue.comments + 1;
            note.textContent = `${count} contributors are discussing improvements to this factor.`;
          }
        });
        
        // Update discussion link if needed
        const discussionBtn = document.querySelector('.factor-cta a[href*="issues/new"]');
        if (discussionBtn) {
          discussionBtn.href = factorIssue.html_url;
          discussionBtn.innerHTML = '<i class="bi bi-chat-dots"></i> Join the discussion';
          discussionBtn.nextElementSibling.textContent = 
            `${factorIssue.comments + 1} contributors are discussing improvements to this factor.`;
        }
      }
    } catch (error) {
      console.error('Failed to update factor page CTA:', error);
    }
  }
  
  // Update factor card with real data
  function updateFactorCard(factor, issueData) {
    const card = document.querySelector(`[data-factor="${factor}"]`);
    if (!card) return;
    
    const activityEl = card.querySelector('.factor-activity');
    if (activityEl) {
      // Show the activity element
      activityEl.style.display = '';
      
      // Update the link
      const link = activityEl.querySelector('.discussion-link');
      if (link) {
        link.href = issueData.html_url;
      }
      
      // Update participant count
      const participantCount = issueData.comments + 1; // +1 for issue author
      const activityText = activityEl.querySelector('.activity-text');
      if (activityText) {
        activityText.textContent = `${participantCount} contributors discussing`;
      }
    }
    
    // Add active class to card
    card.classList.add('factor-card--active');
    
    // Update status to active
    card.setAttribute('data-status', 'active');
    
    // Update git info
    updateGitInfo(card, issueData);
  }
  
  // Update git info section on factor card
  function updateGitInfo(card, issueData) {
    const gitInfoEl = card.querySelector('.factor-git-info');
    if (!gitInfoEl) return;
    
    // Update last activity time
    const lastActivity = issueData.updated_at || issueData.created_at;
    const timeValueEl = gitInfoEl.querySelector('.time-value');
    if (timeValueEl) {
      timeValueEl.textContent = formatRelativeTime(lastActivity);
    }
    
    // Show contributors if recent activity
    const contributors = gitInfoEl.querySelector('.git-contributors');
    if (contributors && issueData.updated_at) {
      const daysSinceUpdate = Math.floor((Date.now() - new Date(issueData.updated_at)) / (1000 * 60 * 60 * 24));
      if (daysSinceUpdate < 90) { // Show contributors for updates within 3 months
        contributors.style.display = '';
        
        // Fetch contributors (simplified - just show avatars of recent participants)
        fetchContributors(issueData).then(avatars => {
          const avatarsContainer = contributors.querySelector('.contributor-avatars');
          if (avatarsContainer && avatars.length > 0) {
            avatarsContainer.innerHTML = avatars
              .slice(0, 3) // Max 3 avatars as per requirement
              .map(avatar => `<img src="${avatar.url}" alt="${avatar.user}" title="@${avatar.user}">`)
              .join('');
          }
        });
      }
    }
  }
  
  // Fetch contributor avatars for an issue
  async function fetchContributors(issue) {
    try {
      // For now, just return issue author
      // In a real implementation, we'd fetch comment participants too
      return [{
        user: issue.user.login,
        url: issue.user.avatar_url
      }];
    } catch (error) {
      console.error('Failed to fetch contributors:', error);
      return [];
    }
  }
  
  // Update git info for factors with no recent activity
  function updateGitInfoForOriginal(card) {
    const gitInfoEl = card.querySelector('.factor-git-info');
    if (!gitInfoEl) return;
    
    // Set status to original
    card.setAttribute('data-status', 'original');
    
    // Update time to original publication date
    const timeValueEl = gitInfoEl.querySelector('.time-value');
    if (timeValueEl) {
      timeValueEl.textContent = '13 years ago'; // Original publication was 2011
    }
    
    // Hide contributors for original factors
    const contributors = gitInfoEl.querySelector('.git-contributors');
    if (contributors) {
      contributors.style.display = 'none';
    }
  }
  
  // Initialize activity indicators
  async function initializeActivity() {
    // Check if we're on a factor page
    const factorCta = document.querySelector('.factor-cta');
    if (factorCta) {
      // Extract factor slug from URL
      const pathParts = window.location.pathname.split('/');
      const factorSlug = pathParts[pathParts.length - 1];
      if (factorSlug && factorSlug !== '') {
        await updateFactorPageCta(factorSlug);
      }
    }
    
    // Add data attributes to factor cards based on their links
    document.querySelectorAll('.factor-card').forEach(card => {
      const link = card.querySelector('a[href^="/"]');
      if (link) {
        const factor = link.getAttribute('href').substring(1);
        card.setAttribute('data-factor', factor);
      }
    });
    
    // Show loading state for cards that have activity divs
    document.querySelectorAll('.factor-activity').forEach(el => {
      el.classList.add('loading');
    });
    
    try {
      const issues = await fetchIssueData();
      
      // Group issues by factor
      const issuesByFactor = {};
      issues.forEach(issue => {
        if (issue.factor) {
          issuesByFactor[issue.factor] = issue;
        }
      });
      
      // Update all factor cards
      const allFactors = ['codebase', 'dependencies', 'config', 'backing-services', 
                         'build-release-run', 'processes', 'port-binding', 'concurrency',
                         'disposability', 'dev-prod-parity', 'logs', 'admin-processes'];
      
      allFactors.forEach(factor => {
        const card = document.querySelector(`[data-factor="${factor}"]`);
        if (!card) return;
        
        if (issuesByFactor[factor]) {
          // Factor has active discussion
          updateFactorCard(factor, issuesByFactor[factor]);
        } else {
          // Factor has no active discussion - show original date
          updateGitInfoForOriginal(card);
        }
      });
      
      // Update progress indicator
      updateProgressIndicator(issues);
      
    } catch (error) {
      console.error('Failed to fetch activity data:', error);
    } finally {
      // Remove loading state
      document.querySelectorAll('.factor-activity.loading').forEach(el => {
        el.classList.remove('loading');
      });
    }
  }
  
  // Update progress indicator (if present)
  function updateProgressIndicator(issues) {
    const progressEl = document.querySelector('.evolution-progress');
    if (!progressEl) return;
    
    const activeCount = issues.filter(i => i.state === 'open').length;
    const totalFactors = 12;
    
    progressEl.innerHTML = `
      <span class="progress-stat">${activeCount} factors in active discussion</span>
      <span class="progress-bar">
        <span class="progress-fill" style="width: ${(activeCount / totalFactors) * 100}%"></span>
      </span>
    `;
  }
  
  // Run when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeActivity);
  } else {
    initializeActivity();
  }
  
  // Refresh data when page becomes visible again
  document.addEventListener('visibilitychange', () => {
    if (!document.hidden) {
      initializeActivity();
    }
  });
})();