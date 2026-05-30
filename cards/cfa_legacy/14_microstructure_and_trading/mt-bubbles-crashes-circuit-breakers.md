---
schema_version: "cacg.v0"
id: "mt-bubbles-crashes-circuit-breakers"
title: "Bubbles, Crashes, and Circuit Breakers: Volatility Interruptions"
reading_id: "14_microstructure_and_trading"
summary: "Bubbles and crashes arise from feedback among sentiment, leverage, and liquidity withdrawal; circuit breakers (halts, price limits) pause trading to disseminate information and replenish liquidity, trading price-discovery delay against panic containment."
tags: ["microstructure", "circuit-breakers", "trading-halts", "bubbles", "crashes", "liquidity"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p584:0985"
    chunk_hash: "c9bfa229bff4915a6829baf3cce50d2fcfe33dc9f5497f63c7b0fdbef49d1e40"
    page_range: [585, 585]
    quote: "Trading halts stop trading when prices have moved, or will imminently"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p042:0058"
    chunk_hash: "da1ed45699cae7b4a69e97624bf392cad32dcb81f0c05548bf1961389b3aec0a"
    page_range: [42, 42]
    quote: "a large sell order may trigger a very sharp price drop, as it"
    edge_type: "supports"
---
# Bubbles, Crashes, and Circuit Breakers: Volatility Interruptions

## Intuition
A bubble is not a single trader's error but a self-reinforcing process. Some
buyers grow overly optimistic about fundamental values and push prices up.
Momentum traders then buy because past gains imply future gains, and order
anticipators buy ahead of the new uninformed buyers they expect. The combined
demand accelerates prices, while value traders and arbitrageurs who suspect
overpricing stay on the sidelines — they may lack capital, the ability to sell
short, or the confidence to trade against the crowd. Eventually selling begins;
momentum buyers lose interest, margined holders and stop-loss orders are forced
to sell, and the same feedback runs in reverse as a crash.

The danger in a crash is that *panicked sellers* demand liquidity faster than
the market can supply it, so price can fall below fundamental value before
"dead-cat-bounce" buyers recognize the overreaction. This is the policy hook:
regulators want a way to interrupt the panic without freezing genuine price
discovery.

```
  BUBBLE                              CRASH
  optimism --> price up               bad news / exhaustion
     ^            |                       |
     |            v                       v
  more buying <- momentum,            margin calls + stop-loss
                 anticipators          + panicked liquidity demand
                                            |
                                            v
                                   price overshoots DOWN
                                            |
                                   [CIRCUIT BREAKER pauses here]
```

A circuit breaker is the institutional pause inserted at the bottom-right of
that diagram: a trading halt, price limit, or related rule that stops or bounds
trading so information can disseminate and liquidity suppliers can re-engage.
**Source:** Harris (2003) ch.28 §28.1.1, §28.3 pp.556-557, 572.

## Definition
Harris groups regulatory responses to extreme volatility under **circuit
breakers** — "trading rules that limit trading activity." The two core forms:

- **Trading halt:** stops trading when prices have moved (or will imminently
  move) by a specified amount; trading stays halted until an order imbalance
  resolves or a fixed period passes.
- **Price limit:** requires all trade prices to lie within a stated range for a
  given day; if traders will not trade inside the band, trade stops until they
  will.

Related restraints in the same family include transaction taxes, margin
requirements / position limits, and collars on system access. A **coordinated
trading halt rule** is a market-wide version (e.g., the post-1987 U.S.
equity/futures rules tied to index-level declines), as opposed to a
**discretionary halt** invoked by floor officials when trading would otherwise
become disorderly.
**Source:** Harris (2003) ch.28 §28.3, §28.3.1 pp.572-575.

## Mathematical Reasoning
The mechanism is a comparative-statics argument over the *source* of volatility,
not arithmetic.

Decompose realized volatility into **fundamental** volatility (driven by new
information about value) and **transitory** volatility (driven by uninformed
order imbalances and liquidity demands). A halt's welfare sign depends on which
dominates:

- If volatility is **fundamental**, a halt merely postpones the inevitable.
  While closed, prices are less informative; the uncertainty can itself cause
  uninformed traders to panic and *generate* transitory volatility at reopening.
  Sign of effect: ambiguous, plausibly adverse.
- If volatility comes from an **uninformed order imbalance**, halting protects
  the market from volatility-inducing trades and protects those uninformed
  traders from losses; it also publicizes the imbalance so informed liquidity
  suppliers can enter. Sign of effect: plausibly beneficial.

A second mechanism is the **pricing-rule switch**: continuous trading uses a
discriminatory rule (standing limit orders execute at their own limit prices and
take immediate losses as price keeps falling), whereas a halt restarts via a
single-price (uniform) auction where all participants clear at one price. Knowing
they are protected by this switch, limit-order traders are more willing to post
liquidity *ex ante*, lowering transitory volatility.

Against these, opponents identify the **gravitational (magnet) effect**: if
traders rationally fear a halt will lock them out, they submit orders earlier to
raise execution probability, which *pulls* price toward the trigger and raises
volatility. Hence the prediction that a **less predictable** discretionary halt
has weaker gravitational pull than a mechanical price-triggered limit. A
fragmentation corollary: an *uncoordinated* halt on one venue diverts order flow
elsewhere, forcing all excess liquidity demand into the still-open market and
potentially *increasing* volatility — so if breakers are desirable they must be
coordinated across venues trading the same risk.
**Source:** Harris (2003) ch.28 §28.1.2, §28.3.1 ("Arguments For/Against,"
"Other Issues") pp.557-575.

## Boundary Notes
The favorable case for halts assumes the disturbance is transitory and that
informed liquidity suppliers exist but are merely inattentive — a halt buys them
time. Where the move reflects genuine fundamental news, the same halt only
defers price discovery and can manufacture reopening panic. Harris stresses that
many crashes are a "final restoration of rational pricing" after a long-building
bubble, not a transitory error to be corrected, which weakens the case for
intervention in those episodes.

Contrast the two failure modes: trading halts can *reduce* transitory volatility
(imbalance-publication, liquidity replenishment, margin/stop-loss relief, lower
margin requirements) **or** *increase* it (gravitational effect, reduced value-
trader surveillance, cross-market fragmentation). The net sign is empirical and
depends on predictability and coordination.

The supporting source extends the picture to modern automated markets: in the
2010 Flash Crash, a single very large sell order first exhausted available
buyers and then precipitated high-frequency selling, so that — absent
price-contingent execution — a large order "may trigger a very sharp price drop"
by hitting ever-lower quotes. This is the same liquidity-evaporation feedback
Harris describes, now operating at machine speed, and motivates modern
single-stock and market-wide breakers.
**Source:** Harris (2003) ch.28 §28.3.1 p.572; Foucault, Pagano & Röell (2013)
ch.1 Box 1.2 p.42.

## See Also
- [`mt-funding-liquidity-fire-sales`](./mt-funding-liquidity-fire-sales.md) -- margin calls and the funding/market-liquidity spiral that fuels crash dynamics
- [`mt-market-quality-volatility-origins`](./mt-market-quality-volatility-origins.md) -- the fundamental-vs-transitory volatility decomposition that decides a halt's welfare sign
- [`mt-competition-within-among-markets`](./mt-competition-within-among-markets.md) -- why uncoordinated breakers fragment liquidity across venues

## Escalate to Raw When
Harris develops the favorable/adverse arguments through a sequence of mechanisms
(imbalance publication, discriminatory-vs-uniform pricing, margin and stop-loss
dynamics, the bankruptcy/margin-collection benefit) and tabulates the post-1987
coordinated NYSE Rule 80B halt thresholds and timing; this card only sketches
them. Re-read Harris (2003) §28.3.1 pp.572-575 for the full argument inventory
and the rule mechanics, §28.1 pp.556-558 for bubble/crash dynamics, and
Foucault, Pagano & Röell (2013) Box 1.2 p.42 plus the funding-liquidity sections
for the modern automated-market feedback channel.
