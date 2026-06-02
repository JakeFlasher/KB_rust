---
Use when: deriving the variance-covariance / parametric VaR estimator under the elliptical-distribution assumption with the quantile-of-normal framing
Primary raw source: 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.340-342
Supporting sources:
  - 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67
Repo touchpoints:
  - rm-var-and-es-taxonomy.md
  - rm-value-at-risk-notes.md
  - rm-historical-simulation-var.md
  - rm-monte-carlo-var.md
Out of scope: variance-covariance matrix estimation machinery (defers to future-01 quantitative econometrics); non-elliptical extensions
CFA Relevance: extension
Source Stance: primary-mcneil
deliverable-ready: true
---

# Parametric (Variance-Covariance) VaR — McNeil Ch.9 §9.2.2

## Intuition

**Parametric VaR** is the simplest VaR estimator under the strongest distributional assumption: assume the portfolio loss `L` follows a known distribution family `F(·; θ)` with parameters `θ` you have estimated; then `VaR_α = F^{-1}(α; θ)` reads off the closed-form quantile. For a multivariate-normal factor model with linearised (delta-only) portfolio exposure, the loss is itself normal and `VaR_α = μ_L + σ_L · Φ^{-1}(α)`, where `μ_L`, `σ_L` are loss mean and standard deviation and `Φ^{-1}` is the standard-normal inverse CDF. This is sometimes called the **variance-covariance method** because it inherits its tractability from the joint normal factor distribution. **Source:** McNeil et al. (2015) Ch.9 pp.340-342.

The parametric route trades **simplicity for distributional risk**. When the assumed family is correct, the estimator is variance-efficient (best possible mean-squared-error for a given sample size). When the assumed family is wrong — most commonly: real loss distributions are fatter-tailed than normal — the parametric VaR systematically **understates** the high-α quantile because the normal tail is too thin. Practice extensions replace the normal assumption with **Student-t** (heavier tails parameterised by degrees-of-freedom `ν`) or **mixture-of-normals** (regime-switching tail thickness). McNeil treats the variance-covariance method as the baseline and develops elliptical extensions in Ch.6. **Source:** McNeil et al. (2015) Ch.9 pp.340-342 + Ch.2 pp.64-67.

The structural assumption that enables parametric VaR is **closed-ness under linear combination**: if risk factors `X ~ N_d(μ, Σ)` are jointly normal and the portfolio P&L is linear in `X` (delta-only), then `L = −a^T X + const` is univariate normal with `μ_L = −a^T μ + const`, `σ_L² = a^T Σ a`. The delta-only assumption is the load-bearing one — adding a quadratic gamma term breaks the closure, and the resulting `L` is a sum of correlated chi-squared-like terms with no closed-form quantile. The delta-gamma extension uses Cornish-Fisher expansion or Monte Carlo on the quadratic form (see `[[rm-monte-carlo-var]]`). **Source:** McNeil et al. (2015) Ch.9 pp.340-342.

```
   Parametric-VaR pipeline
   ───────────────────────

   +-----------------+      +-----------------+
   | Joint factor    |      | Portfolio       |
   | distribution    |      | sensitivities   |
   | X ~ N_d(μ, Σ)   |      | a = ∇V(X_0)     |
   | (or elliptical) |      | (delta vector)  |
   +--------+--------+      +--------+--------+
            |                        |
            +-----------+------------+
                        |
                        v
              +-------------------+
              | Loss distribution |
              | L = −a^T(X − X_0) |
              | ~ N(μ_L, σ_L²)    |  (closed under linear combo)
              +---------+---------+
                        |
                        v
              +-------------------+
              | Closed-form VaR   |
              | VaR_α = μ_L +     |
              | σ_L · Φ^{-1}(α)   |
              +---------+---------+
                        |
                        v
              caveats:  fat tails (real L heavier than normal);
                        gamma curvature (delta-only assumption);
                        non-stationary Σ (calibration window matters)
```

## Definition

Let `X ∈ R^d` be the risk-factor vector and `V(X)` the portfolio value. Assume `(a)` the factor distribution is multivariate normal `X_1 − X_0 = ΔX ~ N_d(μ_X, Σ_X)`, and `(b)` the portfolio value is approximately linear in `X`: `V(X_1) − V(X_0) ≈ a^T ΔX` with sensitivity vector `a = ∇V(X_0)`. Then the **linearised loss** is: **Source:** McNeil et al. (2015) Ch.9 pp.340-341.

```
L  =  −(V(X_1) − V(X_0))  ≈  −a^T ΔX

⇒  L  ~  N(μ_L, σ_L²)       where μ_L = −a^T μ_X  and  σ_L²  =  a^T Σ_X a
```

The **parametric VaR** is the inverse-CDF of the resulting normal loss distribution: **Source:** McNeil et al. (2015) Ch.9 pp.341-342.

```
VaR_α  =  F_L^{-1}(α)  =  μ_L  +  σ_L · Φ^{-1}(α)
```

For an elliptical generalisation (e.g., `ΔX ~ t_d(ν, μ, Σ)` for Student-t with `ν` degrees of freedom), the same linear-combination closure holds: `L` is univariate t with the same `ν`, and the VaR formula becomes `VaR_α = μ_L + σ_L · t_ν^{-1}(α)`, where `t_ν^{-1}` is the univariate Student-t inverse CDF. **Source:** McNeil et al. (2015) Ch.9 pp.341-342 + Ch.6 pp.211-218.

The **parametric ES** under the same Gaussian assumption admits the closed form: **Source:** McNeil et al. (2015) Ch.9 pp.341-342.

```
ES_α  =  μ_L  +  σ_L · ( φ(Φ^{-1}(α)) / (1 − α) )
```

where `φ` is the standard normal density. **Source:** McNeil et al. (2015) Ch.9 pp.341-342.

## Mathematical Reasoning

The variance-covariance method's mathematical core is the **closure of the normal family under linear transformations**: if `ΔX ~ N_d(μ, Σ)` and `L = c + a^T ΔX` for constant `c`, sensitivity `a`, then `L ~ N(c + a^T μ, a^T Σ a)`. This is a basic multivariate-normal fact and is the reason the variance-covariance method has a closed-form quantile at all. Drop the normality assumption and the closed form generally vanishes; the elliptical generalisation (Student-t, generalised hyperbolic) preserves closure under linear transformations but with non-Gaussian quantile factors. **Source:** McNeil et al. (2015) Ch.9 pp.340-341 + Ch.6 pp.211-218.

The **delta-only linearisation** ignores gamma, vega, and higher-order Greeks; this is the dominant error source for option books. Adding the second-order term gives `L ≈ −a^T ΔX − 1/2 · ΔX^T H ΔX` where `H` is the portfolio Hessian. Even under jointly-normal `ΔX`, the quadratic form `ΔX^T H ΔX` is not normal — it is a weighted sum of chi-squared distributions whose quantile is intractable in closed form. Three practical responses: (a) **Cornish-Fisher expansion** approximates the quadratic-form quantile via cumulants; (b) **delta-gamma Monte Carlo** simulates the quadratic form numerically; (c) accept the linearisation error if option exposure is small. McNeil treats Cornish-Fisher and delta-gamma-MC in Ch.9 §9.2.3-§9.2.5. **Source:** McNeil et al. (2015) Ch.9 pp.341-345.

The **covariance matrix estimation** is the second largest error source. The variance-covariance method assumes a known `Σ_X`, but in practice `Σ̂_X` is estimated from a finite sample of past factor returns. For high-dimensional portfolios (`d` large relative to sample size `T`), the sample covariance matrix is ill-conditioned: small eigenvalues become noisy, large eigenvalues stay accurate, and the inverted covariance matrix (which enters factor-weighted VaR formulas) amplifies the noise. Shrinkage estimators (Ledoit-Wolf), factor-model decomposition (principal-components), and exponentially-weighted moving averages (EWMA, RiskMetrics-style) are practical responses. McNeil flags the issue and defers depth to future-01 quantitative econometrics. **Source:** McNeil et al. (2015) Ch.9 pp.342 + future-01 deferral.

The **fat-tail correction via Student-t** is the most common parametric refinement. The univariate `t_ν^{-1}(α)` quantile factor exceeds `Φ^{-1}(α)` by an amount that diverges as `ν → 2` (the minimum `ν` for which the variance exists). For low degrees of freedom typical of daily financial returns, the Student-t quantile factor at high-`α` levels is strictly larger than the Gaussian quantile factor, so parametric `VaR_α` under Student-t is correspondingly higher. The trade-off: estimating `ν` adds parameter risk and the closure-under-linear-combination property requires the elliptical Student-t (not the marginal-only Student-t), which is more demanding to estimate. **Source:** McNeil et al. (2015) Ch.9 pp.341-342 + Ch.6 pp.211-218.

The parametric route is the **fastest** of the three VaR estimators (closed-form evaluation, no simulation) and is the **most efficient** when its assumptions hold. It is **least robust** to distributional mis-specification — fat tails and option non-linearity break it. Risk-management practice typically reports parametric VaR alongside historical-simulation and Monte Carlo VaR; persistent disagreement among the three flags either a calibration problem or a structural assumption violation (see `[[rm-historical-simulation-var]]` and `[[rm-monte-carlo-var]]`). **Source:** McNeil et al. (2015) Ch.9 pp.340-347.

## See Also

- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 card with VaR / ES definitions and the coherence contrast.
- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — Batch-1 L1-notes framing of the 3-route VaR estimator taxonomy.
- [rm-historical-simulation-var](./rm-historical-simulation-var.md) — Batch-2 sibling card on the assumption-free empirical-quantile route.
- [rm-monte-carlo-var](./rm-monte-carlo-var.md) — Batch-2 sibling card on the model-implied simulation route, including the delta-gamma MC hybrid for non-linear payoffs.

## Escalate to Raw When

The conceptual depth in this card stops at the variance-covariance closed form, the Student-t generalisation, and the delta-only / covariance-estimation caveats. When the operator needs the full multivariate distribution theory (elliptical and generalised hyperbolic families, characteristic-function methods, copula construction), Cornish-Fisher higher-order moment corrections, shrinkage / factor-PCA / EWMA covariance-matrix estimation, or the regulatory-text quantile-floor / liquidity-horizon mappings (specific Basel III FRTB internal-model approach rules belong to authorized regulatory text), open McNeil Ch.6 pp.196-250 + Ch.9 §9.2 pp.340-350 directly. **Source:** McNeil et al. (2015) Ch.6 + Ch.9 pp.196-350.
