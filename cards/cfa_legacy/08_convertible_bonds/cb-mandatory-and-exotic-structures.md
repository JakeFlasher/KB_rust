---
schema_version: "cacg.v0"
id: "cb-mandatory-and-exotic-structures"
title: "Mandatory and Exotic Convertible Structures"
reading_id: "08_convertible_bonds"
summary: "Mandatory and Exotic Convertible Structures — placeholder summary               "
tags: ["convertible-bonds", "mandatory-exotic"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p055:0063"
    chunk_hash: "051810f8e5498b774b05fa095895799851c25abec0b47969cd516b65068ac8e4"
    page_range: [55, 56]
    quote: "A mandatory convertible bond always redeems into shares, never into cash."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p055:0063"
    chunk_hash: "051810f8e5498b774b05fa095895799851c25abec0b47969cd516b65068ac8e4"
    page_range: [55, 56]
    quote: "This acronym stands for Participating Equity Preferred Stock. Another name for the same instrument is PRIDE (Preferred Redeemable Increased Dividend Securities) or DECS (Debt Exchangeable for Common Stock)."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p107:0133"
    chunk_hash: "09b1e3550aa9fc16306bc52c5f8f7e23b04dd88cd190c39b5b4e4c975bdb2779"
    page_range: [107, 108]
    quote: "An owner of CoCo bonds exposes himself to a mandatory conversion in shares."
    edge_type: "supports"
card_hash: "853985aba341a82d8898e7a989ce17f78792b84f6e90a38e8e6fd7bc193227de"
---
# Mandatory and Exotic Convertible Structures

## Intuition

The basic optional vs mandatory distinction in the
[mandatory-vs-optional card](./cb-mandatory-vs-optional-conversion.md#definition)
covers the canonical PEPS/DECS family with two thresholds `L < U`
defining the conversion-ratio piecewise rule. Real markets have
many **exotic variants** beyond this baseline: ACES (with a
contingent-conversion gate), ELKS (with a participation-rate
adjustment), contingent capital instruments (CoCos, with a
trigger-based loss-absorption clause), refix / reset structures
(with periodic strike adjustments), and cap-and-floor optional
convertibles (with explicit strike caps and floors). Each variant
modifies the embedded-option leg in a structured way that the
practitioner can decompose as additional long / short option
positions on top of the base structure.
**Source:** DeSpiegeleer et al. (2014) §4.1-§4.6 pp.110-200.

```
exotic structure family tree (qualitative):

   optional CB                       mandatory CB
   |                                 |
   |- vanilla (basic conversion)     |- PEPS / DECS / ACES / ELKS
   |- callable / puttable            |- contingent capital (CoCo)
   |- reset / refix (strike shifts)  |- exchangeable (cross-issuer)
   |- contingent (parity gates)
   |- cap-and-floor (capped upside,
   |   floored downside)
```

## Definition

The major exotic mandatory variants. **Source:** DeSpiegeleer et
al. (2014) §4.1-§4.4 pp.110-160; Zubulake §3-§4 pp.50-150.

- **PEPS / DECS / ACES / ELKS**: piecewise-linear conversion
  ratios at maturity defined by two thresholds `L < U`, with
  family-specific modifications (PEPS pays a higher coupon and
  has a flatter mid-region; DECS has narrower `[L, U]` band; ACES
  adds a contingent-conversion gate that fires only if `S(T) ≥
  L_gate`; ELKS adjusts the participation rate within `[L, U]`).
  **Source:** DeSpiegeleer et al. (2014) §4.1-§4.2 pp.110-130;
  Calamos (2003) §9 pp.180-220.
- **Contingent capital instruments (CoCos)**: bank-issued
  contingent convertibles whose conversion is **forced** if a
  capital-trigger condition is breached (typically a
  Common Equity Tier 1 ratio falling below a contractual
  threshold). The instrument's embedded option is **path-
  dependent** on the issuer's regulatory-capital state, not on
  the share price directly. **Source:** DeSpiegeleer et al.
  (2014) §4.5 pp.160-180.
- **Exchangeable convertibles**: convertible into a third-
  party issuer's shares (e.g. a parent company holds shares of
  a subsidiary and issues an exchangeable converting into the
  subsidiary's shares). The decomposition flips the
  underlying-share-issuer relationship in the
  [bond-anatomy card](./cb-bond-anatomy-and-cashflows.md#definition).
  **Source:** DeSpiegeleer et al. (2014) §4.6 pp.180-200;
  Philips (1997) §2 pp.30-50.

The major exotic optional-structure variants. **Source:**
DeSpiegeleer et al. (2014) §4.3-§4.4 pp.130-160.

- **Refix / reset**: the conversion strike `K_c` is reset
  periodically (typically annually) to the prevailing share
  price (with a floor and a ceiling). The reset removes the
  holder's "stuck out-of-the-money" risk at the cost of
  capping the holder's upside; analogous to the China onshore
  下修 mechanism described in the
  [china-downward-conversion card](./cb-china-downward-conversion.md#definition)
  but contractually triggered rather than vote-based.
  **Source:** DeSpiegeleer et al. (2014) §4.3 pp.130-150.
- **Contingent conversion (CoCo-style optional)**: holder's
  conversion is gated on `S(T) ≥ S^*_co_trigger` (typical
  trigger 130% of `K_c`). Below the trigger, the bond redeems
  at face. The gate is conceptually the issuer's mirror of
  the
  [strong-call (强赎) provision](./cb-china-call-redemption-rules.md#definition)
  in China onshore markets, with the holder rather than the
  issuer holding the option. **Source:** DeSpiegeleer et al.
  (2014) §4.4 pp.150-160.
- **Cap-and-floor**: explicit caps `K_c · α_cap` and floors
  `K_c · α_floor` that limit the conversion-ratio adjustment
  in the prospectus. The structure is a hybrid of optional
  conversion and the mandatory's piecewise rule.
  **Source:** Philips (1997) §2 pp.50-100.

## Mathematical Reasoning

Each exotic variant adds a **named option position** on top of
the base structure. **Source:** DeSpiegeleer et al. (2014)
§4.1-§4.6 pp.110-200.

```
mandatory PEPS / DECS:                                (base)
  V_mandatory = B_mandatory + q_high · S
              − call(S, U) + put(S, L)

ACES (with gate at L_gate):
  V_ACES = V_mandatory − digital_call(S, L_gate) · payoff_diff

ELKS (participation shift):
  V_ELKS = V_mandatory + α_ELKS · [call(S, U) − call(S, U + Δ)]

contingent capital (CoCo, bank issued):
  V_CoCo = V_mandatory                       (base)
         − binary_loss_absorption_event_PV   (forced conversion at trigger)
         + survival_coupon_PV
```

The **CoCo's embedded option** is materially different from the
share-price-driven options in the canonical optional CB: the trigger
is the issuer's regulatory-capital ratio, which is itself a function
of balance-sheet items that are not directly observable in equity-
market prices. Pricing CoCos thus requires a model of the issuer's
capital-ratio process — typically a **structural-credit-Merton-
style** mapping (see the
[default-and-recovery card's](./cb-default-and-recovery.md#mathematical-reasoning)
Merton-mapping treatment) generalized to a regulatory-capital
trigger. **Source:** DeSpiegeleer et al. (2014) §4.5 pp.160-180.

The **refix / reset** structure can be priced via a credit-aware
tree augmented with the reset-date observation: at each reset date
`t_reset`, the strike is updated to `K_c'(t_reset) := S(t_reset)`
clamped to `[K_c · α_floor, K_c · α_cap]`, and the tree's downstream
nodes use the updated strike for the conversion-decision comparison.
**Source:** DeSpiegeleer et al. (2014) §4.3 pp.130-150.

The **contingent-conversion (CoCo-style optional) structure**
introduces a binary gate at maturity. **Source:** DeSpiegeleer et
al. (2014) §4.4 pp.150-160.

```
contingent-conversion payoff at T:

  V(S, T) = max(F, q · S(T)) · 1{ S(T) ≥ S*_co_trigger }
          + F · 1{ S(T) < S*_co_trigger }
```

The structure prices to a base optional-CB minus a digital-put-
above-`S^*_co_trigger`; the holder is short the digital put and
long the base optional CB. The decomposition is exact and the
practitioner-implementation reduces to pricing the digital put.
**Source:** DeSpiegeleer et al. (2014) §4.4 pp.150-160.

The **cap-and-floor** structure is a hybrid that interpolates
between optional and mandatory: within `[K_c · α_floor, K_c · α_cap]`,
the conversion behaves like an optional CB; outside, the conversion
ratio is clamped, mirroring the mandatory's piecewise rule.
**Source:** Philips (1997) §2 pp.50-100.

Asymptotic regimes (cases below). **Source:** DeSpiegeleer et al.
(2014) §4.3-§4.5 pp.130-180.

- **Stable share-price** within reset bounds: refix structures
  reduce to canonical optional CB pricing because the strike
  resets are deferred. **Source:** DeSpiegeleer et al. (2014)
  §4.3 pp.130-150.
- **High-trigger CoCo conversion-trigger regime**: the binary
  loss-absorption event PV dominates; the convertible's value
  approaches the recovery-PV of an unsecured bond minus the
  forced-conversion-equity payoff. **Source:** DeSpiegeleer et
  al. (2014) §4.5 pp.160-180.
- **Deep-OTM contingent-conversion**: the digital put above
  `S^*_co_trigger` is far OTM at issuance; the contingent
  structure approximates a face-redeeming bond, with the
  conversion option engaged only late in the issue's life.
  **Source:** DeSpiegeleer et al. (2014) §4.4 pp.150-160.

## See Also

- [`cb-mandatory-vs-optional-conversion.md`](cb-mandatory-vs-optional-conversion.md) — base PEPS/DECS structure
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — base soft-call mechanism that contingent gates generalize
- [`cb-china-downward-conversion.md`](cb-china-downward-conversion.md) — analogous vote-based reset
- [`cb-default-and-recovery.md`](cb-default-and-recovery.md) — structural Merton mapping that CoCo pricing extends
- [`cb-china-no-call-commitments.md`](cb-china-no-call-commitments.md) — Chinese-market 不赎回承诺 mechanism layering a discretionary issuer commitment on top of the soft-call clause structure
- [`cb-china-strong-call-game-theory.md`](cb-china-strong-call-game-theory.md) — Chinese-market 强赎博弈 strategic-decision layer (call vs. no-call vs. silent) sitting on top of the soft-call clause's mechanical trigger

## Escalate to Raw When

Open DeSpiegeleer §4.1-§4.6 pp.110-200 directly for the exhaustive
exotic-structure taxonomy, the CoCo / contingent-conversion
pricing-tree extensions, and the participation-rate-shift family.
**Source:** DeSpiegeleer et al. (2014) §4.1-§4.6 pp.110-200.

Open Zubulake §3-§4 pp.50-150 for cross-jurisdictional mandatory
variants and the cap-and-floor practitioner taxonomy across US,
European, and Japanese markets. **Source:** Zubulake §3-§4
pp.50-150.

Open Calamos §9 pp.180-220 for the practitioner's guide to PEPS
/ DECS / ACES / ELKS issuer-side tax / accounting motivations,
and the empirical performance of mandatory issues across market
regimes. **Source:** Calamos (2003) §9 pp.180-220.

Open Philips §2 pp.30-100 for the historical development of
exotic structures and the cap-and-floor / participation-rate
families that emerged in the 1990s tech-bubble era.
**Source:** Philips (1997) §2 pp.30-100.
