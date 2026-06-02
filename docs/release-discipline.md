# Release Discipline

Operator-facing checklist for asserting that the byte-equal parity gate (AC-D5 / AC-H3 of the Rust port plan) is enforced server-side. The repo-local `.github/workflows/_validate-workflows.yml` meta-test covers the YAML side of the gate; this doc covers the GitHub-side branch-protection rules that are NOT visible to repo-local CI.

## When to run this audit

- Before every minor release (cutting a release branch from `main`).
- After any change to `.github/workflows/parity.yml`.
- After any change to GitHub branch-protection settings (manual UI changes or `gh api` PATCH calls).
- After a fresh repository fork or org transfer.

## Branch-protection audit

Run the following command (requires `gh` CLI authenticated with the `repo` scope and `admin:repo_hook` on this repo):

```bash
gh api repos/:owner/:repo/branches/main/protection \
  --jq '{
    enforce_admins: .enforce_admins.enabled,
    required_status_checks: .required_status_checks.checks | map(.context),
    require_review: .required_pull_request_reviews.required_approving_review_count,
    dismiss_stale_reviews: .required_pull_request_reviews.dismiss_stale_reviews,
    require_code_owner_reviews: .required_pull_request_reviews.require_code_owner_reviews
  }'
```

### Expected response (minimum required)

```json
{
  "enforce_admins": true,
  "required_status_checks": [
    "Committed-fixture byte-equal parity",
    "Workflow integrity (parity gate cannot be silently disabled)",
    "Rust workspace tests"
  ],
  "require_review": 1,
  "dismiss_stale_reviews": true,
  "require_code_owner_reviews": false
}
```

If any of the following is missing, the release is BLOCKED and the audit FAILS:

- `enforce_admins: true` — without this, repository admins can bypass the parity gate. Per the AC-D5 contract, the parity gate is CI-BLOCKING with NO admin override.
- `required_status_checks` MUST include `"Rust workspace tests"` (the Rust CI job name from `.github/workflows/ci.yml`), `"Committed-fixture byte-equal parity"` (the parity job name from `.github/workflows/parity.yml`), and `"Workflow integrity (parity gate cannot be silently disabled)"` (the meta-test job name from `.github/workflows/_validate-workflows.yml`).
- `require_review >= 1` — at minimum one approving review is required. This is a defense-in-depth measure on top of the parity gate; a parity-green build with no human review is not mergeable.

## Manual evidence collection

After running the audit, copy the `gh api` JSON output into the release notes for the next release tag. The release tag MUST carry a `release-audit-<date>.json` artifact (committed to `.github/release-artifacts/` or attached to the GitHub Release) capturing the branch-protection settings at the moment of release.

## Failure modes

| Symptom | Cause | Recovery |
|---------|-------|----------|
| `required_status_checks` is empty | Branch protection rule was deleted (or never created) | Re-create per the snippet below |
| `enforce_admins: false` | Admin opted out of own gate | Re-enable via UI or `gh api` PATCH |
| Parity check name missing from `required_status_checks` | Workflow name was renamed without updating branch protection | Update branch protection to match the workflow's job-name |
| `_validate-workflows` check name missing | New meta-workflow was not registered with branch protection | Add to required_status_checks |

## Setting up branch protection from scratch

For a fresh fork or after a recovery:

```bash
gh api -X PUT repos/:owner/:repo/branches/main/protection \
  -F enforce_admins=true \
  -F 'required_status_checks[strict]=true' \
  -F 'required_status_checks[checks][][context]=Committed-fixture byte-equal parity' \
  -F 'required_status_checks[checks][][context]=Workflow integrity (parity gate cannot be silently disabled)' \
  -F 'required_status_checks[checks][][context]=Rust workspace tests' \
  -F 'required_pull_request_reviews[required_approving_review_count]=1' \
  -F 'required_pull_request_reviews[dismiss_stale_reviews]=true' \
  -F 'allow_force_pushes=false' \
  -F 'allow_deletions=false'
```

## References

- Plan AC-D5: `.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md` §M0 AC-D5.
- Plan AC-H3: same plan §M2 AC-H3.
- Workflow files: `.github/workflows/parity.yml`, `.github/workflows/_validate-workflows.yml`.
