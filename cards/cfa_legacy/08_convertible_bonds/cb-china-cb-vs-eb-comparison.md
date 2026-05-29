---
schema_version: "cacg.v0"
id: "cb-china-cb-vs-eb-comparison"
title: "China Convertible Bond — 可转债 vs 可交债 (CB vs EB) Comparison"
reading_id: "08_convertible_bonds"
summary: "A 可转债 (CB) converts into the issuer's own newly-issued shares (an equity issuance in disguise); a 可交债 (EB) exchanges for the issuer's HOLDING of another company's already-existing shares (a stake the issuer pledges). 攻守 captures the intuition as '牛市都一样, 熊市大不同' — bull-market behaviour similar (both ride underlying toward strong-call), bear-market diverges sharply via 下修-willingness asymmetry."
tags: ["convertible-bonds", "china-cb"]
citations:
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p060:0054"
    chunk_hash: "21f365f79c1f1761f263141c4dd046a167c77faf2e77266c9665f0f25060d6c5"
    page_range: [60, 61]
    quote: "通过可转债名称基本可以知道对应的正股，可交债多数情况也可以判断可以交 换的股票，毕竟多数情况可交债是由大股东发行，发行方和对应标的股票关联很 大。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p042:0023"
    chunk_hash: "12ba11a3b16c4217a488a92d5add3adf4a97e0cc2f4b287f67412109a1e818a9"
    page_range: [42, 43]
    quote: "‚缺钱的时候拿现釐，不缺钱的时候拿股权‛，或者捡一种表述， 即‚股权值钱的时候留股权，股权不值钱的时候选现釐‛。"
    edge_type: "supports"
card_hash: "e07bb2a5cfcb259b95e2e702c58087a77913e5fb2d2ba7ce83857067e73bc5fc"
---
# China Convertible Bond — 可转债 vs 可交债 (CB vs EB) Comparison

## Intuition

A 可转债 (CB) is a bond issued by company A that the holder can
convert into **company A's own newly-issued common shares**;
conversion creates new shares for the issuer (an equity-issuance
in disguise). A 可交债 (EB) is a bond issued by company A whose
holder can exchange the bond for **company A's HOLDING of company
B's already-existing common shares** (a stake company A already
owns and pledges as collateral). 攻守 captures the practitioner
intuition as **牛市都一样, 熊市大不同** — in bull markets the
two instruments behave similarly because both ride the underlying
toward the strong-call trigger; in bear markets they diverge
sharply because of the shareholder-incentive asymmetry on 下修
willingness. **Source:** 攻守 (2020) Ch.3 §11 pp.61-62.

```
   shareholder incentive on 下修 (lower conversion strike):

      可转债 (CB)              可交债 (EB)
      ────────────             ────────────
      issuer wants conversion   shareholder owns the upside on B
      to avoid put-back AND     and may NOT want to give bondholders
      dilution-cost is paid by  more B-shares; would rather repay
      ALL shareholders          cash than 割肉 (cut flesh)
      → revises aggressively    → may NOT revise → EB trades like
        on bear-market dips        a low-yield bond
```

## Definition

The two products share many structural primitives — conversion
ratio, strike, hard / soft call provisions, downward-conversion
right — because the regulator modeled the EB framework on the
established CB framework. The author identifies several
operational differences that materially change the P&L profile.
**Source:** 攻守 (2020) Ch.3 §10-§11 pp.58-62.

- **Underlying-share source.** CB conversion creates new issuer
  shares (dilutive); EB exchange transfers existing pledged
  target-company shares (non-dilutive on target). **Source:**
  攻守 (2020) Ch.3 §11 pp.61-62.
- **Issuer / shareholder incentive.** For the CB issuer, conversion
  IS the desired outcome (it cancels the debt). For the EB
  shareholder, exchange is **giving up the upside** on shares
  already held — a "red envelope" handed from shareholder to
  bondholder; the shareholder may prefer to redeem cash. **Source:**
  攻守 (2020) Ch.3 §11 pp.61-62.
- **下修 (downward-revision) willingness.** In bear markets,
  CB issuers routinely revise `K_c` downward to avoid put-back
  ("送红包" to holders, paid by shareholder dilution). EB
  shareholders may NOT revise: each revision gives more B-shares
  to bondholders, which is "从股东身丆割万的肉" ("flesh cut from
  the shareholder's body") via the loss-aversion / 剥夺错觉
  (endowment-bias) mechanism the author cites. **Source:** 攻守
  (2020) Ch.3 §11 pp.61-62.
- **Implication on option value.** Because 下修 willingness is
  weaker on the EB side, the author argues EB's embedded-option
  value is far lower than CB's; investors should treat EB as a
  bond requiring higher after-tax yield, not as an "equity-like
  hybrid". **Source:** 攻守 (2020) Ch.3 §11 pp.62.
- **Other operational differences** (per Ch.3 §10): EB's down-
  revision procedure is faster (issuer-only approval) than CB's
  (advance shareholder notice + 2/3 vote); EB put-back periods
  are typically shorter (often last year only) than CB's (often
  last two years); EB conversion-quantity limit equals actual
  holdings (CB allows over-application to the in-hand quantity).
  **Source:** 攻守 (2020) Ch.3 §10 pp.59-61.

```
   per-dimension contrast:

      Dimension                 可转债 (CB)             可交债 (EB)
      ───────────────────       ──────────────          ──────────────
      Underlying source         issuer's new shares     issuer's holding of B
      Dilution                  yes (new supply)         no (existing transfer)
      Issuer revise 下修        routinely in bear        rarely (割肉 effect)
      Embedded-option value     full                     materially lower
      Down-revision procedure   advance notice + vote    issuer-only
      Put-back period           typically 2 years        typically 1 year
```

## Mathematical Reasoning

The dilution distinction is the deepest analytical wedge. CB
conversion creates NEW issuer shares; the issuer's share count
rises at conversion, and existing shareholders bear the dilution.
EB exchange transfers EXISTING shares from the issuer's pledged
holdings of the target company; the target-company share count is
unchanged, so there is no dilution feedback on the target. The
author's framing is that CB conversion is in effect an equity
issuance in disguise, while EB exchange is a collateral transfer.
**Source:** 攻守 (2020) Ch.3 §11 pp.61-62.

The bear-market asymmetry is the second analytical wedge. CB
issuers routinely revise `K_c` downward to avoid put-back ("送红包"
to bondholders, paid by shareholder dilution that is spread across
the broader shareholder base). EB shareholders may NOT revise: each
revision concedes more of THEIR pledged B-shares directly to
bondholders ("从股东身丆割万的肉"), and the loss-aversion / 剥夺
错觉 effect the author cites makes that concession harder to
accept than ordinary dilution. The asymmetric incentive collapses
the EB's embedded-option value toward bond-like in bear regimes,
justifying the author's "treat EB as a bond" recommendation.
**Source:** 攻守 (2020) Ch.3 §11 pp.61-62.

## See Also

- [`cb-china-trading-mechanics.md`](./cb-china-trading-mechanics.md#definition)
  — trading-rules layer shared by both products.
- [`cb-china-downward-conversion.md`](./cb-china-downward-conversion.md#definition)
  — the 下修 mechanics on the CB side, asymmetric with the EB
  case discussed here.
- [`cb-issuer-motives.md`](./cb-issuer-motives.md#intuition)
  — the issuer-incentives framing this card extends to the
  shareholder-as-issuer EB case.

## Escalate to Raw When

Open 攻守 (2020) Ch.3 §10 pp.58-61 directly when the reader needs
the seven operational-difference dimensions (face / lot / put-back
period / conversion-quantity rule / investor eligibility / etc.)
beyond the four 攻守 emphasizes in §11. **Source:** 攻守 (2020)
Ch.3 §10 pp.58-61.

Open 攻守 (2020) Ch.3 §11 pp.61-62 when the reader needs the
shareholder-incentive asymmetry argument or the 牛市都一样 / 熊
市大不同 motivating example. **Source:** 攻守 (2020) Ch.3 §11
pp.61-62.
