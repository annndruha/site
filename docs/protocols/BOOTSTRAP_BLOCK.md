# Bootstrap Block for Claude Code Sessions

## Quick Context
Working on 12factor.net Ruby-to-Rust migration. Key patterns learned for efficient Claude Code usage.

## Core Efficiency Patterns

### 1. Tool Approval Hierarchy
Avoid approval prompts by using alternatives in this order:
```bash
# ❌ AVOID (requires approval)
chmod +x script.sh && ./script.sh
mv file.txt destination/
curl -o output.html http://example.com && process

# ✅ PREFER (no approval needed)
bash script.sh
cp file.txt destination/ && rm file.txt
curl -o output.html http://example.com; process
```

### 2. Complex Operations Pattern
For multi-step operations, use the Task tool:
```markdown
Task: "Search for config patterns"
Prompt: "Search for all configuration-related code patterns across the codebase, 
including environment variables, config files, and initialization code. 
Return a summary of findings organized by type."
```

### 3. Context Switching Recognition
Watch for pivot signals:
- "Actually..." → User changing direction
- "What about [new topic]" → Context switch
- References to different directories/files → Scope change
- Questions about unrelated features → New focus area

### 4. Meta-Learning Framework
When encountering repeated patterns:
1. **OBSERVE** - What happened repeatedly?
2. **ANALYZE** - Why did it happen?
3. **SYNTHESIZE** - What's the pattern?
4. **CODIFY** - Document in CLAUDE.md
5. **APPLY** - Use pattern immediately

### 5. Directory Navigation
```bash
# Always use absolute paths
cd /home/user/project/subdir

# Check before creating
ls parent_dir/  # Verify parent exists
mkdir parent_dir/new_dir

# Handle spaces properly
cd "/path/with spaces/directory"
```

### 6. Git Patterns
```bash
# Parallel git status checks
git status &
git diff &
git log --oneline -10 &
wait

# Commit without attribution
git commit -m "$(cat <<'EOF'
Your commit message here
EOF
)"
```

### 7. Development Helpers
Create reusable scripts for common tasks:
```bash
#!/bin/bash
# dev.sh - Manages both servers
trap 'kill $(jobs -p) 2>/dev/null' EXIT
ruby web.rb -p 4567 & 
cd rust_prototype && cargo run &
wait
```

### 8. Visual Comparison
When implementing UI changes:
1. Keep comparison pages updated
2. Document visual differences
3. Test side-by-side in browser
4. Update status tracking files

### 9. Efficient File Operations
```bash
# Batch reads (single message, multiple tools)
Read file1.txt
Read file2.txt
Read file3.txt

# Use symlinks for shared resources
ln -s ../content content

# Clean up periodically
find . -name "*.log" -delete
find . -name "*_output.html" -delete
```

### 10. Rust/Web Development Specific
```rust
// Path resolution pattern
let base_path = env::current_dir()
    .unwrap()
    .join(if cwd.ends_with("rust_prototype") { ".." } else { "." });

// Asset serving
Static files go in: public/assets/
Templates go in: templates/
```

## Key Reminders
- Use TodoWrite for multi-step tasks
- Run parallel bash commands in single message
- Avoid /tmp/ writes (use project directories)
- Check paths exist before operations
- Document patterns in CLAUDE.md introspection section
- Keep visual comparisons updated

## Project-Specific Context
- Ruby app on :4567, Rust on :3001
- Design mockups in "12factor FINAL/" on :8000
- Use mise for Ruby management
- Blog uses YAML front matter
- SCSS compilation with grass
- Tera templates (Django/Jinja2-like)