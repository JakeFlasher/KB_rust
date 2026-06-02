---
schema_version: "cacg.v0"
id: "be-emergent-heterogeneity-volatility-feedback"
title: "Emergent Fat Tails and Volatility Clustering From Herding"
reading_id: "10_behavioral_finance"
summary: "In the Kirman / Alfarano social-interaction lineage, fat tails and volatility clustering are not assumed but emerge endogenously from herding among heterogeneous agents; the tail heaviness becomes a function of the herding tendency rather than of exogenous shocks."
tags: ["behavioral-finance", "herding", "emergent-criticality", "volatility-clustering", "fat-tails"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p455:0673"
    chunk_hash: "f76a6cf54d84837b4c2443d757d54de6f9b3f2d685787af6811a40f2049d940a"
    page_range: [456, 456]
    quote: "regularities characterizing asset returns and volatility, and they seem to be best understood as emergent properties of a system composed of dispersed activity with conflicting centrifugal and centripetal tendencies."
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p469:0696"
    chunk_hash: "cea642cee1c7be29f4f1d692a702ada4eacf7192b58de7c4aa03aa2d20a5ec7c"
    page_range: [470, 470]
    quote: "In this model, switching is based on social interaction and herding rather than profitability considerations. Alfarano et al. (2005) show that in this model the tail behavior of the distributions of returns is a function of the herding tendency of agents."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p285:0405"
    chunk_hash: "04ea4efc236cf96217200bbdf35f15dcb1c93c0281b0780f123262a478446827"
    page_range: [286, 286]
    quote: "is also in line with Franke and Westerhoff (2012, 2016) who estimate various HAMs and show that herding behavior plays a key role in matching the stylized facts."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p286:0406"
    chunk_hash: "f7ebacc442d4bc617fac7552fb982ed9af65aac6bc768057cde56e3d4ad41c14"
    page_range: [286, 286]
    quote: "the noise traders play an important role in generating insignificant ACs on the returns, while the significantly decayed AC patterns of the absolute returns and squared returns are more influenced by the fundamental noise."
    edge_type: "supports"
card_hash: "afcbaf4028eb0c99c13af6b0e20c6ab33cb371e67979b87b96c8a86d9e5a84a2"
---
# Emergent Fat Tails and Volatility Clustering From Herding

## Intuition

A different lineage of heterogeneous-agent models — running from Kirman (1993) to Alfarano et al. (2005) — explains the stylized facts not by tuning rules but by letting heterogeneity and the stylized facts EMERGE from social interaction. Agents are not permanently fundamentalist or chartist; they switch type through contagion and herding, copying the mood of neighbors rather than coldly comparing profits. The market's statistical regularities — fat tails, volatility clustering — are then best understood as emergent properties of a system of dispersed activity with conflicting centrifugal (destabilizing) and centripetal (stabilizing) tendencies. **Source:** Lux and Zwinkels (2018) §1 pp.438-438.

The decisive result, shown by Alfarano et al. (2005) for the Kirman (1993) model, is that the heaviness of the return-distribution tails is a FUNCTION of the herding tendency of agents: crank up the propensity to imitate and the tails fatten. This makes the fat tails endogenous and tied to a behavioral parameter, rather than imposed by assuming a heavy-tailed shock distribution. Volatility clustering arises the same way — bursts of herding concentrate activity, alternating with calmer periods, reproducing the intermittency of real markets. (A "near-criticality" reading — the system poised where small perturbations can cascade — is an external framing often attached to such interaction models, not a claim made on the cited page.) **Source:** Lux and Zwinkels (2018) §3.3 pp.452-452.

The mechanism is corroborated across the HAM literature: estimated models repeatedly find that herding behavior plays a key role in matching the stylized facts, with noise traders central to generating the near-zero return autocorrelation alongside the slowly decaying autocorrelation of absolute and squared returns. Emergent heterogeneity is therefore a unifying behavioral explanation for the volatility-feedback signature of markets. **Source:** Dieci and He (2018) §2.2 pp.268-268.

## Definition

**Herding (social-interaction) switching** is type change driven by imitation of other agents' current choices (contagion) rather than by comparing the realized profitability of strategies, as in the Kirman (1993) and Alfarano et al. (2005) models. **Source:** Lux and Zwinkels (2018) §3.3 pp.452-452.

**Emergent stylized facts** is the property that fat tails and volatility clustering are not built into the model's primitives but arise endogenously from the interaction of heterogeneous agents, so that the tail exponent depends on a behavioral (herding) parameter. **Source:** Lux and Zwinkels (2018) §1, §3.3 pp.438-452.

**Volatility feedback** is the regime in which alternating herding-driven bursts of activity and calm produce long-memory volatility, arising from the conflict between centrifugal (destabilizing) and centripetal (stabilizing) tendencies in the system. (A "near-criticality" cascade reading is an external label sometimes attached to this regime, not a claim of the cited page.) **Source:** Lux and Zwinkels (2018) §1 pp.438-438.

## Mathematical Reasoning

In the reduced-form herding HAM, aggregate demand is `D_t^m = n_t^f D_t^f + n_t^c D_t^c` with type fractions `n_t^f, n_t^c`, and the market maker sets `p_t = p_{t-1} + lambda D_t^m + eps_t`. What distinguishes the herding lineage is the LAW governing `n_t^f`: instead of a profit-based logit, switching follows social interaction, so the fraction obeys a contagion (master-equation / opinion-dynamics) process in which the transition rate of an agent toward a type rises with the number already in that type. (The source describes this mechanism rather than reproducing the master equation.) **Source:** Lux and Zwinkels (2018) §3.1, §3.3 pp.446-452.

The qualitative consequence, established by Alfarano et al. (2005), is `tail-exponent = f(herding intensity)`: the more strongly agents imitate, the heavier the return tails. This is the contrast with the profit-based Brock-Hommes logit, where instability is governed by the intensity of choice `beta` and the deterministic skeleton's bifurcation. In the herding view there need be no exogenous heavy-tailed input — the tails are generated by the interaction itself. **Source:** Lux and Zwinkels (2018) §3.3 pp.452-452.

```
   two routes to the same stylized facts
   profit route (Brock-Hommes) : raise beta -> bifurcation -> cycles + noise -> clustering
   herding route (Kirman/Alf.) : raise herding propensity -> contagion bursts -> fat tails
                                                     |
                       both: stylized facts EMERGE from interaction, not from shocks
```

Estimation results reinforce the picture: across HAMs, herding is a key ingredient for matching the data, and the decomposition of autocorrelations shows noise traders generate the near-zero return AC while the slowly decaying AC of absolute and squared returns (volatility clustering / long memory) is shaped by the fundamental noise interacting with the coexisting deterministic dynamics. **Source:** Dieci and He (2018) §2.2 pp.268-268.

## See Also

- [be-stylized-facts-financial-markets](./be-stylized-facts-financial-markets.md#intuition) — the catalog of fat tails, volatility clustering, and long memory that herding endogenously reproduces.
- [be-bifurcation-route-instability](./be-bifurcation-route-instability.md#intuition) — the alternative profit-based route to endogenous volatility clustering via bifurcation.
- [be-brock-hommes-switching](./be-brock-hommes-switching.md#intuition) — the profit-based logit switching contrasted with social-interaction herding here.
- [be-noise-trader-equilibrium](./be-noise-trader-equilibrium.md#intuition) — the noise-trader role in generating the near-zero return autocorrelation.

## Escalate to Raw When

- The explicit master-equation / opinion-dynamics formulation of the Kirman (1993) and Alfarano et al. (2005) herding processes is needed beyond the qualitative description. **Source:** Lux and Zwinkels (2018) §3.3 pp.452-452.
- The empirical tail-exponent-vs-herding estimates and the goodness-of-fit comparisons across HAMs must be quoted from the source. **Source:** Lux and Zwinkels (2018) §3.3 pp.452-456.
- The Gaunersdorfer et al. (2008) coexisting-attractor mechanism and its autocorrelation decomposition require the Dieci-He treatment. **Source:** Dieci and He (2018) §2.2 pp.268-270.
