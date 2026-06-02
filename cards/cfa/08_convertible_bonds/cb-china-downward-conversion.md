---
schema_version: "cacg.v0"
id: "cb-china-downward-conversion"
title: "China Convertible-Bond Downward Conversion (下修)"
reading_id: "08_convertible_bonds"
summary: "Chinese CB prospectuses standardly include a 下修 (downward-conversion) clause: the issuer's board may propose lowering the conversion strike K_c after a defined share-price weakness window (typically 15-of-30 below 80% K_c). The proposal must pass a ≥2/3 vote of attending shareholders excluding CB-holders (回避表决), and the revised K_c must be at least the 20-day-and-1-day average price (and per-pr..."
tags: ["convertible-bonds", "china-downward"]
citations:
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p050:0047"
    chunk_hash: "b50615b321696d7ee1a9cd8a5df12b953883be7fedd4a1898deeff5b5c321b3d"
    page_range: [50, 51]
    quote: "上述方案须经出席会议的股东所持表决权的三分之二以上通过方可实 施。"
    edge_type: "defines"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p016:0013"
    chunk_hash: "0c90836e80f05cbf3fb35bd9baaf8875ff0e4989b6e6a20ba7e0a2b17c9e98ca"
    page_range: [16, 17]
    quote: "当公司普通股股票在任意连续三十个交易日中有十五个交易日的收盘价低于当 期转股价格的80%时，公司董事会有权提出转股价格向下修正方案并提交公司股东 大会审议表决。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p028:0016"
    chunk_hash: "ab58d816a3237ef9b6edc4e3f0a0b4616556461fc8e667c80b68501676538b5a"
    page_range: [28, 29]
    quote: "一只刜始转股价 20 元的可转债，正股价可能 10 元，转股价可 能 100 元丆万；而如果通过万修转股价变为 10 元、5 元，等于‚打 折卖股票‛，"
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p130:0167"
    chunk_hash: "1d66f173423a4c1f655eed70d8ed5e4146b8a81f7060da3573b42ec9585872d8"
    page_range: [130, 131]
    quote: "Such a share price collapse is called the “death spiral.”"
    edge_type: "supports"
card_hash: "fe09b5b653c2e729436381fcb640ad24de0b3f50cd9e98d843cfe63b4a2da6a5"
---
# China Convertible-Bond Downward Conversion (下修)

## Intuition

China onshore convertibles include a **downward-conversion** (下修)
clause that lets the issuer's board propose lowering the conversion
strike `K_c` after a stretch of share-price weakness. The proposal must
pass a shareholder vote (typically by two-thirds majority of voting
shareholders excluding the convertible's holders). At the mechanics
level, the clause is the contractual lever the issuer can pull when
share price has drifted far below `K_c`, and lowering `K_c` mechanically
increases dilution (`q = F / K_c` rises). The strategic / signaling
implications of when the board chooses to pull this lever, and the
game-theoretic interaction with holder put behavior, live in the
sibling [`cb-china-downward-reset-signaling`](./cb-china-downward-reset-signaling.md)
card; this card stays at the mechanics layer.
**Source:** 安道全 (2014) §5-§7 pp.100-200.

```
downward-conversion (下修) lifecycle:

  share price S(t)
       ^
       |\
       | \
   K_c |--+----------- current strike
       |   \
       |    \      M of N days < trigger threshold
       |     \      |
       |      \-----+--- board proposes K_c → K_c'
       |             \  (typical: 30 of last 30 days
       |              \  closing < 80% × K_c)
       |               +-- shareholder vote
       |                |
   K_c'|----------------+----- new strike (if approved)
       +---------------------------> t
```

## Definition

The **downward-conversion** (`下修`) provision in a typical China
onshore convertible prospectus carries the following parameter triple.
**Source:** 安道全 (2014) §5-§7 pp.100-150.

- **Trigger threshold** `β`: the share-price ratio below `K_c` that
  arms the board's right to propose. Typical practitioner value is
  `β = 80%`. Lower-quality issuers may set looser thresholds at `90%`.
  **Source:** 安道全 (2014) §5 pp.100-130.
- **Window length** `M-of-N`: the threshold-counting convention.
  Typical China value is `15-of-30` or `20-of-30` consecutive trading
  days closing below `β · K_c`. **Source:** 安道全 (2014) §5 pp.105-135.
- **Floor on the new strike** `K_c_floor`: the new strike `K_c'`
  cannot fall below the maximum of (a) the prior 20-day average share
  price and (b) the prior 1-day average share price. This floor
  prevents pathological dilution. **Source:** 安道全 (2014) §6
  pp.130-160.

The **shareholder-vote requirement** is the structural feature that
distinguishes Chinese 下修 from a generic call provision: the board may
propose `K_c'`, but the proposal must be ratified by a supermajority of
voting shareholders (typically two-thirds of those present at a
special general meeting, with convertible holders excluded from the
vote on conflict-of-interest grounds). **Source:** 安道全 (2014) §6
pp.130-160.

## Mathematical Reasoning

Let `q = F / K_c` denote the conversion ratio at issuance and
`q' = F / K_c'` denote the post-下修 ratio. Because `K_c' < K_c`, we have
`q' > q`, so a 下修 mechanically increases the share count delivered per
unit face on subsequent conversion. **Source:** 安道全 (2014) §6
pp.130-160; DeSpiegeleer et al. (2014) §4.2 pp.115-130.

The **floor constraint** `K_c' ≥ max(S_avg_20, S_avg_1)` produces a
deterministic ratchet structure: the new strike cannot exceed the
recent share price, so the dilution boost is always weakly bounded
below by the minimum recent trading price `K_c' ≥ S_min,recent`. This
is a mechanical floor in the prospectus, not a behavioral constraint.
**Source:** 安道全 (2014) §6 pp.130-160.

The conversion-ratio mechanics interact with the lot-size convention in
the [trading-mechanics card](./cb-china-trading-mechanics.md#definition):
a holder converting one lot of face after a successful 下修 receives
`floor((10 · F) / K_c')` shares plus residual cash for the rounding
remainder. The integer-rounding rule is unchanged by 下修; only the
shares-per-lot count increases. **Source:** 安道全 (2014) §6 pp.130-160;
DeSpiegeleer et al. (2014) §4.2 pp.115-130.

Asymptotic behavior of the **mechanical** clause (no game-theory framing
here — the strategic-interaction analysis lives in the sibling
[`cb-china-downward-reset-signaling`](./cb-china-downward-reset-signaling.md)
card). **Source:** 安道全 (2014) §5-§7 pp.100-200.

- `S → 0` deeply: the trigger is well below threshold; the 下修
  mechanism is available for the board to propose, but actual exercise
  depends on the (out-of-scope) strategic decision. **Source:** 安道全
  (2014) §7 pp.150-200.
- `S → K_c`: the trigger never arms; 下修 is dormant and the
  convertible behaves as a baseline optional CB. **Source:** 安道全
  (2014) §5 pp.100-130.
- **Post-下修**: the new strike `K_c'` becomes the operative anchor for
  the conversion-ratio `q' = F / K_c'`; all downstream payoff
  calculations use `K_c'`. **Source:** 安道全 (2014) §6-§7 pp.130-200;
  DeSpiegeleer et al. (2014) §4.2 pp.115-130 (post-strike-shift
  conversion-ratio mechanics).

## See Also

- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — base conversion mechanics that 下修 modifies
- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore mechanics
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — holder put as a separate prospectus clause (not the 下修 mechanism itself)
- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — strong-call mechanics (issuer-side counterpart in the prospectus clause set)
- [`cb-china-downward-reset-signaling.md`](cb-china-downward-reset-signaling.md) — signaling / game-theoretic depth scope-trimmed out of this mechanics card in v13a Round 4 (authored v13a Round 8 with the strategic-investor optimization, holder-response framework, and pre-put-eligible-date empirical pattern that this mechanics card intentionally omits).

## Escalate to Raw When

Open 安道全 (2014) §5-§7 pp.100-200 directly for the China-specific
practitioner playbook on 下修 mechanics: trigger threshold conventions,
window-counting rules, vote-passing thresholds, post-下修 strike-floor
behavior. **Source:** 安道全 (2014) §5-§7 pp.100-200.

Open DeSpiegeleer §4.2 pp.115-130 for the mandatory-vs-optional payoff
decomposition that adapts to the post-下修 strike-shift case.
**Source:** DeSpiegeleer et al. (2014) §4.2 pp.115-130.

For the strategic / signaling analysis of 下修 (issuer's optimization,
holder's response, empirical pre-put-eligible-date pattern, Stackelberg
game), see the authored [cb-china-downward-reset-signaling](./cb-china-downward-reset-signaling.md) card (authored v13a Round 8). **Source:** 安道全 (2014) §6-§7 pp.130-200 (mechanics-vs-strategy separation rationale).
