---
schema_version: "cacg.v0"
id: "gbj-roll-deep-otm-forward-delta-theta"
title: "Roll deep-OTM near-dated legs forward to free tied-up capital"
reading_id: "07_derivatives_and_volatility"
summary: "When a short near-dated leg has gone deeply out-of-the-money, the author closes it — almost no time-value remains and the fully collateralized position only ties up cash — and opens a farther-dated, closer-to-money leg that earns premium again, e.g. rolling an Alibaba June 100 sell put into a July 105 sell put for a 6.89 premium. The delta-vs-theta framing was a commenter's, not his."
tags: ["xueqiu-2022h1", "dated-levels", "sell-put", "rolling", "capital-efficiency", "hk-stock-options", "alibaba", "earnings-volatility"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2584:2583"
    chunk_hash: "6937166a7f78039ad61ffcc541e7fac7f987e323b76ac595b0bdd340e0e8655d"
    page_range: [2584, 2584]
    quote: "平倉了六月份$阿里巴巴-SW(09988)$ 100元的sell put，新開了七月份105元的sell put，收取期權金6.89元"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2594:2593"
    chunk_hash: "97037b6ee4a42f2558a40187aa0f793283462d0683f4bd0ecfd8ff00af0b3cc5"
    page_range: [2594, 2594]
    quote: "我平的近期已經嚴重價外了，留著占資金"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p5525:5524"
    chunk_hash: "f6ce427617c0b881df25febe319920865b53fb6977d43afc80a3dfa4f1fd483a"
    page_range: [5525, 5525]
    quote: "如果我平掉105元或者112.5元的期權，還是可以調一點的，畢竟那里賺了點錢"
    edge_type: "supports"
card_hash: "d12c8e74818df34f3e2091bcb166fb8561bd28a53183947e5740b12cc78f87ac"
---

## Dated State

All specific levels in this card come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. The dated levels are: the Alibaba-SW (09988) June 2022 100 sell put rolled into a July 2022 105 sell put for a 6.89 premium; his then-profitable 105 and 112.5 legs; and his stated 120 "bottom line" for Alibaba.

## Thesis

When a near-dated leg has gone deeply out-of-the-money it has little remaining time-value and only ties up capital; he closes it and opens a farther-dated, closer-to-money leg — closing the leg where delta now outweighs theta.

## The roll, in his own words

On 2022-06-23 he posted the trade itself: 「平倉了六月份$阿里巴巴-SW(09988)$ 100元的sell put，新開了七月份105元的sell put，收取期權金6.89元」 — he closed the June Alibaba 100 sell put and opened a July 105 sell put, collecting 6.89 in premium. Asked why, he gave a capital reason, not a Greeks reason: 「我平的近期已經嚴重價外了，留著占資金」 — the near-dated leg he closed was already deeply out-of-the-money, so keeping it just ties up capital. A deep-OTM short put has almost nothing left to earn, yet as a fully collateralized position it still locks up the full cash needed to take delivery; closing it frees that cash to reopen at a farther expiry and a strike close enough to the money to collect meaningful premium again.

## "Delta vs theta" is a commenter's lens, not his rule

The Greeks vocabulary in the thesis originates with commenter @希尓瑞斯, who asked 「你看中delta是高于theta的是吗？所以平近期卖出了远期的？」 — "so you judged delta to be higher than theta, and that's why you closed the near leg and sold the far one?" The author never adopted that framing; his reply was the simpler capital-occupancy reason quoted above. Read the delta-vs-theta clause as an observer's translation of the trade that he at most tacitly tolerated, not a decision rule he stated himself.

## Profit gives a little room; an imminent earnings release made him pause

Two weeks later, with Alibaba pressing his 120 line, a commenter asked whether he would adjust that line or switch underlyings. His answer: 「如果我平掉105元或者112.5元的期權，還是可以調一點的，畢竟那里賺了點錢」 — if he closed the 105 or 112.5 options he could still adjust a little, since he had made some money there. That is the full extent of the claim: existing profit gives room to maneuver; he described no financing mechanism beyond it. And he immediately flagged a reason to wait that was specific to that decision — 「但是下月初就公布業績，會比較波動」, earnings were due early the next month and the stock would be volatile — a hesitation about that particular adjustment, not a blanket rule against adjusting before earnings. Throughout, the maneuver stays inside his standing risk frame: every sell put is backed by 全額接貨的錢 (the full cash to take delivery) and no leverage is used — 不用杠杆是底線 (no leverage is the bottom line) — so "freeing capital" means redeploying fully collateralized cash to a better strike, never adding margin exposure; and he opposes ordinary investors touching derivatives at all.

## See Also

For the textbook Greeks behind the commenter's framing, see deriv-greeks-overview, deriv-delta-and-hedging, and deriv-vega-and-theta in the cfa deck. For the HK mechanics that make an open short option leg consume collateral until it is closed — and the American-style assignment risk that lingers on any open short leg — see hk-covered-call-shares-as-margin-collateral and hk-stock-option-american-style-assignment in this deck.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p2584:2583` — 狗不叫, post 223426897, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2594:2593` — 狗不叫, author reply c246213534 (post 223426897), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p5525:5524` — 狗不叫, author reply c248218446 (post 224796566), verbatim ★AUTHOR words
