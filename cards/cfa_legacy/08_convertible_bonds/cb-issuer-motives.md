---
schema_version: "cacg.v0"
id: "cb-issuer-motives"
title: "Issuer Motives"
reading_id: "08_convertible_bonds"
summary: "Issuer Motives — placeholder summary                                            "
tags: ["convertible-bonds", "issuer-motives"]
citations:
  - source_id: "cb_koziol_2004_convertible_bonds_strategic_investors"
    chunk_id: "cb_koziol_2004_convertible_bonds_strategic_investors:p031:0034"
    chunk_hash: "b3a7e529059ac07bb00e76d7d2825555819d1766a59f376a3eb00f1845d8b652"
    page_range: [31, 31]
    quote: "The sweetened-debt feature states that a bond becomes more attractive for investors by the embedded conversion right."
    edge_type: "supports"
  - source_id: "cb_koziol_2004_convertible_bonds_strategic_investors"
    chunk_id: "cb_koziol_2004_convertible_bonds_strategic_investors:p031:0034"
    chunk_hash: "b3a7e529059ac07bb00e76d7d2825555819d1766a59f376a3eb00f1845d8b652"
    page_range: [31, 31]
    quote: "Consequently, the firm can achieve lower coupon and/or face value payments compared to a straight bond and still obtains the same issue price as with the straight bond."
    edge_type: "supports"
  - source_id: "cb_philips_1997_convertible_bond_markets"
    chunk_id: "cb_philips_1997_convertible_bond_markets:p033:0034"
    chunk_hash: "c48656354541f988ff5073c5a6eb65a97974ab0fea066179d48607c8317e0d07"
    page_range: [33, 34]
    quote: "Convertibles provide for flexibility within the capital structure, and may play the part of foreign exchange management."
    edge_type: "supports"
card_hash: "ee3d544ea0f59cdb0f27bd2c0ca15cd38ef2f16bc8f75d7f4236d9628ae78f42"
---
# Issuer Motives

## Intuition

Convertible bonds exist because they let issuers raise capital that is
**neither pure debt nor pure equity**. The coupon is below straight-bond
yields (the implicit-call-premium discount), so cash interest is reduced;
no immediate dilution occurs (unlike a secondary offering); and conversion
delivers equity at a price above today's share price (the conversion
premium), so the eventual dilution is "cheap" relative to spot. These
properties make convertibles attractive to growth issuers, distressed
issuers, and issuers signaling future-cash-flow expectations.
**Source:** Calamos (2003) §10 pp.220-245.

```
issuer's funding-cost frontier (qualitative):

cost
  ^
  |          straight debt
  |             /
  |            /
  |       convertible
  |          /
  |         /
  |    equity (cost of equity)
  |       /
  +--------------------------> credit quality / equity premium
```

## Definition

The standard practitioner taxonomy distinguishes **four** issuer-side
motives, each corresponding to a different convertible structure or
positioning. **Source:** Calamos (2003) §10 pp.220-245; Philips (1997) §1
pp.15-30.

- **Coupon savings**: issuing a convertible at a sub-straight coupon
  `c_conv < c_straight` lowers cash interest expense in exchange for
  potential dilution. **Source:** Calamos (2003) §10 pp.225-235.
- **Delayed-equity issuance**: a growth issuer that expects share price
  to appreciate prefers to "sell equity at a future higher price" via a
  convertible's conversion premium, rather than at today's spot.
  **Source:** Philips (1997) §1 pp.20-35.
- **Information-asymmetry signaling**: under Stein-style models, a
  convertible signals that management has **moderate** confidence in
  future cash flow — strong confidence would justify straight debt
  (avoiding any dilution); weak confidence would justify equity (avoiding
  hard interest payments). **Source:** Koziol (2004) §2 pp.20-50.
- **Distressed funding access**: distressed issuers excluded from the
  straight-bond market may still issue mandatorily-convertible securities
  (PEPS/DECS family; see the
  [mandatory card](./cb-mandatory-vs-optional-conversion.md#definition))
  because the conversion-into-equity feature reduces lender bankruptcy
  risk. **Source:** DeSpiegeleer et al. (2014) §1.5 pp.20-30.

The **issuer call** option (see the
[call/put card](./cb-call-and-put-protection.md#definition)) is the issuer's
mechanism to terminate the dilution exposure once the share price has
risen enough that conversion is rational. The interaction of conversion
premium, coupon savings, and call schedule is the issuer's **structuring
problem**: minimize the all-in funding cost subject to investor demand for
a specific risk/reward profile. **Source:** Calamos (2003) §10 pp.245-260.

## Mathematical Reasoning

The issuer's expected funding cost (per dollar of face) is the present-
value-weighted sum of coupons paid until conversion plus the discounted
share-dilution cost on conversion; for a non-callable, non-puttable,
optional CB with conversion-eligible date set `T_conv`, the expected
funding cost satisfies the identity displayed below. **Source:** Koziol
(2004) §2 pp.30-50.

    Cost_issuer(0)  =  Σ_{t_k ∈ T_conv} c · F · D_rf(0, t_k) · P(τ > t_k)
                       + E^Q[ q · S(τ) · D_rf(0, τ) ]

where `τ` is the holder's optimal conversion time (if any) and `D_rf(0, ·)`
is the riskless discount factor. **Source:** Koziol (2004) §2 pp.40-50.

The implicit **embedded-call premium** the issuer effectively sells is the
PV of the coupon savings between straight-debt and convertible coupons
over the bond's life. **Source:** Calamos (2003) §10 pp.230-245.

    Premium_issuer  ≈  c_straight · F · A_T  -  c_conv · F · A_T

Here `A_T` is the riskless annuity factor over `[0, T]`. The issuer
"trades" `Premium_issuer` of cash interest savings for the future
dilution cost of conversion. **Source:** Calamos (2003) §10 pp.230-245.

In equilibrium under Stein-style asymmetric-information models the
issuance choice (debt vs equity vs convertible) is a function of the
issuer's **private** belief about future cash flow `μ`. The convertible
is the equilibrium choice for moderate `μ`: high `μ` → straight debt;
low `μ` → equity; in between → convertible. The model rationalizes the
empirical regularity that convertible issuance peaks during bull markets
(when share prices are high enough that conversion at premium is plausible)
and during distressed periods (when straight debt is gated by credit).
**Source:** Koziol (2004) §2 pp.20-40.

The issuer's optimal **call schedule** balances two effects. **Source:**
DeSpiegeleer et al. (2014) §3.5 pp.78-90.

- Calling early forces conversion (avoids further dilution from in-the-
  money call), but caps the per-bond funding savings and may stress the
  share price via supply. **Source:** DeSpiegeleer et al. (2014) §3.5
  pp.78-90.
- Calling late preserves coupon-saving optionality but exposes the issuer
  to deeper dilution if the share price keeps rising. **Source:** Calamos
  (2003) §10 pp.245-260.

### Issuer-Side T-Account

Issuance bookkeeping shows cash arriving on the asset side while the
liability side carries both the debt component (face value) and the
implicit equity component (the embedded option valued at `q · c`).
**Source:** Calamos (2003) §10 pp.220-245; DeSpiegeleer et al. (2014)
§2.1 pp.20-30.

```
<!-- primitive: cb-t-account source: _diagram_primitives.md -->
+----------------------+----------------------+
|  ASSETS              |  LIABILITIES & EQUITY|
+----------------------+----------------------+
|  Cash + F            |  Convertible bond F  |
|                      |  (debt component)    |
|                      |  Embedded option q·c |
|                      |  (equity component)  |
+----------------------+----------------------+
```

## See Also

- [`cb-bond-anatomy-and-cashflows.md`](cb-bond-anatomy-and-cashflows.md) — the bond fields the motive analysis acts on
- [`cb-mandatory-vs-optional-conversion.md`](cb-mandatory-vs-optional-conversion.md) — distressed-funding mandatory variant
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — the issuer-call structuring lever
- [`cb-conversion-premium.md`](cb-conversion-premium.md) — the conversion premium the issuer sets at issuance

## Escalate to Raw When

Open Calamos §10 pp.220-260 for the practitioner's taxonomy of issuer
motives and the structuring playbook (premium choice, call schedule,
coupon level). **Source:** Calamos (2003) §10 pp.220-260.

Open Koziol §2 pp.20-50 for the formal asymmetric-information /
strategic-investor analysis that derives convertible issuance as an
equilibrium response. **Source:** Koziol (2004) §2 pp.20-50.

Open Philips §1 pp.15-50 for the historical evolution of issuer motives
across market regimes (1980s LBO era, 1990s tech bubble, post-2000
defensive issuance). **Source:** Philips (1997) §1 pp.15-50.
