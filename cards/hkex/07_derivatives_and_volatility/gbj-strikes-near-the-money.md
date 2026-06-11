---
schema_version: "cacg.v0"
id: "gbj-strikes-near-the-money"
title: "Below his price floor, keep sold-put strikes near the money — deep-OTM puts neither hold the position nor pay"
reading_id: "07_derivatives_and_volatility"
summary: "Within names he is willing to own, and only below his price floor (Tencent $400, Alibaba HK$120), 狗不叫 keeps sold-put strikes close to the current price: drift too far from the strike and you cannot eat the volatility money, collect negligible premium, and tie up capital. Near-the-money serves both staying positioned and premium capture — always with full cash to take delivery, never on margin."
tags: ["xueqiu-2022h1", "dated-levels", "sell-put", "strike-selection", "option-premium", "volatility", "risk-management", "hkex-options"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1880:1879"
    chunk_hash: "5ed8231a281aaae86a279837ed1b52c79e4548765b1e41c24543185b1308599d"
    page_range: [1880, 1880]
    quote: "偏離行權價太遠，吃不了波幅的錢"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p5517:5516"
    chunk_hash: "213c19d8a96e4742791256242e0eda93f275fd57b540703c91344b4c4230f769"
    page_range: [5517, 5517]
    quote: "你這樣賺不到多少期權金的"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2590:2589"
    chunk_hash: "7cd46735ac061156ddc085a167154f66bf9738dd9937df35402b2e59d4559c53"
    page_range: [2590, 2590]
    quote: "不偏離行權價"
    edge_type: "supports"
card_hash: "8e10101e69a8ef9d95809f84adb0de634dad618f172b7cf38677ba60e30b76cb"
---

## Dated State

All price levels in this card come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. The dated levels named here: Tencent $400 and Alibaba HK$120 as his willingness-to-own floors, and a follower's 300-strike sold put placed far below the market in July 2022.

## Thesis

For names he is willing to own and stay positioned in, and only below his willingness-to-own price floor (Tencent under $400, Alibaba under $120 — "120元以上，我可能就不願意繼續sell put了"), he keeps sold-put strikes close to the current price: a strike too far out of the money "吃不了波幅的錢" (can't eat the volatility money) and collects negligible premium ("你這樣賺不到多少期權金的"), while a severely out-of-the-money short put just ties up capital ("我平的近期已經嚴重價外了，留著占資金"). His term '弄丟' (losing the 籌碼/chips) means losing his effective position — and by his own direct gloss, letting the price drift too far from the strike ("偏離行權價太遠，吃不了波幅的錢") so the short put neither keeps him positioned nor earns the volatility premium; near-the-money strikes serve BOTH staying positioned and premium capture, not premium yield alone. Critical preserved caveat: he opposes ordinary investors touching derivatives at all and insists one must never sell puts without the full cash ready to take delivery ("沒有準備好全額接貨的錢，不要sell put。會死人㗎！"; "我sell put有多少錢就接多少的貨，絕不多賣"; "用融資是找死"). The near-the-money rule is therefore conditional on wanting and being able to take the underlying below his price floor, not a standalone premium-optimization tactic.

## The price floor comes before the strike

His option selling is base-position-protected and fully collateralized, and the near-the-money rule lives strictly inside that program. The boundary is his willingness-to-own floor: below Tencent $400 and below Alibaba HK$120 he keeps selling puts and tries hard not to lose his chips (盡可能不弄丟籌碼 — maintaining a definite position, 維持一定倉位); above the floor he simply stops selling puts altogether. The rule is therefore not a strike-picking formula you can lift out on its own — it only applies to a name he actively wants to own, at prices where he is genuinely willing to take delivery. When a reader objected that a sold put cannot "lose the chips" — surely only a sold call can — his entire answer was 「不偏離行權價」: do not drift away from the strike. Keeping the strike near the money is how the short put keeps doing its job for him.

## What 弄丟 means: losing the position AND losing the volatility money

Asked point-blank what he meant by 弄丟 ("losing it"), he answered 「偏離行權價太遠，吃不了波幅的錢」 — when the price drifts too far from your strike, you can no longer eat the volatility money. Read together with his floor statement, 弄丟籌碼 names two failures at once: the short put stops functioning as an effective position in a stock he wants to keep holding, and it stops harvesting the premium that selling near the money is supposed to capture. Near-the-money strikes serve both ends simultaneously; treating this as a standalone premium-yield trick misses half of what he is doing.

## Deep-OTM strikes: negligible premium, tied-up capital — and the hard safety rules

When a follower "copied his bottom" by selling a put struck at 300, far below the market, he warned 「你這樣賺不到多少期權金的」 — you will not earn much option premium that way. The mirror case is his own housekeeping: he closed out a near-term short put that had gone severely out of the money because keeping it open earned nothing and just sat there occupying capital. None of this loosens his safety rules — the very post carrying his 弄丟 gloss is itself a warning. He opposes ordinary investors touching any derivatives, even sell call and sell put, and insists that without 全額接貨的錢 — the full cash to take delivery — you must not sell a put: 會死人㗎 (it can kill). He sells only as many puts as he has cash to take delivery on, fully collateralized, and calls financing the position a death wish (用融資是找死) — no leverage, ever.

## See Also

For the mechanics behind these judgments, the cfa deck's deriv-option-payoff-anatomy lays out the short-put payoff he is managing, deriv-vega-and-theta explains where the "volatility money" in a near-the-money option comes from, and deriv-delta-and-hedging covers why moneyness governs how strongly an option responds to the underlying. Among the hkex grounding cards, hk-stock-option-american-style-assignment matters most to anyone running near-the-money short puts (HK stock options can be exercised on any business day, not only at expiry), and equity-index-volatility-skew-negative gives market-wide context on how implied volatility — and therefore available premium — differs across put strikes.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p1880:1879` — 狗不叫, author reply c246199966 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p5517:5516` — 狗不叫, author reply c248256350 (post 224796566), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2590:2589` — 狗不叫, author reply c246199680 (post 223426897), verbatim ★AUTHOR words
