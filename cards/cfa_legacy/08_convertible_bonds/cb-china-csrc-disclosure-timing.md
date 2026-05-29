---
schema_version: "cacg.v0"
id: "cb-china-csrc-disclosure-timing"
title: "China Convertible-Bond CSRC Disclosure Timing (Standard No. 60)"
reading_id: "08_convertible_bonds"
summary: "CSRC Standard No. 60 governs the prospectus a Chinese listed company must publish to issue a convertible bond to unspecified investors. Article 7 mandates SSE/SZSE-website + CSRC-approved-newspaper-website disclosure; Article 8 forbids any other-channel disclosure earlier than the exchange disclosure; Article 17 enumerates the CB-specific contract terms — conversion-price adjustment with shareh..."
tags: ["convertible-bonds", "china-csrc"]
citations:
  - source_id: "china_cb_csrc_disclosure_standard_no_60"
    chunk_id: "china_cb_csrc_disclosure_standard_no_60:p007:0006"
    chunk_hash: "02229611f9a43cbcb595dd7d2ea77377e7a2b3c5e569078af750c1e4a43fdf83"
    page_range: [7, 8]
    quote: "（八）赎回条款，规定上市公司可按事先约定的条件和价格 赎回尚未转股的可转债；"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p014:0013"
    chunk_hash: "b8d22a0115f690fe31f95410fdee5e3a03f52a1374e926b5fbba64366d3790e3"
    page_range: [14, 15]
    quote: "可转债，全名叫做“可转换公司债券”（Convertible bond；CB），"
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p041:0045"
    chunk_hash: "53fc3079dc48462a65dff49f335124d01c4ec34bcb4e2029a3a7eadf661488ad"
    page_range: [41, 42]
    quote: "2.2 ANATOMY OF A CONVERTIBLE BOND"
    edge_type: "supports"
card_hash: "2085486568d2e23c67b88f42b15ac26f17c111389f3ad66111d5bd0d424ee1bf"
---
# China Convertible-Bond CSRC Disclosure Timing (Standard No. 60)

## Intuition

When a Chinese listed company wants to issue a convertible bond to
unspecified investors (向不特定对象发行可转债), the **prospectus**
(募集说明书) is the single most-load-bearing document in the issuance
chain. CSRC Standard No. 60 governs (a) WHERE the prospectus must be
disclosed, (b) WHEN it can land on which channels, and (c) WHAT
CB-specific clauses (转股价格 / 下修 / 赎回 / 回售) must appear inside
the prospectus. The standard applies uniformly to share offerings,
convertible-bond offerings, and depositary-receipt offerings; the
CB-specific articles (chiefly Article 17) layer on top of the general
prospectus articles. **Source:** CSRC Standard No. 60 §1-§2 pp.1-2.

```
 prospectus disclosure channels (Article 7 + 8)
    │
    │ MUST: exchange website (SSE/SZSE)
    │       + CSRC-approved newspaper-affiliated websites
    │
    │ MAY (Article 8 ordering rule):
    │   other channels — but NOT earlier than the
    │   exchange / approved-newspaper disclosure
    ▼
 prospectus content (Article 17 for CB-specific items)
   ├─ (七) 转股价格调整 (conversion-price adjustment)
   │      + 下修条款 vote rule: shareholder meeting,
   │        ≥ 2/3 majority of voting shareholders,
   │        CB-holders abstain
   ├─ (八) 赎回条款 (call provision; pre-agreed condition + price)
   └─ (九) 回售条款 (put provision; pre-agreed condition + price;
                    use-of-proceeds change auto-triggers one
                    回售 right)
```

## Definition

CSRC Standard No. 60 Article 7 enumerates two **mandatory** disclosure
venues for the prospectus: (i) the relevant exchange's website
(`www.sse.com.cn` or `www.szse.cn`), and (ii) CSRC-approved
newspaper-affiliated websites. The issuer must additionally publish a
prompt-notification announcement (提示性公告) on the same channels
listing the security type, issuance size, face value (面值), issuance
method, planned issuance date, and contact information of the issuer
and underwriters. Article 8 layers the **ordering invariant**: if the
issuer chooses to disclose the prospectus on additional channels
beyond the two mandatory venues, the content must be identical AND
the additional-channel disclosure must not precede the exchange /
approved-newspaper disclosure. This anti-arbitrage rule prevents
selective pre-release to favored channels. **Source:** CSRC Standard
No. 60 §7-§8 pp.2-3.

```
forbidden ordering (Article 8 negative test):
    other_channel_disclosure_time  <  exchange_disclosure_time
                          ^                    ^
                          │                    │
                          │                    └─ MUST be earlier or equal
                          └─ MUST NOT be earlier
```

## Mathematical Reasoning

### Channel-ordering invariant

Let `T_ex` denote the timestamp of prospectus disclosure on the
exchange website (or CSRC-approved newspaper-affiliated website), and
`T_other` the timestamp of disclosure on any optional additional
channel chosen by the issuer. Let `C_ex` and `C_other` denote the
respective disclosed contents. Standard No. 60 Article 8 imposes the
joint invariant on `(T_ex, T_other, C_ex, C_other)`.
**Source:** CSRC Standard No. 60 §7-§8 pp.2-3.

```
(C_other ≡ C_ex)   ∧   (T_other ≥ T_ex)
```

The first conjunct (`≡`, identical-content) is enforced by the
disclosure-completeness requirement; the second conjunct
(`T_other ≥ T_ex`) is the no-prior-disclosure ordering rule. The
contrapositive — `T_other < T_ex` — yields a Standard-No.-60
violation regardless of intent, since the rule is structural rather
than mens-rea-based. **Source:** CSRC Standard No. 60 §7-§8 pp.2-3.

### Clause-set composition for Article 17 CB-specific items

Let `P` denote the prospectus clause-set required by Article 17 for a
CB issuance. Standard No. 60 partitions `P` into a general-prospectus
core (clauses 一–六) and a CB-specific extension (clauses 七–九).
**Source:** CSRC Standard No. 60 §17 pp.6-7.

```
P = P_core  ∪  P_CB
  where
  P_core ⊇ {issuer-info, financial-statements, use-of-proceeds, ...}
  P_CB   = {(七) conversion-price-adjustment,
            (八) call-provision,
            (九) put-provision}
```

`P_CB` is the **embedded-option declaration set**: clause (七)
specifies the strike-adjustment dynamics, clause (八) the issuer's
unilateral call right, clause (九) the holder's unilateral put right.
The three clauses together encode the optionality surface that the
binomial-tree / PDE valuation machinery in `cb-binomial-tree-valuation`
and `cb-pde-and-free-boundary` operates on. **Source:** CSRC Standard
No. 60 §17 pp.6-7.

### Voting-rule constraint on 下修 (clause 七)

The 下修 sub-clause of Article 17(七) layers a discrete voting-rule
constraint on top of the strike-adjustment formula.
**Source:** CSRC Standard No. 60 §17(七) pp.7.

```
Let V_total = total voting shares present at the meeting
Let V_yes   = shares voting in favor of the 下修 proposal
Let V_CB    = shares held by entities that also hold the CB
              (these MUST abstain — 回避表决)

Article 17(七) admissibility:
    (V_total - V_CB)  ≥  (1 share)             # meeting valid
    V_yes / (V_total - V_CB)  ≥  2/3           # supermajority
    K_c_new  ≥  meeting-time benchmark         # floor on adjustment
```

The "voting CBs abstain" rule (`V_CB` removed from both numerator and
denominator of the supermajority ratio) prevents the convertible
holders from voting themselves into a favorable conversion-price
adjustment. The China-market 下修 mechanism is therefore a **board
proposal + non-CB-holder shareholder ratification** procedure, not a
pure issuer right. **Source:** CSRC Standard No. 60 §17(七) pp.7.

## CB-specific prospectus content (Article 17)

Article 17 lists the CB-specific items that the prospectus of a
向不特定对象-issued convertible bond MUST disclose. The salient items
for the embedded-option valuation perspective are clauses (七), (八),
and (九). **Clause (七) — 转股价格调整原则 (conversion-price adjustment)**:
Two mechanisms apply. First, when the issuer undertakes corporate actions
that mechanically dilute the underlying — e.g., 配股 (rights issue), 派
息 (dividends), 送股 (bonus shares), 资本公积金转增 (capitalization),
拆股 (splits) — the conversion price `K_c` must be adjusted in lockstep
under a pre-disclosed formula. Second, the prospectus may include a
**downward-revision** (向下修正) clause; if it does, the prospectus
must specify that any 下修 proposal goes to a shareholder meeting
requiring a ≥ 2/3 majority of voting shareholders, with the convertible
holders themselves abstaining (回避表决). The modified `K_c` cannot
fall below the meeting-time benchmark.
**Clause (八) — 赎回条款 (call provision)**: The prospectus must spell
out the conditions and price at which the issuer can call un-converted
convertible bonds. The condition + price combination determines the
issuer's **strong-call** (强赎) lever, and downstream practitioner
mechanics — the typical 130% × `K_c` trigger and the 15-of-30
consecutive-trading-day window — are anchored in this clause without
being mandated by Standard No. 60 itself.
**Clause (九) — 回售条款 (put provision)**: The prospectus must
disclose the conditions and price at which the convertible holder can
put the bond back to the issuer. A special **auto-triggered** put right
applies whenever the issuer changes the announced use-of-proceeds — the
prospectus must grant CB holders one additional 回售 opportunity in
that scenario. **Source:** CSRC Standard No. 60 §17(七)-(九) pp.6-7.

## Why these three clauses bind the entire CB lifecycle

The 下修 + 赎回 + 回售 trio in Article 17 establishes the **strategic
interaction surface** for every Chinese-CB lifecycle event documented
in the 08 vertical. The issuer pulls 下修 to stretch the conversion
window when share price drifts below `K_c`; pulls 强赎 (under clause
(八)'s pre-agreed conditions) to force conversion when share price runs
≥ 130% × `K_c`; and the holder pulls 回售 (under clause (九)) when the
embedded equity option is far OTM or when the issuer mis-uses
proceeds. The prospectus is therefore both the *disclosure* document
and the *contract* for the three game-theoretic levers that drive
post-2014 Chinese-CB practice. **Source:** 安道全 (2014) §3-§5
pp.30-90; CSRC Standard No. 60 §17(七)-(九) pp.7.

## Comparison with Western-market prospectus mechanics

In the US, the equivalent disclosure document is the Form 424 prospectus
filed with the SEC under Rule 415; Western convertible-bond
practitioner sources describe the prospectus as the contract for the
embedded option + bond cash flows, but the SEC framework does not
mandate the 2/3-supermajority shareholder vote on conversion-price
adjustments that Standard No. 60 Article 17(七) requires — Western
issuers typically reserve unilateral anti-dilution adjustments via a
trustee-administered formula. The China clause requires shareholder
participation, which makes 下修 a **board-shareholder negotiation**
rather than a mechanical adjustment. **Source:** DeSpiegeleer et al.
(2014) §1.4-§1.7 pp.21-30.

## See Also

- [cb-conversion-feature-mechanics](./cb-conversion-feature-mechanics.md) — generic CB conversion-price
  mechanics that Article 17(七) instantiates for the Chinese market.
- [cb-china-trading-mechanics](./cb-china-trading-mechanics.md) — broader Chinese-market trading
  context (T+0 settlement, daily price limits) within which the
  Standard No. 60 disclosure regime sits.
- [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md) — practitioner detail on the 130%
  × `K_c` strong-call trigger and the 15-of-30 window referenced by
  Article 17(八).
- [cb-china-downward-conversion](./cb-china-downward-conversion.md) — practitioner mechanics of the
  下修 clause whose disclosure form is mandated by Article 17(七).

## Escalate to Raw When

The card covers the **disclosure** side of Standard No. 60 + the
high-level CB-clause content of Article 17. For investigations that
require the **post-issuance** ongoing disclosure cadence (e.g., the
exact timing of a 强赎公告 within the 15-of-30 trigger window, the
prospectus-supplement timing for a 下修 board resolution, or the
disclosure timing of a 回售 trigger event), refer to the sibling card
[`cb-china-exchange-price-limit-suitability-rules`](./cb-china-exchange-price-limit-suitability-rules.md)
which covers the SSE/SZSE rulebook layer. For industry-specific
information-disclosure overrides — banks, insurance, certain state-
owned enterprises — Standard No. 60 Article 11 defers to industry-
specific CSRC rules; consult those before relying on this card for
industry-specific issuers. **Source:** CSRC Standard No. 60 §11 pp.3;
§7-§8 pp.2-3.
