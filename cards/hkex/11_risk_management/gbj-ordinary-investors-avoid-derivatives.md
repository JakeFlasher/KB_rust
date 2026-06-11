---
schema_version: "cacg.v0"
id: "gbj-ordinary-investors-avoid-derivatives"
title: "Ordinary Investors Should Not Touch Derivatives: Never Naked, Never Short, Never on Margin"
reading_id: "11_risk_management"
summary: "He publicly opposes ordinary investors touching any derivatives, even sell-call/sell-put: never sell a call without 100% of the underlying, never sell a put without full cash for delivery, never short, never finance delivery on margin. The ban targets imitators who only think they understand the risk; he himself trades fully collateralized and unleveraged."
tags: ["xueqiu-2022h1", "risk-management", "derivatives", "option-selling", "short-selling", "no-leverage", "retail-investors"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1789:1788"
    chunk_hash: "7d979805dd5329ff4aed7d21c67cc266a9b50ebd0b4979fa157e1afdfd7269ac"
    page_range: [1789, 1789]
    quote: "本人一向反對普通投資者觸碰任何的衍生工具，即便是sell call和sell put。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1789:1788"
    chunk_hash: "7d979805dd5329ff4aed7d21c67cc266a9b50ebd0b4979fa157e1afdfd7269ac"
    page_range: [1789, 1789]
    quote: "高危遊戲，手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p6579:6578"
    chunk_hash: "7ceb89e6211d9b12bc29a3f6aca54fda95775da316213c7a242b4cdcc4678f0e"
    page_range: [6579, 6579]
    quote: "如果多手做空0.5%倉位，已經爆倉，想想也蠻可怕的。對於一般人，不做空保平安！"
    edge_type: "supports"
card_hash: "037291406e207d14cc7fe5d4183f832f393fb1bc2931eb0b49571bdd752563b6"
---

## Thesis

He opposes ordinary investors touching any derivatives, including sell-call/sell-put: never sell a call without 100% of the underlying, never sell a put without full cash to take delivery, and never short — misjudging the risk you bear can be fatal ('會死人').

## A public warning against imitators

In June 2022, with hundreds of followers copying his option trades in real time, the author posted a standalone declaration written explicitly so that inexperienced followers (his word: 小朋友, "kids") would not imitate him and get hurt: 「本人一向反對普通投資者觸碰任何的衍生工具，即便是sell call和sell put。」 — "I have always opposed ordinary investors touching any derivative instruments, even sell call and sell put." The scope is precise: the ban is on 普通投資者 (ordinary investors), not on himself. In the same post he says he knows exactly what game he is playing and what risk he is carrying; what worries him is the follower who believes he knows what he is doing — 「會有一些小朋友，以為知道自己在做什麼，實質上完全不理解自己承擔着什麼樣的風險」, "some kids who think they know what they are doing, but in substance completely fail to understand what risk they are bearing."

The mechanical rule follows in the same breath: 「高危遊戲，手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！」 — "A high-risk game: without 100% of the underlying shares in hand, never sell a call; by the same token, without the money ready to take full delivery, never sell a put. It can kill you!" This is the full-collateralization spine of his entire option-selling practice: every short call sits on top of the shares, every short put sits on top of the cash to take the entire delivery. The one position he ran without settled shares behind it — a call written against shares still pending delivery from an expiring in-the-money short put — he himself flagged with his own term 半裸 ("half-naked").

## Never short, never on margin

The companion rule covers shorting. Commenting on the AMTD Digital (尚乘数科) melt-up — a thinly held new listing briefly priced at a market capitalization rivalling the largest Hong Kong stocks — he wrote: 「如果多手做空0.5%倉位，已經爆倉，想想也蠻可怕的。對於一般人，不做空保平安！」 — "If you had rashly shorted a 0.5% position, you would already be wiped out; frightening just to think about. For ordinary people: don't short, and stay safe!" Note that 多手 here is the Cantonese idiom for acting rashly (itchy-handed), not a count of contracts — there is no leverage in the sentence. The point is the bare asymmetry of an unleveraged short: in a more-than-hundredfold squeeze, a position that began as 0.5% of the account can by itself exceed the account's entire equity, while the maximum gain was always capped at that same 0.5%. That blow-up is context-bound to an extreme squeeze; his general rule is the explicit closing line — for ordinary people, no shorting.

The same discipline recurs across his replies in the warning thread. Asked whether a fully invested account could simply let put assignment draw on margin financing, he answered 「用融資是找死」 — "using margin financing is asking to die": the delivery money must be real cash. On sizing, he sells puts only up to the cash on hand and 絕不多賣 ("absolutely never sells more"), and he refuses accumulators as a 無底深淵 — a "bottomless abyss" whose worst case cannot even be stated. His closing maxim makes the boundary explicit: 「不用杠桿是底線，忌貪！別動任何歪念！」 — "no leverage is the bottom line; beware greed — don't entertain any crooked ideas!"

## See Also

For the official mechanics behind a fully collateralized short call on HKEX — shares pledged in place of cash margin, assignment and settlement timing, American-style early assignment — see hk-covered-call-shares-as-margin-collateral, hk-short-call-assignment-settlement-timing, and hk-stock-option-american-style-assignment; hk-stock-connect-excludes-derivatives explains why Stock Connect investors cannot trade these instruments at all. From the cfa deck, rm-risk-objectives-and-tolerance and rm-risk-type-taxonomy cover matching risk taken to risk capacity and naming the risks you actually bear, and mt-market-manipulation-bluffing is relevant to squeeze episodes like the one anchoring his no-shorting warning.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p1789:1788` — 狗不叫, post 223114673, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p1789:1788` — 狗不叫, post 223114673, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p6579:6578` — 狗不叫, post 227101293, verbatim ★AUTHOR words
