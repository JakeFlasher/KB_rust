---
schema_version: "cacg.v0"
id: "be-experimental-asset-bubbles"
title: "Experimental Asset-Market Bubbles and Crashes"
reading_id: "10_behavioral_finance"
summary: "Smith-Suchanek-Williams laboratory asset markets: inexperienced subjects systematically over-price an asset relative to its known rational-expectations fundamental, producing bubbles and crashes even when dividends are common knowledge; experience and cognitive sophistication reduce mispricing."
tags: ["behavioral-finance", "experimental-economics", "asset-bubbles", "fundamental-value", "positive-feedback"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p509:0760"
    chunk_hash: "85e1b7bc6f316d8cd13758a3cef65ea10fa1be41f66c63b498af145314214348"
    page_range: [510, 510]
    quote: "Experimental tests, beginning with Smith et al. (1988), have consistently found that inexperienced subjects over-price such assets relative to the"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p509:0760"
    chunk_hash: "85e1b7bc6f316d8cd13758a3cef65ea10fa1be41f66c63b498af145314214348"
    page_range: [510, 510]
    quote: "they are less prone to exhibit mis-pricing in repeated interactions."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p509:0760"
    chunk_hash: "85e1b7bc6f316d8cd13758a3cef65ea10fa1be41f66c63b498af145314214348"
    page_range: [510, 510]
    quote: "bubbles are less likely among more cognitively sophisticated subjects and more likely among groups with mixed cognitive abilities."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p524:0781"
    chunk_hash: "49fcf6634232e269791b7b75bd4b178e49004c399fa11788c135f098bd03155f"
    page_range: [524, 524]
    quote: "in 5 of their six experiments, prices periodically hit the upper bound of 1000"
    edge_type: "supports"
card_hash: "f79ef21cae911f4821e29e022dad9da6e5fecb7c6c936e0495927848cbc8d09d"
---
# Experimental Asset-Market Bubbles and Crashes

## Intuition

The cleanest laboratory demonstration that markets can detach from fundamentals comes from Smith, Suchanek, and Williams (1988). Subjects trade a finite-lived asset that pays a stochastic dividend each period with a known mean, so the rational-expectations fundamental value is computable and DECLINES deterministically as the remaining dividend stream shrinks. Despite this transparency, inexperienced subjects consistently over-price the asset: prices balloon into a bubble and then crash toward fundamentals near the end. The mispricing is not due to confusion about value — the value is common knowledge — but to the dynamics of trading among heterogeneous, boundedly rational participants. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.492-492.

Two robust regularities sharpen the lesson. First, EXPERIENCE matters: once a cohort has lived through a bubble, mispricing shrinks in repeated sessions — so inexperience versus experience is itself a dimension of heterogeneity governing bubble incidence. Second, COGNITIVE composition matters: bubbles are less likely among more cognitively sophisticated subjects and more likely among groups with mixed cognitive abilities. Heterogeneity, not a uniform irrationality, drives the phenomenon. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.492-492.

The result is foundational for behavioral asset pricing because it isolates bubble formation from any fundamental-information story: with fundamentals fixed and known, the only remaining engine is belief-and-outcome interaction among traders. When the price-forecast interval is widened and robot fundamentalists are removed, lab prices can reach more than fifteen times fundamentals — driven by positive-feedback, trend-following expectation formation. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.506-506.

## Definition

**Experimental asset market** is a controlled laboratory market for a finite-lived asset paying a known-distribution dividend each period, in which the rational-expectations fundamental value is the present value of remaining expected dividends and is common knowledge to subjects. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.492-492.

**Bubble (in the lab)** is a sustained, large positive deviation of the traded price above the rational-expectations fundamental value, typically followed by a crash toward fundamentals as the asset's remaining life shortens. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.492-506.

**Experience effect** is the empirical regularity that a subject cohort which has previously experienced a bubble exhibits substantially reduced mispricing in subsequent repeated markets. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.492-492.

## Mathematical Reasoning

In the Hommes et al. (2005) version of the asset-pricing experiment, a risk-free bond pays gross return `1 + r` and a long-lived risky asset pays i.i.d. dividends with mean `d-bar`. Arbitrage gives the pricing equation `p_t = (1/(1+r))(p^e_{t+1} + d-bar)`. Under rational expectations `p^e_{t+1} = p_t = p^f = d-bar/r`, so the REE price path is the constant fundamental `p^f`. Whether subjects coordinate on this prediction is the empirical question; they typically do not. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.504-504.

Expanding the price by the law of iterated expectations, `p_t = sum_{i=1}^{n}(1+r)^{-i} d-bar + (1+r)^{-n} E_t(p_{t+n})`. Taking `n -> infinity` and assuming a limit, the solution splits into a fundamental and a bubble term, `p_t = p^f + b_t`. For a RATIONAL bubble the term must grow at rate `r`. Hommes et al. found no rational bubble in this strict sense, but observed prices repeatedly hit the experiment's upper bound (1000, > 15x fundamentals) before crashing — bubbles driven by positive feedback rather than by rational `r`-growth. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.506-506.

```
   typical lab asset-price path
   price
     |        bubble peak (>> p^f)
     |          /\
     |         /  \
     |        /    \  crash
     |  ___  /      \___
   p^f|     \/           \____  declining fundamental
     +-----------------------------> period (finite horizon)
```

Huesler et al., using the same data, fit the bubble growth as `log(p_t/p_{t-1}) = r + gamma p_{t-1}` with anchoring weight `gamma > 0`, giving "super-exponential" growth (faster than `r`) — the positive weight on the recent price is what makes prices grow faster than the rational rate. (The source reports these as empirical fits, not derivations.) **Source:** Arifovic and Duffy (2018) §2.3.1 pp.506-507.

## See Also

- [be-noise-trader-equilibrium](./be-noise-trader-equilibrium.md#intuition) — the noise-trader-risk apparatus that lets mispricing persist when fundamentals are known.
- [be-keynesian-beauty-contest-level-k](./be-keynesian-beauty-contest-level-k.md#intuition) — the higher-order-belief coordination dynamic underlying speculative over-pricing.
- [be-rationally-heterogeneous-expectations](./be-rationally-heterogeneous-expectations.md#intuition) — the learning-to-forecast experiments and heuristic-switching model explaining these bubbles.
- [be-destabilizing-arbitrage-positive-feedback](./be-destabilizing-arbitrage-positive-feedback.md#intuition) — positive-feedback trading that amplifies rather than corrects bubbles.

## Escalate to Raw When

- The exact dividend structure, payoff scoring rules, and treatment designs of specific bubble experiments (Smith et al. 1988; Hommes et al. 2005, 2008) are needed. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.504-507.
- The cognitive-ability and experience treatment results (Bosch-Rosa et al.; Hanaki et al.; Dufwenberg et al.) require the source's references. **Source:** Arifovic and Duffy (2018) §1, §2.3.1 pp.492-492.
- The full super-exponential bubble-growth fits and alternative functional forms must be read from the source. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.506-507.
