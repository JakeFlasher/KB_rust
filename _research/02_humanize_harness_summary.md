# Humanize Agentic Harness — Exploration Summary

Read-only reference: `/home/jakeshea/humanize/`. Captured here for use as research input only.

## What it is

Humanize is a Claude Code plugin (v1.17.0) implementing **RLCR** (Ralph-Loop with Codex Review): Claude implements plans while independent Codex reviews provide continuous quality feedback. Derived from the GAAC (GitHub-as-a-Context) project.

## Core philosophy

1. Iteration over Perfection — continuous refinement, not perfect first output.
2. One Build + One Review — Claude implements, Codex independently reviews.
3. Ralph Loop with optional Swarm Mode (Agent Teams).
4. Begin with the End in Mind — pre-flight Plan Understanding Quiz verifies human understands the plan.

## Slash commands

### `/humanize:gen-idea` — directed-swarm exploration

Args: `<idea-text-or-path> [--n <int>] [--output <path>]`.

Phases:

1. Parse input (inline text or `.md` path; `--n` default 6; `--output` default `.humanize/ideas/<slug>-<timestamp>.md`).
2. IO validation via `scripts/validate-gen-idea-io.sh`.
3. Generate exactly N orthogonal directions from repo context.
4. Dispatch N subagents in a single message; each returns APPROACH_SUMMARY, OBJECTIVE_EVIDENCE, KNOWN_RISKS, CONFIDENCE.
5. Synthesis: pick strongest as PRIMARY, render alternatives as Alt-1..Alt-K.

Output is draft-only; no code, no commits.

### `/humanize:gen-plan` — draft → structured plan (10 phases)

Args: `--input <path> --output <path> [--auto-start-rlcr-if-converged] [--discussion|--direct]`.

Sequence:

0. Parse CLI flags.
0.5. Merge config: defaults → `~/.config/humanize/config.json` → `.humanize/config.json`. Extracts `alternative_plan_language` and `gen_plan_mode`.
1. IO validation.
2. Relevance check via `humanize:draft-relevance-checker` (haiku).
3. Codex first-pass analysis via `ask-codex.sh`; captures CORE_RISKS, MISSING_REQUIREMENTS, TECHNICAL_GAPS, ALTERNATIVE_DIRECTIONS, QUESTIONS_FOR_USER, CANDIDATE_CRITERIA.
4. Claude candidate plan v1 using Explore agents.
5. Iterative convergence loop (discussion mode): max 3 rounds of second-pass Codex review; Codex outputs AGREE | DISAGREE | REQUIRED_CHANGES | OPTIONAL_IMPROVEMENTS | UNRESOLVED. Sets PLAN_CONVERGENCE_STATUS=converged | partially_converged. Direct mode skips convergence.
6. Issue & disagreement resolution; consolidates pending user decisions into plan's `## Pending User Decisions`.
7. Final plan generation with required sections (Goal Description, Acceptance Criteria, Path Boundaries, Feasibility Hints, Dependencies & Sequence, Task Breakdown, Claude-Codex Deliberation, Pending User Decisions, Implementation Notes).
8. Write + optional translated variant; optional auto-start if conditions met.

Task routing: every task **must** have exactly one tag — `coding` (Claude) or `analyze` (Codex).

### `/humanize:refine-plan` — refine annotated plan + generate QA ledger (8 phases)

Stateful comment scanner supports `CMT: ... ENDCMT`, `<cmt>...</cmt>`, `<comment>...</comment>` (inline and multi-line); respects fenced code and HTML comments. Classifies as `question | change_request | research_request`, then processes (answer / apply / research / defer / resolve). Produces refined plan + QA doc (Summary, Comment Ledger, Answers, Research Findings, Plan Changes Applied, Remaining Decisions, Refinement Metadata). Atomic write transaction (temp files → atomic rename).

### `/humanize:start-rlcr-loop` — iterative loop

Args: `[path/to/plan.md | --plan-file path] [--max N] [--codex-model MODEL:EFFORT] [--codex-timeout SECONDS] [--track-plan-file] [--push-every-round] [--base-branch BRANCH] [--full-review-round N] [--skip-impl] [--claude-answer-codex] [--agent-teams] [--yolo] [--skip-quiz] [--privacy]`.

Pre-checks:

- Plan compliance pre-check via `humanize:plan-compliance-checker` (sonnet): repo relevance + no branch-switching instructions.
- Plan Understanding Quiz via `humanize:plan-understanding-quiz` (opus): 2 multiple-choice questions + plan summary; user answers; if wrong → show summary + ask user to proceed or review.

Then `scripts/setup-rlcr-loop.sh $ARGUMENTS` initializes `.humanize/rlcr/<timestamp>/` with `state.md`, `goal-tracker.md`, and ensures `.humanize/bitlesson.md` exists.

Loop state machine:

```
IMPLEMENTATION PHASE
- Round N: read goal-tracker.md, for each task:
    coding → Claude implements; analyze → /humanize:ask-codex
  For each task: run bitlesson-selector → apply selected lessons.
  Write round-N-summary.md (changes, tests, BitLesson Delta, Knowledge Consulted).
  Attempt exit → Stop Hook fires.
  Codex reviews summary; classifies Mainline Gaps | Blocking Issues | Queued; verdict ADVANCED | STALLED | REGRESSED.
  If issues → round-N-review-result.md blocks exit, Claude iterates.
  If COMPLETE → REVIEW PHASE.
  If STOP → circuit breaker (stagnation).
REVIEW PHASE
- `codex review --base <branch>` checks code quality with [P0-9] severity markers.
- If issues → Claude fixes; if none → FINALIZE PHASE.
FINALIZE PHASE
- Final summary; Codex confirms; optional methodology analysis (skipped if --privacy).
```

Goal tracker has two sections: IMMUTABLE (Ultimate Goal + ACs copied from plan) and MUTABLE (Active, Completed/Verified, Blocked, Explicitly Deferred, Blocking Side Issues, Queued Side Issues, Plan Evolution Log).

Full Alignment Checks at rounds N-1, 2N-1, 3N-1, ... per `--full-review-round` (default 5).

### `/humanize:cancel-rlcr-loop` — graceful cancel

Returns NO_ACTIVE_LOOP | CANCELLED | CANCELLED_METHODOLOGY_ANALYSIS | CANCELLED_FINALIZE | FINALIZE_NEEDS_CONFIRM.

## Skills (user-facing wrappers)

- `humanize:ask-codex` — one-shot Codex consultation; output to `.humanize/skill/<timestamp>/output.md`.
- `humanize:ask-gemini` — Gemini research consultation (always web-research-backed).
- `humanize:humanize-gen-plan`, `humanize:humanize-refine-plan`, `humanize:humanize-rlcr`, `humanize:humanize` — wrapper skills exposing commands.

## Agents

- `humanize:bitlesson-selector` (haiku) — picks relevant BitLesson IDs.
- `humanize:draft-relevance-checker` (haiku) — RELEVANT vs NOT_RELEVANT.
- `humanize:plan-compliance-checker` (sonnet) — PASS | FAIL_RELEVANCE | FAIL_BRANCH_SWITCH.
- `humanize:plan-understanding-quiz` (opus) — 2 MCQs + plan summary.

## Hooks (`hooks/hooks.json`)

- `UserPromptSubmit` → `loop-plan-file-validator.sh` (track-plan-file enforcement).
- `PreToolUse` Write/Edit/Read/Bash validators (protect state files, allowlist paths, block dangerous git).
- `PostToolUse` Bash hook captures output for Codex review context.
- `Stop` → `loop-codex-stop-hook.sh` (timeout 7200s = 2h) — **core RLCR engine**: detects active loop, runs Codex review per round, transitions phases.

## State directory

`.humanize/rlcr/<timestamp>/`:

```yaml
---
plan_tracked: true|false
start_branch, base_branch, base_commit
plan_file
current_round, max_iterations
push_every_round
codex_model, codex_effort, codex_timeout
review_started, full_review_round, ask_codex_question
session_id (UUID)
agent_teams, privacy_mode
bitlesson_required, bitlesson_file, bitlesson_allow_empty_none
mainline_stall_count, last_mainline_verdict (advanced|stalled|regressed|unknown)
drift_status (normal|replan_required)
---
```

Transitions: `state.md` → `finalize-state.md` (Codex COMPLETE) | `cancel-state.md` (user cancel) | `methodology-analysis-state.md` (privacy mode exit).

## BitLesson contract

Per-round summary must include:

```markdown
## BitLesson Delta
- Action: none|add|update
- Lesson ID(s): <IDs or NONE>
- Notes: <what changed and why>
```

Lesson entry structure:

```markdown
## Lesson: <unique-id>
Lesson ID: BL-YYYYMMDD-short-name
Scope: <component/subsystem/files>
Problem Description: <specific failure mode with trigger>
Root Cause: <direct technical cause>
Solution: <exact fix>
Constraints: <limits, assumptions>
Validation Evidence: <tests/logs/PR evidence>
Source Rounds: <round numbers where problem appeared + solved>
```

## "Knowledge Consulted" provenance contract

Every round summary must include:

```markdown
## Knowledge Consulted
- `.claude/knowledge/base.md` — why opened
- `docs/schema.md` — why opened
# or
N/A -- task not KB-relevant this round
```

Codex's regular-review.md performs a **Knowledge Provenance Check**: if work touches domain/modeling/metrics tasks, the section must list concrete paths or explicitly state N/A. Missing on KB-relevant work → Blocking Side Issue.

## Config hierarchy

`config/default_config.json`:

```json
{
  "codex_model": "gpt-5.5",
  "codex_effort": "high",
  "bitlesson_model": "haiku",
  "agent_teams": false,
  "alternative_plan_language": "",
  "gen_plan_mode": "discussion"
}
```

Merge order: defaults → `~/.config/humanize/config.json` → `${HUMANIZE_CONFIG:-.humanize/config.json}`.

## Strengths to preserve

- Modular composition: commands × agents × skills × hooks.
- Transparent: every decision recorded in artifacts.
- Knowledge-aware: enforces traceability via Knowledge Consulted + Codex provenance audits.
- Deterministic: frontmatter-driven config, strict parsing.
- Extensible: config hierarchy, alternative providers, agent teams.

## Friction observed

- The harness is **plan-and-implementation-focused**, not knowledge-card-focused. It enforces *traceability* of which KB files were opened, but does not verify the **content** of generated knowledge.
- BitLessons require manual curation; capture is round-summary-driven, not automatic from CI/lint signal.
- The stop-hook timeout is 7200s — long for short tasks, short for very deep ones. No adaptive scaling.
- "Knowledge Consulted" check is post-hoc text-pattern, not active retrieval verification.
- No content-addressable verification: a card cited as `.claude/knowledge/foo.md` could have changed between rounds without any flag.
