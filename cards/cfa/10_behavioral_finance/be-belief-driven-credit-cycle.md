---
schema_version: "cacg.v0"
id: "be-belief-driven-credit-cycle"
title: "Belief-Driven Credit Cycle"
reading_id: "10_behavioral_finance"
summary: "Diagnostic beliefs generate credit cycles: good news breeds over-optimism that compresses spreads and over-expands risky-debt issuance, which then systematically and predictably reverses as optimism cools -- yielding excess volatility, mean reversion stronger than fundamentals, and low forward returns on the junk share."
tags: ["behavioral-finance", "credit-cycles", "diagnostic-expectations", "junk-share"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p192:0183"
    chunk_hash: "efc38a0dc5d0e86f63c42fb040e50f86aecdad7a20d34519776fc3bba76e0121"
    page_range: [193, 193]
    quote: "Under diagnostic expectations, with θ > 0, the issuance of risky debt overreacts to news."
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p194:0185"
    chunk_hash: "3d55ca040e42a109d2303c5d3cfa15bf1d337aedd1bde26b11f09f24fd3f7945"
    page_range: [194, 194]
    quote: "It displays stronger reversals than fundamentals"
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p197:0188"
    chunk_hash: "18185127844c24622e37379b4f041985db8405e2463d473e1f99af953bf5079d"
    page_range: [197, 197]
    quote: "in good times debt expands, the junk share is high, and the average return going forward is low."
    edge_type: "supports"
card_hash: "76e281a6ac125cca70fdc28143e5a91fcb1f16d3faeee09a4cc05e47e7c2952d"
---
# Belief-Driven Credit Cycle

## Intuition

Embedding the diagnostic operator in a repeated economy of borrowers and savers turns boom-bust credit cycles into an output of beliefs rather than of exogenous risk-premium shocks. As economic conditions improve, high future cash flows become representative in savers' minds, their probability is exaggerated and the risk of low cash flows neglected, so savers willingly fund excessive issuance of risky ("junk") debt and spreads compress. When bad news arrives, low cash flows become representative, exaggerating perceived risk and choking off issuance. The boom is thus mechanically linked to its own demise: optimism that over-expands credit predictably cools, producing a bust without any new fundamental shock.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.180, 182.

This supplies a theory of *where* credit-supply shocks come from -- a question reduced-form risk-premium models leave open. It matches the documented credit-cycle facts: rapid credit expansions and low spreads / high junk share forecast declines in economic activity and low forward bond returns (Greenwood-Hanson 2013; Lopez-Salido-Stein-Zakrajsek 2017; Mian-Sufi-Verner 2017). Because diagnostic beliefs over-react then systematically revert, the junk share negatively predicts realized returns: in good times debt expands, the junk share is high, and average forward returns are low; in bad times the reverse.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.168, 182, 184.

## Definition

**Junk share** is the share of risky debt in total debt issuance; with constant safe issuance `s`, its dynamics follow those of risky-debt issuance `N^theta_t`.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.180.

**Preferred-habitat risk threshold** `delta*` is the default probability up to which risk-neutral savers value debt; risky borrowers issue until default probability reaches `delta*`, pinning the quantity of risky debt.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.178-179.

**Predictable reversal** is the systematic correction of past news: high issuance after good news at `t-1` is followed by a contraction at `t` reflecting the cooling of current optimism, not future bad news.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.181.

## Mathematical Reasoning

With risky log cash flow following an AR(1), the diagnostic issuance condition `ln N^theta_t = E^theta_t(X_{t+1}) + sigma*z*` yields

```
  ln N^theta_t = rho*X_t + rho*theta*(X_t - rho*X_{t-1}) + sigma*z*,
```

so issuance overreacts to current news (good news -> too much risky debt; bad news -> too little). Solving the law of motion gives an ARMA(1,1),

```
  ln N^theta_t = (1 - rho)*sigma*z* + rho*ln N^theta_{t-1} + rho*(1+theta)*eps_t - rho^2*theta*eps_{t-1}.
```

The `+rho*(1+theta)*eps_t` term is overreaction to current news; the `-rho^2*theta*eps_{t-1}` term is the systematic reversal of past news. Under RE (`theta = 0`) issuance is a pure AR(1) tracking fundamentals.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.179-180.

Proposition 6.2 collects the cycle properties for `theta > 0`:

```
  (i)   expands too much after good news:  cov(ln N^theta_t, eps_t) = rho*(1+theta)*sigma^2 > 0
  (ii)  stronger reversals than fundamentals:
          cov(ln N^theta_t, ln N^theta_{t-1})/var(.) = rho - rho(1+theta)theta(1-rho^2)/[1+(1-rho^2)theta(2+theta)]
  (iii) excessively volatile:  var(ln N^theta_t) = rho^2*sigma^2*[1+(1-rho^2)theta(2+theta)]/(1-rho^2)
  (iv)  predictable forecast errors: too optimistic after good news, too pessimistic after bad news.
```

Mean reversion is stronger than under RE because expectation errors are corrected on average, adding to fundamental mean reversion. Proposition 6.3 closes the loop on returns: after positive news the average realized return is *below* the riskless rate `beta_h^{-1}` (savers were too optimistic and are disappointed); after negative news it is above -- matching the junk-share-predicts-low-returns finding.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.181, 184.

## See Also

- [be-diagnostic-expectations](./be-diagnostic-expectations.md#mathematical-reasoning) -- the AR(1) operator that drives issuance.
- [be-rational-vs-diagnostic-expectations](./be-rational-vs-diagnostic-expectations.md#mathematical-reasoning) -- the overreaction / predictable-error property underlying the cycle.
- [be-neglected-tail-risk](./be-neglected-tail-risk.md#intuition) -- the fragility built up during the credit boom.
- [be-financial-crisis-narrative](./be-financial-crisis-narrative.md#intuition) -- the 2008 episode as the most dramatic belief-driven credit cycle.

## Escalate to Raw When

- You need the borrower/saver setup, discount factors `beta_l < beta_h`, or the `delta*`-issuance integral condition (pp.177-179).
- You need the proofs of Propositions 6.2 / 6.3 or the spread law of motion `phi_t = rho*phi_{t-1} + rho^2*theta*eps_{t-1} - (rho(1+theta)-1)*eps_t` (pp.181-186, Appendix).
- You need the credit-spread (concave-utility) extension and the Euler equations (6.9)-(6.10) (pp.185-186).
