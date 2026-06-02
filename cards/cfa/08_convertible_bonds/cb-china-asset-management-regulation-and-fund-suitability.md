---
schema_version: "cacg.v0"
id: "cb-china-asset-management-regulation-and-fund-suitability"
title: "Chinese CB Demand-Side: Asset-Management Regulation + Fund Suitability"
reading_id: "08_convertible_bonds"
summary: "The 2018 PBOC 资管新规 (Asset-Management Guiding Opinions) created the net-value bank-wealth-management product class that became the principal incremental buyer of CBs (the 固收+ flow), and the 2025-10-29 中国银河证券 fund-suitability re-rating crystallized the first explicit allocator-side risk-tier reclassification of CB-heavy bond funds (R1-R5 system)."
tags: ["convertible-bonds", "china-asset"]
citations:
  - source_id: "china_cb_china_galaxy_fund_suitability_2025_10_29"
    chunk_id: "china_cb_china_galaxy_fund_suitability_2025_10_29:p001:0000"
    chunk_hash: "c3b0ef83636553658aab9916f09b63c6fae5baf85aebda396bb293311a9d011a"
    page_range: [1, 2]
    quote: "中国银河证券基金研究中心将公募基金产品按照风险由低 到高顺序，依次划分为：R1、R2、R3、R4、R5 五个等级，相应 的风险等级名称是：低风险、中低风险、中风险、中高风险、高 风险。"
    edge_type: "defines"
  - source_id: "china_cb_pboc_asset_management_guiding_opinions_2018"
    chunk_id: "china_cb_pboc_asset_management_guiding_opinions_2018:p001:0000"
    chunk_hash: "0ce78d35a6b13a8e0c9a352e65601b4c850d11cb88cdaaebc25a08049a109088"
    page_range: [1, 2]
    quote: "《意见》按照产品类型统一监管标准，从募集方式和投资性质两个维度对资 产管理产品进行分类，分别统一投资范围、杠杆约束、信息披露等要求。"
    edge_type: "supports"
  - source_id: "china_cb_lianhe_annual_bond_market_2025"
    chunk_id: "china_cb_lianhe_annual_bond_market_2025:p001:0000"
    chunk_hash: "9508b0fe8ea0123e3e4c9620113ee313ebac27c619bdda0d0a7c0780e43d66d5"
    page_range: [1, 2]
    quote: "我国债券市场共 发行各类债券 88.52 万亿元，同比上升 12.35%，除同业存单外，各类债券合计发行 54.70 万亿元，同比上升 15.40%。"
    edge_type: "supports"
card_hash: "da87f374ad15099c61689c9825290e435e8e2a843295df2ffd42843a4f6d8dcb"
---
# Chinese CB Demand-Side: Asset-Management Regulation + Fund Suitability

## Intuition

The Chinese onshore convertible bond market's demand-side flow
since 2018 has been dominated by a single regulatory regime: the
PBOC 资管新规 (《关于规范金融机构资产管理业务的指导意见》, 银发
〔2018〕106号, Dec 2018) that ended the implicit-guarantee
("刚性兑付") era of bank wealth-management products. The new
regime mandated **net-value (NAV-based)** product valuation,
which structurally re-classified bank wealth-management
liabilities from deposit-like (guaranteed redemption at face
plus accrued) to mutual-fund-like (NAV moves with underlying
asset prices, no guarantee). The asset-allocation response across
the 2018-2025 transition window was the **固收+** (fixed-income-
plus) product class: a NAV-based, predominantly-fixed-income
portfolio augmented with a small (typically 10-20%) allocation to
convertible bonds for return enhancement. By the end of the
post-资管新规 transition window, net-value bank wealth-management
products dominated the total market per Lianhe Ratings annual
report; the CB-heavy bond-fund subset eventually became material
enough that 中国银河证券 issued the first explicit allocator-side
fund-suitability re-rating that pulled CB-heavy bond funds out
of the medium-low risk tier into the medium risk tier, codifying
the structural re-pricing of CB-fund risk that the recent default
cohort had made unavoidable. **Source:** 中国银河证券 fund-
suitability rating (2025-10-29) §1 pp.1-4; PBOC 资管新规
(Dec 2018) §1-§3 pp.1-8; Lianhe annual report (2025) §1 pp.1-3.

```
   Chinese CB demand-side pipeline (post-资管新规 reform)
   -----------------------------------------------------

   household savings   bank wealth-management   bond/mixed funds
       ↓                  ↓                       ↓
   (pre-reform: implicit guarantee — "刚性兑付")
   (post-reform: PBOC 资管新规 → NAV-based products)
       ↓                  ↓                       ↓
       └──→ 固收+ product class with small CB allocation
       └──→ CB-heavy bond funds
                                ↓
                          CB market demand
                                ↓
   (default cohort era → CB-fund risk re-pricing)
                                ↓
                  Galaxy Securities suitability re-rating
                  for CB-heavy bond funds (moved up one tier)
```

## Definition

The 2018 资管新规 framework codifies asset-management products
into a regulated class with three structurally-distinct rules.
**Source:** PBOC 资管新规 (Dec 2018) §1-§3 pp.1-8.

- **Net-value (NAV-based) product valuation**: bank wealth-
  management and equivalent asset-management products must be
  marked to market (or to model where market prices are absent)
  on a periodic basis (typically daily for open-end products).
  Redemption is at the prevailing NAV, NOT at face plus accrued.
  The rule ended the "刚性兑付" expected-guarantee equilibrium
  that prevailed pre-2018. **Source:** PBOC 资管新规 (Dec 2018)
  §2 pp.3-6.

- **Single-investor concentration limit**: each asset-management
  product faces a per-issuer concentration limit (typically 10%
  of NAV for a single underlying issuer), constraining the
  product's ability to take outsized positions in any single CB
  or single CB issuer. This is the supply-side check that
  prevents allocator-driven concentration risk. **Source:** PBOC
  资管新规 (Dec 2018) §3 pp.6-8.

- **Tiered investor-suitability requirement**: the rule mandates
  that asset-management products be sold only to investors whose
  risk-tolerance assessment matches the product's risk tier. The
  five-tier risk vocabulary (low / medium-low / medium / medium-
  high / high) becomes the operational standard. **Source:** PBOC
  资管新规 (Dec 2018) §3 pp.6-8.

The 中国银河证券 2025-10-29 fund-suitability re-rating codifies
the allocator-side response to the 2024-2025 default cohort
documented in
[`cb-china-default-cohort-attribution`](cb-china-default-cohort-attribution.md#definition).
**Source:** 中国银河证券 (2025-10-29) §1-§3 pp.1-17.

- **5-tier risk vocabulary**: R1 (低风险), R2 (中低风险), R3
  (中风险), R4 (中高风险), R5 (高风险) — Galaxy's classification
  follows the standard 资管新规-derived tier-set. **Source:**
  中国银河证券 (2025-10-29) §1 pp.1-3.

- **CB-heavy bond fund re-classification (up one tier from medium-low to medium)**: bond funds
  whose portfolio CB allocation exceeds a threshold (per Galaxy's
  internal methodology, a structurally non-trivial CB exposure;
  the exact quantitative threshold is practitioner-discretionary
  and not specified in the public document) are reclassified out
  of the medium-low (中低风险) tier into medium (中风险) tier. This is the FIRST explicit
  allocator-side reclassification of CB-heavy funds in the
  post-资管新规 regime. **Source:** 中国银河证券 (2025-10-29) §2
  pp.4-10.

- **Small-cap-index equity-fund parallel re-rating**: in the
  same 2025-10-29 release Galaxy also re-rated small-cap-index
  equity funds, framing the CB re-rating as part of a broader
  systemic-risk-tier recalibration rather than CB-specific
  reactive risk-management. **Source:** 中国银河证券 (2025-10-29)
  §3 pp.10-17.

## Mathematical Reasoning

The structural identity that the 2018 资管新规 enforced is the
NAV-based valuation rule, which decouples bank wealth-management
liabilities from deposit-like guarantees. **Source:** PBOC 资管
新规 (Dec 2018) §2 pp.3-6.

```
pre-2018 implicit-guarantee equilibrium:
  V_redemption(t) = P_face + Σ accrued_yield
                    (regardless of underlying asset NAV)
  
post-2018 NAV-based regime:
  V_redemption(t) = NAV(t) · units_held
                    = Σ (market_value_i / units_outstanding) · units_held
                    i over portfolio assets
```

**Source:** PBOC 资管新规 (Dec 2018) §2 pp.3-6.

The structural consequence: bank wealth-management products
became sensitive to CB price moves in proportion to their CB
allocation. The 固收+ product class crystallized as the typical
allocator response with a CB allocation weight `w_CB ∈ [0.10,
0.20]` of NAV. **Source:** Lianhe (2025) §1 pp.1-3.

```
固收+ fund NAV sensitivity to CB price:
  ∂NAV / ∂P_CB = w_CB
  
  (the per-unit-CB-price impact on fund NAV equals the CB
  weight in the portfolio, excluding diversification effects
  and other portfolio holdings).
```

**Source:** Lianhe (2025) §1 pp.1-3; 中国银河证券 (2025-10-29)
§2 pp.4-10.

The Galaxy 2025-10-29 R2 → R3 re-classification of CB-heavy bond
funds is the allocator-side recognition that 固收+ products
carry materially higher tail-risk than the original R2 (中低
风险) tier admitted. The re-classification implies that under the
资管新规 tiered-investor-suitability rule, CB-heavy funds may
only be sold to investors whose own risk-tolerance assessment
clears R3 (中风险). **Source:** 中国银河证券 (2025-10-29) §1-§2
pp.1-10; PBOC 资管新规 (Dec 2018) §3 pp.6-8.

The dynamic effect on the CB demand-side: investor-suitability
filtering reduces the eligible pool of buyers for CB-heavy
funds, which structurally reduces the natural demand for
incremental CB issuance from the 固收+ channel. The downstream
implication for
[`cb-investor-clientele`](cb-investor-clientele.md#mathematical-reasoning)
is a structurally lower demand-side floor for the Chinese CB
market post-2025-10-29 — though the quantitative magnitude of the
demand-side shift is practitioner-tracked rather than rule-
codified. **Source:** 中国银河证券 (2025-10-29) §2 pp.4-10.

## See Also

- [`cb-investor-clientele.md`](cb-investor-clientele.md) — generic CB clientele taxonomy that this card's Chinese-specific demand-side framework feeds (the Chinese-specific 固收+ channel is one regional instance of the generic clientele typology)
- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — trading-rule baseline; the demand-side regulatory layer covered here sits above the trading layer
- [`cb-china-default-cohort-attribution.md`](cb-china-default-cohort-attribution.md) — 2024-2025 default cohort that drove the 2025-10-29 Galaxy re-rating
- [`cb-china-distressed-workouts.md`](cb-china-distressed-workouts.md) — distressed-CB resolution mechanics that surfaced the CB-fund risk-tier mismatch

## Escalate to Raw When

Open 中国银河证券 公募基金产品适当性风险等级 (2025-10-29 版) §1-§3
pp.1-17 directly for the allocator-side fund-suitability rating
methodology, the R1-R5 tier vocabulary, the CB-heavy bond fund
R2 → R3 reclassification, and the parallel small-cap-index
equity fund re-rating. **Source:** 中国银河证券 (2025-10-29) §1-§3
pp.1-17.

Open PBOC 资管新规 (银发〔2018〕106号, Dec 2018) §1-§3 pp.1-8
for the original NAV-based product valuation rule, the single-
investor concentration limit, and the tiered investor-suitability
requirement that the 2025-10-29 Galaxy re-rating operationalizes.
**Source:** PBOC 资管新规 (Dec 2018) §1-§3 pp.1-8.

Open Lianhe 2025 年债券市场发展报告 §1 pp.1-3 for the 2024 end-
year statistics on net-value bank wealth-management share
(27.84 trillion yuan, 97.6% net-value-converted) that quantifies
the maturity of the post-2018 demand-side transition.
**Source:** Lianhe (2025) §1 pp.1-3.
