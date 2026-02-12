## Using SB Tracker

This project uses [SB Tracker](https://github.com/yourusername/sb-tracker) for task tracking.

**Agents**: Please use the `sb` command to track work:
- `sb add "Task title" [priority] [description]` - Create a task
- `sb list` - View open tasks
- `sb ready` - See tasks ready to work on
- `sb done <id>` - Mark a task as complete
- `sb promote <id>` - Generate a summary of task progress

### Priority Levels (Required Numeric Values)

When using `sb add`, specify priority as a **numeric value** (0-3):
- **0** = P0 (Critical) - Blocking other work
- **1** = P1 (High) - Important, do soon
- **2** = P2 (Medium) - Normal priority (default)
- **3** = P3 (Low) - Nice to have

Example: `sb add "Fix critical bug" 0 "This blocks release"`

Run `sb --help` or check the README for more commands.

### Landing the Plane (Session Completion)

**When ending a work session**, complete these steps:

1. **File remaining work** - Create issues for any follow-up tasks
2. **Update task status** - Mark completed work as done with `sb done <id>`
3. **Promote for handoff** - Run `sb promote <id>` on significant tasks to document progress
4. **Clean up** - Run `sb compact` to archive closed tasks and keep the tracker lean

**CRITICAL RULES:**
- Always update task status before ending a session
- Use `sb promote` to hand off context about what was accomplished
- Never leave tasks in an ambiguous state—close them or create sub-tasks
