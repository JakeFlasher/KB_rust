---
schema_version: "cacg.v0"
id: "cb-china-no-call-commitments"
title: "China Convertible-Bond No-Call Commitments (不赎回承诺)"
reading_id: "08_convertible_bonds"
summary: "Chinese CB prospectuses give the issuer a strong-call (强赎) right activated when the share price crosses 130%·K_c for 15 of 30 trading days. Under post-2018 practice, when triggered, a majority of issuers publicly issue a 不赎回承诺 — a voluntary commitment not to exercise the call for a specified window (often 12 months) — preferring to delay forced conversion. The 2021 Zheshang/Huatai survey: 57% o..."
tags: ["convertible-bonds", "china-no"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p040:0022"
    chunk_hash: "6c641188f21c64a9af394b056a2dab99c68937e4b7262594407ed2ac2caeb9ea"
    page_range: [40, 41]
    quote: "近些年，有的可转债明明已经触収了强赎条件，但最终却明确 公告不强赎。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p071:0068"
    chunk_hash: "c0cd048f2f0122d728aff1c012021176e80927b5a894690840c2af944694cf1f"
    page_range: [71, 72]
    quote: "如果公司股票在5年内碰上一个大牛市，或者公司在5年内遇到或者创 造一次以上的大利好"
    edge_type: "supports"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p018:0014"
    chunk_hash: "95ff1ecb153d3a50520c3bcda12fe3d2e157ae46a7be808eff64935f23ac089c"
    page_range: [18, 19]
    quote: "中装转债流通面值为2978.94万元，已低于3000万元， 已经触发《募集说明书》中约定的有条件赎回条款。"
    edge_type: "supports"
card_hash: "9a945e56c7102bc5dae8a09728da86deb99d464a1eecf1e253bd6d931caab3b3"
---
# China Convertible-Bond No-Call Commitments (不赎回承诺)

## Intuition

China onshore CB prospectuses give the issuer a **strong-call** (强赎)
right that activates once the share price crosses a 130%-of-strike
threshold for 15 of the last 30 trading days (see
[cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md)
for the trigger mechanics). When the threshold is breached, the issuer
may either pull the strong-call (forcing conversion) or **voluntarily
commit not to call for a stated window** (不赎回承诺). This second path
is a unilateral contractual commitment: the issuer publicly declares it
will not exercise the strong-call right for the committed period,
giving CB holders a temporary signal that the issuer prefers continued
debt-route accrual over immediate dilution. **Source:** 安道全 (2023 3ed)
§3 pp.41-45; 攻守 §2 pp.17-48 (强赎 + 不赎回 mechanism cross-check).

```
不赎回承诺 lifecycle

  strong-call trigger met (S/K_c ≥ 130% on 15 of last 30 days)
                       │
            ┌──────────┴──────────┐
            │                     │
  Option A: pull 强赎     Option B: 公告不赎回 (commit not to call)
            │                     │
            │                     │  commitment window:
            │                     │  typically 3-6 months
            │                     │  (issuer-stated)
            │                     │
   forced conversion       window expires →
   within ~30 days          trigger re-armed; issuer
                            re-decides Option A vs B
```

## Definition

A **不赎回承诺** is a public commitment by the issuer (typically the
board, sometimes the controlling shareholder) to not exercise the
strong-call right for a stated window even though the prospectus
trigger condition is currently met. The commitment is announced via the
SSE / SZSE issuer-disclosure channel along with the strong-call-
trigger-met announcement (公告). The structural features of a typical
commitment are: (a) a fixed **window length** (3-6 months in practice,
sometimes longer), (b) explicit waiver of the call right within that
window even if the price re-meets the trigger, and (c) auto-re-arming
of the trigger when the window expires. **Source:** 安道全 (2023 3ed)
§3 pp.41-45.

## Mathematical Reasoning

Let `W` denote the committed no-call window length and `T_c_eff` the
effective post-commitment call-eligible date. Without the commitment,
the call-eligible date is `T_c_trigger = t_now` (the moment the trigger
condition is satisfied). With the commitment, `T_c_eff = t_now + W`,
shifting the issuer's optionality forward by W. **Source:** 安道全
(2023 3ed) §3 pp.41-45; DeSpiegeleer (2014) §3 pp.50-78 (general call-
protection-window framework that no-call commitments instantiate).

```
without commitment:    T_c_eff = t_now (immediate)
with W-month commitment: T_c_eff = t_now + W
```

The **credible-commitment** structure is contractual rather than
strategic: the commitment is enforceable through securities-disclosure
law (the issuer's public announcement creates a fiduciary obligation
not to act inconsistently), and reneging would trigger securities-law
penalties + reputational damage. The structural distinction from a
prospectus-level no-call window (which is fixed at issuance) is that
this is a **discretionary post-trigger** commitment: the issuer chooses
whether to commit at each trigger event, and may decline the
commitment to pull the call immediately. **Source:** 安道全 (2023 3ed)
§3 pp.41-45; 攻守 §2 pp.17-48 (commitment vs. immediate-call decision
discipline).

The CB market response to a 不赎回 announcement is empirically a small
positive move on the bond (uncertainty resolution favoring continued
equity-route accrual) and a smaller negative move on the underlying
share (suppressed dilution-uncertainty premium). The **valuation
impact** maps to a temporary cap on the issuer-call hazard rate
`λ_call(t) → 0` for `t ∈ [t_now, t_now + W]`, recovered to baseline at
`t = t_now + W`. **Source:** 安道全 (2023 3ed) §3 pp.41-45 (post-
announcement empirical CB-price + share-price behaviour, qualitative
direction without worked numerics).

The decision-mechanic comparison with the **强赎 (immediate call)**
path is covered in [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md);
the full **strategic-interaction game theory** (issuer optimization over
the commit-vs-call choice, holder belief-formation, equilibrium
selection) is carried by the sibling card
[cb-china-strong-call-game-theory](./cb-china-strong-call-game-theory.md).
This card stays at the mechanics + valuation-impact layer.
**Source:** DeSpiegeleer (2014) §3 pp.50-78.

## See Also

- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — strong-call trigger mechanics (130% × K_c, 15-of-30 window) that the 不赎回承诺 conditionally waives
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — general call-protection-window framework that the discretionary commitment instantiates
- [`cb-mandatory-and-exotic-structures.md`](cb-mandatory-and-exotic-structures.md) — exotic-structure context for prospectus-level vs. discretionary call protections
- [`cb-china-strong-call-game-theory.md`](cb-china-strong-call-game-theory.md) — sibling card (authored Round 9) carrying the strategic-interaction analysis of when the issuer chooses Option A (pull call) vs. Option B (commit not to call), mirroring the mechanics-vs-strategy split between `cb-china-downward-conversion` and `cb-china-downward-reset-signaling`

## Escalate to Raw When

Open 安道全 (2023 3ed) §3 pp.41-45 directly for the practitioner
treatment of 不赎回承诺 mechanics: typical commitment-window lengths,
disclosure-channel + timing conventions, post-announcement CB and share
price behaviour, and the commit-vs-call decision discipline. Open 攻守
§2 pp.17-48 for the cross-check practitioner handbook treatment of the
same mechanism within the broader strong-call clause ecosystem. Open
安道全 (2014 2ed) §5 pp.80-150 for the pre-2018 baseline practitioner
language (useful when comparing pre/post-2018 commitment-norm shifts).
Open DeSpiegeleer (2014) §3 pp.50-78 for the general
call-and-put-protection framework that the discretionary commitment
instantiates in the Chinese-market context. **Source:** 安道全 (2023
3ed) §3 pp.41-45; 攻守 §2 pp.17-48; 安道全 (2014) §5 pp.80-150;
DeSpiegeleer (2014) §3 pp.50-78.
