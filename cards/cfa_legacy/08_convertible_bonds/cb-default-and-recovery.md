---
schema_version: "cacg.v0"
id: "cb-default-and-recovery"
title: "Default Models and Recovery Conventions"
reading_id: "08_convertible_bonds"
summary: "Default Models and Recovery Conventions — placeholder summary                   "
tags: ["convertible-bonds", "default-recovery"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p139:0166"
    chunk_hash: "0f3d74209af0aeb613305c0d18f7e9fd98ac824a3a042dac3d368fc1fc765814"
    page_range: [139, 139]
    quote: "The three recovery assumptions are as follows. (1) Recovery of face value. This measures the value to the investors as a fraction of face value."
    edge_type: "defines"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p139:0167"
    chunk_hash: "28db49bf84eb98c6707dd57c06eb45cc8aa6437614a6a902ecd643a4e4272834"
    page_range: [139, 140]
    quote: "(3) Recovery of treasury. Under this assumption, the corporate bond in default is replaced with a treasury bond with the same maturity but a reduced payment."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p070:0081"
    chunk_hash: "09f65bee6d64f05275774a9f67240e8e58ba4175a26edf97fdb94e0d6803b5ab"
    page_range: [70, 71]
    quote: "This is the recovery rate ."
    edge_type: "supports"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p089:0093"
    chunk_hash: "ba70bef746471afdf5de37d2a589ecadf2e2f39e4893074b7473e7e3adc02a73"
    page_range: [89, 90]
    quote: "It is critical to estimate credit risk, including default risk and the expected recovery rate in the event of default."
    edge_type: "supports"
card_hash: "c302a737465b05cb1ed4877ec2d7c8686e13c996e68647cdb17e58f84deb99cb"
---
# Default Models and Recovery Conventions

## Intuition

Convertible-bond pricing requires two complementary modeling
choices: how default events arise (the **default model**), and what
the bondholder receives at default (the **recovery convention**).
The two structural-vs-reduced-form modeling families differ in
whether default is triggered endogenously by the firm's asset
process (Merton/structural) or arises as an exogenous Poisson event
(reduced-form/intensity). The three recovery conventions differ in
whether the holder receives a fixed fraction of face, of pre-default
market value, or of an equivalent Treasury position. The choices
matter because they reshape both the convertible's **bond floor**
and the **double-signed gamma** dynamic in stress.
**Source:** Lando (2004) §6-§9 pp.100-200.

```
default-model + recovery-convention matrix:

                       face-value     market-value    Treasury-value
                       (R · F)        (R · V_pre-)    (R · D_rf · F)
   ----------------- |-------------|---------------|------------------
   Structural         |  rare       |  natural      |  rare
   (Merton)           |             |  (firm value  |
                      |             |   absorbed)   |
   Reduced-form       |  standard   |  occasional   |  classical CDS
   (intensity)        |  (CB pricing)|  (loss models)|  convention
```

## Definition

The **two default-model families**. **Source:** Lando (2004) §6-§7
pp.100-160.

- **Structural / Merton (1974) model**: default occurs when the
  firm's asset value `A(t)` falls below a debt-trigger barrier (e.g.
  the face value of debt at maturity, or a continuous default
  barrier). Default time `τ = inf{ t : A(t) ≤ B(t) }`. The model is
  **endogenous** — default arises from the firm's own dynamics.
  **Source:** Lando (2004) §6 pp.100-130.
- **Reduced-form / intensity-based model**: default is the first jump
  of an inhomogeneous Poisson process with intensity `h(t)`. The
  model is **exogenous** with respect to the firm's value process,
  although `h(t)` may itself be modeled as a function of `S(t)`,
  bond yield, or external factors. Default time `τ = first jump of
  Poisson(∫_0^t h(u) du)`. **Source:** Lando (2004) §3 pp.60-90.

The **three recovery conventions** differ in what the holder
receives at default. **Source:** Lando (2004) §5 pp.100-130.

- **Face-value recovery** (`RFV`): holder receives `R · F` at the
  default time `τ`. The most common convention in convertible-bond
  pricing trees because of its simplicity and direct compatibility
  with the credit-aware tree's default branch (see the
  [binomial-tree card](./cb-binomial-tree-valuation.md#definition)).
  **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.
- **Market-value recovery** (`RMV`): holder receives `R · V(τ⁻, τ)`,
  the recovery fraction of the bond's pre-default market value.
  Natural when default is endogenously linked to the firm's asset
  value (Merton-style). **Source:** Lando (2004) §5 pp.110-130.
- **Treasury-value recovery** (`RTV`): holder receives `R · D_rf(τ,
  T) · F`, i.e., the present value of an equivalent Treasury bond
  paying off at maturity. The classical CDS pricing convention.
  **Source:** Lando (2004) §5 pp.110-130.

## Mathematical Reasoning

Each convention produces a different **defaultable bond price**
formula. **Source:** Lando (2004) §5 pp.100-130.

```
Defaultable bond PV under the three recovery conventions:

  B^d_{RFV}(0, T) = Σ_k c · F · D_rf(0, t_k) · P^Q(τ > t_k)
                  + F · D_rf(0, T) · P^Q(τ > T)
                  + R · F · ∫_0^T D_rf(0, u) · h(u) · P^Q(τ > u) du

  B^d_{RMV}(0, T) ≈ B^d_{RFV} but with R replaced by R · (B^d / F)

  B^d_{RTV}(0, T) ≈ Σ_k c · F · D_rf(0, t_k) · P^Q(τ > t_k)
                  + F · D_rf(0, T) · P^Q(τ > T)
                  + R · D_rf(0, T) · F · P^Q(τ ≤ T)
```

The **structural Merton model** maps the firm's asset value process
`A(t)` (geometric Brownian motion) to the equity `S(t)` and the
defaultable-debt value via the Black-Scholes formula. **Source:**
Lando (2004) §6 pp.100-130.

```
Merton mapping:

  S(t) = call(A(t), F, σ_A, r, T-t)              (equity is a call on assets)
  B^d(t, T) = A(t) - S(t)                          (debt is residual)
  P^Q(τ ≤ T) = N(-d_2)                             (default = put exercise)
```

The Merton mapping has the **important consequence** that `P^Q(τ ≤
T)` is naturally a function of `S(t)` and `σ_S` — the equity-coupled
hazard model from the
[credit-spread card](./cb-credit-spread-machinery.md#mathematical-reasoning)
arises endogenously. **Source:** Lando (2004) §6 pp.100-130.

The **double-signed-gamma dynamic** is the practitioner's empirical
fingerprint of the equity-coupled-hazard regime: as `S → 0`, the
structural model's `P^Q(τ ≤ T)` rises sharply, the recovery PV under
any convention drops, and the convertible's bond floor falls below the
constant-credit prediction. **Source:** DeSpiegeleer et al. (2014) §3.6
pp.95-130; Calamos (2003) §7 pp.150-170.

The **recovery-rate empirical regularity** is that `R` is highest for
secured debt, lower for senior unsecured, lowest for subordinated
convertibles; practitioner-quoted convertible recovery rates cluster
at `R = 30%-40%` for senior unsecured issues, and the convention is to
use `R = 35%` as the default in pricing trees absent issuer-specific
data. **Source:** Calamos (2003) §7 pp.130-170.

Asymptotic behavior (cases below). **Source:** Lando (2004) §5-§6
pp.100-130; DeSpiegeleer et al. (2014) §3.6 pp.95-130.

- `R → 1` (full recovery): defaultable bond → riskless bond
  regardless of `h`. **Source:** Lando (2004) §5 pp.100-130.
- `R → 0` (zero recovery): defaultable bond → `D_rf · P^Q(τ > T) ·
  F`; the bond floor collapses to the riskless-discounted survival
  PV. **Source:** Lando (2004) §5 pp.100-130.
- `h → 0` (riskless): all three recovery conventions collapse to
  the same answer (the riskless-discounted face); the default
  branch of the credit-aware tree carries zero weight. **Source:**
  DeSpiegeleer et al. (2014) §3.6 pp.95-130.
- `S → 0` under structural Merton: `P^Q(τ ≤ T) → 1`; the
  defaultable bond → `R · F` (or recovery-equivalent); the
  convertible's bond floor collapses to the recovery floor.
  **Source:** Lando (2004) §6 pp.100-130.

## See Also

- [`cb-credit-spread-machinery.md`](cb-credit-spread-machinery.md) — the hazard rate `h(t)` consumed by reduced-form pricing
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — the survival/recovery split of `B(t)`
- [`cb-binomial-tree-valuation.md`](cb-binomial-tree-valuation.md) — the tree's default-branch recovery
- [`cb-credit-vs-equity-decomposition.md`](cb-credit-vs-equity-decomposition.md) — practitioner credit/equity ratios that shift across recovery models
- [`cb-china-distressed-workouts.md`](cb-china-distressed-workouts.md) — Chinese-market 6-rung workout taxonomy applying these recovery mechanics to post-2024 defaults
- [`cb-china-default-cohort-attribution.md`](cb-china-default-cohort-attribution.md) — Chinese-CB life-cycle credit-risk taxonomy + post-2014 default cohort attribution

## Escalate to Raw When

Open Lando §5-§9 pp.100-200 directly for the rigorous treatment of
recovery conventions, structural-model derivations, and the
formal proofs that link `P^Q(τ ≤ T)` to observable equity volatility
under Merton. **Source:** Lando (2004) §5-§9 pp.100-200.

Open DeSpiegeleer §3.6 pp.95-130 for the practitioner-quoted
convertible-bond default-and-recovery conventions and the implied
double-signed-gamma fingerprint. **Source:** DeSpiegeleer et al.
(2014) §3.6 pp.95-130.

Open Calamos §7 pp.130-170 for the practitioner's recovery-rate
ranges across convertible seniority classes and the impact on
recovery-floor estimates. **Source:** Calamos (2003) §7 pp.130-170.
