# Project Skills

These skills are maintained as Markdown with a small YAML frontmatter (`name` and
`description`) and supporting files kept beside each `SKILL.md`. This is the
canonical copy in this repository.

| Skill                                                     | Use for                                                        |
| --------------------------------------------------------- | -------------------------------------------------------------- |
| [api-design-reviewer](api-design-reviewer/SKILL.md)       | Reviewing REST API contracts, conventions, and compatibility   |
| [api-test-suite-builder](api-test-suite-builder/SKILL.md) | Designing API, integration, and contract test coverage         |
| [changelog-generator](changelog-generator/SKILL.md)       | Conventional Commit analysis, version bumps, and release notes |
| [pr-review-expert](pr-review-expert/SKILL.md)             | Structured pull request and merge request reviews              |
| [self-eval](self-eval/SKILL.md)                           | Evaluating completed work with a calibrated scoring rubric     |

## Using skills across agents

- Claude Code can discover project skills in `.claude/skills/`.
- Other agents may not discover that directory or support Claude-specific
  invocation syntax. Follow the root `AGENTS.md`/agent guide, open the relevant
  skill explicitly, and use its portable Markdown guidance where supported.
- If an agent needs a native skill location or metadata format, add a thin
  adapter or generated mirror for that agent. Keep this directory canonical;
  avoid manually diverging copies.
- Commands are written for the environment and working directory stated in each
  skill. Resolve bundled references, scripts, and assets relative to that skill's
  directory unless documented otherwise. Confirm referenced files exist before
  running a command; do not assume examples imply bundled tooling.
- Treat agent-specific slash commands, tool names, and environment variables as
  integration examples, not portable Markdown features. Provide an equivalent
  instruction or omit that integration when the current agent does not support it.
