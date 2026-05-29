---
schema_version: "cacg.v0"
id: "cb-china-strong-call-game-theory"
title: "China Convertible-Bond Strong-Call Game Theory (强赎博弈)"
reading_id: "08_convertible_bonds"
summary: "Once a Chinese CB satisfies the 130%·K_c / 15-of-30 strong-call trigger, the issuer has three moves: pull the call (force conversion at ~100·103%), post a 不赎回承诺 (decline for a stated window), or stay silent through the eligibility window. Under post-2018 norms, ~57% of triggered CBs publicly declined to call (Zheshang Securities); declining is itself a signal — typically that the issuer has cas..."
tags: ["convertible-bonds", "china-strong"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p032:0018"
    chunk_hash: "186acff34a5dd1161006a058ea42934a3eac982b9c7d06804c85eec252be2c51"
    page_range: [32, 33]
    quote: "①在转股期内，如果公司股票在仸意违续三十个交易日中至少 十五个交易日的收盘价栺不低于当期转股价栺的 130%（含 130%）；"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p042:0023"
    chunk_hash: "12ba11a3b16c4217a488a92d5add3adf4a97e0cc2f4b287f67412109a1e818a9"
    page_range: [42, 43]
    quote: "我们的看法是，强赎与否，利益使然。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p071:0068"
    chunk_hash: "c0cd048f2f0122d728aff1c012021176e80927b5a894690840c2af944694cf1f"
    page_range: [71, 72]
    quote: "可转债的持有人会看到，可以以几年前约好的低价格（转股价）把可 转债转换为公司股票"
    edge_type: "supports"
  - source_id: "cb_koziol_2004_convertible_bonds_strategic_investors"
    chunk_id: "cb_koziol_2004_convertible_bonds_strategic_investors:p049:0062"
    chunk_hash: "000191f40939737ccb83cee8ebcf2be2dd1dd3a55c20aa569f74c387fe9e69f2"
    page_range: [49, 50]
    quote: "If the conversion value is remarkably above the conversion price, i.e. the conversion probability is high, the value of a convertible bond is essentially driven by the firm value."
    edge_type: "supports"
card_hash: "90fab78bc9dfb7db7cedfbe5dbb4afd72153e93577c553db589e4dd121f6a057"
---
# China Convertible-Bond Strong-Call Game Theory (强赎博弈)

## Intuition

The mechanical 强赎 clause (see [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md))
gives the issuer a contractual right — once `S/K_c ≥ 130%` for 15 of the
last 30 trading days — to force redemption at par. The mechanics layer
treats the post-trigger holder choice as a clean screw-clause:
convert if parity > par, otherwise accept cash. The **strategic** and
**signaling** content is the issuer's prior choice — once the trigger
is met — between (a) pulling the call, (b) posting a 不赎回承诺 (see
[cb-china-no-call-commitments](./cb-china-no-call-commitments.md)) that
voluntarily delays the call for a stated window, or (c) silently
letting the announcement window expire without acting. This card
carries that strategic layer: the issuer's optimization over the
现金-vs-股权 (cash-now versus equity-later) tradeoff, the holder's
response, the asymmetric-information signaling channel, and the
post-2018 empirical regularity in which a majority of triggered
Chinese CBs publicly decline to call.
**Source:** 安道全 (2023 3ed) §1.10 pp.32-34.

```
issuer's 强赎 strategic decision (game theory layer)

           S/K_c >= 130% for 15 of last 30 days (trigger armed)
                                 |
                                 v
          +----------------------------------------------+
          |  Option A: pull 强赎                          |
          |    -> forced conversion within ~30-day notice |
          |    -> issuer gets equity (debt extinguished)  |
          |    -> dilution increases (issued shares q*F)  |
          |    -> "缺钱时拿现金, 不缺钱时拿股权" --- call when |
          |       the firm still needs the cash route     |
          |       closed and equity is the cheap source.  |
          |                                              |
          |  Option B: 公告不赎回 (no-call commitment)     |
          |    -> bond stays outstanding for W months     |
          |    -> low coupon continues to accrue          |
          |    -> equity overhang stays positive (shares  |
          |       not yet diluted; share price relieved)  |
          |    -> issuer retains its option for the next  |
          |       trigger event after window expires.     |
          |                                              |
          |  Option C: silent (let trigger pass)          |
          |    -> no public commitment, but no call       |
          |    -> trigger window resets; next breach can  |
          |       re-arm. Carries weaker signaling weight |
          |       than Option B because no enforceable    |
          |       commitment is created.                  |
          +----------------------------------------------+
                                 |
                                 v
              issuer trades cash-now PV (forced conversion)
              against retained-optionality PV + low-coupon PV
              + signaling cost (announcement informs market)
```

**Source:** 安道全 (2023 3ed) §1.10 pp.32-34; 攻守 §2 pp.17-48.

## Definition

The 强赎 strategic decision is governed by three jointly-determined
quantities at the issuer level: (a) the **funding-need PV** that
forced conversion immediately satisfies (debt becomes paid-in equity;
the company no longer owes coupons + principal), (b) the
**retained-optionality PV** that no-call preserves (the issuer keeps
the right to call at the next trigger event, when equity may be even
more valuable), and (c) the **signaling cost** of the public
announcement (the act of declining to call reveals information about
the issuer's current funding position and outlook). On the holder
side, the decision is whether to convert during a called-bond's
notice window (the [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md)
screw-clause comparison) or to hold through the no-call window in the
hope of further share-price appreciation.
**Source:** 安道全 (2023 3ed) §1.10 pp.32-34; 安道全 (2014) §3-§5
pp.80-150.

## Mathematical Reasoning

The issuer's net-of-call expected-PV optimization decomposes into the
three components above, schematically. **Source:** Koziol (2004)
§1-§2 pp.20-50; 安道全 (2023 3ed) §1.10 pp.32-34.

```
maximize over action in {A: call, B: commit-no-call, C: silent}:

   if A (call):   + E_Q[ q * S(t_call) * D_rf(0, t_call) ]  (forced-conv PV)
                  - F * D_rf(0, t_call)                      (debt offset)
                  - cost_dilution(q * F)                     (dilution)

   if B (commit): + E_Q[ Sum c * F * D_rf * 1{t > t_k} ]     (low-coupon PV)
                  + E_Q[ option_value(S, t_now + W) ]        (retained-call PV)
                  - signaling_cost(public-announcement)      (disclosure cost)

   if C (silent): same as B but with weaker signaling weight
                  and unenforceable de-facto commitment
       subject to: trigger condition K(t) >= 15 (already satisfied)
```

The first-order condition trades the **immediate cash route** (Option
A: convert debt to equity now) against the **deferred cash route**
(Option B / C: keep the low-coupon debt outstanding and call later
when equity-value is higher). 安道全's practitioner framing is
"缺钱的时候拿现金, 不缺钱的时候拿股权" — when the firm urgently
needs the funding-need PV resolved (real cash crunch, ongoing capex
funding gap, debt-covenant pressure), it calls; when the firm has
adequate liquidity, it prefers to retain the option and the equity
stake's residual upside.
**Source:** 安道全 (2023 3ed) §1.10 pp.33-34; Koziol (2004) §1-§2
pp.20-50 (strategic-investor framework on call timing under
asymmetric information).

The **post-2018 empirical regularity** is striking: a Zheshang
Securities 2021 practitioner survey cited by 安道全 reports that
roughly 57% of triggered Chinese CBs in 2021 publicly announced
**not** to call, and a subset committed to no-call windows extending
1 year or longer beyond the trigger event. The three publicly-stated
issuer reasons for declining to call cluster into: (i) the company
has alternative funding arrangements and does not need the
forced-conversion cash route; (ii) the CB's remaining time to
maturity is short so the issuer prefers the low-coupon debt route
over forcing dilution; (iii) the issuer is concerned that forced
conversion would create equity-price pressure that depresses the
share price. A fourth passive reason is general equity-market
weakness that makes immediate forced conversion economically
unattractive relative to waiting.
**Source:** 安道全 (2023 3ed) §1.10 pp.32-33 (citing Zheshang Securities
2021 + Huachuang 2020 + CICC 2020 + China Merchants Securities 2020
practitioner reports as cross-checks).

The **英科转债 case study** (英科 Medical) anchors the 不缺钱时拿股权
intuition: the issuer raised CB capital at a 16.25-yuan strike
during a low-funding-need-buffer period, experienced COVID-driven
share-price appreciation to roughly 299.99 yuan, and observed CB
prices climb correspondingly. At the strong-call trigger, the issuer
faced the choice of calling (forcing conversion at the post-下修
strike of 5.55 yuan and effectively giving holders shares worth
several multiples of par) versus holding the existing equity stake
through the next trigger event. 安道全's analysis is that the
英科 issuer's strong post-COVID cash position made Option B
economically dominant: the issuer had no funding-need PV pressure,
the retained-optionality PV was high, and the signaling cost was
low because the market had already priced in the firm's robust
balance sheet.
**Source:** 安道全 (2023 3ed) §1.10 pp.34.

The **holder's response** to a no-call announcement (Option B) is
typically a small positive move in the bond and a smaller negative
move in the share. The bond response reflects resolution of the
forced-conversion risk and the continued accrual of low-coupon
debt; the share response reflects the issuer's implicit positive
signal about its current funding position offset by the dilution-
overhang persistence. Asymmetric-information theory predicts that
the no-call announcement is **costly** in equilibrium precisely
because issuers in cash-distress states cannot credibly mimic the
no-call action (the funding-need PV pressure forces them toward
Option A), so the separating equilibrium has high-quality / cash-
adequate issuers posting no-call commitments and lower-quality /
cash-distressed issuers calling.
**Source:** Koziol (2004) §2 pp.30-50 (strategic-investor framework
on the signaling content of call timing); DeSpiegeleer et al. (2014)
§2.5 pp.50-78 (option-on-bond framework for soft-call timing).

The **rational holder's choice** in Option A (called) is the
screw-clause comparison covered in [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md):
convert at parity (typically ≥130%·F) versus accept par (100%·F);
conversion is dominant. The **rational holder's choice** in Option
B (no-call) is to continue holding the CB through the committed
window, taking the small bond-price uplift as a windfall and
preserving the embedded-call optionality for the next trigger event;
exit before window expiry is rational only if the holder needs
liquidity or has updated her belief about issuer credit quality.
**Source:** 安道全 (2014) §4 pp.100-130 (holder's screw-clause
decision rule that the strategic game-theory layer inherits).

Asymptotic behavior of the **strategic** 强赎 in the Chinese-market
context follows three regime cases. **Source:** 安道全 (2023 3ed)
§1.10 pp.32-34. Issuer funding-distressed regime: when the firm
faces a binding cash-flow constraint or imminent maturity wall,
Option A dominates; the asymmetric-information channel is mute
because the call is forced by external constraint rather than
chosen strategically. Issuer cash-adequate regime: when the firm
has a strong balance-sheet position and the share has appreciated
meaningfully past the trigger, Option B (or C) typically dominates;
the no-call announcement carries positive signaling weight on issuer
quality. Late-life regime (CB approaches maturity with `T - t < 1`
year): retained-optionality PV is small because few trigger events
remain, so Option B's optionality benefit shrinks and Option A
becomes more likely — though signaling-cost considerations and
prospectus put-clause interactions may still tip the issuer toward
the no-call route.
**Source:** 安道全 (2014) §5 pp.130-150 (Chinese-market empirical
late-life call-vs-no-call pattern).

## See Also

- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — the mechanical 强赎 clause (130% × K_c trigger, 15-of-30 window, ~30-day notice period) that this card's strategic layer sits on top of
- [`cb-china-no-call-commitments.md`](cb-china-no-call-commitments.md) — the commitment-side mechanics layer for Option B (the 不赎回承诺 instrument: window length, disclosure channel, valuation impact); this card carries the strategic-decision content of when the issuer chooses to post the commitment
- [`cb-mandatory-and-exotic-structures.md`](cb-mandatory-and-exotic-structures.md) — the broader call-protection-clause taxonomy in which the China strong-call game-theoretic surface sits

## Escalate to Raw When

Open 安道全 (2023 3ed) §1.10 pp.32-34 directly for the China-specific
practitioner playbook on the call-vs-no-call decision: the empirical
2021 57% no-call statistic, the four reasons issuers publicly cite
for declining to call, the 英科 case study, and the
"缺钱时拿现金, 不缺钱时拿股权" framing. Open 安道全 (2014) §3-§5
pp.80-150 for the pre-2018 baseline practitioner language and the
formal screw-clause holder-decision framework that the strategic
game-theory layer inherits. Open 攻守 §2 pp.17-48 for the
cross-check practitioner-handbook treatment of the strong-call
clause game-play within the broader Chinese-CB clause ecosystem.
Open Koziol (2004) §1-§2 pp.20-50 for the strategic-investor formal
analysis of call timing under asymmetric information, including the
separating-equilibrium derivation that underlies the no-call
announcement's signaling content. Open DeSpiegeleer §2.5 pp.50-78
for the soft-call option-on-bond framework that the path-dependent
trigger generalizes. **Source:** 安道全 (2023 3ed) §1.10 pp.32-34;
安道全 (2014) §3-§5 pp.80-150; 攻守 §2 pp.17-48; Koziol (2004) §1-§2
pp.20-50; DeSpiegeleer et al. (2014) §2.5 pp.50-78.
