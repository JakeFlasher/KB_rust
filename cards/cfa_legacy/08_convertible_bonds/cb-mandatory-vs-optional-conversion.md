---
schema_version: "cacg.v0"
id: "cb-mandatory-vs-optional-conversion"
title: "Mandatory vs Optional Conversion"
reading_id: "08_convertible_bonds"
summary: "Mandatory vs Optional Conversion — placeholder summary                          "
tags: ["convertible-bonds", "mandatory-optional"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p053:0060"
    chunk_hash: "9ec10ac25ac4f579e25eaecbbce66f1ca31e67388046f9f92539cc87b2a4cbdd"
    page_range: [53, 54]
    quote: "The holder of a convertible bond has the option to end the bond’s existence prematurely by converting it into shares. This right is the optional conversion."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p055:0063"
    chunk_hash: "051810f8e5498b774b05fa095895799851c25abec0b47969cd516b65068ac8e4"
    page_range: [55, 56]
    quote: "A mandatory convertible bond always redeems into shares, never into cash."
    edge_type: "defines"
card_hash: "3ff3575092ea77b49ac970d8717484827943a10b48fa33ed15638b29a97913c3"
---
# Mandatory vs Optional Conversion

## Intuition

In a standard **optional convertible** the holder owns the conversion right
and exercises only when conversion is value-improving. In a **mandatory
convertible** (PEPS, ACES, DECS, ELKS family) the conversion is required at
maturity — only the conversion ratio is variable, dependent on the share
price at maturity. The difference flips which side of the contract owns the
embedded equity option, and therefore flips the bond-plus-call decomposition
sign.
**Source:** DeSpiegeleer et al. (2014) §4.1-§4.2 pp.110-130.

```
Maturity payoff per face F:

  optional CB                       mandatory CB (PEPS-style)
       ^                                      ^
       |     /                                |     /
   F---+----.                              F--+----.--------- (cap)
       |   /                                  |   /
       |  /                                   |  /
       | /                                    | /  variable Cr
       |/____ par at S=K_c                    |/____ floor at S=L
       0                                      0
       +--------> S                           +--------> S
```

## Definition

An **optional convertible** matures at face `F` if not converted; the holder
holds an embedded American or European call on `q · S` with strike `F`.
The convertible's value satisfies `V(S, T) = max(F, q · S(T))` at maturity
(plus accrued interest, ignored here for clarity). **Source:** DeSpiegeleer
et al. (2014) §2.3 pp.30-44.

A **mandatory convertible** of the PEPS/DECS family converts to a
**variable** number of shares at maturity, defined by a piecewise rule on
the share price `S(T)` against two thresholds `L < U`. **Source:**
DeSpiegeleer et al. (2014) §4.1-§4.2 pp.110-130; Calamos (2003) §9 pp.180-220.

```
Conversion ratio at maturity:

  q_low      if S(T) >= U  (high-strike threshold; holder gets fewer shares)
  F / S(T)   if L < S(T) < U   (par-preserving region)
  q_high     if S(T) <= L  (low-strike threshold; holder gets more shares)
```

The maturity payoff is therefore `min(q_low · S(T), max(F, q_high · S(T)))`,
which is equivalent to a long share position **plus** a short call at `U`
**plus** a long put at `L`. The holder is forced to take equity exposure
within `[L, U]`. **Source:** DeSpiegeleer et al. (2014) §4.1-§4.2
pp.110-130.

## Mathematical Reasoning

The embedded-option ownership flips between the two structures. **Source:**
DeSpiegeleer et al. (2014) §4.2 pp.115-130.

- **Optional CB**: holder is **long** an embedded call on `q · S`; the
  bond-plus-call decomposition has the call as a positive-value addend,
  with the issuer implicitly short the call (see the
  [payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning)).
- **Mandatory CB**: holder is **short** the upside above `U` and **long**
  exposure below `L`; the issuer is **long** the cap and **short** the put,
  yielding the decomposition shown below.
  **Source:** DeSpiegeleer et al. (2014) §4.2 pp.115-130.

      V_mandatory(S, t) = B_mandatory(t)
                          + q_high · S(t) · D_div(t, T)        (long stock)
                          - call(S, U, σ, r, δ, T-t)           (short cap)
                          + put(S, L, σ, r, δ, T-t)            (long floor)

The straight-bond leg `B_mandatory(t)` reflects the contractually fixed coupon
stream up to maturity (no `F` redemption — the bond does not pay face at
`T`). **Source:** DeSpiegeleer et al. (2014) §4.2 pp.115-130; Calamos
(2003) §9 pp.190-210.

Asymptotics differ from optional convertibles. **Source:** DeSpiegeleer
et al. (2014) §4.3 pp.130-145.

- `S → ∞`: optional CB delta `→ q` (capped at the conversion ratio);
  mandatory CB delta `→ q_low < q` (the upside is sold to the issuer).
  **Source:** DeSpiegeleer et al. (2014) §4.3 pp.130-140.
- `S → 0`: optional CB delta `→ 0` (call is OTM; bond-floor cushion);
  mandatory CB delta `→ q_high > q` (the holder is forced to take heavier
  equity exposure on the downside). The mandatory's "floor" is therefore
  shallower than the optional's bond floor in absolute terms. **Source:**
  Calamos (2003) §9 pp.200-220.
- `S = K_c` (balanced): optional CB exhibits maximum convexity; mandatory
  CB exhibits maximum **concavity** within `[L, U]` because of the
  short-call leg. **Source:** DeSpiegeleer et al. (2014) §4.3 pp.130-145.

The mandatory's coupon `c_mand` is **higher** than the optional's
`c_opt` for the same issuer — the holder must be compensated for selling
the upside. This is the practitioner intuition for why mandatories trade
in a different yield band from optionals. **Source:** Calamos (2003) §9
pp.180-200.

## See Also

- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — the holder-discretion exercise rule for optional CBs
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the optional-CB decomposition this contrasts with
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — the bond floor (mostly absent for mandatories)

## Escalate to Raw When

Open DeSpiegeleer §4.1-§4.3 pp.110-145 directly when pricing a mandatory or
matching its parameter triple `(q_low, q_high, L, U, c_mand)` against a
prospectus. **Source:** DeSpiegeleer et al. (2014) §4.1-§4.3 pp.110-145.

Open Calamos §9 pp.180-220 for the practitioner taxonomy of mandatory
families (PEPS, ACES, DECS, ELKS) and their issuer-side tax / accounting
motivations. **Source:** Calamos (2003) §9 pp.180-220.

Open Zubulake pp.50-90 for cross-jurisdictional mandatory variants
(European contingent capital instruments differ from US mandatories in
materials ways). **Source:** Zubulake §3 pp.50-90.
