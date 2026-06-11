---
schema_version: "cacg.v0"
id: "gbj-hk-option-liquidity-megacaps"
title: "HK option liquidity is concentrated in a few mega-caps; pick market by liquidity not frequency"
reading_id: "14_microstructure_and_trading"
summary: "HK stock options had usable liquidity only in a few mega-caps (Alibaba, Tencent, CNOOC) in the author's 2022-H1 practice. A mid-price quote in HK almost always fills; in US options mids often miss — his own guess blames the far larger strike/expiry count — so spread loss is an explicit trade-selection cost. He picks markets and expiries by liquidity, not expiry frequency."
tags: ["xueqiu-2022h1", "hk-options", "liquidity", "bid-ask-spread", "market-microstructure", "option-selling"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p4108:4107"
    chunk_hash: "1ae954a0d9ea539c68aac07b7c8103aa2bb67f669d468f286e6be0f35f4b9e62"
    page_range: [4108, 4108]
    quote: "阿里騰訊中海油還可以，其他的真的不怎麼樣"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p655:0654"
    chunk_hash: "0deefab405b8908be2383330b0a1b62cf175be43c78f2f8afaa49e9b234bc3a4"
    page_range: [655, 655]
    quote: "港股期權我基本上掛中間價一定會成交，美股期權卻經常出現掛中間價沒有成交的情況。我估計是因為行使價太多以及到期日太多的原故，價差的損失也是我的一個考慮"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3737:3736"
    chunk_hash: "8dfb4f090b7859416c7ea9e762c56b39db384fb91929dafc9498e0922a2deed7"
    page_range: [3737, 3737]
    quote: "多久一次並不是問題，問題在於流動性。"
    edge_type: "supports"
card_hash: "06c45afff4c518aa783a343d085fe5004f38aac6de81a45e5879a08ee255cf53"
---

## Thesis

HK equity options have usable liquidity only in a handful of mega-caps (Alibaba, Tencent, CNOOC); other names are too thin. In HK, quoting the mid almost always fills, while in US options the huge strike/expiry count fragments liquidity and widens spreads. When choosing markets/expiries, liquidity — not how often contracts expire — is the deciding factor, and don't chase a poor market-maker quote.

## Where HK Option Liquidity Actually Sits

When a reader asked whether HK options weren't the *less* liquid venue compared with the US, the author answered with a name-by-name liquidity map: 「阿里騰訊中海油還可以，其他的真的不怎麼樣」 — "Alibaba, Tencent and CNOOC are OK; the others really aren't much good." Read this as his assessment of which option books are usable, not as a list of the only names he trades: the same month's settlement post records sell puts on 太保 (CPIC) and sell calls on 中石油 (PetroChina) collecting premium alongside Alibaba, CNOOC and Tencent. The practical point is narrower — in HK, only a few mega-cap option books are deep enough that execution quality can be taken for granted; everywhere else, the book itself is the first thing to check.

## Mid-Price Fills in HK, Often Not in the US

His execution test is simple: where does a mid-price quote fill? 「港股期權我基本上掛中間價一定會成交，美股期權卻經常出現掛中間價沒有成交的情況。我估計是因為行使價太多以及到期日太多的原故，價差的損失也是我的一個考慮」 — "With HK options, when I quote the mid it basically always fills; with US options it frequently happens that a mid quote does not fill. My guess is that it's because there are too many strikes and too many expiry dates, and the spread loss is also one of my considerations." Note his own hedge: the strikes-and-expiries explanation is offered as his estimate (我估計) of the cause, not as established microstructure fact. What is operational is the consequence he draws from the observation — the bid-ask spread loss is an explicit line item in trade selection, weighed alongside the premium itself.

## Expiry Frequency Is Not the Variable

Asked whether US weeklies on the same underlying (e.g., Alibaba) beat HK monthlies because they roll more often, he cut the question down: 「多久一次並不是問題，問題在於流動性。」 — "How often is not the problem; the problem is liquidity." Expiry cadence is a non-issue next to whether the book will absorb your order near fair value. The same discipline shows in how he handled a wide quote in his one US trade: selling a Weibo (WB) put, he skipped the near expiry because, in his words, 「莊家買賣差價太大。剛好看到下個月的這只有個交易對手出價合適。」 — the market maker's bid-ask spread was too wide, and he happened to see a counterparty in the next month's contract quoting an acceptable price. That was a demonstrated choice in a US option, not a stated general rule, and he made no parallel claim about HK market-maker behavior — but it is of a piece with the rest: he transacts where and when the quote is acceptable, rather than paying up to trade on a schedule.

This is a Xueqiu-only practitioner observation — no official source was available for this claim.

## See Also

For the textbook treatment of the same forces — the spread as the price of immediacy, depth and width as dimensions of liquidity, and what empirically drives illiquidity — see mt-bid-ask-spread-immediacy-price, mt-liquidity-depth-immediacy-width and mt-empirical-determinants-illiquidity. Within this deck, hk-stock-option-american-style-assignment and hk-short-call-assignment-settlement-timing cover the contract mechanics of the HK books he favors, hk-special-dividend-option-contract-adjustment covers the corporate-action edge cases, and hk-stock-connect-excludes-derivatives marks the access boundary: Stock Connect does not include derivatives, so these option books are reached only through direct HK brokerage access.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p4108:4107` — 狗不叫, author reply c247212904 (post 224093303), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p655:0654` — 狗不叫, author reply c251251375 (post 222517599), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3737:3736` — 狗不叫, author reply c246998840 (post 223990393), verbatim ★AUTHOR words
