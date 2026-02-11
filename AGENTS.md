## Using SB Tracker

This project uses [SB Tracker](https://github.com/sirius-cc-wu/sb-tracker) for task tracking.

### Prerequisite

Install `sb` and ensure it is on your `PATH` before using these commands:
- `pipx install sb-tracker` (recommended for CLI tools)
- or `pip install sb-tracker`
- verify with `sb --help`

**Agents**: Please use the `sb` command to track work:
- `sb add "Task title" [priority] [description]` - Create a task
- `sb list` - View open tasks
- `sb ready` - See tasks ready to work on
- `sb done <id>` - Mark a task as complete
- `sb promote <id>` - Optional: generate a Markdown summary of task progress

### Priority Levels (Required Numeric Values)

When using `sb add`, specify priority as a **numeric value** (0-3):
- **0** = P0 (Critical) - Blocking other work
- **1** = P1 (High) - Important, do soon
- **2** = P2 (Medium) - Normal priority (default)
- **3** = P3 (Low) - Nice to have

Example: `sb add "Fix critical bug" 0 "This blocks release"`

Run `sb --help` for more commands.

### Landing the Plane (Session Completion)

**When ending a work session**, complete these steps:

1. **File remaining work** - Create issues for any follow-up tasks
2. **Verify** - Run project tests or take a screenshot to confirm the work is complete
3. **Update task status** - Mark completed work as done with `sb done <id>`
4. **Clean up** - Run `sb compact` if you want to remove closed tasks
5. **Commit local changes** - Commit code and `.sb.json` together. If a `commit` skill is available in the agent environment, use it. Otherwise run:
   ```bash
   git add -A
   git commit -m "[scope]: complete <task-id>"
   ```
6. **Final state check** - Run `sb list --all` and confirm no tasks are left ambiguous
7. **Handoff** - Share a short summary of what was completed and what remains

**CRITICAL RULES:**
- Always update task status before ending a session
- Never leave tasks in an ambiguous state; close them or create explicit sub-tasks
- Do not leave finished work uncommitted; commit issue-by-issue so progress is resumable
- Prefer the `commit` skill for commits when available; use raw git commit only as fallback
- `sb promote` is optional and only needed when you want a Markdown report
