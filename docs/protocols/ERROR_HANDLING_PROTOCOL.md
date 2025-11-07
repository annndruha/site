# Error Handling Protocol for Claude Code

## System Design Proposal

### 1. Enhanced Error Recognition System

```yaml
error_patterns:
  execution_failed:
    - pattern: "^Error: Error$"
    - pattern: "^Error$"
    - meaning: "Command could not be executed"
    - action: "Try alternative command"
    
  permission_denied:
    - pattern: "Permission denied"
    - pattern: "Operation not permitted"
    - meaning: "Insufficient privileges"
    - action: "Use unprivileged alternative"
    
  command_not_found:
    - pattern: "command not found"
    - pattern: "No such file or directory"
    - meaning: "Binary not in PATH or doesn't exist"
    - action: "Use different tool"
    
  empty_success:
    - pattern: "^$" (empty string)
    - meaning: "Command succeeded with no output"
    - action: "Interpret as valid negative result"
```

### 2. Proactive Error Prevention

Before executing ANY command, check:

1. **Command Availability Matrix**
   ```
   ALWAYS AVAILABLE: ls, pwd, echo, cat, grep, sed
   SOMETIMES AVAILABLE: lsof, netstat, pgrep
   RARELY AVAILABLE: sudo commands, system utilities
   ```

2. **Permission Requirements**
   ```
   NO PERMISSION NEEDED: Most read operations
   MAY NEED PERMISSION: Port checks, process details
   ALWAYS NEEDS PERMISSION: System modifications
   ```

### 3. Error Response Decision Tree

```
Bash Command Executed
    ↓
Is response "Error" or "Error: Error"?
    ├─ YES → Command failed to execute
    │         ├─ Was it a network command? → Try curl alternatives
    │         ├─ Was it a process command? → Try ps alternatives  
    │         └─ Was it a file command? → Use Read/Write tools
    │
    └─ NO → Check if empty
            ├─ YES → Valid result (nothing found)
            └─ NO → Parse actual output/error
```

### 4. Integration with Existing Tools

Modify tool usage patterns:

```python
# Current approach (problematic)
def check_server():
    result = bash("lsof -i :3001")
    if not result:
        print("Port not in use")  # WRONG! Could be error
        
# Better approach
def check_server():
    result = bash("lsof -i :3001")
    if result == "Error" or result == "Error: Error":
        # Try alternative
        result = bash("ss -tln | grep 3001")
    
    if not result and result != "Error":
        print("Port not in use")  # Correct interpretation
```

### 5. Feedback Loop Enhancement

When an error occurs:
1. Log the failed command
2. Log the successful alternative
3. Update COMMAND_FALLBACKS.md
4. Learn for next session

### 6. Tool Enhancement Proposal

Ideal Bash tool behavior:
```json
{
  "command": "lsof -i :3001",
  "result": {
    "status": "error",
    "error_type": "permission_denied",
    "suggestion": "Try 'ss -tln | grep 3001' instead",
    "output": null
  }
}
```

## Implementation Priority

1. **Immediate**: Document all "Error: Error" patterns in CLAUDE.md
2. **Short-term**: Create fallback chains for common operations
3. **Long-term**: Advocate for enhanced tool responses
4. **Continuous**: Update patterns based on new discoveries