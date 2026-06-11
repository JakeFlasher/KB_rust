---
schema_version: "cacg.v0"
id: "gbj-base-position-protected-selling"
title: "Never lose the 底倉: base-position-protected, fully collateralized option selling around fair value"
reading_id: "07_derivatives_and_volatility"
summary: "He sells calls and puts only around a base position (底倉) he refuses to surrender below his own fair-value estimate, rolling strikes to harvest premium (Tencent 330→380 puts under a 400 ceiling). He calls his own sell-calls 半裸 (half-naked), not covered, and warns ordinary investors off derivatives: 100% of the underlying before any sell-call, full cash before any sell-put, no leverage."
tags: ["xueqiu-2022h1", "dated-levels", "options", "sell-put", "sell-call", "premium-rolling", "risk-discipline", "hkex-stock-options"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p461:0460"
    chunk_hash: "597cbf4b8759541c5eeba850da27a292a05486b70012a743e2d53873882a096f"
    page_range: [461, 461]
    quote: "我sell call sell put的原則是不弄丟底倉，至少在我認為的合理價以下不弄丟。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p249:0248"
    chunk_hash: "886a927571e75c337a0ce3371e5a21452e85955e2558af46a532ee5e2167a63b"
    page_range: [249, 249]
    quote: "我是打算400元之前盡量不弄丟底倉，所以才會這麼做。"
    edge_type: "supports"
  - source_id: "hkex_stock_options_corner_guide"
    chunk_id: "hkex_stock_options_corner_guide:p014:0014"
    chunk_hash: "a88f45762c2550ed35579540809ed68a8b72982d3667229e3b81de5ecbdf88fd"
    page_range: [14, 15]
    quote: "Investors owning the underlying stock of the derived stock option may use shares of the underlying stock to replace cash for the margin of writing the call"
    edge_type: "supports"
card_hash: "d9185ecc36c5e5d652db94d8823ca98d4740637832c3f87a072e60d518664a01"
---

## Dated State

All specific levels below come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. Dated levels in this card: Tencent (00700) — a 400 fair-value ceiling, a June 2022 roll of a 330 sell-put into a 380 sell-put (12 of premium, 27 cumulative across the sequence), and a 350 sell-call already written; CNOOC (00883) — sell-call around 12 and sell-put around 10.

## Thesis

His option program is selling calls and puts against a core stake (底倉) he refuses to surrender below his own estimate of fair value, only letting shares go above his target (e.g. Tencent 400; CNOOC sell-call ~12, sell-put ~10). He rolls puts/calls to harvest premium while keeping the base position, on names he wants to stay invested in (不掉隊). Crucially, he frames this NOT as conventional covered-call writing — he describes his own sell-calls as '半裸' (half-naked), since he sells calls against shares he plans to acquire through put assignment rather than already-held stock. He attaches explicit risk caveats the card must preserve: he opposes ordinary investors touching these derivatives at all, and his hard rule is never sell a call without 100% of the underlying and never sell a put without the full cash to take delivery ('會死人㗎'), with no leverage as the bottom line. Best titled 'base-position-protected option selling around fair value' rather than 'covered'.

## The Mechanic: Never Lose the 底倉 Below Your Own Fair Value

The whole program hangs on one rule, stated when a reader objected that rolling a Tencent put up from 330 to 380 just courts a bigger loss on assignment: 「我sell call sell put的原則是不弄丟底倉，至少在我認為的合理價以下不弄丟。」 — "My principle for selling calls and selling puts is to not lose the base position; at the very least, not to lose it below what I judge to be fair value." Strikes are not chosen to maximize premium; they are chosen so that every outcome — assignment, expiry, or roll — leaves him still holding (or adding to) names he wants to stay invested in, his 不掉隊 ("don't fall behind") concern.

The roll itself is mechanical: buy back the lower-strike put, immediately write a higher strike, and let the premiums stack. On the same Tencent sequence he explained: 「我是打算400元之前盡量不弄丟底倉，所以才會這麼做。」 — "My plan is to do my best not to lose the base position before 400; that is why I am doing it this way." Willingness to part with shares is bounded by his own valuation: below his number, the short calls and puts are positioned so the stake survives; only above the target (400 for Tencent) is he content to let stock go.

He also rejected the textbook label for the call side. Asked whether he held the stock behind his short calls or was writing them naked, he answered 「Sell call算是半裸吧？」 — "the sell-call counts as half-naked, I suppose" — because part of his call-writing was against shares he expected to receive through pending put assignment, not stock already settled in his account. The accurate description is therefore base-position-protected, fully collateralized option selling around a fair-value estimate: the backing is the held 底倉 plus the full cash already earmarked for put delivery, never an uncovered exposure.

## The Risk Spine

He attaches the caveats himself, and they are absolute. First, the gate: 「本人一向反對普通投資者觸碰任何的衍生工具，即便是sell call和sell put」 — "I have always opposed ordinary investors touching any derivative instruments, even sell call and sell put." Second, the hard preconditions: 「手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！」 — "Without 100% of the underlying shares in hand, absolutely never sell a call; by the same logic, without the full cash prepared to take delivery, never sell a put. It will kill you!" Third, the financing rule: 「不用杠杆是底線，忌貪！」 — "No leverage is the bottom line; guard against greed!" Hong Kong market mechanics make the fully collateralized call side operational: HKEX's stock-options guidance notes that investors owning the underlying stock "may use shares of the underlying stock to replace cash for the margin of writing the call", so holding 100% of the shares is not just his discipline — it is recognized collateral at the exchange.

## See Also

For the payoff anatomy of the short-call and short-put legs, see deriv-option-payoff-anatomy; for the fair-value estimate that bounds when he lets shares go, see eq-intrinsic-value. On HK mechanics: hk-covered-call-shares-as-margin-collateral (shares as margin for written calls), hk-stock-option-american-style-assignment and hk-short-call-assignment-settlement-timing (why a 半裸 short call can be assigned before the offsetting put delivers stock), hk-special-dividend-option-contract-adjustment (how special dividends re-strike HK option contracts), and equity-index-volatility-skew-negative for the premium environment that option sellers operate in.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p461:0460` — 狗不叫, author reply c244799468 (post 222436722), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p249:0248` — 狗不叫, author reply c244813218 (post 222375639), verbatim ★AUTHOR words
- `hkex_stock_options_corner_guide:p014:0014` — grounding snapshot `hkex_stock_options_corner_guide`
