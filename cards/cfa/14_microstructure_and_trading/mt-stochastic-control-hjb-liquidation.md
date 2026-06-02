---
schema_version: "cacg.v0"
id: "mt-stochastic-control-hjb-liquidation"
title: "Stochastic Optimal Control for Trading: The Dynamic Programming / HJB Equation"
reading_id: "14_microstructure_and_trading"
summary: "Optimal execution and market-making are stochastic control problems whose value function satisfies the Dynamic Programming Principle and a Hamilton-Jacobi-Bellman PDE, yielding the optimal control in feedback form."
tags: ["microstructure", "stochastic-control", "hjb", "dynamic-programming", "optimal-execution"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p126:0168"
    chunk_hash: "f31fb900209ae16e0eec17c798fe706dfd61c4e2aa74c1d3f40f608603e1cd75"
    page_range: [127, 127]
    quote: "the optimal control can often be found in feedback control form in terms of the value function itself."
    edge_type: "defines"
card_hash: "dc77561a6c33c182ea2c294fda0a689a2c137bdf0532fbbb75445d476c928c84"
---
# Stochastic Optimal Control for Trading: The Dynamic Programming / HJB Equation

## Intuition

A trader who must unwind a large position, or quote two-sided prices, is never
choosing a single number — she is choosing a *strategy*: a rule that maps the
current state of the world (time remaining, inventory left, current price) into
an action (how fast to sell, how deep to post). The hard part is that her own
actions feed back into the system: trading faster moves the price against her,
holding inventory exposes her to risk. Stochastic optimal control is the
machinery for choosing the best such rule when the future is random and the
agent's choices shape the dynamics she faces.

The central object is the **value function** `H(t, x)` — the best expected
reward attainable from state `x` at time `t`, optimised over all admissible
strategies. The key trick is not to optimise the whole strategy at once, but to
embed the problem in a family of sub-problems indexed by start time, and exploit
the fact that an optimal strategy must remain optimal on every remaining
sub-interval. This is the **Dynamic Programming Principle (DPP)**: solve from
the terminal date backwards.

```
   today t ............ small step h ............ horizon T
   state x ---- act u over [t, t+h] ----> X_{t+h}
       |                                      |
       | running reward F(s, X, u) ds         | value of acting OPTIMALLY
       |   accrued over [t, t+h]              | from X_{t+h} onward = H(t+h, .)
       +----------------- DPP ----------------+
   H(t,x) = sup_u E[ running reward over step  +  H(t+h, X_{t+h}) ]
                         |
                 let h -> 0  (infinitesimal version)
                         v
   HJB / DPE:  d_t H + sup_u ( L^u H + F ) = 0,   H(T, x) = G(x)
```

Shrinking the step `h` to zero turns this recursion into a single nonlinear PDE
— the Hamilton-Jacobi-Bellman (HJB) equation — whose pointwise maximisation
recovers the optimal action *as a function of the current state*, i.e. in
feedback form.

**Source:** Cartea, Jaimungal & Penalva (2015) Ch.5 §5.1–5.3 pp.118–127.

## Definition

Let the controlled state `X^u` follow an Itô diffusion whose drift and
volatility depend on the control `u`. For an admissible control `u`, define the
**performance criterion**

  `H^u(t, x) = E_{t,x}[ G(X^u_T) + ∫_t^T F(s, X^u_s, u_s) ds ]`,

where `G` is the terminal reward and `F` the running reward/penalty. The
**value function** is the supremum over the admissible set `A_{t,T}`:

  `H(t, x) = sup_{u ∈ A_{t,T}} H^u(t, x)`.

Three canonical examples in the source share this template:
- **Merton problem** — maximise expected utility of terminal wealth `U(X_T)`;
  the agent's trades affect only her wealth, not the asset price.
- **Optimal liquidation** — sell inventory `Q` by horizon `T`; the control is
  the liquidation rate `v`, with permanent impact `g(v)` on the fundamental
  price and temporary impact `h(v)` on the execution price, plus a running
  inventory penalty `φ ∫ (Q_s)^2 ds`.
- **Optimal limit-order placement** — choose posting depth `δ` in the LOB,
  trading off fill probability against price improvement.

**Source:** Cartea, Jaimungal & Penalva (2015) §5.2–5.3 pp.101–106.

## Mathematical Reasoning

The DPP is proved by two-sided bounding of the value function. Using iterated
expectations, for any admissible `u` and stopping time `τ ≤ T`,

  `H^u(t,x) = E_{t,x}[ H^u(τ, X^u_τ) + ∫_t^τ F(s, X^u_s, u_s) ds ]`.

Since `H ≥ H^u` pointwise, replacing the future performance criterion by the
value function and taking the supremum gives the upper inequality
`H(t,x) ≤ sup_u E_{t,x}[ H(τ, X^u_τ) + ∫_t^τ F ds ]`. The reverse inequality is
obtained with an ε-optimal control that is optimal after `τ` but arbitrary
before it. Together they yield the DPP (Theorem 5.1):

  `H(t,x) = sup_{u ∈ A} E_{t,x}[ H(τ, X^u_τ) + ∫_t^τ F(s, X^u_s, u_s) ds ]`.

To pass to the infinitesimal version, take `τ` as the minimum of a small fixed
time `h` and the first exit from an ε-ball, apply Itô's lemma to write
`H(τ, X^u_τ)` in terms of `H(t,x)` plus integrated increments. The stochastic
integral has zero expectation (its integrand is bounded inside the ball), so
dividing by `h` and letting `h → 0` gives, for any constant control `v`,
`∂_t H + L^v H + F(t,x,v) ≤ 0` after taking the supremum, and equality along the
optimal `u*`. The result is the **HJB / DPE**:

  `∂_t H(t,x) + sup_{u ∈ A} ( L^u H(t,x) + F(t,x,u) ) = 0,   H(T,x) = G(x)`,

where `L^u` is the infinitesimal generator of `X^u`. Crucially the supremum is
over the control's *value at time t only*, not over whole paths — so the
optimiser can be found pointwise, giving the optimal control in **feedback
form** as a function of `H` and its derivatives. The inner optimisation defines
the **Hamiltonian** `𝓗(t, x, D_x H, D²_x H) = sup_u (L^u H + F)`; substituting
the feedback maximiser back into the DPE produces a nonlinear PDE for `H`.

For the Merton example the Hamiltonian is quadratic in the position `π`, so
completing the square gives `π*` in closed feedback form and reduces the HJB to
a nonlinear PDE in `h(t,x)` driven by the market price of risk `λ = (μ−r)/σ`.

**Source:** Cartea, Jaimungal & Penalva (2015) §5.3.1–5.3.2 pp.105–111.

## Boundary Notes

- The HJB derivation here is *pragmatic*: it assumes a **classical solution**
  (once differentiable in `t`, twice in the diffusive states) so the generator
  applies; the verification argument that such a solution is the true value
  function, and viscosity-solution machinery when smoothness fails, are
  deferred to the cited theory texts (Yong & Zhou; Fleming & Soner; Pham).
- The admissible set encodes economically meaningful constraints — e.g. the
  Merton set excludes doubling strategies; the liquidation set forces
  non-negative bounded rates (no repurchasing, finite trade rate).
- The Merton problem assumes the agent's trades do *not* move the asset price —
  reasonable on long horizons but exactly the assumption that breaks for large
  fast orders, which is why optimal-execution problems add permanent/temporary
  impact terms `g`, `h` to the dynamics.
- The "optimal control is Markov in the state" conclusion is a *consequence* of
  the DPP feedback structure, even when the agent contemplated history-dependent
  controls; it is not assumed.

**Source:** Cartea, Jaimungal & Penalva (2015) §5.1–5.3 pp.101–111.

## See Also

- [`mt-almgren-chriss-optimal-execution`](./mt-almgren-chriss-optimal-execution.md) — a concrete liquidation control problem solvable within this HJB framework.
- [`mt-avellaneda-stoikov-market-making`](./mt-avellaneda-stoikov-market-making.md) — quote-setting cast as stochastic control with an HJB equation for the value function.
- [`mt-vwap-pov-volume-targeting`](./mt-vwap-pov-volume-targeting.md) — scheduling strategies that approximate or constrain the optimal control.

## Escalate to Raw When

The source *proves* the DPP via the ε-optimal-control bounding argument
(Theorem 5.1, pp.106–107) and *derives* the HJB equation as its infinitesimal
limit (eq. 5.19, pp.108–110) — this card only sketches both. Re-read §5.3.1 for
the two-sided inequality construction, §5.3.2 for the Itô/Mean-Value-Theorem
steps that kill the stochastic integral, and the Merton example (pp.110–111) for
how the feedback maximiser is recovered by completing the square. For the
existence/verification theory the card deliberately omits, follow the source's
pointers to Yong & Zhou (1999), Fleming & Soner (2006), and Pham (2010).
