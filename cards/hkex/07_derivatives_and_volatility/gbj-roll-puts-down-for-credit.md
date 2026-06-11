---
schema_version: "cacg.v0"
id: "gbj-roll-puts-down-for-credit"
title: "Rolling a Cash-Secured Short Put Down to a Lower Strike for a Net Credit"
reading_id: "07_derivatives_and_volatility"
summary: "When a cash-secured short put on Tencent moved against him, the author rolled it down for a net credit: close the 360 strike at a loss, sell the 340 strike for more premium, improving breakeven and reducing the percentage-based HK commission and stamp duty due at assignment. Preconditions: full delivery cash, no margin; the trade-off is forgoing recovery above the new strike."
tags: ["xueqiu-2022h1", "dated-levels", "sell-put", "roll-down", "cash-secured", "tencent", "hong-kong", "stock-options"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3475:3474"
    chunk_hash: "588557ee7a4dfb974b033f3440d1a6f541338b844ea45a6920063d0177095858"
    page_range: [3475, 3475]
    quote: "平倉360元的sell put，虧9元每股，新開340元七月份sell put，收回11元。就這張期權而言，相當於騰訊七月份升回$340，能夠淨賺$2。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3480:3479"
    chunk_hash: "1d321ece74a3b6be63436cc0f2f729210a1e18da1ae052411e897d8a73538ec6"
    page_range: [3480, 3480]
    quote: "如果騰訊一直不回升，我按照$340接貨，無論交易佣金和印花稅都要比按照$360接貨少支付一些。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p6208:6207"
    chunk_hash: "9f19e760ce62c6db315866caf6612a865526c02fd64fb77dc26961d0490d2c21"
    page_range: [6208, 6208]
    quote: "如果騰訊反彈至$350，你會後悔的"
    edge_type: "supports"
  - source_id: "hkex_stock_options_info_sheet"
    chunk_id: "hkex_stock_options_info_sheet:p002:0002"
    chunk_hash: "98443f597118122d13b89eb3b783d3c52383fa6bed42bd3389168aa5bec04f81"
    page_range: [2, 2]
    quote: "Options can be exercised at any time up to 6:45 pm on any business day up to and including the last trading day"
    edge_type: "supports"
card_hash: "a35910c661a5fc11f5ebeeb2bf6b43d39c53557685a6cb3fdcc0d0ca87d7ddc0"
---

## Dated State

All specific levels in this card come from the author's 2022-H1 corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. The dated levels: Tencent (00700) July-2022 sell puts at strikes 360 and 350 (premiums HK$17.4 and HK$13) alongside a July 370 sell call (HK$13.88); the roll itself — close the July 360 put at −9 per share, open the July 340 put at +11, net +2; the imitator's 350-to-340 roll; and the roughly 0.3% HK exercise cost he quotes.

## Thesis

Holding a cash-secured short put that has gone against him, the author rolls it DOWN to a lower strike for a net credit: he closes the higher-strike put at a loss and opens a lower-strike put collecting more premium, so the position still nets a credit even if the stock only recovers to the lower strike (e.g. close the 360 put at -9, open the 340 put at +11 = net +2, which he frames as 相當於 Tencent recovering merely to $340 — a breakeven improvement, not an unconditional gain). A lower assignment strike also means paying less percentage-based HK commission and stamp duty if he is eventually assigned. Challenged that the roll's round-trip option fees and slippage erase the edge, he asserts these costs are within his calculation and the roll is still worthwhile, though he shows no figures. Critical precondition he insists on: sell or roll puts only with the full cash ready to take delivery, never on margin, and only on names he is willing to own. Acknowledged downside: if the stock rebounds past the original strike he forgoes that higher-strike recovery — he warns an imitator who rolled 350 down to 340, '如果騰訊反彈至$350，你會後悔的'.

## The roll-down: realize a loss, collect more premium, lower the breakeven

In July 2022 the author described the maneuver on his own Tencent position: "平倉360元的sell put，虧9元每股，新開340元七月份sell put，收回11元。就這張期權而言，相當於騰訊七月份升回$340，能夠淨賺$2。" — he closed the July 360 sell put at a loss of $9 per share and opened a July 340 sell put collecting $11, so this contract pair is "equivalent to" Tencent merely recovering to $340 by July while still netting $2. The hedge in his own wording (相當於, "equivalent to") is load-bearing: the $2 is a breakeven improvement conditional on the stock recovering to the lower strike, not an unconditional gain. When a skeptic objected that the round-trip 手续费和滑点 (fees and slippage) of closing one option and opening another would erase the edge, he replied "這些成本都在我考慮之內，還是划算" — these costs are within my consideration, and it is still worthwhile — but he produced no figures, so the cost netting should be read as his asserted judgment, not a documented computation.

## Why the lower strike is also cheaper at assignment in Hong Kong

He gave a second, distinctly HK reason for preferring the lower strike: "如果騰訊一直不回升，我按照$340接貨，無論交易佣金和印花稅都要比按照$360接貨少支付一些。" — if Tencent never recovers, he takes delivery at $340, and both the trading commission and the stamp duty, charged as a percentage of the consideration, come out smaller than taking delivery at $360. Elsewhere in the same thread he puts the HK exercise cost at about 0.3%. One mechanical constraint bounds the timing of any roll: HK stock options are American-style and exercisable on any business day, so a short put that has gone deep in the money can be assigned before there is any chance to roll it.

## Preconditions and the regret trade-off

The roll lives inside his base-position-protected, fully collateralized option-selling program — it is not a free-standing premium-harvesting trick. His standing rule: "沒有準備好全額接貨的錢，不要sell put" — without the full cash ready to take delivery, do not sell (or roll) a put; no margin, no leverage, and only on names he is willing to own. He is genuinely indifferent to assignment — "我不介意接貨，接了就反手賣call" (I don't mind taking delivery; once assigned I turn around and sell calls) — which is why rolling down is optional housekeeping for him rather than a forced escape. The acknowledged cost is forgone upside: to an imitator who rolled a 350 put down to 340 he warned "如果騰訊反彈至$350，你會後悔的" — if Tencent rebounds to $350, you will regret it — because the rolled position no longer participates in the recovery between the new strike and the original one.

## See Also

For the payoff anatomy behind a short put's breakeven, see deriv-option-payoff-anatomy; mt-effective-cost-trade-benchmark frames the all-in transaction-cost comparison he is making implicitly; deriv-delta-and-hedging covers how moving the strike changes the position's delta. On the HK mechanics: hk-stock-option-american-style-assignment (assignment possible any business day), hk-stock-transfer-stamp-duty-per-side (the percentage stamp duty he is minimizing), hk-short-call-assignment-settlement-timing, and hk-covered-call-shares-as-margin-collateral (the sell-call leg he flips to after taking delivery).

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p3475:3474` — 狗不叫, author reply c249013851 (post 223819429), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3480:3479` — 狗不叫, author reply c249026833 (post 223819429), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p6208:6207` — 狗不叫, author reply c249028864 (post 225297451), verbatim ★AUTHOR words
- `hkex_stock_options_info_sheet:p002:0002` — grounding snapshot `hkex_stock_options_info_sheet`
