---
schema_version: "cacg.v0"
id: "cb-call-and-put-protection"
title: "Call and Put Protection"
reading_id: "08_convertible_bonds"
summary: "Most issued convertibles include early-redemption rights: an issuer call lets the issuer redeem at price K after a non-call period (hard or soft call with parity-based trigger), and a holder put lets the holder put the bond back at a stated put price; both are embedded options whose payoffs modify the holder's bond-plus-call decomposition."
tags: ["convertible-bonds", "call-put"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p068:0078"
    chunk_hash: "267a3eae776fe08cacefcf8b36127e54f1618d7df650bcd482a4144396d57c75"
    page_range: [68, 69]
    quote: "The real expiry date of the convertible was 4 years later but the issuer decided to redeem its convertible prematurely."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p142:0158"
    chunk_hash: "62e379869151c4c59e2c9068e3affb289a56beb4d28faeb6d622ad033e3f1619"
    page_range: [142, 143]
    quote: "One way to think about a callable convertible is to consider when and why it would be called."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p650:0966"
    chunk_hash: "e972bd11f1061f9542bb0de121768a78e178cb4a8482c4dec60fa7c3a2225f3b"
    page_range: [650, 651]
    quote: "The bonds are almost always callable (i.e., the issuer has the right to buy them back at certain times at a predetermined prices)."
    edge_type: "supports"
card_hash: "bf6c140922644a02ffed078016cc8c402826ba0a0f759d5941c3f72248cc70d7"
---
# Call and Put Protection

## Intuition

Most issued convertibles include early-redemption rights for one or both
sides of the contract. An **issuer call** is the issuer's right to redeem
the bond at a stated call price `K` after a non-call period; a **holder put**
is the holder's right to put the bond back to the issuer at a stated put
price after a put-eligible date. Both are embedded options whose payoffs
modify the holder's bond-plus-call decomposition.
**Source:** DeSpiegeleer et al. (2014) §2.5 pp.50-65.

```
                  CB price V(t)
                       ^
                       |
                       |        without provisions
                       |       /
                       |      /
                       |     /
   issuer call cap  ---|----.------ K (call price)
                       |   /
                       |  /
                       | /
                       |/_________________ holder put floor
  put price            |
                       +----------------------> share price S
```

## Definition

The **issuer call** is the issuer's right (not obligation) to repurchase the
bond at price `K` (typically `K = 100% face`, sometimes accreted) on or after
a stated **call protection** date. Two sub-categories appear in practice:
**Source:** DeSpiegeleer et al. (2014) §2.5 pp.55-65.

- **Hard call**: unconditional issuer right after the non-call period.
  **Source:** DeSpiegeleer et al. (2014) §2.5 pp.55-58.
- **Soft call**: issuer right is gated on a parity-based trigger, e.g. the
  share price has closed above `α · K_c` for `M` of the last `N` trading
  days (typical practitioner triple: `α` modestly above par, `M` near `N`).
  The trigger is designed to let the issuer call only when the bond is
  in-the-money enough that rational holders would convert rather than
  accept the cash redemption. **Source:** DeSpiegeleer et al. (2014) §2.5
  pp.58-65; Calamos (2003) §6 pp.95-110.

The **holder put** is the holder's right to redeem the bond at price `P_put`
(typically `100% face`) on or after a stated **put-eligible** date `T_put`.
The put protects the holder from coupon shortfalls and from credit-spread
widening. **Source:** DeSpiegeleer et al. (2014) §2.5 pp.65-70.

The composite **call schedule** records the contractual `(t, K(t))` pairs;
similarly the **put schedule** records `(t, P_put(t))` pairs. Soft-call
triggers are recorded with the parameter triple `(α, M, N)` of the
parity-trigger rule. **Source:** DeSpiegeleer et al. (2014) §2.5 pp.55-70.

## Mathematical Reasoning

The bond-plus-call decomposition (see the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning))
gains **two additional** option terms when call and put provisions are
active. **Source:** Hull (recent ed.) §27.4 pp.650-653.

    V(S, t) ≈ B(t) + q · c_holder(S, K_c, σ, r, δ, T-t)
                  - N_call · c_issuer(V_continue, K, σ_V, r, δ, T-t)
                  + N_put  · p_holder(V_continue, P_put, σ_V, r, δ, T_put-t)

The issuer's call is a **short call** held by the bondholder against the
continuation value `V_continue` (struck at `K`); the holder's put is a
**long put** on `V_continue` (struck at `P_put`). The decomposition is
schematic; in practice the joint exercise boundary is solved by a
credit-aware tree or PDE since the embedded options are written on the
convertible's own continuation value, which itself depends on every other
embedded option. **Source:** Hull (recent ed.) §27.4 pp.650-653;
DeSpiegeleer et al. (2014) §3.5 pp.78-95.

Soft-call mechanics introduce a **path-dependent** trigger: the issuer's
call is alive only after `M` of the last `N` daily closing prices satisfy
`S(τ) ≥ α · K_c`. Pricing therefore requires Monte Carlo or a
high-dimensional tree that tracks the trailing window; closed-form
approximations exist for the instantaneous-trigger limit (`M` and `N`
collapse to a single observation). **Source:** DeSpiegeleer et al. (2014)
§2.5 pp.60-65.

Holder optimal-conversion behavior shifts under an issuer call: when a soft-
call trigger is approaching, the rational holder may **forced-convert** to
avoid being called at par when parity exceeds par — the so-called
"screw-clause" effect. The continuation value comparison from the
[conversion-mechanics card](./cb-conversion-feature-mechanics.md#mathematical-reasoning)
remains the foundation, but issuer-call timing optionality is layered onto
it. **Source:** Calamos (2003) §6 pp.95-130.

Asymptotics (cases below). **Source:** DeSpiegeleer et al. (2014)
§2.5 pp.50-78; Hull (recent ed.) §27.4 pp.650-653.

- `S → ∞` with active soft call: rational holder converts before the
  issuer's call is exercised; the convertible's effective delta `→ q` and
  the bond inherits an equity-like profile. **Source:** DeSpiegeleer et al.
  (2014) §3.5 pp.78-90.
- `S → 0` with put protection: the holder exercises the put on `T_put` if
  `V_continue(T_put) < P_put`; the put **caps the downside** at
  `P_put · D_rf(t, T_put)` (riskless-discounted), with the residual cushion
  shrinking as `T_put` approaches. **Source:** DeSpiegeleer et al. (2014)
  §2.5 pp.65-70.

### Holder Put Payoff Diagram

The holder's put protection at strike `K_p` plots as a long-put payoff:
when `S(t)` falls deeply or credit stress lowers `V(t)`, the put
bounds the holder's downside at `K_p` regardless of the continuation
value. **Source:** DeSpiegeleer et al. (2014) §2.5 pp.50-65;
Calamos (2003) §5 pp.95-130.

```
<!-- primitive: put-payoff source: _diagram_primitives.md -->
payoff
   ^
   | \
   |  \
   |   \
   |    \
   |-----+----------------> S
         K
   intrinsic = max(K - S, 0)
```

## See Also

- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — base conversion mechanics
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — base decomposition
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — `B(t)` underlying floor

## Escalate to Raw When

Open DeSpiegeleer §2.5 pp.50-78 directly when soft-call trigger windows or
holder-put schedules need exact pricing. **Source:** DeSpiegeleer et al.
(2014) §2.5 pp.50-78.

Open Hull §27.4 pp.650-653 for the credit-aware tree machinery that prices
joint issuer-call and holder-put boundaries. **Source:** Hull (recent ed.)
§27.4 pp.650-653.

Open Calamos §6 pp.95-130 for the practitioner's playbook around screw-
clauses and forced conversions in active markets. **Source:** Calamos
(2003) §6 pp.95-130.
