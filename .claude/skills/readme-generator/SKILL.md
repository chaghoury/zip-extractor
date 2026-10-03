---
name: readme-generator
description: Generate or update a project's README.md against the team's standard template — covering project structure (including the .code-assist/agents, .code-assist/prompts, and .code-assist/skills folders), all package/build/test/run commands, every environment variable (with description, required/optional, and a dummy example value), and the team's development workflow. Use when the user asks to create, generate, write, update, refresh, or sync a README; when a project is missing one; when commands, env vars, or the .code-assist folder have changed and the docs are stale; or when starting a new project that needs a README before onboarding others. Works across languages and stacks — Node, Python, Java, Go, Rust, and beyond.
---

# README Generator / Updater

A structured workflow for producing and maintaining a README.md that follows
one standard across every project, regardless of language or stack. It
covers project structure (including the team's `.code-assist` convention),
every available command, every environment variable, and the development
workflow — and it works in two modes: **generate** a README from nothing, or
**update** an existing one so it stays accurate as the project grows.

**Model-agnostic by design.** This is plain workflow guidance and content
standards, not tied to a specific assistant's tools or UI. It assumes only
that whoever runs it can read the project's files — any coding assistant or
a human following it manually can use it as-is.

## Why a Standard README Matters

- A new developer should be able to clone the repo, read the README, and be
  running the project without asking anyone a question.
- Undocumented env vars are one of the most common causes of "works on my
  machine" — every var a project reads should be visible, explained, and
  have an example.
- A `.code-assist` folder that grows silently (new skills, prompts, agents
  added over time) becomes useless if the README doesn't reflect what's
  actually in it — this is why the skill supports **update**, not just
  generate.
- One template across every project means a developer moving between repos
  already knows where to look.

## When to Use

Trigger this workflow when the user:

- Asks to create, write, or generate a README
- Asks to update, refresh, sync, or fix a stale README
- Has just added a command, env var, dependency, or `.code-assist` skill/prompt/agent that the README doesn't reflect
- Is starting a new project and needs a README before others can onboard
- Asks what commands or env vars a project needs and the answer should also be documented, not just stated in chat

---

## Core Principle: Generate vs. Update

Always check for an existing `README.md` first — the workflow branches from there:

- **No README exists →** full generation from scratch, built entirely by
  inspecting the actual project (Stage 1), not by asking the user to
  dictate content that's already discoverable in code.
- **A README exists →** read it in full before changing anything. Diff its
  content against the current state of the project: which sections match
  the standard template, which are missing, which are stale (a command
  that no longer exists, an env var no longer read anywhere, a
  `.code-assist` skill that was added since the README was last touched).
  Update what's stale or missing. **Never silently delete or overwrite
  content that isn't part of the standard structure** — hand-written
  sections (a team FAQ, a design-decision note, a "here be dragons"
  warning) may exist for a reason. Flag anything unrecognized to the user
  instead of removing it, and show a short summary of what you changed
  before finalizing.

---

## Stage 1: Discover the Project

Read the actual files — never guess or invent a command or variable that
isn't verifiable in the codebase. If something can't be confirmed from the
repo, ask the user rather than assuming.

**Identify the stack.** Look for manifest/config files: `package.json`,
`pyproject.toml` / `requirements.txt` / `setup.py`, `pom.xml` / `build.gradle`,
`go.mod`, `Cargo.toml`, `Gemfile`, `composer.json`, `CMakeLists.txt`, or
similar. A project can use more than one (e.g. a Node frontend with a Python
backend) — document each separately if so.

**Find the real commands.** Sources, roughly in order of reliability:

1. CI/CD config (`.github/workflows/*`, `.gitlab-ci.yml`, etc.) — often the
   ground truth, since it's what's actually run in the real pipeline
2. `package.json` `"scripts"`, `Makefile`, `justfile`, `tox.ini`, Gradle
   tasks, Cargo aliases
3. `Dockerfile` / `docker-compose.yml` for containerized setup/run commands
4. Any existing setup docs or CONTRIBUTING file

**Find every environment variable.** Sources:

- `.env.example`, `.env.sample`, `.env.template` if present
- Config loaders in code: `dotenv` usage, `os.environ` / `os.getenv` calls,
  `process.env` references, `System.getenv`, Pydantic/Spring/Rails config
  classes, `application.yml`/`.properties`
- `docker-compose.yml` `environment:` blocks, Dockerfile `ENV`/`ARG`
- Deploy manifests if present (Kubernetes YAML, Terraform, `serverless.yml`)

Cross-check these against each other: a variable read in code but absent
from `.env.example` is a gap worth flagging; one in `.env.example` but never
read in code may be dead and worth asking about.

**Inventory the `.code-assist` folder.** List what's under `.code-assist/agents/`,
`.code-assist/prompts/`, and `.code-assist/skills/`. For skills specifically,
read each `SKILL.md`'s frontmatter `description` to summarize its purpose in
one line — don't just list folder names.

---

## Stage 2: Gather What Can't Be Inferred From Code

Ask the user only for what genuinely isn't discoverable in the repo:

1. A one-paragraph project description — the elevator pitch (purpose, not
   implementation)
2. The team's development workflow, if it isn't already documented
   somewhere reusable (branching model, commit convention, review
   expectations, testing philosophy) — if the user already has a canonical
   version of this from another project, ask whether to reuse it verbatim
   for consistency rather than redrafting
3. Whether any discovered env vars are secrets that must use an obviously
   fake example value rather than a realistic-looking one
4. License, ownership, or contact info, if applicable

---

## Stage 3: Draft the Sections

Standard structure, in order. Skip sections that don't apply (e.g. no
Deployment section for a library with no deploy step) rather than leaving a
section that says "N/A."

| #   | Section                      | Purpose                                           |
| --- | ---------------------------- | ------------------------------------------------- |
| 1   | Title & one-line description | What this project is, in one line                 |
| 2   | Overview                     | The elevator pitch — purpose and key capabilities |
| 3   | Project Structure            | Directory layout, including `.code-assist/`       |
| 4   | Prerequisites                | Required tool/runtime versions                    |
| 5   | Getting Started              | Step-by-step setup from clone to running          |
| 6   | Available Commands           | Every package/build/test/run command              |
| 7   | Environment Variables        | Every var, with description and example           |
| 8   | Development Workflow         | Branching, commits, review, testing, style        |
| 9   | Testing                      | How to run tests, coverage expectations           |
| 10  | Deployment                   | If applicable                                     |
| 11  | Troubleshooting / FAQ        | Optional, grows over time                         |
| 12  | License / Contact            | If applicable                                     |

**Section guidance:**

- **Overview** — Write this after everything else; it should stand alone as
  a summary someone reads before deciding to read further.
- **Project Structure** — Show the real top-level tree, not an idealized
  one. Always include and briefly annotate `.code-assist/agents/`,
  `.code-assist/prompts/`, and `.code-assist/skills/` — see the dedicated
  section below for how to describe these.
- **Prerequisites** — Use exact pinned versions where the project specifies
  them (e.g. from `.nvmrc`, `.python-version`, `go.mod`'s `go` directive),
  not vague "recent version of X."
- **Getting Started** — A new developer should be able to follow this
  section top to bottom with no other context and end up running the
  project.
- **Available Commands / Environment Variables** — See the dedicated
  standards below; these are the two sections most likely to go stale and
  most damaging when they do.
- **Development Workflow** — Document what the team actually does, not an
  aspirational process nobody follows. If unsure, ask rather than guess.
- **Testing** — How to run the suite, how to run a single test, and any
  coverage threshold that's enforced.

---

## Stage 4: Command Table Standard

Present commands in a table, grouped by lifecycle stage:

```markdown
### Setup

| Command       | Description          |
| ------------- | -------------------- |
| `npm install` | Install dependencies |

### Development

| Command       | Description                          |
| ------------- | ------------------------------------ |
| `npm run dev` | Start the dev server with hot reload |

### Build

| Command         | Description                |
| --------------- | -------------------------- |
| `npm run build` | Produce a production build |

### Test

| Command                 | Description                  |
| ----------------------- | ---------------------------- |
| `npm test`              | Run the full test suite      |
| `npm test -- <pattern>` | Run tests matching a pattern |

### Lint / Format

| Command        | Description           |
| -------------- | --------------------- |
| `npm run lint` | Check for lint errors |

### Run

| Command     | Description                 |
| ----------- | --------------------------- |
| `npm start` | Start the built application |
```

Every command listed must be verified to actually exist (a real script
entry, Makefile target, or CI step) before it goes in the README —
hallucinated commands are worse than no documentation, since they cost a
developer real debugging time.

---

## Stage 5: Environment Variable Table Standard

One table per logical group if there are many (Database, Auth, Third-Party
APIs, Feature Flags); a single table is fine for a handful:

```markdown
| Variable            | Description                                                 | Required             | Example                                           |
| ------------------- | ----------------------------------------------------------- | -------------------- | ------------------------------------------------- |
| `DATABASE_URL`      | Connection string for the primary Postgres database         | Yes                  | `postgresql://user:password@localhost:5432/appdb` |
| `PORT`              | Port the server listens on                                  | No (default: `3000`) | `3000`                                            |
| `STRIPE_SECRET_KEY` | Secret key for Stripe API calls — never commit a real value | Yes                  | `sk_test_XXXXXXXXXXXXXXXXXXXX`                    |
```

Rules:

- **Every variable actually read in code must appear here, and every
  variable here must actually be read in code.** Flag mismatches instead of
  silently resolving them.
- For secrets (API keys, passwords, tokens), the example value must be
  **obviously fake** — a placeholder pattern, never a real or
  real-looking credential. Add a short note like "never commit a real
  value" directly in the table for anything sensitive.
- Note the default when a variable is optional, so it's clear what happens
  if it's left unset.
- If the project has a `.env.example` file, this section and that file
  should say the same thing — treat a mismatch between them as a bug to
  flag, not something to quietly paper over.

---

## Stage 6: Development Workflow Standard

Cover, to the extent the project has one:

- **Branching model** — trunk-based, Git Flow, or whatever the team uses
- **Commit convention** — e.g. Conventional Commits, and whether it's
  enforced (commit hook, CI check)
- **Code review expectations** — required approvals, who reviews what
- **Testing philosophy** — what must be tested before merge, coverage
  expectations if any
- **Linting/formatting** — tools used and whether they're enforced
  automatically (pre-commit hook, CI gate) or left to convention

If the user has a canonical version of this section from another project in
the same standard, reuse it rather than redrafting — consistency across
projects is the point.

---

## Stage 7: Documenting `.code-assist`

Every project under this standard has `.code-assist/agents/`,
`.code-assist/prompts/`, and `.code-assist/skills/`. Document it like this:

```markdown
## AI Development Tooling

This project uses a standardized `.code-assist/` structure:

.code-assist/
├── agents/ # <what agents are for, one line>
├── prompts/ # <what prompts are for, one line>
└── skills/ # <what skills are for, one line>

### Skills

| Skill              | Purpose                                                        |
| ------------------ | -------------------------------------------------------------- |
| `prd-writing`      | Structured workflow for writing Product Requirements Documents |
| `readme-generator` | Generates/updates this README against the team standard        |
```

Pull each skill's one-line purpose from its `SKILL.md` frontmatter
`description` rather than re-describing it from scratch — that keeps this
table truthful without extra interviewing. Since this folder is expected to
grow over time, **this is the section most likely to be stale on any given
update pass** — always re-scan `.code-assist/skills/` (and `agents/`,
`prompts/` if populated) rather than trusting what the README currently
says is there.

---

## Stage 8: Validation Checklist

Before finalizing, confirm:

- [ ] Every command in the table was verified against an actual script,
      Makefile target, or CI step — none invented
- [ ] Every env var read in code appears in the table, and every var in the
      table is actually read somewhere in code
- [ ] No real secrets or credentials appear as example values
- [ ] The Project Structure section matches the actual current repo layout
- [ ] The `.code-assist` section reflects what's currently in
      `agents/`, `prompts/`, and `skills/` — not what was there last time
- [ ] Prerequisites list exact pinned versions where the project specifies them
- [ ] A developer with no prior context could clone, set up, and run the
      project using only this README
- [ ] The Development Workflow section matches how the team actually works,
      not an aspirational process
- [ ] If updating an existing README, any non-standard, hand-written
      content was preserved or explicitly flagged — never silently removed

---

## Output Style

- Markdown, consistent header hierarchy, no more than one H1 (the title)
- Tables for commands and environment variables — this is structured data
  and belongs in tables, not prose
- Fenced code blocks for every command and for the directory tree
- Plain, direct language — no marketing tone
- Keep the top of the file (Overview, Getting Started) short enough that
  someone can grasp the project in under a minute; push deeper detail
  (Troubleshooting, Deployment specifics) further down

## Common Pitfalls

- Documenting a command or env var that doesn't actually exist in the
  codebase — always verify, never infer from convention alone
- Real or realistic-looking secrets pasted in as "example" values
- Project Structure or `.code-assist` sections going stale as the project
  grows — this is exactly why this skill supports update mode, not just
  generation
- Copy-pasting a generic workflow section that doesn't reflect what this
  specific team actually does
- Overwriting hand-written sections during an update pass instead of
  preserving or flagging them
- A README that re-explains what the code already documents in comments,
  instead of covering what a newcomer actually needs to get running

---

## Quick-Reference Template

```markdown
# <Project Name>

<One-line description>

## Overview

<Elevator pitch: what this is, what it does, key capabilities>

## Project Structure

<Annotated top-level directory tree, including .code-assist/>

## Prerequisites

- <Runtime/tool> <pinned version>

## Getting Started

1. Clone the repo
2. <Install dependencies>
3. <Configure environment variables — see below>
4. <Run the project>

## Available Commands

### Setup

| Command | Description |
| ------- | ----------- |

### Development

| Command | Description |
| ------- | ----------- |

### Build

| Command | Description |
| ------- | ----------- |

### Test

| Command | Description |
| ------- | ----------- |

### Lint / Format

| Command | Description |
| ------- | ----------- |

### Run

| Command | Description |
| ------- | ----------- |

## Environment Variables

| Variable | Description | Required | Example |
| -------- | ----------- | -------- | ------- |

## Development Workflow

- Branching:
- Commit convention:
- Code review:
- Testing:
- Linting/formatting:

## Testing

<How to run tests, single-test invocation, coverage expectations>

## AI Development Tooling

.code-assist/
├── agents/
├── prompts/
└── skills/

### Skills

| Skill | Purpose |
| ----- | ------- |

## Troubleshooting

## License
```

---

## Adapting Across Languages & Stacks

Use this as a lookup when Stage 1 needs to know where to look for a given ecosystem:

| Ecosystem         | Manifest / Config                                | Where Commands Live                                       | Where Env Vars Live                                           |
| ----------------- | ------------------------------------------------ | --------------------------------------------------------- | ------------------------------------------------------------- |
| Node / TypeScript | `package.json`, `tsconfig.json`                  | `"scripts"` block in `package.json`                       | `.env`, `.env.example`, `process.env` calls                   |
| Python            | `pyproject.toml`, `requirements.txt`, `setup.py` | `Makefile`, `tox.ini`, Poetry `[tool.poetry.scripts]`     | `.env`, `os.environ`/`os.getenv`, Pydantic `Settings` classes |
| Java / Kotlin     | `pom.xml`, `build.gradle`                        | Maven goals (`mvn ...`), Gradle tasks (`./gradlew ...`)   | `application.yml`/`.properties`, `System.getenv`              |
| Go                | `go.mod`                                         | `Makefile`, `go build`/`go test`/`go run`                 | `os.Getenv` calls, `.env` via a dotenv library                |
| Rust              | `Cargo.toml`                                     | `cargo build`/`test`/`run`, `justfile`                    | `std::env::var` calls, `.env` via a dotenv crate              |
| Ruby              | `Gemfile`                                        | `Rakefile`, `bundle exec` commands                        | `.env`, `ENV[...]` calls                                      |
| PHP               | `composer.json`                                  | Composer scripts, `Makefile`                              | `.env`, `getenv()`/`$_ENV`                                    |
| Containers        | `Dockerfile`, `docker-compose.yml`               | `docker build`/`run`, `docker compose` commands           | `environment:` blocks, Dockerfile `ENV`/`ARG`                 |
| Any (cross-check) | `.github/workflows/*`, `.gitlab-ci.yml`          | Pipeline job `run` steps — often the most accurate source | CI secrets/variables configuration                            |

A project spanning multiple ecosystems (e.g. a monorepo with a Node frontend
and a Python backend) should document each stack's commands and env vars in
clearly separated subsections rather than merging them into one table.

## Adapting This Skill to Your Team

The section names and table formats above can be adjusted to match a
different house style, but the underlying guarantees should hold regardless:
every command is verified against the actual codebase, every environment
variable is documented and cross-checked against where it's actually read,
the `.code-assist` structure is re-scanned (not assumed) on every update,
and no hand-written content is silently discarded during an update pass.
