---
name: prd-writing
description: Guide for drafting, structuring, and validating Product Requirements Documents (PRDs) through a repeatable Discovery -> Structure -> Draft -> Validate -> Review workflow. Use whenever the user asks to write a PRD, product spec, feature requirements doc, one-pager, or similar product documentation; when they're kicking off a new feature or initiative and need to turn an idea into a structured requirements doc; or when they want to review, tighten, fill gaps in, or validate the completeness of an existing PRD draft. Trigger even if the user doesn't use the exact word "PRD" — phrases like "spec this out," "write up the requirements for X," or "help me scope this feature" all qualify.
---

# PRD Writing Workflow

## Purpose

A PRD's job is to let people who weren't in the room make correct decisions without guessing. It aligns engineering, design, and stakeholders on **what** is being built and **why**, before the team commits real engineering time to **how**. The output of this skill is not "a document that looks like a PRD" — it's a decision record specific enough that an engineer unfamiliar with prior discussions could implement it correctly, and a stakeholder could sign off knowing exactly what they are, and aren't, agreeing to.

Keep that bar in mind throughout: every section below exists to close a specific gap that causes PRDs to fail in practice (vague goals, undiscovered scope, unreviewable requirements). If a step feels like busywork for the task at hand, it's fine to lighten it — see Stage 0 — but understand what it's protecting against before skipping it.

## When to Use This Skill

Trigger on requests such as:

- "Write a PRD for X" / "Draft requirements for X"
- "Help me spec out this feature"
- "I'm starting a new initiative and need a requirements doc"
- "Review this PRD draft and tell me what's missing"
- "Tighten up these requirements" / "Is this spec ready to share?"

Work through the stages below by default. If the user explicitly declines the structured process ("just write it," "skip the questions"), move straight to drafting using reasonable assumptions, and flag those assumptions clearly in the output so they're easy to correct.

## Guiding Principles

These are the "why" behind every stage that follows:

1. **Specificity beats completeness for its own sake.** A short PRD where every line is decision-useful beats a long one padded with boilerplate. Cut anything that doesn't change what someone will build or how they'll evaluate it.
2. **Requirements must be testable.** If nobody can look at the shipped feature and say pass/fail against a requirement, it isn't a requirement yet — it's a hope.
3. **Separate the "what/why" from the "how."** A PRD defines the problem and the constraints on the solution; it generally should not dictate implementation details unless those details are the whole point (e.g., a technical PRD, or a hard external constraint). Over-specifying the "how" removes the engineering team's ability to find a better path and dates the doc quickly.
4. **Right-size the document to the decision.** A one-day fix and a multi-quarter cross-team initiative should not go through the same amount of process. See Stage 0.
5. **A PRD is a living record, not a one-shot artifact.** Scope changes; the doc should show its history rather than silently drift, so readers can trust the current version.

## Workflow Overview

| Stage                 | Goal                                                                         | Output                                             |
| --------------------- | ---------------------------------------------------------------------------- | -------------------------------------------------- |
| 0. Calibrate Depth    | Decide how much process this PRD actually needs                              | A target tier (Lightweight / Standard / Strategic) |
| 1. Context Gathering  | Understand the problem well enough to write about trade-offs, not just facts | Answered core questions + raw context              |
| 2. Structure          | Choose which sections apply and in what order                                | A section outline                                  |
| 3. Drafting           | Fill each section to a testable, specific standard                           | Full draft                                         |
| 4. Validation         | Check the draft against completeness and quality bars                        | Gap list, then a corrected draft                   |
| 5. Review & Iteration | Track changes and open questions as the doc evolves                          | Versioned, reviewable PRD                          |

Work through these stages in order for a new PRD. For an existing draft, jump straight to Stage 4 (Validation) unless the user wants a fuller rewrite.

## Stage 0: Calibrate Depth

Before gathering context, get a rough sense of scope so the rest of the process isn't oversized or undersized. Ask yourself (or the user, briefly) which tier this is closer to:

- **Lightweight** — a single feature, small UI change, or well-understood addition with one team involved. Skip Timeline, Appendix, and formal sign-off tracking; keep the rest brief.
- **Standard** — a typical feature with real design and engineering trade-offs, one or two teams, weeks of work. Use the full structure in Stage 2.
- **Strategic** — a cross-team or multi-quarter initiative with organizational visibility. Use the full structure plus a fuller Risks/Dependencies section, explicit stakeholder sign-off, and a rollout/timeline section.

Don't force this into a formal question to the user unless it's genuinely ambiguous — often the initial ask makes the tier obvious ("quick PRD for a button change" vs. "we're planning next quarter's biggest bet"). State your assumed tier back to the user in one line so they can correct it before you invest in drafting.

## Stage 1: Context Gathering

The goal here isn't to collect trivia — it's to reach a point where you could discuss trade-offs in this problem space without needing the basics re-explained. Stop asking once you're there; more questions past that point slow the user down for no benefit.

Ask about whatever isn't already known from the conversation:

1. What problem does this solve, and for whom?
2. Who are the target users? (Be specific — "power users on the admin dashboard," not "users.")
3. What does success look like, and how will it be measured?
4. What technical constraints, dependencies, or existing systems does this need to work within?
5. What's explicitly out of scope, or already decided against?
6. What's the timeline or priority relative to other work?
7. Is there an existing template, house style, or prior PRD this should match?

Invite a context dump rather than a rigid Q&A — user research notes, competitive analysis, architecture diagrams, stakeholder requirements, prior discussion threads. Raw, messy context is more useful here than polished answers; you can structure it later.

From the gaps in what's provided, ask 5-8 targeted clarifying questions — enough to remove ambiguity, not so many that it feels like an interrogation. Skip questions whose answers are already implied by the conversation or by a linked template.

## Stage 2: Document Structure

Adjust this to match an existing company or team template if one exists — consistency with what the team already reads matters more than this specific section list. Otherwise, use the structure below. The **Tier** column reflects Stage 0: include a section if the PRD's tier is at or above the tier listed.

| #   | Section                        | Purpose                                                                 | Min. Tier   |
| --- | ------------------------------ | ----------------------------------------------------------------------- | ----------- |
| 1   | Title & Metadata               | Author, date, status (Draft/In Review/Approved), stakeholders, version  | Lightweight |
| 2   | Overview / Problem Statement   | The problem in plain terms and the proposed solution in one paragraph   | Lightweight |
| 3   | Goals & Success Metrics        | What success looks like, in measurable terms                            | Lightweight |
| 4   | Non-Goals / Out of Scope       | What this explicitly will not address, and why                          | Lightweight |
| 5   | User Stories / Use Cases       | Who benefits and how, from their perspective                            | Standard    |
| 6   | Requirements                   | Functional and non-functional requirements, each testable               | Standard    |
| 7   | UX / Design Considerations     | Links to mocks/wireframes, key interaction decisions                    | Standard    |
| 8   | Dependencies & Risks           | What this relies on, what could go wrong, and mitigations               | Standard    |
| 9   | Open Questions                 | Unresolved decisions, explicitly tracked rather than silently assumed   | Standard    |
| 10  | Rollout Plan / Timeline        | Milestones, phased rollout, feature flags                               | Strategic   |
| 11  | Appendix / Supporting Research | Data, links, prior art that informed the doc but would clutter the body | Strategic   |

### Section-by-section guidance

- **Overview / Problem Statement** — Lead with the problem, not the solution. A reader should understand _why this matters_ before they read _what we're building_. One paragraph each is usually enough.
- **Goals & Success Metrics** — Metrics need a baseline, a target, and a timeframe ("reduce checkout abandonment from 34% to under 25% within one quarter of launch," not "improve checkout conversion"). Distinguish north-star/lagging metrics from the leading indicators the team will actually watch week to week.
- **Non-Goals / Out of Scope** — This is one of the highest-leverage sections and the most commonly skipped. It's what stops silent scope creep and keeps reviewers from assuming their pet feature is included. Write it as decisions ("we are not supporting X in this phase"), not just a topic list.
- **User Stories** — Standard "As a [user], I want [goal], so that [benefit]" format works well, but don't force every requirement into story form if a direct statement is clearer.
- **Requirements** — Split functional (what the system does) from non-functional (performance, security, accessibility, scalability, reliability). Give each requirement a short ID (e.g., `REQ-01`) so it can be referenced in tickets, tests, and review comments later. See the format guidance below.
- **Risks & Dependencies** — Every listed risk needs an owner and, where possible, a mitigation or contingency — a risk list with no mitigations is just a list of things to worry about.
- **Open Questions** — Don't let unresolved decisions block the whole draft. Log them here with an owner and a "resolve by" point, and move on.

## Stage 3: Section-by-Section Drafting

For each section in the chosen structure, run this loop rather than writing top-to-bottom in one pass:

1. **Clarify** — ask any section-specific questions not already covered in Stage 1.
2. **Brainstorm** — generate 5-10 candidate items (requirements, user stories, risks, etc.) before narrowing down. Breadth first avoids anchoring on the first idea.
3. **Curate** — let the user pick, cut, and reprioritize from the brainstormed list rather than silently deciding for them.
4. **Draft** — write the section to the format standards below.
5. **Refine** — incorporate feedback before moving to the next section.

### Writing style and format standards

- **Use imperative, unambiguous language for requirements.** Prefer "The export must complete within 5 seconds for files under 10MB" over "The export should be fast." Vague qualifiers — _fast_, _robust_, _scalable_, _user-friendly_, _seamless_ — are a signal to add a number or a concrete condition.
- **Use requirement-strength keywords deliberately**: _must_ (hard requirement), _should_ (strong default, may be waived with justification), _may_ (optional). Keeping this distinction consistent lets reviewers scan for what's actually load-bearing.
- **Prioritize explicitly.** MoSCoW (Must/Should/Could/Won't) or P0/P1/P2 both work — pick one and apply it consistently across all requirements so trade-off conversations have a shared vocabulary.
- **Give every requirement an ID and an acceptance criterion.** This is what makes a PRD traceable into tickets and test plans later, not just a narrative document.

**Requirements table pattern:**

```markdown
| ID     | Requirement                               | Priority | Acceptance Criteria                                                                            |
| ------ | ----------------------------------------- | -------- | ---------------------------------------------------------------------------------------------- |
| REQ-01 | Users can export their data as CSV        | Must     | Export completes in <5s for files under 10MB; malformed rows are skipped with a logged warning |
| REQ-02 | Export respects the user's active filters | Must     | Exported rows match exactly what's shown in the filtered table view                            |
```

**User story pattern:**

```markdown
**Input (raw ask):** "Admins need to see who changed a record."
**Output (drafted story):** As an admin, I want to see a change history on each record,
so that I can audit who modified sensitive data and when.
```

## Stage 4: Validation

Run two separate passes — completeness and quality are different failure modes, and a section can pass one while failing the other.

**Completeness checklist** (does every required section exist and hold real content?):

- [ ] Problem is clearly and specifically stated
- [ ] Target users are named, not just implied
- [ ] Success metrics exist and are measurable
- [ ] Non-goals / out-of-scope items are explicit
- [ ] Requirements cover both functional and non-functional needs
- [ ] Dependencies and risks are identified
- [ ] Open questions are logged rather than silently unresolved

**Quality checklist** (is the content actually usable, not just present?):

- [ ] Every requirement is testable — someone could verify pass/fail against it
- [ ] Every metric has a baseline, target, and timeframe
- [ ] Non-goals read as decisions, not vague caveats
- [ ] Every risk has an owner and a mitigation or contingency
- [ ] The doc leads with problem/why before solution/how
- [ ] No section overspecifies implementation beyond what's actually constrained

When a check fails, don't just flag it — return to the relevant part of Stage 3's loop for that section rather than patching the gap in isolation.

## Stage 5: Review & Iteration

A PRD keeps working as a source of truth only if changes are visible rather than silent:

- Maintain a lightweight **version/changelog** (date, author, summary of change) once the doc moves past first draft — even two or three lines per revision is enough.
- Track a **status** field (Draft / In Review / Approved / Shipped) in the metadata so readers know how much weight to give the current content.
- For Standard/Strategic tier docs, track **stakeholder sign-off** explicitly (who needs to approve, who has) rather than assuming silence means agreement.
- When scope changes materially after approval, don't quietly edit the original goals or requirements — note what changed and why, so anyone who approved the original version can see what they're now implicitly agreeing to.

## Common Pitfalls to Avoid

- **Solutioning too early.** Jumping to "how" before the "why" and "what" are solid tends to lock in a worse solution and makes the doc harder to review.
- **Vague, unmeasurable success metrics.** "Improve user engagement" isn't a metric — it's a direction. Push for a number and a timeframe.
- **Treating the PRD as static.** Docs that don't reflect reality after week one lose the team's trust and stop being consulted.
- **Requirements that restate the feature instead of constraining it.** "The feature will let users export data" is a description; "Exports must complete in under 5 seconds" is a requirement.
- **Skipping Non-Goals.** Silence on scope is where scope creep and reviewer disagreements come from.
- **Wall-of-text sections with no structure.** Tables and short lists are easier to review and reference in tickets than prose paragraphs.

## Adapting This Skill Across Projects

This file is meant to be a shared starting scaffold, not a rigid mold — teams and projects should fork the section list in Stage 2 to match their own conventions while keeping the core Discovery -> Structure -> Draft -> Validate -> Review loop intact, since that loop is what prevents the common failure modes above regardless of the specific template used.

The workflow is intentionally decoupled from any particular tool or model: it doesn't assume a specific document editor, ticketing system, or AI assistant's mechanics — it works whether the PRD ends up in Markdown, Google Docs, Confluence, Notion, or Word, and whether it's drafted by a person or with AI assistance. Anything tool-specific (e.g., "paste this into Jira," "use our Confluence macro") belongs in a team-specific addendum, not in this base file, so the base file stays reusable across projects.

## Appendix: Ready-to-Copy PRD Skeleton

```markdown
# [Feature/Product Name] — PRD

**Author:**
**Date:**
**Status:** Draft | In Review | Approved | Shipped
**Stakeholders:**
**Version:** 1.0

## Overview

[Problem statement — one paragraph]
[Proposed solution — one paragraph]

## Goals & Success Metrics

- Goal:
- Metric: [baseline] -> [target] within [timeframe]

## Non-Goals / Out of Scope

-

## User Stories

- As a [user], I want [goal], so that [benefit].

## Requirements

| ID     | Requirement | Priority          | Acceptance Criteria |
| ------ | ----------- | ----------------- | ------------------- |
| REQ-01 |             | Must/Should/Could |                     |

## UX / Design Considerations

[Links to mocks/wireframes; key interaction decisions]

## Dependencies & Risks

| Risk | Impact | Owner | Mitigation |
| ---- | ------ | ----- | ---------- |
|      |        |       |            |

## Open Questions

| Question | Owner | Resolve By |
| -------- | ----- | ---------- |
|          |       |            |

## Rollout Plan / Timeline

[Milestones, phased rollout, feature flags — Strategic tier only]

## Appendix

[Supporting research, data, prior art]
```
