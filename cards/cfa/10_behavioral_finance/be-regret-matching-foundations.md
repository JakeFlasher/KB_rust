---
schema_version: "cacg.v0"
id: "be-regret-matching-foundations"
title: "Regret-Matching Foundations"
reading_id: "10_behavioral_finance"
summary: "Regret-Matching Foundations: framing the Hart+Mas-Colell regret-matching algorithm as the adaptive-strategies game-theoretic bridge between behavioral assumptions and equilibrium concepts — agents play strategies in proportion to past regret, the resulting empirical play converges to correlated equilibria, and the framework supports clientele-segmentation / crowd-behavior cards in subcorpus 10"
tags: ["behavioral-finance", "regret-matching", "game-theory"]
citations:
  - source_id: "econ_hart_mascolell_2013_simple_adaptive_strategies"
    chunk_id: "econ_hart_mascolell_2013_simple_adaptive_strategies:p023:0026"
    chunk_hash: "091fed234825fe3cc5279a1ec773305a329a9652040756ca01e32911962b10e7"
    page_range: [23, 24]
    quote: "joint distribution of play converges to the set of correlated equilibria"
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p121:0138"
    chunk_hash: "9e4cdfc3e292cad5a45fa55011c61b8fe5ca4053785a2a651c26acea3787c0f2"
    page_range: [121, 122]
    quote: "conservatism, defined as the slow updating of models in the face of new evidence"
    edge_type: "supports"
card_hash: "50856b77ec7189ce1e462d6ee0e3e79d7b5102aa1f296964a1f9c24f4079b6a3"
---
# Regret-Matching Foundations

## Intuition

Behavioral finance often invokes the cognitive vocabulary (representativeness, anchoring, overconfidence) of subcorpus 10's [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition) without grounding it in a game-theoretic equilibrium framework. Hart+Mas-Colell's regret-matching algorithm DOCUMENTS a minimal adaptive-strategies bridge: an agent who plays strategies in proportion to the average regret they would have felt by deviating from each strategy in the past converges to play that, in aggregate across agents, supports a correlated equilibrium under broad conditions. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-15.

Shleifer ASSERTS that the regret-matching framework is the right behavioral-finance foundation because it is the weakest reasonable rationality assumption compatible with observed bias patterns: agents do not need to compute expected utilities or run dynamic programs; they only need to remember which actions they regret most and play accordingly. The framework supports clientele-segmentation dynamics in [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) because each clientele's capital allocation can be modeled as regret-matching against the recent-period realised returns. **Source:** Shleifer (2000) Ch.5 pp.130-153.

The framework EXPLAINS why behavioral-finance equilibria can be path-dependent without being chaotic: regret-matching produces stable convergence to correlated equilibria, but the specific equilibrium reached depends on the history of realised regrets — so two markets with identical fundamentals but different past trajectories can settle on different clientele-weighted price formations from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#definition). **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.15-30 + Shleifer (2000) Ch.5 pp.140-153.

```
<!-- primitive: behavioral-bias-taxonomy source: _diagram_primitives.md -->
                          +-----------------------+
                          |   Behavioral biases   |
                          +-----------+-----------+
                                      |
              +-----------------------+------------------------+
              |                       |                        |
       +------+------+         +------+-------+         +------+--------+
       | Heuristic   |         | Framing /    |         | Emotional /   |
       | (cognitive) |         | reference    |         | social        |
       +------+------+         +------+-------+         +------+--------+
              |                       |                        |
   +----------+----------+   +--------+--------+    +----------+----------+
   | representativeness  |   | anchoring       |    | overconfidence      |
   | availability        |   | framing effect  |    | loss aversion       |
   | recency / hindsight |   | mental accounts |    | herding / consensus |
   | gambler's fallacy   |   | status quo bias |    | regret aversion     |
   +---------------------+   +-----------------+    +---------------------+

   downstream pricing effect: each bias drives a specific deviation
   from the rational-expectations benchmark documented in card bodies.
```

The regret-matching framework APPLIES to each bias above by representing it as a deviation from uniform-weighted regret accumulation: recency-driven representativeness corresponds to exponentially-decayed regret history (the agent over-weights recent regrets); conservatism / anchoring corresponds to slow-update regret accumulation (the agent under-weights new regret signals); overconfidence corresponds to amplified-private-signal regret (the agent over-weights regrets on actions matching their private view). The mapping lets behavioral-finance bias patterns inherit the convergence machinery of correlated equilibria. **Source:** Shleifer (2000) Ch.5 pp.112-153 + Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-30.

## Definition

**Regret** at time `t` for an unchosen action `a'` (given the chosen action `a_t`) is the difference between the realised payoff under `a'` (had it been played, holding others' play fixed) and the realised payoff under the actually-chosen `a_t`. Hart+Mas-Colell ASSERTS that regret is the cognitively-minimal quantity an adaptive agent must compute: it requires only counterfactual payoff comparison, not utility-theory foundations or rational-expectations forecasts. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-15.

**Average regret** `R̄(a → a')` is the time-average of regret for switching from `a` to `a'` over the agent's history. Higher average regret for switching to `a'` (from past chosen `a`) signals that `a'` would have been a better play more often than not. The regret-matching algorithm plays action `a'` with probability proportional to `max(0, R̄(a → a'))`, normalised across the available alternatives. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.15-22.

**Correlated equilibrium** is the game-theoretic equilibrium concept that regret-matching converges to: a probability distribution over joint action profiles such that no agent gains by deviating from the recommended action given that others follow theirs. Correlated equilibrium is weaker than Nash equilibrium (every Nash equilibrium is a correlated equilibrium but not conversely) and supports broader joint behaviour, including coordination on history-dependent strategies. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.22-30.

**Adaptive-strategies bridge** is the framing that connects behavioral-finance bias patterns to game-theoretic equilibria via regret-matching: cognitive biases (representativeness, conservatism) are interpreted as deviations from optimal regret-matching (e.g., overweighting recent observations corresponds to a regret-matching variant with exponentially-decayed history rather than uniform-weighted history). The bridge lets behavioral finance inherit the convergence machinery of game theory without requiring fully-rational agents. **Source:** Shleifer (2000) Ch.5 pp.130-153.

## Mathematical Reasoning

The regret-matching update EXPLAINS the path-dependent equilibrium-selection mechanism: at each period the agent plays action `a` with probability `π(a) ∝ max(0, R̄(·, a))`, normalised across the available actions. Symbolically, if the agent's action set is `A = {a_1, ..., a_n}` and the running average regret for switching from past-chosen action to `a_j` is `R̄_j`, then `π(a_j) = max(0, R̄_j) / Σ_k max(0, R̄_k)` (with a uniform default when all R̄ are non-positive). **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.15-22.

The update DOCUMENTS three structural properties: agents put zero probability on actions with non-positive average regret (no rational reason to play strictly-worse alternatives in the long run); positive-regret actions are played in proportion to their regret magnitude (stronger regret → higher probability); the rule depends only on counterfactual payoffs against the agent's own history, not on beliefs about others' play (a "no-belief" learning rule). **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.20-30.

The convergence theorem PREDICTS that under broad conditions (finite action set, bounded payoffs, all agents using regret-matching) the joint empirical play converges to the set of correlated equilibria almost surely. The proof relies on Blackwell's approachability theorem and is given in full in Hart+Mas-Colell §2; this card paraphrases the result without re-deriving it. The convergence is to the SET of correlated equilibria, not to a specific one — the path-dependent equilibrium-selection is the source of history-dependence in clientele-weighted price formation from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning). **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.30-50.

The behavioral-finance APPLICATION reframes each clientele tier's capital-allocation decision as a regret-matching update: noise traders regret missed-momentum opportunities and bias toward continuation strategies; arbitrageurs regret carrying-cost losses and bias against extending convergence horizons after drawdowns; fundamental holders regret deviations from policy benchmarks and bias toward policy-anchored rebalancing. The resulting clientele-weight dynamics from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) are then a regret-matching equilibrium over clientele-portfolio choices. **Source:** Shleifer (2000) Ch.5 pp.130-153 + Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-50.

## See Also

- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) — the clientele-weighted price-formation identity whose `w`-dynamics are driven by regret-matching across clientele tiers.
- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — the bounded-arbitrage equilibrium whose convergence-horizon parameter `H` is shaped by regret-matching dynamics on arbitrage-capital allocation.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the divergence-path framing under which regret-matching dynamics produce bounded but persistent mispricing.
- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the representativeness / conservatism taxonomy that maps to specific regret-matching deviations from uniform-weighted history.

## Escalate to Raw When

Open Hart+Mas-Colell *Simple Adaptive Strategies* and Shleifer 2000 Ch.5 directly when any of the criteria below applies. **Source:** Shleifer (2000) Ch.5 pp.112-153 + Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-50.

- A specific behavioral-finance application requires the full regret-matching convergence proof rather than the verbal summary in this card — open Hart+Mas-Colell §2 for the Blackwell-approachability machinery. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.30-50.
- The mapping from a specific cognitive bias (representativeness, conservatism, overconfidence) to a regret-matching variant (history-weighted, attention-discounted, signal-amplified) requires the original Shleifer 2000 Ch.5 case-study discussion rather than the bridge summary in this card. **Source:** Shleifer (2000) Ch.5 pp.130-153.
- A clientele-segmentation case study requires the Hart+Mas-Colell adaptive-strategies machinery applied to a specific market microstructure (closed-end fund discount, narrative-driven sector rotation) — open Ch.5 + Hart+Mas-Colell §1-3 for the joint derivation. **Source:** Shleifer (2000) Ch.5 pp.130-153 + Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-50.
