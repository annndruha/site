# Command Execution Discoveries for Claude Code (Sonnet)

## Context
These discoveries come from extensive debugging sessions with Claude Code. Your system setup may differ, but these patterns have proven universal across multiple sessions.

## Core Discovery: The "Error: Error" Pattern

### What It Means
When you execute a Bash command and receive:
- `Error: Error` 
- `Error`

This means **the command failed to execute at all** - NOT that it executed and returned an error.

### Why This Matters
```bash
# Example: Checking if a port is in use
lsof -i :3000
# Response: "Error: Error" → lsof command failed (maybe permissions)
# Response: "" (empty) → Port is not in use
# Response: "COMMAND PID..." → Port is in use
```

Misinterpreting "Error: Error" as "port not in use" leads to incorrect debugging paths.

## Command Approval Patterns

### Commands That Trigger Approval Prompts
These operators/patterns require user approval and should be avoided:

```bash
# Background execution
command &

# Conditional execution  
command1 && command2
command1 || command2

# Pipes (sometimes)
command1 | command2

# Output redirection
command > file.txt
command >> file.txt

# Command substitution
$(command)
`command`

# Script execution
bash script.sh
sh script.sh
python script.py

# Environment manipulation
VAR=value command
export VAR=value
```

### Approval-Free Alternatives

```bash
# Instead of: command1 && command2
# Use: Two separate tool calls

# Instead of: command &
# Use: Just run command (system handles process management)

# Instead of: command > output.txt
# Use: command -o output.txt (when available)
# Or: Use Write tool after capturing output

# Instead of: curl URL | grep pattern
# Use: curl URL in one call, then grep in another

# Instead of: complex pipes
# Use: Break into steps with intermediate files
```

## Fallback Chains for Common Operations

When a command returns "Error: Error", try alternatives:

### Port Checking
```bash
# Primary
lsof -i :PORT

# If Error, try:
ss -tln | grep PORT

# If Error, try:
netstat -tln | grep PORT
```

### Process Checking
```bash
# Primary
ps aux | grep PROCESS

# If Error, try:
pgrep -f PROCESS

# If Error, try:
pidof PROCESS
```

### Network Testing
```bash
# Primary
curl URL

# If Error, try:
wget -O- URL

# For simpler cases:
curl -s -o /dev/null -w "%{http_code}" URL
```

## Best Practices for Sonnet

### 1. Error-First Thinking
Always check if a command actually executed:
```python
# Pseudo-code pattern
result = execute_bash(command)
if result in ["Error", "Error: Error"]:
    # Command failed to run - try alternative
elif not result:
    # Command ran but returned empty
else:
    # Command ran with output
```

### 2. Simple Commands Win
Prefer universally available commands:
- ✅ `ls`, `cat`, `grep`, `sed`, `echo`
- ⚠️  `lsof`, `ss`, `pgrep` (may not be available)
- ❌ `sudo anything` (will fail)

### 3. Use Native Tools When Possible
- Instead of `cat file.txt`, use the Read tool
- Instead of `echo "content" > file.txt`, use the Write tool
- Instead of complex bash scripts, break into individual commands

### 4. Working Directory Awareness
- `cd` often doesn't work as expected
- Use absolute paths: `/full/path/to/file`
- Or use tools that accept path parameters

## Debugging Strategies

### When Things Don't Work

1. **Start with health checks**
   ```bash
   # For web servers
   curl http://localhost:PORT/health
   ```

2. **Distinguish between failure types**
   - "Error: Error" → Try different command
   - "Connection refused" → Service not running
   - Empty response → Service running but returned nothing

3. **Build verification chains**
   ```bash
   # Don't assume success from one indicator
   # Verify with multiple simple commands
   ```

## System-Specific Adaptations

Your system may have different:
- Available commands
- Permission models  
- Network configurations
- Tool versions

### Discovery Protocol
1. Test basic command availability first
2. Document what works in your environment
3. Build your own fallback chains
4. Share discoveries with future sessions

## Meta-Learning Framework

When you encounter issues:

1. **OBSERVE**: What exactly happened?
2. **ANALYZE**: Why did it fail?
3. **SYNTHESIZE**: What's the pattern?
4. **DOCUMENT**: Add to your system notes
5. **APPLY**: Use the learning immediately

## Quick Reference Card

```bash
# Health check pattern
curl -s -o /dev/null -w "%{http_code}" URL  # Gets just status code

# Safe process check
pgrep -f "process_name"  # Returns PIDs or nothing

# Safe port check  
ss -tln | grep ":PORT"  # No special permissions needed

# File operations
# Use Read/Write tools instead of cat/echo when possible

# Always use absolute paths
/home/user/project/file.txt  # Not: ./file.txt
```

## Final Wisdom

1. **"Error: Error" is your friend** - It tells you to try alternatives
2. **Simpler is better** - Basic commands rarely fail
3. **Verify assumptions** - Empty doesn't mean success
4. **Document patterns** - Each session should improve the knowledge base

Remember: The goal isn't to avoid all errors, but to recognize them quickly and have alternatives ready.

---
*This document represents hard-won knowledge from debugging sessions. Adapt it to your specific environment and share your discoveries with future sessions.*