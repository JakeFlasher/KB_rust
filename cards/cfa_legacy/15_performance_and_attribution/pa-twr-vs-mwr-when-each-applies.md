---
schema_version: "cacg.v0"
id: "pa-twr-vs-mwr-when-each-applies"
title: "Time-Weighted vs Money-Weighted Return: When Each Applies"
reading_id: "15_performance_and_attribution"
summary: "Distinguishes the time-weighted return (sub-periods weighted equally, cash-flow-neutral, the manager-evaluation canon) from the money-weighted return/IRR (each dollar earns one effective rate, the asset owner dollar experience), and when each applies."
tags: ["time-weighted-return", "money-weighted-return", "cash-flows"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p060:0068"
    chunk_hash: "366759d4631678f342c5e18658e3b3669b030235b35558f5f86538005be4172e"
    page_range: [61, 61]
    quote: "The time-weighted rate of return adjusts for cash flow and weights each time period equally, measuring the return that would have been achieved had there been no cash flows."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p052:0047"
    chunk_hash: "a3fea8a49a7a625bfae2393911027ae2c6628d1d856eafb9bb91908523406d3c"
    page_range: [53, 53]
    quote: "Because the time-weighted return controls for cash flows, it is a better measure to use when comparing the performance of different managers."
    edge_type: "supports"
---
# Time-Weighted vs Money-Weighted Return: When Each Applies

## Intuition

A portfolio earns two things at once: the *skill* of whoever picks the assets, and the *luck of timing* of when money flowed in and out. External cash flows — contributions, withdrawals, redemptions — are usually decided by the asset owner, not the manager. So the central question is: does the chosen return number reward (or punish) the manager for cash-flow timing that the manager did not control?

The time-weighted return (TWR) answers "no." It strips the cash-flow timing out, measuring the return that a single unit of capital would have compounded to had it stayed invested the whole period — exactly what a unitized mutual-fund NAV tracks. The money-weighted return (MWR), computed as an internal rate of return, answers "yes": it lets every invested dollar earn the same effective rate, so a period when a lot of money was deployed counts more. The MWR is therefore the asset *owner's* lived dollar experience, while the TWR is the *manager's* report card.

**Source:** Bacon (2023) §3 (unit-price/NAV method pp.58-59; ex-post IRR p.50; Time-Weighted Versus Money-Weighted Rates of Return pp.60-62) pp.50-62

## Definition

**Time-weighted rate of return.** "The time-weighted rate of return adjusts for cash flow and weights each time period equally, measuring the return that would have been achieved had there been no cash flows." Because each sub-period gets equal weight regardless of the amount invested, the timing and size of external cash flows do not affect it.

**Money-weighted (dollar-weighted) rate of return.** The single rate at which each dollar invested grows — operationally the internal rate of return that equates the discounted cash flows and ending value. Over a single measurement period the MWR always reflects the sign of the actual cash gain or loss, which makes it intuitive for presentation to private clients.

**When each applies.** The "received wisdom" is that TWR measures the asset manager's performance adjusting for cash flows, while MWR measures the asset owner's invested assets including external-cash-flow impact: if the manager does not control external-cash-flow timing, use TWR; if the manager does control it, use MWR. Bacon flags this as only *partially* correct — he agrees TWR should be used for fair comparability when the manager does not control flow timing, but argues that for managers who *do* control timing the choice hinges more on the asset class's liquidity than on control per se. The card asserts this as Bacon asserts it; he does not formalize the liquidity criterion.

**Source:** Bacon (2023) §3 (ex-post IRR / money-weighted definition p.50; Time-Weighted Versus Money-Weighted Rates of Return pp.60-62) pp.50-62

## Mathematical Reasoning

Let a measurement window be split at each external cash flow into sub-periods with holding-period returns r_1, ..., r_T. The TWR links them geometrically, each sub-period entering with equal (per-period) weight:

    1 + R_TWR = (1 + r_1)(1 + r_2) ... (1 + r_T).

Because EV_t / BV_t for each sub-period uses values immediately before / after each cash flow, the cash-flow magnitude never enters the product — it only sets the sub-period boundaries. Hence TWR is invariant to flow size and timing.

The MWR is instead the rate i solving the value-conservation identity (cash flows C_k applied for fraction W_k of the period):

    EV = BV(1 + i) + sum_k C_k (1 + i)^{W_k}.

There is generally no closed form; i is solved numerically. The key structural contrast: TWR is a *time*-weighted (geometric) average of sub-period returns, whereas the IRR is approximately a *money*-weighted average — each sub-period return weighted by the accumulated principal invested up to that point. When more principal is deployed in the higher-returning sub-periods, MWR > TWR; the inequality flips when large money arrives ahead of poor sub-periods. The two coincide only when there are no external cash flows.

```
                 external cash flow timing
                          |
        does the MANAGER control it?
                 /                 \
               no                   yes
                |                     |
   use TIME-WEIGHTED          situation more complex;
   (cash-flow neutral,        Bacon: liquidity of the
   manager report card,       asset class drives choice,
   benchmark-comparable)      not control alone
                                      |
                          MONEY-WEIGHTED reflects the
                          asset OWNER's dollar experience
```

**Source:** Bacon (2023) §3 (chain-linking Eq. 3.6 p.45; ex-post IRR Eq. 3.14 p.50; Time-Weighted Versus Money-Weighted Rates of Return pp.60-62) pp.45-62

The cross-reference makes the manager/owner split explicit: "Because the time-weighted return controls for cash flows, it is a better measure to use when comparing the performance of different managers." Christopherson, Cariño & Ferson summarize the IRR as a single number that gives more weight to periods with more principal invested, while the TWR summarizes the growth of a unit of initial value, unaffected by cash flows — so when flows are outside the manager's control, TWR is the appropriate comparison measure.

**Source:** Christopherson, Cariño & Ferson (2009) ch.5 "Time-Weighted Versus Money-Weighted Returns" (printed p.40; PDF p.53)

## See Also

- [`pa-irr-money-weighted-return.md`](pa-irr-money-weighted-return.md) — the IRR mechanics behind the money-weighted side of this contrast.
- [`pa-dietz-methods-mwr-approximations.md`](pa-dietz-methods-mwr-approximations.md) — Dietz / modified-Dietz approximations used when valuations at each cash flow are unavailable.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — how the geometric sub-period linking that defines TWR is constructed in practice.
- [`pa-gips-2020-composites-and-mechanics.md`](pa-gips-2020-composites-and-mechanics.md) — why GIPS-compliant presentation leans on time-weighted (and, for controlled-flow vehicles, money-weighted) returns. See also the 17 ethics GIPS performance-presentation standard.

## Escalate to Raw When

- You need the actual worked numbers showing TWR and MWR diverging (e.g. Bacon's Exhibit 3.16, where a client loses money over the window yet the TWR is positive, or CCF's Table 5.4 / 5.5 with a 12.25% TWR vs 14.62% IRR) — these worked arithmetic examples are deferred to the source pages.
- You must decide whether a controlled-flow vehicle (private equity, real assets, a fund the manager can call capital into) should be reported money-weighted: see Bacon's liquidity-based argument and GIPS money-weighted-return provisions in the raw text.
- You need the full Fisher dollar-weighted-return theorem (CCF ch.5) proving the IRR is approximately a money-weighted average of sub-period returns, including the continuous-compounding derivation and weighting terms.
