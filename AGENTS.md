# Agent Guide

## Project conventions

- Read the existing implementation and follow its patterns before changing code.
- Keep changes focused, preserve existing behavior, and avoid unrelated edits.
- This is a Rust project. Run `cargo fmt --check` and the narrowest relevant Cargo
  check or test before reporting a code change as complete.
- Do not claim a check passed unless it was actually run.

## Skills

- The project skills are maintained in [`.claude/skills/`](.claude/skills/README.md).
  Each skill's `SKILL.md` is the canonical content; its sibling `references/`,
  `scripts/`, and `assets/` directories are supporting material.
- Before using a skill, verify that its referenced files and scripts exist. Resolve
  paths relative to the skill directory unless the skill explicitly says otherwise.
- Treat commands and tool integrations in a skill as examples, not guarantees:
  adapt them to the current repository and available tools, and never invent
  script output or claim an unavailable integration ran.
- Prefer updating the canonical skill over maintaining hand-edited copies for
  different agents. Add agent-specific discovery metadata only when that agent
  requires it.
