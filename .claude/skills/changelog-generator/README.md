# Changelog Generator

Automates release notes from Conventional Commits with Keep a Changelog output and strict commit linting. Designed for CI-friendly release workflows.

## Quick Start

```bash
# Run commands from the repository root; scripts are bundled in this directory.
python3 .claude/skills/changelog-generator/scripts/generate_changelog.py \
  --from-tag v1.2.0 \
  --to-tag v1.3.0 \
  --next-version v1.3.0 \
  --format markdown

# Lint commit subjects
python3 .claude/skills/changelog-generator/scripts/commit_linter.py --from-ref origin/main --to-ref HEAD --strict --format text
```

## Included Tools

- `scripts/generate_changelog.py`: parse commits, infer semver bump, render markdown/JSON, optional file prepend
- `scripts/commit_linter.py`: validate commit subjects against Conventional Commits rules
- `scripts/version_bumper.py`: compute the recommended next version from `git log --oneline` output (`--current-version`, `--prerelease`, `--include-commands`)

## References

- `references/ci-integration.md`
- `references/changelog-formatting-guide.md`
- `references/monorepo-strategy.md`
- `references/hotfix-procedures.md` (hotfix severity SLAs + rollback triggers, absorbed from the retired release-manager skill)

## Using with other agents

This skill is stored in the repository at
`.claude/skills/changelog-generator/`. Agents that do not discover Claude Code's
skill directory can use the Markdown explicitly or copy/generate it into their
documented native skill location. Keep this repository copy canonical and update
any generated copy from it.
