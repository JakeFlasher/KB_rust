---
schema_version: "cacg.v0"
id: "gbj-sell-put-only-names-you-would-own"
title: "Sell Puts Only Where You Want the Shares: Fully Cash-Backed Acquisition, Not a Premium Grab"
reading_id: "07_derivatives_and_volatility"
summary: "狗不叫 treats a sell-put as a stock-acquisition tool: he writes puts only at strikes where he genuinely wants delivery of the shares, only after forming his own value judgment on the underlying, and only with the full take-delivery cash already on hand — never on leverage — so the worst case is bounded at owning, in deliberately small size, a stock he already wanted."
tags: ["xueqiu-2022h1", "sell-put", "cash-secured", "options", "hong-kong", "assignment", "no-leverage", "risk-management"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2129:2128"
    chunk_hash: "b2a7671ecf1ad9ad731c4c6301575eba5e695bc3cac92008f7d30a12252f0641"
    page_range: [2129, 2129]
    quote: "我只sell put我願意以那個價錢接貨的公司"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2098:2097"
    chunk_hash: "a64e072da3faed221ba5a4491502747eb3a9da082654067e6fc372c64600d7be"
    page_range: [2098, 2098]
    quote: "我現在sell put，就算騰訊一夜之間價值為零，我虧損也就3%的事情。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3994:3993"
    chunk_hash: "cf5165f1c0b11dbb01881810888db9637f4f997c30c4b77fef144566dd297e83"
    page_range: [3994, 3994]
    quote: "前提是你對該股的價值有基本判斷"
    edge_type: "supports"
  - source_id: "hkex_stock_options_info_sheet"
    chunk_id: "hkex_stock_options_info_sheet:p002:0002"
    chunk_hash: "98443f597118122d13b89eb3b783d3c52383fa6bed42bd3389168aa5bec04f81"
    page_range: [2, 2]
    quote: "Options can be exercised at any time up to 6:45 pm on any business day up to and including the last trading day"
    edge_type: "supports"
card_hash: "98f5e46396f0bca109f04ffd933ed811e032bf07fcd76aff70b129b7fe1fc8d4"
---

## Thesis

A sell-put is an acquisition tool, not a premium grab: he only writes puts at strikes where he genuinely wants to be assigned the stock, fully cash-backed, so the worst case is bounded (owning a stock he wanted even if it goes to zero costs only ~3% of book).

## The underwriting standard: the strike is a buy decision

When a commenter pointed him at a fatter premium on another name, the author refused in one line: 「我只sell put我願意以那個價錢接貨的公司」 — "I only sell puts on companies I am willing to take delivery of at that price." The strike is a price he has already decided is a buy; the premium alone never justifies the trade. The second precondition is valuation work: even on an Alibaba put he found genuinely tempting, he added 「前提是你對該股的價值有基本判斷」 — "the precondition is that you have a basic judgment of that stock's value." Assignment is the base case, not a failure mode — he says plainly 「我不介意接貨，接了就反手賣call」 (he doesn't mind taking delivery; once assigned he turns around and writes calls against the shares) and tells worriers 「擔心接貨就別sell put」 (if assignment worries you, don't sell puts at all). None of this is an aversion to premium income: within names that pass his bar he openly harvests volatility premium. The discipline lies in which names and strikes qualify, not in refusing the income.

## Bounded worst case: full cash, no leverage

Sizing is what bounds the downside. Answering an accumulator question, he contrasted his then-current (mid-2022) Tencent sell-put exposure with the accumulator's bottomless pit: 「我現在sell put，就算騰訊一夜之間價值為零，我虧損也就3%的事情。」 — "with my current sell-puts, even if Tencent's value went to zero overnight, my loss would only be about a 3% matter." That 3% is an illustration of his own position size at that moment, not a universal ratio: the operative rule in the same reply is 「我sell put有多少錢就接多少的貨，絕不多賣」 — he sells puts only up to the cash he holds to take delivery, never more — so even a total wipeout of the underlying costs only the slice of the book committed to that strike.

The whole practice sits inside an emphatic warning he posted as its frame: he has always opposed ordinary investors touching any derivative, sell calls and sell puts included (「本人一向反對普通投資者觸碰任何的衍生工具，即便是sell call和sell put」), calls it a high-risk game, and states the put-side rule bluntly — 「沒有準備好全額接貨的錢，不要sell put。會死人㗎！」: if you have not prepared the full cash to take delivery, do not sell puts; it can kill. Leverage is excluded outright: 「不用杠桿是底線，忌貪」 (no leverage is the bottom line — guard against greed), and backing the puts with margin financing is 「用融資是找死」 (courting death). Fully collateralized, or not at all.

## Why the cash must stand ready the whole time (HK mechanics)

Hong Kong single-stock options are American-style: per HKEX's stock options information sheet, "Options can be exercised at any time up to 6:45 pm on any business day up to and including the last trading day". A short put can therefore be assigned before expiry, so the full take-delivery cash must be available for the entire life of the position, not merely at expiry — which is precisely what his full-cash rule provides. He even schedules expiries around cash arrival: 「中海油七月中股息到賬，所以我才選擇sell put七月到期的」 — he chose a July-expiry CNOOC put because the mid-July dividend would land in his account in time to fund any assignment.

## See Also

For the payoff anatomy of a short put, see deriv-option-payoff-anatomy; the full-cash, no-leverage sizing rule is a lived application of rm-risk-objectives-and-tolerance, and the value-judgment precondition is the practitioner's version of eq-intrinsic-value. On the Hong Kong mechanics side, hk-stock-option-american-style-assignment covers the early-exercise exposure discussed above, hk-covered-call-shares-as-margin-collateral covers the share-backed call leg he flips into after assignment, hk-short-call-assignment-settlement-timing explains the post-assignment settlement clock, and equity-index-volatility-skew-negative explains why downside puts are structurally rich for a seller with his standards.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p2129:2128` — 狗不叫, author reply c247917105 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2098:2097` — 狗不叫, author reply c247180488 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3994:3993` — 狗不叫, author reply c247049685 (post 224025593), verbatim ★AUTHOR words
- `hkex_stock_options_info_sheet:p002:0002` — grounding snapshot `hkex_stock_options_info_sheet`
