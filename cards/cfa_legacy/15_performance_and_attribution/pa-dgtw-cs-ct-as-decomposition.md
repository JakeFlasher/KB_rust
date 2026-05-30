---
schema_version: "cacg.v0"
id: "pa-dgtw-cs-ct-as-decomposition"
title: "The DGTW Characteristic-Based Decomposition (CS/CT/AS)"
reading_id: "15_performance_and_attribution"
summary: "Daniel-Grinblatt-Titman-Wermers benchmarks each held stock against one of 125 size x book-to-market x prior-return fractile portfolios, then splits gross holdings return into characteristic selectivity (CS), characteristic timing (CT), and average style (AS): GR = CS + CT + AS."
tags: ["dgtw", "holdings-based-attribution", "characteristic-benchmark"]
citations:
  - source_id: "pa_fischer_wermers_2013"
    chunk_id: "pa_fischer_wermers_2013:p114:0141"
    chunk_hash: "114b37f6d8b5a7d13282c618ba20697b6e78ee720d4bfa38df8d7286f8e7b7a2"
    page_range: [114, 114]
    quote: "Recent research has shown that mutual funds show a distinct preference for other stock characteristics that are related to average returns— for example, stocks with greater liquidity (see Chen et al., 2000 and Wermers, 2000).19 For example, one might argue that our CS measure underestimates the stock-picking talents of funds since we do not control for the lower average returns that accrue to stocks with greater liquidity."
    edge_type: "defines"
  - source_id: "pa_connor_goldberg_korajczyk_2010"
    chunk_id: "pa_connor_goldberg_korajczyk_2010:p332:0400"
    chunk_hash: "0a784d821a61538e7dd9988fd8c67cd5952416308bca4f3c72ad899d5bc4a36c"
    page_range: [333, 333]
    quote: "The three measures have an additive consistency property that their sum equals the excess return to the managed portfolio."
    edge_type: "supports"
---
# The DGTW Characteristic-Based Decomposition (CS/CT/AS)

## Intuition

A returns-based alpha asks "did this fund beat a factor regression?" A
holdings-based alpha asks the sharper question "for each stock the manager
actually held, did it beat *other stocks that look just like it*?" The
Daniel-Grinblatt-Titman-Wermers (DGTW) approach builds a personalized yardstick
per stock: take its size, its book-to-market ratio, and its prior-year return,
and drop it into the matching bucket of a 5 x 5 x 5 = 125-cell grid of
value-weighted fractile portfolios. The stock's "characteristic-adjusted return"
is then how much it beat its own bucket — not how much it beat the market.
Summed over the fund's holdings, this isolates stock-picking skill *within* a
style from the return the fund earned simply by *being* in that style. Because
the matched bucket already absorbs the size/value/momentum tilts, what remains is
cleaner evidence of selection than a factor regression leaves behind.

**Source:** Fischer & Wermers (2013) §4.2.2.1 pp.108-110 (PDF pp.113-115)

## Definition

DGTW assigns each stock, at the start of each quarter, to one of **125 fractile
portfolios**. The grid is built by sequential ranking at the end of each June:
first into NYSE-breakpoint size quintiles, each size quintile then into
book-to-market quintiles, and each of those 25 cells then into prior-year-return
(momentum) quintiles — so the "ranking procedure results in 125 fractile
portfolios, each having a distinct combination of size, book-to-market, and
momentum characteristics." Each stock's **characteristic benchmark** is the
value-weighted buy-and-hold return of the fractile portfolio it belongs to that
month.

The fund's gross (pre-cost, pre-expense) holdings return then decomposes into
three additive month-t components:

- **Characteristic Selectivity (CS)** — current portfolio-weighted stock return
  in excess of its matched characteristic-benchmark return. This is
  **stock-picking ability**, controlled for style.
- **Characteristic Timing (CT)** — current-weight x current-benchmark return
  minus lagged-weight x lagged-benchmark return. This captures the manager's
  ability to **tilt weights toward characteristics just before those
  characteristics pay off** (style timing).
- **Average Style (AS)** — lagged-weight x current-benchmark return. This is the
  return earned simply from the fund's **persistent tendency to hold stocks with
  certain characteristics**, stripped of any timing skill (by lagging both
  weights and benchmarks by k periods).

**Source:** Fischer & Wermers (2013) §§4.2.2.1-4.2.2.3 pp.108-119 (PDF pp.113-124)

## Mathematical Reasoning

Write `w(j,t-1)` for the start-of-month portfolio weight on stock j, `R(j,t)` for
stock j's month-t buy-and-hold return, and `Rb(j,t-1)` for the month-t return of
the characteristic-matched fractile portfolio j was assigned to at t-1. The three
components are sums over the n holdings:

```
CS_t = SUM_j  w(j,t-1) * [ R(j,t) - Rb(j,t-1) ]
CT_t = SUM_j  [ w(j,t-1)*Rb(j,t-1) - w(j,t-k-1)*Rb(j,t-k-1) ]
AS_t = SUM_j  w(j,t-k-1) * Rb(j,t-k-1)
```

The decomposition is a telescoping identity. CT carries a `+w(j,t-1)*Rb(j,t-1)`
term and AS supplies, with offsetting and matching lagged pieces, the structure
that lets the benchmark-return contributions cancel cleanly, so the three sum to
the total portfolio-weighted benchmark-plus-selectivity return:

```
GR_t = CS_t + CT_t + AS_t.
```

DGTW assert this additive identity; Fischer & Wermers note it is only
*approximately* exact in practice, because a stock must satisfy data
requirements (e.g., be listed in COMPUSTAT) to enter the CS/CT/AS calculation —
the card asserts the identity at the source's stated level and flags this caveat
rather than proving exactness. The independent confirmation is that Connor,
Goldberg & Korajczyk restate the same Daniel-et-al. decomposition and likewise
report that "[t]he three measures have an additive consistency property that
their sum equals the excess return to the managed portfolio."

**Sources:** Fischer & Wermers (2013) Eqs. 4.7-4.10 §4.2.2.4 pp.109-119 (PDF pp.114-124); Connor, Goldberg & Korajczyk (2010) §14.2.1 p.308 (PDF p.333)

The deeper foundation for *why* holdings skill shows up as a weight-return
interaction is the covariance decomposition of expected portfolio return. For
covariance-stationary weights and returns, `E[w*R] = E[w]*E[R] + cov(w,R)`, so

```
E[r_p] = SUM_j  cov(w_j, R_j)  +  SUM_j  E[w_j]*E[R_j]
              \---- active ----/      \---- passive ----/
```

The **active** term `cov(w_j, R_j)` measures the dynamic comovement between the
weights a manager chooses at the start of a period and the returns realized at
its end — a positive covariance is exactly the signature of informed weighting.
A passive (e.g., value-weighted index) investment has an active component of
zero. CS and CT are the characteristic-benchmark realizations of this same
"weights move with returns" idea; AS is the style-loading piece that survives
even with zero active covariance.

**Source:** Connor, Goldberg & Korajczyk (2010) §14.2.2 Eq. 14.14 p.308 (PDF p.333)

```
 DGTW 125-cell grid              decomposition of gross holdings return
 (sequential ranking)
                                  GR = CS + CT + AS
  SIZE quintile (5)                |    |    |
    +-> B/M quintile (5)           |    |    +-- AS: persistent style loading
         +-> prior-ret (5)         |    |        (lagged w x lagged benchmark)
              = 125 fractiles      |    +------- CT: timing the characteristic
                                   |             (w x benchmark, current - lagged)
  each held stock -> one cell      +------------ CS: pick winners within a cell
  benchmark = VW return of cell                  (w x [stock - own benchmark])
```

**Source:** Fischer & Wermers (2013) §4.2.2 pp.108-110 (PDF pp.113-115)

## Boundary Notes

The CS measure controls for only three characteristic dimensions — size,
book-to-market, and prior return — so it can mis-state skill when funds tilt
toward priced characteristics outside the grid (e.g., liquidity); Fischer &
Wermers flag these as unresolved "missing factors," not as proven biases. DGTW is
the *characteristic-matched* alternative to the Carhart four-factor regression
alpha (RMRF, SMB, HML, UMD): Daniel-Titman argue characteristic benchmarks
control for mechanical value/momentum strategies more accurately than a
covariance-based factor model, since a value portfolio can post positive Carhart
alpha merely by having low covariance with HML. The factor-regression alpha
family lives in the sibling cards below.

**Source:** Fischer & Wermers (2013) §§4.2.2.1, 4.2.2.5 pp.108-121 (PDF pp.113-126)

## See Also

- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — the returns-based Carhart/Fama-French factor alpha that DGTW characteristic benchmarking is the holdings-based alternative to.
- [`pa-factor-model-types-and-covariance-decomposition.md`](pa-factor-model-types-and-covariance-decomposition.md) — fundamental/statistical factor models and the additive covariance-of-risk machinery behind the cov(w,R) foundation.
- [`pa-luck-vs-skill-fdr-and-bootstrap.md`](pa-luck-vs-skill-fdr-and-bootstrap.md) — once CS isolates a skill estimate, separating it from luck across many funds.
- [`pa-return-gap-kacperczyk-sialm-zheng.md`](pa-return-gap-kacperczyk-sialm-zheng.md) — the complementary holdings-vs-realized gap measure of unobserved (interim-trading) skill.

Related cross-vertical: pm-* portfolio-management cards develop the active-vs-passive return and SML/beta foundations these holdings measures sit on.

## Escalate to Raw When

- You need the worked Janus Twenty fund example (Tables 4.5-4.6) where DGTW
  benchmark returns and stock-level alphas are computed for April-June 1998 —
  deferred here per the no-worked-arithmetic rule.
- You need the precise reconstitution rules (NYSE breakpoints, the
  industry-normalized book-to-market, the non-June quarter-end handling, the ADR
  exclusion patch) for actually forming the 125 portfolios.
- You need the choice and justification of the timing lag k (DGTW use k = 12 for
  U.S. domestic equity funds) in the CT and AS equations.
- You need the full Carhart four-factor equation (Eq. 4.11) and the formal
  Daniel-Titman characteristics-vs-covariance comparison that motivates the
  DGTW approach over a factor regression.

**Source:** Fischer & Wermers (2013) §4.2.2 pp.108-121 (PDF pp.113-126)
