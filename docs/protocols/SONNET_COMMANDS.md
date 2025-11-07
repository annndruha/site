# Sonnet-Optimized Commands for 12factor.net

## Server Management

### Starting the Server
```bash
# Start Rust server (will timeout after 2m - this is normal)
/home/ykatz/Code/Heroku/12factor/rust_prototype/target/release/twelve_factor

# Start Ruby server
ruby /home/ykatz/Code/Heroku/12factor/web.rb -p 4567
```

### Health Checks (NEW!)
```bash
# Quick health check - returns "OK" or connection refused
curl http://localhost:3002/health

# Detailed health check - returns JSON with uptime
curl http://localhost:3002/health/detailed

# Check Ruby server
curl -s -o /dev/null -w "%{http_code}" http://localhost:4567/
```

## Debugging Flow

### Phase 1: Start and Verify
1. Start server (let it timeout)
2. Check health: `curl http://localhost:3001/health`

### Phase 2: If Server Failed
Check error logs in this order:
```bash
# Template errors
cat template_error.log

# SCSS compilation errors  
cat scss_error.log

# Blog data errors
cat blog_error.log
```

### Phase 3: Process Checks
```bash
# See if server process exists
ps aux | grep twelve_factor | grep -v grep

# Check if port is in use
ss -tln | grep :3001
```

## File Operations (No Pipes!)

### Search Operations
```bash
# Instead of: grep -r "pattern" . | head
rg "pattern" --max-count=10

# Instead of: find . -name "*.rs" | xargs grep "pattern"
rg "pattern" --type rust

# List files
ls /home/ykatz/Code/Heroku/12factor/rust_prototype/templates/
```

### Read Operations
```bash
# Read error logs
cat template_error.log

# Check specific line numbers
sed -n '20,30p' /path/to/file

# View file size
wc -l /path/to/file
```

## Testing Specific Features

### Check Design Elements
```bash
# Breadcrumbs on factor page
curl -s http://localhost:3001/codebase | grep -c "breadcrumb"

# Numeral icons
curl -s http://localhost:3001/codebase | grep -c "icon-numeral"

# Diamond icons on home
curl -s http://localhost:3001/ | grep -c "icon-diamond"
```

### Quick Page Tests
```bash
# Home page
curl http://localhost:3001/

# Factor page
curl http://localhost:3001/codebase

# Blog listing
curl http://localhost:3001/blog

# Community
curl http://localhost:3001/community
```

## Common Issues and Solutions

### Server Won't Start
1. Check: `cat template_error.log`
2. Fix template syntax
3. Restart server

### Server Crashes on Request
1. Start server
2. Immediately run: `curl http://localhost:3001/health`
3. If crashes, check logs

### Empty Responses
1. First check if server is alive: `curl http://localhost:3001/health`
2. Only then check specific features

## Rules for Sonnet Sessions

### NEVER Use These
- `&&` - Use separate commands
- `&` - Let Claude Code handle background
- `||` - Use separate commands
- `|` - Save to file first, then read
- `>` or `>>` - Use `-o` flag with curl
- `bash script.sh` - Run commands directly
- `$(...)` - Avoid command substitution

### ALWAYS Do These
1. One command at a time
2. Check health before debugging features
3. Read error logs when server fails
4. Use full paths for binaries
5. Let commands timeout naturally

## Quick Reference

### Server Status Check Sequence
```bash
# 1. Health check
curl http://localhost:3001/health

# 2. If failed, check process
ps aux | grep twelve_factor | grep -v grep

# 3. Check error logs
cat template_error.log
cat scss_error.log
cat blog_error.log

# 4. Fix and restart
```

### Feature Verification Sequence
```bash
# 1. Ensure server is healthy
curl http://localhost:3001/health

# 2. Check specific feature
curl http://localhost:3001/codebase | grep -c "breadcrumb"

# 3. If zero, server might be dead - go back to step 1
```

## Bootstrap for New Session

When starting a new Sonnet session on this project:

1. Check server status: `curl http://localhost:3001/health`
2. If failed, start server: `/home/ykatz/Code/Heroku/12factor/rust_prototype/target/release/twelve_factor`
3. Verify with health check again
4. Continue with task

Remember: Empty curl results usually mean server is dead, not feature is missing!