---
schema_version: "cacg.v0"
id: "ec-game-theory-and-strategic-equilibrium"
title: "Game Theory and Strategic Equilibrium"
reading_id: "02_economics"
summary: "MWG Ch.7-9 strategic interaction: normal- vs extensive-form games; Nash equilibrium (best response to others' strategies; Nash 1950 mixed-strategy existence in finite games); subgame-perfect equilibrium for extensive-form games (rules out non-credible threats)."
tags: ["economics", "game-theory"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p253:0422"
    chunk_hash: "4815b16d9323ebb7a0a67537f07c55d5817697f5eae8f1386b7193a3b52c8d89"
    page_range: [253, 254]
    quote: "We discuss in some detail the reasonableness of this requirement, as well as the conditions under which we can be assured that a Nash equilibrium exists."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p240:0399"
    chunk_hash: "431a859743eb82ba33d02203273c60adf891de7b3a1b7de340610e41335313c5"
    page_range: [240, 241]
    quote: "(This is a very nice game for player 2!) The extensive form representation of this game is depicted in Figure 7.C."
    edge_type: "defines"
  - source_id: "econ_hart_mascolell_2013_simple_adaptive_strategies"
    chunk_id: "econ_hart_mascolell_2013_simple_adaptive_strategies:p007:0004"
    chunk_hash: "cfd72813950e0cb5c4a8f6999c4ccb9add86a07af9ab1f9db2643ab2a6043bdd"
    page_range: [7, 8]
    quote: "Left unstated in its definition, however, is how a Nash equilibrium might actually come about."
    edge_type: "supports"
  - source_id: "econ_mascolell_general_equilibrium_game_theory"
    chunk_id: "econ_mascolell_general_equilibrium_game_theory:p182:0224"
    chunk_hash: "52e90cf54ecf619f524473f35bfa5e559b706164336e45327f8c4d02e65c9282"
    page_range: [182, 183]
    quote: "(1987) have pushed a logic similar to this section to its limit and defined a Consistent Bargaining Set."
    edge_type: "supports"
card_hash: "35a5e7c6dde33971f56992a2a949c8ff7b12993b0ffbdd18638217314fbc9a6a"
---
# Game Theory and Strategic Equilibrium

## Intuition

Game theory studies strategic interaction between rational agents whose payoffs depend on each other's choices. The canonical solution concept is the **Nash equilibrium (NE)**: a strategy profile where each player's strategy is a best response to the others' strategies, so no player has incentive to unilaterally deviate. NE always exists in finite games (Nash 1950 fixed-point theorem) — possibly in mixed strategies — making it the workhorse solution concept for all of modern microeconomics, industrial organization, mechanism design, and contract theory. The challenge with NE is uniqueness: most games have multiple NE, requiring refinement criteria to pick one. **Source:** Mas-Colell et al. (1995) Ch.7-8 pp.219-306.

```
   Nash equilibrium intuition (symbolic payoff rankings)

   Prisoner's Dilemma                 Coordination Game (multiple NE)

                Player 2                          Player 2
              Coop    Defect                    A         B
   Player 1                            Player 1
     Coop    (R,R)  (S,T)               A      (a,a)    (zero,zero)
     Defect  (T,S)  (P,P)               B      (zero,zero) (b,b)

     with  T > R > P > S                 with  a > zero, b > zero
                                         (typically a > b for asymmetric
   unique NE = (Defect,Defect)            payoff dominance)
   Pareto-dominated by (Coop,Coop)
   but Defect is dominant strategy        two pure NE: (A,A), (B,B)
   for each player                        plus mixed NE
                                          refinement criteria pick one
                                          (focal points, payoff dominance)
```

The **subgame-perfect equilibrium (SPE)** refines NE for extensive-form games (sequential moves) by requiring the strategy profile to be a Nash equilibrium in every subgame, not just at the start. This rules out non-credible threats — strategies that would not actually be played at the relevant decision node. The classic example: a chain-store entry-deterrence game where the incumbent threatens predation if a new firm enters. NE includes "entry never happens because the incumbent always predates"; but SPE rules this out because at the actual entry node, predation is suboptimal for the incumbent given the cost. **Source:** Mas-Colell et al. (1995) Ch.9 pp.282-306.

## Definition

The **normal-form game** in symbolic form. **Source:** Mas-Colell et al. (1995) Ch.7 pp.219-235.

```
G  =  (N, {S_i}_{i ∈ N}, {u_i}_{i ∈ N})

where:  N        = set of players
        S_i      = strategy set for player i
        u_i: S → R   = payoff function (S = ×_i S_i)
```

A **strategy profile** `s = (s_1, ..., s_n) ∈ S` is a **Nash equilibrium** if for every player `i` and every alternative strategy `s_i' ∈ S_i`. **Source:** Mas-Colell et al. (1995) Ch.7 pp.219-235.

```
u_i(s_i, s_{-i})  ≥  u_i(s_i', s_{-i})        [Nash condition]
```

where `s_{-i}` denotes the strategies of all players other than `i`. **Source:** Mas-Colell et al. (1995) Ch.7 pp.219-235.

The **mixed strategy** generalization: let `σ_i ∈ Δ(S_i)` be a probability distribution over player `i`'s strategies. The expected payoff is `U_i(σ) = E_σ[u_i(s)]`. A mixed-strategy NE is a profile `σ = (σ_1, ..., σ_n)` where no player gains by switching to a different mixed strategy. **Nash's existence theorem (1950)**: every finite normal-form game has at least one mixed-strategy NE. The proof uses Kakutani's fixed-point theorem on the best-response correspondence. **Source:** Mas-Colell et al. (1995) Ch.8 pp.235-280.

The **extensive-form game** representation makes the sequence of moves and information sets explicit. **Source:** Mas-Colell et al. (1995) Ch.9 pp.282-306.

```
G^E  =  (N, V, ι, A, ξ, π, u)

where:  V        = nodes in the game tree
        ι(v)     = player to move at node v (or "Nature")
        A(v)     = actions available at v
        ξ        = predecessor function on V
        π        = information-set partition (player i cannot distinguish
                   nodes in the same element of i's partition)
        u_i      = payoff at terminal nodes (leaves of the tree)
```

A **subgame-perfect equilibrium (SPE)** is a strategy profile that induces a Nash equilibrium in every proper subgame of the extensive form. Solved by **backward induction** for games of perfect information: at each terminal-most node, choose the optimal action; given the optimal continuations, choose the optimal action at the next-most-terminal node; iterate to the root. **Source:** Mas-Colell et al. (1995) Ch.9 pp.282-306.

## Mathematical Reasoning

The **Nash-existence theorem** proof sketches. **Source:** Mas-Colell et al. (1995) Ch.8 pp.235-280.

Define each player `i`'s best-response correspondence `BR_i: Δ(S_{-i}) → Δ(S_i)` mapping the other players' mixed strategies to player `i`'s best-response mixed strategy. Player `i`'s expected payoff is continuous and linear in `σ_i` (holding `σ_{-i}` fixed), so `BR_i` is upper-hemicontinuous, convex-valued, and non-empty. The product correspondence `BR = ×_i BR_i: Δ(S) → Δ(S)` inherits these properties on the convex compact space `Δ(S)`. Kakutani's fixed-point theorem then guarantees a fixed point `σ* ∈ BR(σ*)`, which is by definition a mixed-strategy Nash equilibrium. The proof is constructive enough to suggest computational methods (Lemke-Howson algorithm for 2-player finite games), though general computation of NE is PPAD-complete and intractable in worst-case complexity. **Source:** Mas-Colell et al. (1995) Ch.8 pp.235-280.

The **subgame-perfection via backward induction** is constructive for finite-horizon perfect-information games. **Source:** Mas-Colell et al. (1995) Ch.9 pp.282-306. Starting from terminal nodes, identify each player's optimal action; substitute the optimal continuation payoff for the action; the predecessor node becomes a terminal node with the substituted payoff; repeat until reaching the root. The resulting strategy profile is the unique SPE under generic payoffs (ties at any decision node create multiplicity). The famous **centipede game** illustrates how SPE can predict "defect immediately" outcomes that are starkly suboptimal — a feature SPE shares with other rational-equilibrium concepts, and a motivator for behavioral-game-theory refinements (regret matching, level-k thinking) covered in the Hart-MasColell supporting source and the future-10 Behavioral Finance vertical. **Source:** Mas-Colell et al. (1995) Ch.9 pp.282-306.

The **Bayesian-game extension** handles incomplete information (each player has private information about their own type, drawn from a common-prior distribution). The solution concept is **Bayesian Nash equilibrium (BNE)**: each player's strategy, mapping their type to an action, is a best response in expectation over the other players' type distributions. The extensive-form generalization, **perfect Bayesian equilibrium (PBE)**, adds belief-consistency requirements at every information set, including off-equilibrium-path nodes. PBE is the foundation for signaling games (Spence job-market signaling), screening (Rothschild-Stiglitz insurance markets), and mechanism design (Myerson optimal auction). The cross-vertical bridge: applications to industrial organization (sibling [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) for monopoly; oligopoly applications deferred to future-04 Corporate Finance) and to financial-economics topics (signaling in capital-structure decisions, asymmetric-information lemons markets in 09 + 06) are extensive but out of v10 scope. **Source:** Mas-Colell et al. (1995) Ch.8-9 pp.235-306.

## See Also

- [`ec-consumer-preference-and-choice`](./ec-consumer-preference-and-choice.md) — rationality axioms underlying the game-theoretic "rational player" assumption
- [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) — strategic-pricing applications in single-firm settings (oligopoly applications deferred to future-04)
- [`ec-welfare-theorems`](./ec-welfare-theorems.md) — connection to general-equilibrium analysis (welfare theorems are a non-strategic complement to game theory)

## Escalate to Raw When

The full MWG treatment of Bayesian games, mechanism design, and the revelation principle is in Mas-Colell et al. (1995) Ch.8-9 pp.235-306 and Ch.23 pp.857-908 (auction theory + Myerson optimal auction). The cooperative-game-theory branch (the core, Shapley value, Nash bargaining solution, axiomatic bargaining) is treated separately in Mas-Colell-General-Equilibrium and is out of v10 scope. The behavioral-game-theory literature on adaptive strategies, regret matching, and bounded-rationality refinements is in Hart & Mas-Colell (the supporting source for this card) and is the natural home for future-10 Behavioral Finance work. The applied industrial-organization literature on oligopoly equilibria (Cournot, Bertrand, Stackelberg, repeated-game collusion) and on auction-theory applications is mainstream IO graduate material out of v10 scope. **Source:** Mas-Colell et al. (1995) Ch.7-9 pp.219-306.
