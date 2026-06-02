---
schema_version: "cacg.v0"
id: "ec-currency-exchange-rates-and-parity"
title: "Currency Exchange Rates and Parity Framework (Notes Ch.9)"
reading_id: "02_economics"
summary: "CFA L1 R14 currency-exchange framework: spot/forward quote mechanics with bid-offer spread, three parity conditions (CIRP, absolute PPP, relative PPP), capital-flow effects on FX, and the Mundell-Fleming taxonomy with the impossible trinity."
tags: ["economics", "currency-exchange"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1020:1457"
    chunk_hash: "2a9f1f1ad3d2177755705ea12c33ba5d5e67b47d052b9873ac197518a4a8888b"
    page_range: [1020, 1021]
    quote: "Also, even under an “independent float” regime monetary authorities will occasionally intervene in foreign exchange markets in order to influence the value of their domestic currency."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1021:1458"
    chunk_hash: "a698072d983d6c63e4f7d8d94d1e3a13b2ec4832a41eeadace5e95a41e5b1b88"
    page_range: [1021, 1022]
    quote: "In either case, the country gives up the ability to conduct its own monetary policy."
    edge_type: "defines"
card_hash: "77db46b8a0db9c47106480dac69e86f0f0fa90faa45f8f1f3446502b51b5e06c"
---
# Currency Exchange Rates and Parity Framework (Notes Ch.9)

## Intuition

A **currency exchange rate** is the relative price of one currency in terms of another, conventionally quoted as `S(A/B)` = units of currency A per one unit of currency B. The spot market has a **bid-offer spread**: the dealer bids (buys B at low price) and offers (sells B at higher price), with the spread covering inventory risk, processing cost, and dealer markup. The forward market trades currency at a future date with a price set today; the no-arbitrage relation linking the spot rate, the forward rate, and the two countries' interest rates is **covered interest-rate parity (CIRP)** — the cornerstone parity condition that holds essentially exactly because any deviation creates a riskless arbitrage. The other two parity conditions — **absolute PPP** (the same basket should cost the same in different currencies after FX conversion) and **relative PPP** (FX changes should equal inflation differentials) — hold only as long-run statistical regularities, not as no-arbitrage conditions. **Source:** CFA Notes Ch.9 pp.90-94.

```
   the three parity conditions (CFA notes Ch.9)

   spot S(A/B)  ←→  forward F(A/B)
        \                /
         \  CIRP arbitrage:
          \    F/S = (1+r_A) / (1+r_B)
           \  /
            \/
        no-arbitrage
        (essentially exact)

   absolute PPP:                  relative PPP:
   S(A/B) = P_A / P_B             (S2 - S1)/S1 = π_A - π_B
   (basket equals across          (FX change equals
    currencies, after FX)          inflation differential)
   ----------------------------   ------------------------
   holds only approximately       holds as long-run
   in long run; commodity         statistical regularity;
   bundles + tradability gaps     not exact period-to-period
```

The **Mundell-Fleming framework** characterizes how monetary and fiscal policy work under three combinations: (a) fixed FX with perfect capital mobility (monetary policy ineffective because central bank must intervene to defend the peg; fiscal policy effective and amplified); (b) flexible FX with perfect capital mobility (monetary policy effective and amplified via the exchange-rate channel; fiscal policy crowded out via FX appreciation); (c) intermediate regimes with partial capital mobility (intermediate outcomes). The framework's policy-design implication is the "impossible trinity" — a country can pick at most two of {fixed FX, free capital mobility, independent monetary policy}, never all three. **Source:** CFA Notes Ch.9 pp.94; supporting CFA Institute (2022) Vol.2 pp.443-473.

## Definition

The **spot exchange-rate quote** conventions. **Source:** CFA Notes Ch.9 pp.90-91.

```
S(USD/EUR)  =  exchange rate, USD per one EUR              (price-quote)
S(EUR/USD)  =  1 / S(USD/EUR)                              (inverse quote)
bid:        rate at which dealer BUYS the base currency
offer:      rate at which dealer SELLS the base currency
spread:     offer − bid (positive; dealer revenue + risk margin)
```

**Source:** CFA Notes Ch.9 pp.90-91.

The **covered interest-rate parity (CIRP)** in symbolic form. **Source:** CFA Notes Ch.9 pp.92.

```
F(A/B)  =  S(A/B) · (1 + r_A · T) / (1 + r_B · T)              [exact]
F(A/B) / S(A/B)  ≈  1 + (r_A − r_B) · T                         [linearized]

where:  F = forward rate at maturity T
        S = spot rate at quote date
        r_A, r_B = period interest rates in currencies A and B
        T = maturity in periods
```

CIRP holds essentially exactly in deep liquid markets because any deviation creates a riskless arbitrage (borrow in cheap-rate currency, convert at spot, lend in expensive-rate currency, convert back at forward — profit equals the CIRP deviation). The 07 derivatives vertical develops the trading-desk arbitrage in detail; this 02 card states the parity as an L1 fact. **Source:** CFA Notes Ch.9 pp.92.

The **absolute and relative PPP** in symbolic form. **Source:** CFA Notes Ch.9 pp.92-93.

```
absolute PPP:   S(A/B)  =  P_A / P_B                       [basket-equality]
relative PPP:   (S_t − S_{t-1}) / S_{t-1}  ≈  π_A − π_B     [inflation diff]
```

where `P_A`, `P_B` are the basket prices in each currency. Both forms hold only as long-run statistical regularities; period-to-period deviations are large and persistent because non-tradeable goods, transportation costs, tariff barriers, and market segmentation prevent immediate arbitrage. **Source:** CFA Notes Ch.9 pp.92-93.

The **capital-flow-FX relationship** (notes pp.93-94). **Source:** CFA Notes Ch.9 pp.93-94.

```
↑ Capital inflows (foreigners buy domestic assets) → ↑ demand for domestic
                                                     → ↑ S(foreign/domestic)
                                                       (domestic appreciates)

↓ Capital outflows (domestic buyers buy foreign assets) → ↑ demand for foreign
                                                        → ↑ S(domestic/foreign)
                                                          (foreign appreciates)
```

The L1 framing emphasizes the directional reasoning: a country with attractive yields draws capital → currency appreciates → export competitiveness suffers; conversely, a country with poor yields loses capital → currency depreciates → exports become more competitive. **Source:** CFA Notes Ch.9 pp.93-94.

## Mathematical Reasoning

The **CIRP derivation**: consider two strategies for $1 today held until time `T`. Strategy 1: invest in currency A at rate `r_A`, ending with `1 + r_A · T` units of A. Strategy 2: convert $1 to currency B at spot `S(A/B)`, invest in B at rate `r_B`, ending with `(1 + r_B · T) / S(A/B)` units of B, then convert back at the forward rate `F(A/B)` to get `(1 + r_B · T) · F(A/B) / S(A/B)` units of A. The two strategies must produce the same A-denominated payoff (else arbitrage), so `1 + r_A · T = (1 + r_B · T) · F(A/B) / S(A/B)`, which rearranges to the canonical CIRP `F/S = (1 + r_A · T)/(1 + r_B · T)`. The derivation is exact under no-frictions; with bid-ask spreads and credit/transaction costs, a narrow band of CIRP-consistent forward rates exists rather than a single equilibrium. **Source:** CFA Notes Ch.9 pp.92.

The **Mundell-Fleming open-economy IS-LM-BP framework** characterizes simultaneous equilibrium in goods market (IS), money market (LM), and FX / balance-of-payments market (BP). Under perfect capital mobility, BP becomes a horizontal line at the world interest rate, and the IS-LM equilibrium pins down output and exchange rate. The four canonical cases. **Source:** CFA Notes Ch.9 pp.94.

```
Fixed FX, perfect mobility:
  monetary policy → ineffective (central bank must intervene to defend peg)
  fiscal policy   → amplified (crowding-out neutralized by capital inflows
                                that prevent rate rise)

Flexible FX, perfect mobility:
  monetary policy → amplified (rate change moves FX, which moves NX)
  fiscal policy   → reduced (FX appreciation from capital inflows
                              crowds out NX)
```

The L1 exam tests directional outcomes ("under flexible exchange rates and perfect capital mobility, expansionary monetary policy will ___ the currency and ___ net exports") rather than algebraic solution of the IS-LM-BP system. **Source:** CFA Notes Ch.9 pp.94; supporting CFA Institute (2022) Vol.2 pp.443-473.

The **impossible trinity** is the structural-policy implication of Mundell-Fleming. Among the three policy goals — (a) fixed exchange rate, (b) free capital mobility, (c) independent monetary policy — a country can achieve at most two simultaneously. A fixed FX + open capital account forces the central bank to import the anchor country's monetary policy (lose independence). Free capital + independent monetary policy requires letting the exchange rate float (lose fixed FX). Fixed FX + independent monetary policy requires capital controls (lose mobility). The trinity frames every modern international-monetary regime choice: Hong Kong (a + b, sacrifices c via dollar peg); China historically (a + c, sacrifices b via capital controls); most advanced economies (b + c, sacrifices a via floating). **Source:** CFA Notes Ch.9 pp.94.

## See Also

- [`ec-international-trade-and-capital-flows`](./ec-international-trade-and-capital-flows.md) — CFA L1 R13 trade-policy + BOP framework that uses the same capital-mobility taxonomy
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — domestic policy mechanics that interact with the FX channel here
- [`ec-monetary-policy-and-inflation`](./ec-monetary-policy-and-inflation.md) — Romer-anchored monetary-policy framework underlying the FX response

## Escalate to Raw When

The full CFA L1 R14 currency-exchange-rates content (chapter-detail treatment of cross-rate triangulation, the forward-rate biasedness debate, currency overlay strategies, FX intervention sterilization mechanics) is in CFA Institute (2022) Vol.2 pp.443-473. The 07 derivatives-vertical treatment of CIRP arbitrage at trading-desk depth (basis-swap pricing, cross-currency swap mechanics, x-CCY OIS dislocations during stress) is in the convertible-bonds + derivatives subcorpora. The Mundell-Fleming graduate-macro treatment (full IS-LM-BP algebra, perfect-vs-imperfect-substitutes asset model, Dornbusch overshooting) is in the international-macro literature out of v10 scope. **Source:** CFA Notes Ch.9 pp.90-94.
