---
schema_version: "cacg.v0"
id: "rm-credit-var-portfolio"
title: "Portfolio Credit-VaR — McNeil Ch.11 §11.1-§11.3"
reading_id: "11_risk_management"
summary: "Portfolio credit-VaR is the high quantile of L_portfolio = Σ_i EAD_i · LGD_i · 1_{default_i} computed under threshold models (Merton-style with correlated latent variables) or Bernoulli/Poisson mixture models (CreditRisk+) that capture default correlation explicitly."
tags: ["risk-management", "credit-var"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p447:0646"
    chunk_hash: "40db207ff25ba40ee124e5c6588ff231e2039c662ef16214a66aa27daa60b031"
    page_range: [447, 448]
    quote: "Their defining attribute is the idea that default occurs for a company i over the period [0,T ] if some critical rv Xi lies below some deterministic threshold di."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p465:0674"
    chunk_hash: "c6c947401cbfae5d8e0bbe8fffd4f36c480e0ce372b10a733570d9c9ffb8f1c8"
    page_range: [465, 465]
    quote: "We now give a formal definition of a Poisson mixture model for counting variables that parallels the definition of a Bernoulli mixture model in Section 11.2.1."
    edge_type: "supports"
card_hash: "c9f7e3dc2ed4c8129063e5bef34d584232decf65ba4003c7dea7ba73d07fb0d3"
---
# Portfolio Credit-VaR — McNeil Ch.11 §11.1-§11.3

## Intuition

**Portfolio credit-VaR** is the quantile of the aggregate credit loss distribution `L_portfolio = Σ_i EAD_i · LGD_i · 1_{default_i}` (see `[[rm-credit-risk-metrics-restatement]]` for the per-counterparty components). Unlike market-risk VaR where the loss is a continuous function of factor moves, credit-risk VaR is dominated by **discrete default events**, so the loss distribution is a mixture: most of the mass concentrates near zero (no defaults in the period) with a long tail to the right (multiple defaults coinciding). The right tail is where capital sits. **Source:** McNeil et al. (2015) Ch.11 pp.425-432.

The structural challenge is **default correlation**: independent defaults give a diversification-friendly aggregate (variance shrinks with portfolio size), while correlated defaults give a non-diversifying tail. The two canonical model families that capture default correlation are: **threshold models** (each obligor defaults when a latent variable crosses a threshold; correlated latent variables generate correlated defaults — Merton-style) and **mixture models** (default probabilities are themselves random and conditionally independent given a common factor — CreditRisk+-style Poisson mixtures). Both families produce the same first-moment expected loss but very different tail distributions. **Source:** McNeil et al. (2015) Ch.11 pp.432-456.

The **Basel IRB (Internal Ratings-Based) asymptotic formula** is the regulatory closed form: under a one-factor Gaussian threshold model and the assumption of an infinitely fine-grained portfolio (no single-name concentration), portfolio credit-VaR at level `α` admits an explicit formula in terms of per-obligor PD, the asset-correlation `ρ` to the common factor, and the regulatory confidence level. The formula is asymptotic — finite-portfolio corrections (concentration penalties, granularity adjustments) are added on top in the full IRB rule set. McNeil treats the asymptotic derivation; the regulatory implementation details belong to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.11 pp.456-475.

```
   Portfolio credit-VaR pipeline
   ──────────────────────────────

   per-counterparty inputs:
     {(PD_i, LGD_i, EAD_i)}_i      <- see [[rm-credit-risk-metrics-restatement]]
          |
          v
   default-correlation model:
     +---------------------+      +---------------------+
     | Threshold model     |      | Mixture model       |
     | obligor i defaults  | OR   | PD_i = random       |
     | when latent X_i < c |      | conditionally       |
     | correlated X drives |      | independent given   |
     | correlated defaults |      | common factor Z     |
     +----------+----------+      +----------+----------+
                |                            |
                +-------------+--------------+
                              |
                              v
                  +-----------------------+
                  | Aggregate loss        |
                  | L_portfolio = Σ_i L_i |
                  +-----------+-----------+
                              |
                              v
                  +-----------------------+
                  | Portfolio credit-VaR  |
                  | VaR^credit_α          |
                  | = q_α(L_port)         |
                  +-----------+-----------+

   Basel IRB asymptotic closed form:
     infinitely fine-grained portfolio + one-factor Gaussian threshold model
     → VaR^credit_α admits explicit formula in PD_i, asset-correlation ρ, α
     (regulatory text owns the specific α and ρ schedules)
```

## Definition

Let `{(PD_i, LGD_i, EAD_i)}` be the per-counterparty inputs for the portfolio's obligors. The **single-counterparty loss** is `L_i = EAD_i · LGD_i · 1_{default_i}` and the **portfolio loss** is `L_portfolio = Σ_i L_i`. The **portfolio credit-VaR** at level `α` is the α-quantile of the portfolio loss distribution: **Source:** McNeil et al. (2015) Ch.11 pp.425-432.

```
VaR^credit_α  =  inf { l ∈ R : P(L_portfolio ≤ l) ≥ α }
              =  q_α(L_portfolio)
```

In a **threshold model**, each obligor has a latent variable `X_i` and a default threshold `c_i`; default occurs when `X_i ≤ c_i`. The marginal `PD_i = P(X_i ≤ c_i)`. Correlated defaults arise from correlated `{X_i}`: typically the latent variables follow a joint normal (or t) with a correlation structure (often factor-model decomposed into a common factor `Z` and idiosyncratic `ε_i`). **Source:** McNeil et al. (2015) Ch.11 pp.432-444.

```
threshold model:
    X_i  =  √ρ · Z  +  √(1 − ρ) · ε_i        (one-factor Gaussian)
    1_{default_i}  =  1{ X_i ≤ Φ^{-1}(PD_i) }
    Z, ε_i  ~  N(0, 1)  i.i.d.    ρ ∈ [0, 1] is the asset-correlation
```

In a **Bernoulli mixture model**, conditional on a common factor `Z`, defaults are independent with conditional probabilities `P(default_i | Z) = p_i(Z)`; the unconditional default correlation arises because all `p_i(Z)` are driven by the same `Z`. The CreditRisk+ specialisation uses Poisson conditional defaults and gamma-distributed `Z`. **Source:** McNeil et al. (2015) Ch.11 pp.444-456.

The **Basel IRB asymptotic formula** under the one-factor Gaussian threshold model with infinitely fine-grained portfolio gives the closed-form expression for the conditional default probability given a stressed common-factor realisation: **Source:** McNeil et al. (2015) Ch.11 pp.456-475.

```
p_i(z)  =  Φ ( ( Φ^{-1}(PD_i)  −  √ρ · z ) / √(1 − ρ) )

  ↳ stress z to the (1 − α)-quantile of Z
  ↳ conditional expected loss given z → portfolio credit-VaR per obligor
```

The asymptotic formula assumes infinite-fineness; finite-portfolio corrections (granularity adjustment) are added in the full IRB rule. **Source:** McNeil et al. (2015) Ch.11 pp.456-475.

## Mathematical Reasoning

The structural reason portfolio credit-VaR requires dedicated machinery — and is not just "VaR applied to credit losses" — is the **discreteness** of default events. Market-risk losses are continuous functions of factor moves, so the loss distribution is continuous and standard quantile machinery applies. Credit losses are sums of binary events, so the loss distribution is a finite mixture with mass concentrated near zero and discrete jumps where multiple defaults coincide. The portfolio-VaR quantile sits in the upper tail where these multi-default events live, and the tail's structure depends critically on the default-correlation specification. **Source:** McNeil et al. (2015) Ch.11 pp.425-432.

The **threshold-model** decomposition `X_i = √ρ · Z + √(1−ρ) · ε_i` is the work-horse single-factor Gaussian construction. Under this model: (a) the marginal `PD_i = Φ(c_i)` is recovered when the threshold `c_i = Φ^{-1}(PD_i)`; (b) the pairwise correlation of default indicators is non-zero and increases in `ρ`; (c) conditional on `Z = z`, defaults are independent with probability `p_i(z) = Φ((Φ^{-1}(PD_i) − √ρ · z) / √(1−ρ))`. The conditional independence given `Z` is the structural property that enables the large-portfolio asymptotic — the law of large numbers applies to the conditional defaults, and the portfolio loss conditional on `Z = z` concentrates around its expectation. **Source:** McNeil et al. (2015) Ch.11 pp.432-456.

The **infinite-fineness limit** is the key step toward the IRB closed form. As portfolio size grows with all `EAD_i` becoming small relative to total exposure, the law of large numbers smooths the conditional portfolio loss into its conditional expectation. The portfolio's unconditional loss distribution is then just the distribution of the conditional expectation as `Z` ranges over its distribution. The α-quantile is achieved by stressing `Z` to a level that makes the conditional probabilities large enough to deliver the target tail loss. The IRB formula is the per-obligor capital contribution under this construction. **Source:** McNeil et al. (2015) Ch.11 pp.456-475.

The **mixture-model alternative** captures default correlation through random default probabilities rather than correlated latent variables. The CreditRisk+ specialisation chooses Poisson-distributed default counts conditional on a gamma-distributed `Z` (the gamma mixing parameter); this gives a tractable closed-form unconditional default-count distribution (negative binomial) and is calibration-friendly for portfolios where rating-migration data is unavailable. The trade-off: mixture models lose the latent-variable interpretation that threshold models offer (no "asset value crosses default boundary" story) and require a different calibration pipeline. **Source:** McNeil et al. (2015) Ch.11 pp.444-456.

The **finite-portfolio concentration adjustment** matters when the IRB asymptotic formula is applied to real portfolios with single-name exposure concentration. The asymptotic formula assumes all `EAD_i` are negligibly small relative to total exposure; finite portfolios with large single names retain idiosyncratic risk that the asymptotic limit assumes away. Regulatory and internal-model treatments add a **granularity adjustment** that increases the asymptotic VaR by a portfolio-specific concentration penalty. McNeil sketches the adjustment; full regulatory implementation depth belongs to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.11 pp.456-475.

A subtle structural point: **portfolio credit-VaR is more sensitive to correlation assumptions than to per-obligor PD assumptions**. Increasing all `PD_i` proportionally raises expected loss but does not necessarily raise the tail VaR by the same factor. Increasing the asset-correlation `ρ` can multiply the tail VaR by a much larger factor because correlation drives the multi-default-coincidence tail. This is the structural reason why correlation calibration (asset-correlation, rating-migration correlation, sector concentration) absorbs more analyst attention than per-obligor PD calibration in serious portfolio-credit-risk practice. **Source:** McNeil et al. (2015) Ch.11 pp.432-456.

## See Also

Cross-vertical (Fixed Income — single-counterparty derivation territory):

- [fi-default-models-and-recovery](../06_fixed_income_and_credit/fi-default-models-and-recovery.md) — single-counterparty Merton-Lando structural and hazard-rate intensity models that feed PD_i.

Within v11 Risk Management:

- [rm-credit-risk-metrics-restatement](./rm-credit-risk-metrics-restatement.md) — Batch-3 sibling card defining PD / LGD / EAD inputs.
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 card on VaR / ES definitions reused for credit VaR.
- [rm-portfolio-xva-aggregation](./rm-portfolio-xva-aggregation.md) — Batch-3 sibling card on portfolio-XVA aggregation (CVA / DVA / FVA layer).

## Escalate to Raw When

The conceptual depth in this card stops at the threshold-model / mixture-model split + the IRB asymptotic closed form + concentration-adjustment overview. When the operator needs full rating-migration matrix machinery (Markov-chain estimation, transition-probability calibration, time-inhomogeneous chains), full Basel IRB implementation depth (risk-weighted-asset computations, slot-rating concentrations, downturn LGD specifications, EAD conversion factors), or the formal large-portfolio asymptotic proofs, open McNeil Ch.11 §11.4-§11.5 pp.456-475 directly. Regulatory implementation details belong to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.11 pp.425-475.
