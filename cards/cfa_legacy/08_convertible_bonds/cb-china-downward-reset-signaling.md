---
schema_version: "cacg.v0"
id: "cb-china-downward-reset-signaling"
title: "China Convertible-Bond Downward-Reset Signaling and Game Theory"
reading_id: "08_convertible_bonds"
summary: "Atop the mechanical 下修 clause, the strategic layer is a signaling game between issuer and CB-holders. Issuers optimize dilution vs. coupon savings vs. put-avoidance: well-timed 下修 (often pre-put-eligible-date) credibly signals intent to push conversion before the upcoming put window, while announced non-reset signals that the issuer has cash on hand and prefers to redeem. The shareholders'-vote..."
tags: ["convertible-bonds", "china-downward"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p028:0016"
    chunk_hash: "ab58d816a3237ef9b6edc4e3f0a0b4616556461fc8e667c80b68501676538b5a"
    page_range: [28, 29]
    quote: "只有惱尽办法让可转债持 有人转股，这些钱才能成为‚股东投资给自己的钱‛，丆市公司才能 斱便自由地使用。"
    edge_type: "defines"
  - source_id: "cb_koziol_2004_convertible_bonds_strategic_investors"
    chunk_id: "cb_koziol_2004_convertible_bonds_strategic_investors:p049:0062"
    chunk_hash: "000191f40939737ccb83cee8ebcf2be2dd1dd3a55c20aa569f74c387fe9e69f2"
    page_range: [49, 50]
    quote: "If the conversion value is remarkably above the conversion price, i.e. the conversion probability is high, the value of a convertible bond is essentially driven by the firm value."
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p056:0053"
    chunk_hash: "38a73ca25817b531fe044eca5788b3a82daa8f95b0f22a9c4a6046ee30514180"
    page_range: [56, 57]
    quote: "换言之，“下调转股价”是一种权利，而不是义务。"
    edge_type: "supports"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p016:0013"
    chunk_hash: "0c90836e80f05cbf3fb35bd9baaf8875ff0e4989b6e6a20ba7e0a2b17c9e98ca"
    page_range: [16, 17]
    quote: "为了确保下修议案能 顺利通过，控股股东往往会在拟下修前清仓可转债，这一点可作为判断可转债是否 会下修的线索。"
    edge_type: "supports"
card_hash: "e20386c14619d9ba74f7779468ddfa3060294445f1e95eaac89830de71151f4e"
---
# China Convertible-Bond Downward-Reset Signaling and Game Theory

## Intuition

The mechanical 下修 clause (see [cb-china-downward-conversion](./cb-china-downward-conversion.md))
gives the issuer's board a contractual lever to lower the conversion
strike `K_c` after share-price weakness, subject to a supermajority
shareholder vote (convertible holders abstain). The **strategic** and
**signaling** content is when and why the board chooses to pull the
lever, what the announcement reveals about the issuer's outlook, and
how holders respond. This card carries that strategic layer — the
issuer's optimization over the dilution-vs-coupon-savings-vs-put-
avoidance tradeoff, the holder's response, asymmetric-information
signaling, and the empirical pre-put-eligible-date timing pattern.
**Source:** 安道全 (2023 3ed) §3-§5 pp.25-83; Koziol (2004) §3
pp.50-100.

```
issuer's 下修 strategic decision (game theory layer)

           depressed S << K_c, put-eligible date approaching
                                 │
                                 ▼
          ┌──────────────────────────────────────────────┐
          │  Option A: do nothing                        │
          │    → holders likely exercise put             │
          │    → forced cash redemption                  │
          │    → cash-flow + balance-sheet strain        │
          │                                              │
          │  Option B: propose 下修                       │
          │    → shareholders vote (CB holders abstain)  │
          │    → if approved: K_c' < K_c                 │
          │       → dilution increases (q' > q)          │
          │       → equity-route conversion likelier     │
          │       → put avoided                          │
          │       → market may infer issuer's outlook    │
          │         (signaling channel)                  │
          └──────────────────────────────────────────────┘
                                 │
                                 ▼
              issuer trades dilution PV cost
              against coupon-savings PV + put-avoidance PV
              (the "Koziol-style" strategic-investor optimization)
```

## Definition

The 下修 strategic decision is governed by three jointly-determined
quantities at the issuer level: (a) the **dilution PV cost** of the
proposed strike reduction, (b) the **coupon-savings PV** if the post-
reset conversion completes earlier than scheduled, and (c) the
**put-avoidance PV** if the reset blocks an otherwise-imminent put
exercise. On the holder side, the decision is whether to accept the
post-reset CB value `V_new(S, t; K_c')` over the put-exercise PV
`E^Q[P_put · D_rf(t, T_put)]`. **Source:** 安道全 (2023 3ed) §4 pp.45-83;
Koziol (2004) §3 pp.50-100 (strategic-investor framework migrated here
from the cb-china-downward-conversion card in v13a Round 4 scope-trim);
DeSpiegeleer et al. (2014) §4.2 pp.115-130 (post-strike-shift CB
valuation).

## Mathematical Reasoning

The issuer's net-of-下修 expected-PV optimization decomposes into the
three components above, schematically. **Source:** Koziol (2004) §3 pp.50-100; 安道全 (2023 3ed) §4-§5 pp.45-83.

```
maximize over K_c':  −E^Q[ q' · S(τ) · D_rf(0, τ) ]   (dilution PV cost)
                      + E^Q[ Σ c · F · D_rf · 1{τ > t_k} ]   (coupon-savings PV)
                      − P(forced put) · F · D_rf(0, T_put)   (put-avoidance PV)
       subject to:  K_c' ≥ max(S_avg_20, S_avg_1)   (floor constraint)
                    K_c' ≤ K_c                        (downward only)
```

The first-order condition trades marginal dilution against marginal
put-avoidance. The empirical regularity is that 下修 votes pass when
the issuer is approaching a put-eligible date AND the put strike is
close to par — the board wants to lower the strike enough to make
conversion the equity-likely outcome before the put binds. **Source:**
Koziol (2004) §3 pp.50-100; 安道全 (2023 3ed) §4-§5 pp.45-83.

The **holder's response** to a 下修 proposal depends on the comparison
between (a) the new convertible value `V_new(S, t; K_c')` and (b) the
expected put exercise PV. The pre-vote signaling channel modifies the
naive comparison: when the board chooses to propose 下修, it implicitly
reveals private information about the issuer's outlook (chief among
which: that the board believes the company will be able to honor the
diluted-share trajectory rather than face cash-redemption stress).
**Source:** 安道全 (2023 3ed) §4 pp.45-83; DeSpiegeleer et al. (2014)
§4.2 pp.115-130 (mandatory-vs-optional framing transferred to the
post-reset case).

```
holder's vote-window decision tree:

  下修 announced
     │
     ▼
  compare V_new(S, t; K_c') vs. E^Q[P_put · D_rf(t, T_put)]
     │
     ├── if V_new > put PV: holder favors 下修
     │   (bond keeps trading; bond floor reset; signaling positive)
     │
     └── if V_new < put PV: holder prefers put
         (forced put if not blocked by 下修; signaling negative)
```

Asymptotic behavior of the **strategic** 下修 in the Chinese-market
context follows three regime cases. Pre-put-eligible period with
depressed `S`: 下修 votes are most common in the 6-12 months prior
to a put-eligible date (the empirical regularity documented across the
post-2014 Chinese-CB sample). Deep S<<K_c (no put-eligible date
within horizon): 下修 is rarer because put-avoidance PV is zero; the
issuer is choosing between dilution cost and continued debt-route
accrual. S→K_c (trigger never arms): 下修 dormant; signaling channel
inactive. **Source:** 安道全 (2023 3ed) §5 pp.55-83; 安道全 (2014)
§7 pp.150-200 (Chinese-market empirical pre-put-eligible-date pattern).

The **issuer's signaling tradeoff** has a clean intuition: pulling 下修
preserves the bond's continued equity-route option (good for the board
if it believes share price can recover), but the act of pulling
implicitly reveals that the board no longer thinks share price will
recover on its own — a negative signal that may itself depress S
further. The empirical resolution of this tension is what makes 下修
decisions strategically informative rather than mechanical, and is the
focus of the asymmetric-information academic literature on Chinese-CB
reset behaviour (e.g., the Martin / Qiu / Zhang 2015 paper to be
acquired in a future round and added to Supporting). **Source:**
Koziol (2004) §3 pp.50-100; 攻守 §2 pp.16-26 (Chinese-market 下修
practitioner cross-check).

## See Also

- [`cb-china-downward-conversion.md`](cb-china-downward-conversion.md) — the mechanical 下修 clause (trigger threshold, M-of-N window, strike floor, shareholder-vote supermajority) that this card's strategic layer sits on top of
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — the holder put right whose avoidance is the issuer's chief reason for proposing 下修
- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — the issuer-side counterpart (强赎); the 下修-and-强赎 are jointly the contractual game-theoretic surface

## Escalate to Raw When

Open 安道全 (2023 3ed) §4-§5 pp.45-83 directly for the China-specific
practitioner playbook on 下修 strategic decisions: typical board-
proposal patterns, vote-passing rates, post-下修 stock-price dynamics,
and the empirical correlation with put-eligible dates. Open Koziol
(2004) §3 pp.50-100 for the strategic-investor formal analysis that
derives the issuer's 下修-decision rule as a Stackelberg game between
issuer and holder. Open DeSpiegeleer §4.2 pp.115-130 for the
mandatory-vs-optional payoff decomposition that adapts to the post-
下修 strike-shift case. Open 安道全 (2014) §7 pp.150-200 for the
pre-2018 baseline practitioner language on 下修-vote-rate observations.
Open 攻守 §2 pp.16-26 for the cross-check practitioner-handbook
treatment of 下修 in the broader Chinese-CB clause-game-play ecosystem.
**Source:** 安道全 (2023 3ed) §4-§5 pp.45-83; Koziol (2004) §3
pp.50-100; DeSpiegeleer et al. (2014) §4.2 pp.115-130; 安道全 (2014)
§7 pp.150-200; 攻守 §2 pp.16-26.
