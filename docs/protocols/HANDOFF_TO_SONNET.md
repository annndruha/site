# Handoff to Sonnet: Twelve-Factor Cleanup Task

## Current Status
The Twelve-Factor website has been successfully migrated from Ruby to Rust. All features are implemented and working in the `rust_prototype/` directory.

## Your Task
Execute the cleanup process documented in `SAFER_CLEANUP_PROMPT.md`. This will:
1. Commit all current work
2. Archive the Ruby implementation and design assets
3. Clean up temporary files
4. Prepare the project for Heroku deployment with Rust

## Important Context
- **There are tons of uncommitted changes** - commit them all first for safety
- **The Rust app must stay in `rust_prototype/`** - moving it will break import paths
- **Test frequently** - verify the app works after each major step
- **Archive before delete** - never delete anything without archiving first

## Files to Reference
1. **SAFER_CLEANUP_PROMPT.md** - Your step-by-step instructions
2. **CLAUDE.md** - Project conventions and guidelines
3. **PROJECT_STATUS.md** - Current implementation status

## Verification Points
After each phase in the cleanup prompt, verify:
- Rust server still runs: `cd rust_prototype && cargo run`
- Home page loads: http://localhost:12012/
- Enhanced community page works: http://localhost:12012/community

## Critical Warning
Do NOT use any other cleanup instructions you might find. The file `CLEANUP_SONNET_PROMPT.md` was deleted because it contained dangerous operations that would break the application.

## Success Criteria
- All temporary files cleaned up
- Ruby implementation safely archived
- Rust app still fully functional
- Project ready for Heroku deployment
- Clear documentation of what was done

## If Something Goes Wrong
The safer cleanup prompt includes a rollback plan:
```bash
git checkout pre-cleanup-backup
```

This will restore everything to the pre-cleanup state.

Good luck! Take your time and verify each step.