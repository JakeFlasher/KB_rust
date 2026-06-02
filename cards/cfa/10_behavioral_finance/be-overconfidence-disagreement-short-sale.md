---
schema_version: "cacg.v0"
id: "be-overconfidence-disagreement-short-sale"
title: "Disagreement with a Short-Sale Constraint"
reading_id: "10_behavioral_finance"
summary: "Miller's static and Harrison-Kreps / Scheinkman-Xiong dynamic mechanisms: when investors disagree (driven by overconfidence) and short sales are constrained, prices reflect only optimists, producing overvaluation; the dynamic resale option also generates speculative bubbles and very high trading volume."
tags: ["behavioral-finance", "overconfidence", "short-sale-constraint", "bubbles", "trading-volume"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p125:0190"
    chunk_hash: "29712839f743704531c8ab3f020e5f433b0ba0c3cee42538dd72ae1e0c304454"
    page_range: [125, 125]
    quote: "if short sales are not possible, the pessimists do not take a position in the asset. The price of the asset then reflects only the expectations of the optimists. As such, the asset is overpriced."
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p125:0190"
    chunk_hash: "29712839f743704531c8ab3f020e5f433b0ba0c3cee42538dd72ae1e0c304454"
    page_range: [125, 125]
    quote: "means that, when there is significant disagreement, the asset is overvalued relative to the present value of its future cash flows as perceived by the"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p126:0192"
    chunk_hash: "9ae9df948378b6991a55e899bc0081fe1571031398514eb0b06e6f549fd0828f"
    page_range: [126, 126]
    quote: "Scheinkman and Xiong (2003) show how overconfidence-based disagreement and a short-sale constraint can explain this coincidence of overvaluation and high volume."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p126:0192"
    chunk_hash: "9ae9df948378b6991a55e899bc0081fe1571031398514eb0b06e6f549fd0828f"
    page_range: [126, 126]
    quote: "this leads to larger fluctuations in the relative optimism of the two investor groups. This, in turn, leads to higher trading volume, but also greater"
    edge_type: "supports"
card_hash: "fb652e47a1fe17229b11d12f08dcf8aa47238c9d9554080e312b4a30a239b77e"
---
# Disagreement with a Short-Sale Constraint

## Intuition

A central behavioral framework couples overconfidence-driven disagreement with a specific friction: a constraint on short sales. The appeal is that it explains why an asset can become significantly overvalued -- and why overvaluation tends to come with heavy trading. There are two mechanisms, a static one (Miller, late 1970s) and a dynamic one (Harrison-Kreps; Scheinkman-Xiong). Both papers were largely ignored for two decades and were "discovered" only in the late 1990s when they turned out to explain the U.S. technology-stock bubble.
**Source:** Barberis (2018) §5.1 pp.125.

The static channel is simple. Suppose one group of investors is optimistic about an asset's cash flows and another is pessimistic. If short sales were possible, pessimists would short, and the price would reflect both groups' views -- no mispricing. But if short sales are not possible, the pessimists simply stay out of the asset. The price then reflects only the optimists, so the asset is overpriced.
**Source:** Barberis (2018) §5.1 pp.125.

The dynamic channel is a *resale option*. Even when there is a lot of disagreement, each holder is willing to pay more than his own valuation because he reasons that, after future news, some other investor may become more optimistic than he is, letting him resell at a premium. The short-sale constraint is essential: the only way to exploit another's future optimism is to buy today and resell later. This extra demand causes overvaluation, and the constant turnover as the identity of the most-optimistic group shifts generates very high trading volume -- exactly the volume-overvaluation co-movement seen in bubbles.
**Source:** Barberis (2018) §5.1 pp.125, 126.

## Definition

**Differences of opinion (disagreement)** is investors holding different beliefs about an asset's future cash flows; here the source of disagreement is overconfidence (each group overestimates the informativeness of its preferred signal).
**Source:** Barberis (2018) §5.1 pp.126.

**Short-sale constraint** is a friction preventing or limiting short positions, so pessimistic investors cannot impound their views by selling the asset short.
**Source:** Barberis (2018) §5.1 pp.125.

**Static overpricing mechanism (Miller)** is the result that, with binding short-sale constraints, the price reflects only optimists' expectations and the asset is overpriced.
**Source:** Barberis (2018) §5.1 pp.125.

**Resale option (Harrison-Kreps / Scheinkman-Xiong)** is the dynamic source of overvaluation: each holder values the asset above his own present value because he expects to be able to resell it to a more optimistic future buyer.
**Source:** Barberis (2018) §5.1 pp.125.

## Mathematical Reasoning

In the Scheinkman-Xiong setup there is a risky asset claiming a dividend stream with unobserved mean `f`. Two groups of risk-neutral investors, A and B, both observe two public signals `N_A` and `N_B` ("editorial pages of two newspapers"). Group A overestimates the informativeness of `N_A`; group B overestimates that of `N_B`. Each day investors update on both signals but each group puts more weight on its preferred one. Short sales are not allowed.
**Source:** Barberis (2018) §5.1 pp.126.

Because of the resale option, the equilibrium price exceeds the present value of future cash flows as perceived by *any* current holder: group A pays a premium because there is a chance group B will later turn more bullish (so A can sell to B at a premium), and group B reasons symmetrically. The short-sale constraint forces this optimism to be expressed by buying today rather than by shorting.
**Source:** Barberis (2018) §5.1 pp.125, 126.

The comparative statics tie overvaluation to volume: if investors become *more* overconfident, or signals `N_A, N_B` become more informative, the relative optimism of the two groups fluctuates more. Larger swings in relative optimism mean more frequent ownership turnover -- higher trading volume -- and a more valuable resale option -- greater overvaluation. A separate cross-sectional prediction (Diether et al.): assets people disagree about more (more dispersed analyst forecasts) are more overvalued and so earn lower average returns.
**Source:** Barberis (2018) §5.1 pp.126, 127.

## See Also

- [be-limits-of-arbitrage](./be-limits-of-arbitrage.md#intuition) -- short-sale constraints as one of the frictions that let mispricing survive.
- [be-overconfidence-self-attribution-prices](./be-overconfidence-self-attribution-prices.md#intuition) -- the other half of the overconfidence framework (momentum/reversal in prices).
- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) -- overconfidence as the volume-explaining framework.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the bubble and high-volume facts this framework targets.
- [be-experimental-asset-bubbles](./be-experimental-asset-bubbles.md#intuition) -- laboratory bubbles with disagreement and resale.

## Escalate to Raw When

- You need the contrast between the overconfidence, dismissiveness, and "cursedness" channels for generating volume (Eyster et al.) (pp.114-115).
- You need the different-priors model (Morris 1996) and the Milgrom-Stokey no-trade theorem that motivates why rational difference-of-information yields low volume (pp.114-115).
- You need Xiong (2013) Section IV's numerical example illustrating the resale-option logic (p.126, footnote 23).
