---
schema_version: "cacg.v0"
id: "mt-temporary-permanent-price-impact"
title: "Price Impact: Temporary vs Permanent and the Empirics of Walking the LOB"
reading_id: "14_microstructure_and_trading"
summary: "A trade's price impact splits into a transient temporary component (execution worse than the midprice, then recovers) and a permanent informational component that shifts the midprice; both are measured empirically by walking the limit order book."
tags: ["microstructure", "price-impact", "limit-order-book", "execution", "market-impact"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p104:0141"
    chunk_hash: "4216fac075e724cf57094ae8e83591aa4433c76d71d9e435aa0160f118a43fee"
    page_range: [104, 104]
    quote: "empirically assess the parameter values for the different effects"
    edge_type: "defines"
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p164:0212"
    chunk_hash: "f28c0f63d1bb3c8f412a714f5f6e0cfa621c48c2e24b63852468f7f6d4e9954e"
    page_range: [165, 165]
    quote: "it does not persist over time or cumulate"
    edge_type: "supports"
card_hash: "72c08e52a5cddc68f661d6388c1684b4e040b25321cd1d75420ecdcefe630fe4"
---
# Price Impact: Temporary vs Permanent and the Empirics of Walking the LOB

## Intuition

When you submit a marketable order, you pay for it twice. First, you pay an
*immediate, mechanical* cost: a market order of size `Q` eats through resting
limit orders at progressively worse prices, so the volume-weighted execution
price is strictly worse than the prevailing midprice. The moment your order is
done, that gap closes — the book refills and the midprice is roughly where it
was. This recoverable slice is the **temporary** (transient) price impact.
Second, your order *teaches the market something*. Other participants infer that
a buyer who paid up may know something, so they revise quotes upward; the
midprice itself drifts and stays there even after you stop trading. This
non-recoverable slice is the **permanent** (informational) price impact.

```
 price
   ^                      temporary impact (recovers)
   |                      |<------>|
   |        exec price  --o        .
   |                     /|        .
   |  new midprice  ----+-|--------+----  <- permanent shift (stays)
   |                  ^ | |
   |  old midprice  --+-+-+-------------
   |                  | informational
   |                  | revision
   +------------------+----+---------------> time
        buy MO arrives    MO done, book refills
```

Empirically, Cartea, Jaimungal, and Penalva separate the two: the permanent
component is read off how *net order flow* over five-minute windows moves the
midprice, while the temporary component is read off the cost of *walking the
limit order book* with hypothetical marketable orders of various sizes.

**Source:** Cartea, Jaimungal & Penalva (2015) §4.3.5 pp.104-105.

## Definition

Hasbrouck formalizes the split with two equations. Let `s_t` be the signed size
of the trade in period `t`, `m_t` the quote midpoint, and `p_t` the trade price.

- **Permanent impact** enters the midprice law of motion:
  `m_t = m_{t-1} + μ + λ s_t + ε_t`, where `λ s_t` is the permanent price
  impact, `μ` is drift, and `ε_t` is an innovation uncorrelated with our trades.
  Because it enters the level of `m_t`, it **cumulates** across successive
  trades and persists.
- **Temporary impact** is a *discrepancy between the midpoint and the trade
  price*: `p_t = m_t + γ s_t`. The `γ s_t` term is transient because it neither
  persists over time nor cumulates over multiple trades.

In the execution-model language of Cartea et al., the same dichotomy appears as
two functions of the trading rate `v_t`: a temporary-impact function `f(v_t)`
acting only on the agent's execution price, and a permanent-impact function
`g(v_t)` that moves the midprice itself (often modeled linearly as
`f(v) = k v`, `g(v) = b v`).

**Source:** Hasbrouck (2007) §15.1 pp.165-166; Cartea, Jaimungal & Penalva (2015) §6.5 p.154.

## Mathematical Reasoning

Take the linear specifications. Walking the buy side of the book, the per-share
execution cost above the best ask is modeled as `s^{exec,ask} = s^{ask} + k Q`,
so a least-squares regression of execution-price-minus-best-quote on traded
volume `Q` recovers the temporary slope `k`. Independently, regressing the
five-minute midprice change `ΔS_n` on net order flow `μ_n` recovers the
permanent slope `b`. Two structural consequences follow:

- **Cumulation asymmetry.** Since permanent impact enters `m_t` additively
  (`m_t = m_{t-1} + λ s_t + ...`), splitting one parent order of size `s̄` into
  `N` children leaves total permanent cost `λ s̄` *unchanged* — it telescopes.
  Temporary impact `γ s_t` (or `k Q`) is paid afresh on each child and, being
  convex in size for any single fill, is *reduced* by spreading the order over
  time. This is precisely why optimal-execution schedules trade slowly: they
  shrink the temporary bill while the permanent bill is invariant to scheduling.

- **The ratio `b/k` indexes depth.** A deep market refills quickly and reveals
  little, so both `b` and `k` are small; a thin market shows large impact of
  both kinds. Empirically `b` and `k` move together across days (positive
  correlation), and the ratio `b/k` for the sample ranges roughly 1 to 4,
  symmetric around about 2.5 — consistent with the theoretical link between
  price impact and depth.

No worked arithmetic is needed: the comparative statics (telescoping permanent
cost, convex temporary cost, `b/k` rising as depth falls) carry the argument.

**Source:** Cartea, Jaimungal & Penalva (2015) §4.3.5 pp.104-106.

## Boundary Notes

- **Whose information?** Hasbrouck stresses the permanent component is
  informational only relative to a *particular* agent. An uninformed buyer who
  cannot alter the firm's cash flows should expect *no* permanent effect on the
  security's value; the persistence she observes is others' inference, not her
  own information. The decomposition is therefore perspective-dependent.
- **Identification rests on observing the reversal.** Under Hasbrouck's
  decomposition `p_t = m_t + s_t`, the **permanent** impact is the piece
  impounded into the random-walk efficient price `m_t` that persists in the
  long run, while the **temporary** impact is the covariance-stationary pricing
  error `s_t` that subsequently *reverses*. Separating the two therefore relies
  on observing that reversal within the estimation window: it is the transient
  component that reverts toward `m_t`, leaving the persisting (random-walk)
  response as the permanent part. The random-walk variance is identified, but
  the pricing-error variance has only a lower bound (no upper bound) — so a
  move that looks permanent over a short window can prove transitory once more
  of the reversal is seen.
- **Linearity is an approximation.** The LOB temporary-impact curve is better
  described by a power law; linear `f(v) = k v` is a tractable local fit that
  fluctuates intraday (largest in the morning, flat midday). Permanent
  linearity (`ΔS` linear in net flow) holds across a wide range but fails in the
  extreme tails of price changes, where observations are sparse.
- **Block vs continuous.** For a single-block liquidation, a *transient* pressure
  that subsides after the agent stops trading yields her no benefit, so such
  models often fold it into the temporary term and exclude post-execution price
  correction.

**Source:** Hasbrouck (2007) §15.1 pp.165-166; Cartea, Jaimungal & Penalva (2015) §4.3.5 pp.105-106.

## See Also

- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) -- Kyle's λ is the permanent (informational) impact slope in a structural equilibrium.
- [`mt-almgren-chriss-optimal-execution`](./mt-almgren-chriss-optimal-execution.md) -- uses the temporary-vs-permanent split to schedule trading and trade off cost against risk.
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- VAR of trades and quotes estimates the long-run (permanent) impact of order flow.
- [`mt-market-impact-price-concession`](./mt-market-impact-price-concession.md) -- the broader market-impact / price-concession framing this decomposition sits inside.

## Escalate to Raw When

Cartea, Jaimungal & Penalva (2015) §4.3.5 (pp.104-106) gives the exact
regression specifications, the Winsorisation/half-hour-trimming estimation
protocol, the NASDAQ parameter table, and the intraday `k`-slope and `b/k`
figures — re-read there for the empirical procedure. Hasbrouck (2007) §15.1
(pp.165-166) and §5.6 develop the structural permanent/temporary equations and
the random-walk (permanent/transitory) decomposition that justify treating the
permanent component as the efficient-price innovation; re-read for the
identification argument and the static-optimization solution this card only
gestures at.
