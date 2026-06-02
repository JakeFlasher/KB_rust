---
schema_version: "cacg.v0"
id: "be-stylized-facts-financial-markets"
title: "Stylized Facts of Financial Markets"
reading_id: "10_behavioral_finance"
summary: "The empirical catalog every market model must match: returns are nearly uncorrelated (random-walk-like) yet have fat power-law tails (tail exponent near 3), volatility clusters with long memory, and order-flow signs are strongly long-range autocorrelated."
tags: ["behavioral-finance", "stylized-facts", "fat-tails", "volatility-clustering", "long-memory"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p415:0610"
    chunk_hash: "846e61d7a7339089bac7322678fe8c8480cf203e55732b441f1056cb49d8c2e4"
    page_range: [416, 416]
    quote: "unconditional distribution of returns has fat tails, which decay as a power law for large arguments and are much heavier than the corresponding tails of the Gaussian distribution"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p416:0611"
    chunk_hash: "9026c24e4d35aa0ef4e32e054e7b10aefd98084da407fddf6ad1ec61ef0c853a"
    page_range: [416, 416]
    quote: "is consistently found to be around 3 for a wide variety of different markets, which suggests some kind of universality in the mechanism"
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p417:0612"
    chunk_hash: "38525959b7330163c062c0aa380a9145954fd0d79d6114c50400efeee8ac68a0"
    page_range: [417, 417]
    quote: "The dynamics of financial markets is in fact highly intermittent, with periods of intense activity intertwined with periods of relative calm."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p417:0613"
    chunk_hash: "d0984629dcf1671aac8a508f11e70b8862d2e51747cc6045ef82a71f0da08f8f"
    page_range: [418, 418]
    quote: "Another striking stylized fact of financial markets is the persistence in the sign of the order flow."
    edge_type: "supports"
card_hash: "2cc4c81ece25333ade7d42a503553be3c3d8353314fb47cf14c16435f1968b89"
---
# Stylized Facts of Financial Markets

## Intuition

Across stocks, currencies, commodities, futures, and even Bitcoin, financial price series display a small set of remarkably universal statistical regularities — the **stylized facts** — that any credible market model is expected to reproduce. The central tension among them is that returns are at once nearly UNPREDICTABLE and decidedly NON-Gaussian: price changes are essentially uncorrelated over a wide range of frequencies (a near random walk), yet their distribution has heavy power-law tails, their magnitude clusters in time, and the signs of the underlying order flow are strongly persistent. These are not "anomalies" to be explained away but the basic texture of markets. **Source:** Bouchaud (2018) §2 pp.397-400.

The facts matter because they discipline theory. The mainstream rational paradigm never genuinely explained them, whereas agent-based and heterogeneous-agent models aim precisely to generate them as emergent outcomes of interacting heterogeneous traders. The fat tails are universal (tail exponent near 3) in a way that suggests fundamental news is largely irrelevant to the size of large jumps; the persistence of order-flow signs coexists with near-random-walk prices in an apparent "efficiency paradox." **Source:** Bouchaud (2018) §2 pp.397-400.

A unifying lesson is that long-term volatility is almost entirely determined by the short-term, high-frequency price-formation process, and that a large share of short-to-medium-term price variance appears to be self-referential (self-excited) rather than driven by fundamentals. The stylized facts thus point toward endogenous, feedback-driven dynamics. **Source:** Bouchaud (2018) §2 pp.400-400.

## Definition

**Heavy (fat) tails** is the empirical fact that the unconditional return distribution decays as a power law `f(r) ~ |r|^{-1-mu}` for large `r`, far heavier than Gaussian, with tail exponent `mu` consistently around 3 across markets. **Source:** Bouchaud (2018) §2.5 pp.398-398.

**Volatility clustering** is the intermittency of market dynamics: returns factor as `r_t = sigma_t xi_t` with `xi_t` i.i.d. unit-variance and `sigma_t` a positive, slowly varying (long-memory) volatility process, so large moves cluster with large and calm with calm. **Source:** Bouchaud (2018) §2.6 pp.399-399.

**Long memory in the order flow** is the slow, power-law decay `C(l) = Cov[eps_t, eps_{t+l}] ~ l^{-gamma}` (with `gamma < 1`) of the autocorrelation of trade signs `eps_t in {+1, -1}`, indicating strong persistence in buying/selling direction. **Source:** Bouchaud (2018) §2.8 pp.400-400.

**Near-zero return autocorrelation** is the flatness of the volatility signature plot: returns are covariance-stationary with essentially no linear predictability, so prices behave like a random walk over a broad range of time scales. **Source:** Bouchaud (2018) §2.2 pp.396-397.

## Mathematical Reasoning

Bachelier's first law states that, with zero-mean price changes, the price variogram `V(tau) := E[(p_{t+tau} - p_t)^2]` grows linearly, `V(tau) = D tau`. Defining the volatility at scale `tau` as `sigma^2(tau) := V(tau)/(p_0^2 tau) = sigma_0^2[1 + 2 sum_{u=1}^{tau}(1 - u/tau)C_r(u)]`, a pure random walk (`C_r(u) = delta_{u,0}`) gives a flat signature plot; positive correlations (trends) make `sigma(tau)` rise with `tau`, negative correlations (mean reversion) make it fall. Empirically the signature plots of liquid assets are nearly flat — returns are close to uncorrelated. **Source:** Bouchaud (2018) §2.1-2.2 pp.396-397.

For the tails, the short-time return density is well fit by a Student-t with `f(r) ~ |r|^{-1-mu}`, `mu ~ 3`. Crucially this is an UNCONDITIONAL property; returns are not i.i.d. draws, because under time aggregation an i.i.d. model would converge quickly to Gaussian, whereas real returns stay non-Gaussian out to weeks. The resolution is the multiplicative volatility structure `r_t = sigma_t xi_t` with long-memory `sigma_t`. The leverage effect adds asymmetry: `<xi_t sigma_{t+tau}> < 0` for `tau > 0` (past returns predict future volatility) while `<xi_t sigma_{t+tau}> ~ 0` for `tau < 0` (past volatility does not predict return sign). **Source:** Bouchaud (2018) §2.5-2.6 pp.398-399.

```
  the apparent efficiency paradox
   order-flow signs eps_t : C(l) ~ l^{-gamma},  gamma < 1   (LONG memory)
                                |
                                v  yet
   price returns r_t       : C_r(u) ~ 0                      (NO memory, random walk)
   --> persistent, predictable order flow somehow does NOT make prices predictable
```

The persistence of order-flow signs (`gamma ~ 0.5` for stocks, `~0.8` for futures) is attributed chiefly to order splitting rather than herding, and it is logically distinct from long memory in activity (one can build models with one and not the other). The coexistence of predictable order flow with unpredictable prices is the efficiency paradox that microstructure models must reconcile. **Source:** Bouchaud (2018) §2.8 pp.400-400.

## See Also

- [be-square-root-impact-zero-intelligence](./be-square-root-impact-zero-intelligence.md#intuition) — the order-flow microstructure (square-root impact, zero-intelligence) underlying these facts.
- [be-emergent-heterogeneity-volatility-feedback](./be-emergent-heterogeneity-volatility-feedback.md#intuition) — agent-interaction mechanisms that generate fat tails and clustering as emergent criticality.
- [be-bifurcation-route-instability](./be-bifurcation-route-instability.md#intuition) — how HAM bifurcation dynamics under noise reproduce volatility clustering and long memory.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) — the cross-sectional companion catalog of pricing anomalies.

## Escalate to Raw When

- Precise tail-exponent estimates, GARCH/Hawkes parameterizations, or the exact decay exponents `gamma` for specific markets are needed. **Source:** Bouchaud (2018) §2.5-2.8 pp.398-400.
- The volatility-signature-plot machinery (high-frequency-noise correction, Eq. 5-7) must be applied to a specific dataset. **Source:** Bouchaud (2018) §2.2-2.4 pp.396-398.
- The evidence on self-excitation versus news-driven variance (the "80% endogenous" claim) requires the source's citations and discussion. **Source:** Bouchaud (2018) §2.9 pp.400-400.
