---
schema_version: "cacg.v0"
id: "cb-china-call-redemption-rules"
title: "China Convertible-Bond Call-Redemption Rules"
reading_id: "08_convertible_bonds"
summary: "China onshore convertibles almost universally include a 强赎 (strong-call) provision: issuer may force redemption at face once the share price spends 15 of 30 consecutive days at ≥ 130% of K_c; once triggered, the announcement gives holders ~1 month to convert before remaining bonds redeem at face — the structural mechanism that mid-life converts the average China-onshore CB into equity."
tags: ["convertible-bonds", "china-call"]
citations:
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p111:0107"
    chunk_hash: "d52dc30103c21a11c546426ea592940c24958bbd454ce19e1ec7a5ea8061d891"
    page_range: [111, 112]
    quote: "投资者只要坚守纪律，甚 至无须太多关心股市和股票本身，耐心等待强制赎回的到来就行了， 这特别适合信息和技术都处于劣势的小散户们。"
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p143:0159"
    chunk_hash: "d1759bf1d5a2e227d8042e883e2f7c5b23578ceafb56bde706808339f8696de5"
    page_range: [143, 144]
    quote: "If parity does fall below the call price, the company will be forced to pay cash instead of stock to the holders."
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p068:0078"
    chunk_hash: "267a3eae776fe08cacefcf8b36127e54f1618d7df650bcd482a4144396d57c75"
    page_range: [68, 69]
    quote: "The real expiry date of the convertible was 4 years later but the issuer decided to redeem its convertible prematurely."
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p042:0023"
    chunk_hash: "12ba11a3b16c4217a488a92d5add3adf4a97e0cc2f4b287f67412109a1e818a9"
    page_range: [42, 43]
    quote: "我们的看法是，强赎与否，利益使然。 ‚缺钱的时候拿现釐，不缺钱的时候拿股权‛，或者捡一种表述， 即‚股权值钱的时候留股权，股权不值钱的时候选现釐‛。"
    edge_type: "supports"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p016:0013"
    chunk_hash: "0c90836e80f05cbf3fb35bd9baaf8875ff0e4989b6e6a20ba7e0a2b17c9e98ca"
    page_range: [16, 17]
    quote: "当公司普通股股票在任意连续三十个交易日中有十五个交易日的收盘价低于当 期转股价格的80%时，公司董事会有权提出转股价格向下修正方案并提交公司股东 大会审议表决。"
    edge_type: "supports"
card_hash: "f49a65849b97b609d9a7a236ec4500ea0a56c2caf3a2cc1ce32767e18117b6e5"
---
# China Convertible-Bond Call-Redemption Rules

## Intuition

China onshore convertibles (可转债) almost universally include a **strong
call** (强赎) provision: the issuer may force redemption at face value
once the share price has spent a configurable window above a high
multiple of the strike. The standard practitioner-quoted parameters are
"15 of the last 30 trading days at ≥130% of `K_c`". Once triggered, the
issuer's announcement typically gives holders a roughly month-long
window to convert voluntarily; remaining bonds are redeemed at face.
The trigger is the structural mechanism that mid-life converts the
average China-onshore CB into equity. **Source:** 安道全 (2014) §3-§5
pp.80-150.

```
strong-call trigger lifecycle (qualitative):

  share price S(t)
       ^
       |        ____________ 130% × K_c level
       |       /
       |      /  threshold breached
       |     /   for 15 of last 30 days
       |    /                       |
   K_c |---+----------------- ------+--- forced conversion +
       |   |                        |    redemption window opens
       |   |                        |
       +-------------------------- t ---+----> t
                                        |
                                  bond delisted
                                  (or 100% face redemption)
```

## Definition

The **strong-call** (`强赎`) provision in a typical China onshore
convertible prospectus carries the following parameter triple. **Source:**
安道全 (2014) §3 pp.80-100.

- **Trigger multiple** `α`: the share-price ratio above the conversion
  strike `K_c` that arms the call. Typical practitioner value is
  `α = 130%`. Higher-quality issuers may use `α = 120%`; lower-quality
  may use `α = 150%`. **Source:** 安道全 (2014) §3 pp.80-100.
- **Window length** `M-of-N`: the daily-close threshold-counting
  convention. Typical value is `15-of-30` consecutive trading days.
  **Source:** 安道全 (2014) §3 pp.85-105.
- **Notice period** `T_notice`: the issuer-announcement window during
  which holders may either convert or accept the par redemption. Typical
  value is 20-30 trading days from the issuer's call notice. **Source:**
  安道全 (2014) §4 pp.100-130.

The mathematical structure mirrors the **soft call** as defined in the
[call-and-put card](./cb-call-and-put-protection.md#definition): the
issuer's right is gated on a parity-trigger rule with a path-dependent
trailing-window observation. **Source:** Calamos (2003) §6 pp.95-130;
DeSpiegeleer et al. (2014) §2.5 pp.50-78.

After triggering, holders face a **screw-clause** decision: if the
bond's intrinsic conversion value (parity) exceeds the par redemption
price `P_call` (typically 100% of face), rational holders convert;
otherwise they accept par. Because the trigger is set at
`α · K_c = 130% · K_c`, parity is roughly `q · 130% · K_c = 130% · F`
at the moment of trigger, well above `P_call = 100% · F` — so rational
holders convert. **Source:** Calamos (2003) §6 pp.95-130.

The China-onshore screw-clause has an additional **liquidity dimension**:
the issuer's announcement of the call is typically followed by a
multi-day liquidity squeeze in the bond as exchange-listed quotes
collapse toward parity. **Source:** 安道全 (2014) §5 pp.130-150.

## Mathematical Reasoning

The path-dependent trigger condition can be written as a counting
indicator. **Source:** 安道全 (2014) §3 pp.85-105.

```
Call-trigger indicator (M-of-N over trailing window):

  K(t) := sum_{i=1}^{N} 1{ S(t - i+1) ≥ α · K_c }

  Trigger fires at t if K(t) ≥ M
```

Once fired, the issuer's optimal-call decision tree from the
[issuer-motives card](./cb-issuer-motives.md#mathematical-reasoning)
applies: the issuer compares the cost of further dilution against the
cost of further coupon outlay. In the strong-call regime, the
issuer-optimal action is almost always to call immediately because
parity has already exceeded `α · F > P_call`. **Source:** 安道全 (2014)
§4 pp.100-130; Calamos (2003) §6 pp.95-130.

The **rational holder's choice** during the notice window is governed by
the inequality. **Source:** Calamos (2003) §6 pp.95-130.

```
Convert if:    q · S(t) · D_div(t, T_notice)  >  P_call · D_rf(t, T_notice)

Otherwise:     accept the cash redemption at P_call
```

Substituting the trigger condition `S(t) ≥ α · K_c = 130% · K_c` and
`q = F / K_c`, the holder's parity at trigger time is
`q · S(t) ≥ 130% · F`. Since `P_call = 100% · F`, conversion is
economically dominant unless dividend yield, share-price trajectory, or
post-announcement stock-price collapse during `T_notice` reverse the
inequality. **Source:** 安道全 (2014) §5 pp.130-150.

The **bond price collapse** during the notice window has two
regime-dependent shapes. **Source:** 安道全 (2014) §5 pp.130-150;
Calamos (2003) §6 pp.95-130.

- **Equity-tracking regime**: the bond's quoted price tracks parity
  closely as the conversion discount approaches zero; investors hold to
  convert and capture the residual time value. **Source:** 安道全 (2014)
  §5 pp.130-150.
- **Liquidity-distress regime**: in retail-dominated markets the
  exchange-quoted bond price can dislocate temporarily as forced sellers
  liquidate — creating a brief arbitrage opportunity for an investor
  willing to convert intraday. **Source:** 安道全 (2014) §5 pp.130-150.

Asymptotic behavior of the strong-call provision (cases below).
**Source:** 安道全 (2014) §3-§5 pp.80-150.

- `S → ∞` after the conversion-eligible date: the trigger arms quickly;
  the embedded-call decomposition collapses into immediate forced
  conversion. **Source:** 安道全 (2014) §3-§4 pp.80-130.
- `S < α · K_c` consistently: the trigger never arms; the convertible
  behaves as a full-life optional CB up to maturity. **Source:** 安道全
  (2014) §3 pp.80-100.

## See Also

- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — base soft-call structure that strong-call instantiates
- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore mechanics
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — holder's continuation-value comparison
- [`cb-issuer-motives.md`](cb-issuer-motives.md) — issuer-side optimal-call timing

## Escalate to Raw When

Open 安道全 (2014) §3-§5 pp.80-150 directly for the China-specific
practitioner playbook on strong-call mechanics: typical parameter
ranges by issuer credit quality, the empirical screw-clause dynamics,
the post-announcement liquidity regime. **Source:** 安道全 (2014) §3-§5
pp.80-150.

Open Calamos (2003) §6 pp.95-130 for the cross-jurisdictional framing of
parity-trigger calls and the screw-clause concept. **Source:** Calamos
(2003) §6 pp.95-130.

Open DeSpiegeleer §2.5 pp.50-78 for the formal soft-call definition and
the path-dependent pricing tree for trailing-window triggers.
**Source:** DeSpiegeleer et al. (2014) §2.5 pp.50-78.
