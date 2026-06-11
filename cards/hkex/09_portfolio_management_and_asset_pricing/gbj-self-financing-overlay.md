---
schema_version: "cacg.v0"
id: "gbj-self-financing-overlay"
title: "Sell puts only against full-delivery cash: dividend inflow as the capital plan, not a sizing formula"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Cash is the hard gate on his sell-put program: he sells only as many puts as he has money to take full delivery on, with no leverage. Assured dividend inflow — timing a CNOOC put expiry to the mid-July dividend — is part of the capital plan, a rough habit, not a precise formula; deeply out-of-the-money near-term puts get closed to free the capital they occupy."
tags: ["xueqiu-2022h1", "sell-put", "capital-discipline", "dividend-timing", "no-leverage", "option-overlay"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1821:1820"
    chunk_hash: "4ef0b3dd5c91163d4b0f473bcc1632f6412cb019bbbb7e6dde6f7dd04a8aea84"
    page_range: [1821, 1821]
    quote: "中海油七月中股息到賬，所以我才選擇sell put七月到期的"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3893:3892"
    chunk_hash: "9f2b64e582f327de37b33e9704071f0c4ba692be2529c0b52c9b0819fe75f114"
    page_range: [3893, 3893]
    quote: "是啊！至少知道會有股息馬上到賬，這是原則。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3992:3991"
    chunk_hash: "c637919c1c16e7c5d9ef68248e5b22094410b7ab38297c338697024fda7756f1"
    page_range: [3992, 3992]
    quote: "這個sell put有點吸引，但沒錢"
    edge_type: "supports"
card_hash: "b2286b13517dd7aabd6e3ae659674bc2518785bb0aa407d114c1b92f66883ecc"
---

## Thesis

Capital availability is a hard gate on his sell-put program: an attractive premium is irrelevant without the cash to take full delivery ("沒有準備好全額接貨的錢，不要sell put"; "這個sell put有點吸引，但沒錢"), he sells only as many puts as he has money to take delivery on ("我sell put有多少錢就接多少的貨，絕不多賣"), and no leverage is the bottom line ("用融資是找死"). Assured dividend inflow is part of that capital plan rather than a self-financing formula: he chose a July CNOOC expiry specifically because the dividend lands in mid-July ("中海油七月中股息到賬，所以我才選擇sell put七月到期的"), and treats knowing dividend cash will arrive immediately as "the principle" ("至少知道會有股息馬上到賬，這是原則"). When a commenter asked whether his sell-put quantity matches the shares his dividends can buy, he confirmed only loosely ("大概能匹配吧"), so sizing-to-dividend is a rough habit he endorsed when prompted, not a precise rule he stated himself. He also dislikes long capital tie-ups: he passed on a five-month put because "the capital doesn't line up" ("所以資金才對不上") — while still calling it attractive given a valuation judgment — and he closed a near-term put once it went deeply out of the money rather than let it keep occupying capital ("留著占資金").

## The capital gate comes first

An attractive premium never overrides the cash constraint. He passed on an Alibaba put with the flattest possible reasoning — "這個sell put有點吸引，但沒錢" ("this sell put is somewhat attractive, but no money"): the trade was named, judged attractive, and dropped in one sentence because the delivery cash was not there. His own unprompted formulation of the rule is fully collateralized sizing — never sell a put without 全額接貨的錢, the cash for full delivery — and he sells only as many puts as that cash covers, 絕不多賣 ("absolutely never sell more"). Leverage is excluded outright: asked whether a full account could simply let the broker finance an assignment, he answered 用融資是找死 ("using margin financing is courting death"), and he frames no leverage as the bottom line — 不用杠桿是底線.

## Dividend inflow is the capital plan, not a formula

When a commenter asked whether he makes sure the account always has enough cash to take delivery each time he sells a put, his answer tied the cash gate to dividend timing: "是啊！至少知道會有股息馬上到賬，這是原則。" ("Yes! At least I know the dividend will arrive immediately — that is the principle."). The expiry choice follows the cash calendar, not the other way round: "中海油七月中股息到賬，所以我才選擇sell put七月到期的" ("CNOOC's dividend lands in mid-July, so that is exactly why I chose to sell the put expiring in July") — he had no money left to take delivery on a June expiry, so he sold the month whose assignment, if it came, would land after known dividend cash did. Both utterances come from his 2022-H1 CNOOC threads.

One attribution caution before hardening this into a sizing rule: the idea of matching the number of puts sold to the shares the incoming dividend cash can buy was a commenter's framing, and his entire endorsement was 大概能匹配吧 ("they roughly match, I suppose"). It is a loose habit he confirmed when prompted — usable as a sanity check, not a formula he stated himself.

## Capital tie-up cuts both ways

The same cash logic governs duration and exits. Offered a five-month put he found genuinely attractive, he declined because the money would be locked past his cash calendar — 所以資金才對不上 ("so the capital just doesn't line up") — while still calling it attractive, with the caveat 前提是你對該股的價值有基本判斷 ("the premise is that you have a basic judgment of that stock's value"). And when a near-term short put went deeply out of the money, he closed it rather than 留著占資金 ("leave it sitting there occupying capital"): a position that has little left to give does not get to keep tying up delivery cash.

## See Also

In the cfa deck, pm-portfolio-constraints treats liquidity and capital availability as binding inputs to portfolio decisions — exactly the role cash plays here; pm-rebalancing-mechanics covers the mechanics of redeploying freed cash; and eq-payout-policy-and-growth explains why an established payer's dividend stream is predictable enough to plan a cash calendar against. Among the hkex grounding cards, hk-stock-option-american-style-assignment is the sharpest companion — HK stock options can be assigned before expiry, so full-delivery cash must be available throughout the position's life, not just at expiry; hk-short-call-assignment-settlement-timing covers when assignment cash flows actually settle; hk-mainland-dividend-withholding-cdta bears on how much dividend cash actually lands in the account; and hk-special-dividend-option-contract-adjustment covers how dividend events interact with option contract terms.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p1821:1820` — 狗不叫, author reply c245791853 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3893:3892` — 狗不叫, author reply c247049368 (post 224024550), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3992:3991` — 狗不叫, post 224025593, verbatim ★AUTHOR words
