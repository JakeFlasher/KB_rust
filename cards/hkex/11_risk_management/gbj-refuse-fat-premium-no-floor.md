---
schema_version: "cacg.v0"
id: "gbj-refuse-fat-premium-no-floor"
title: "Refusing fat premiums on 沒底 (no-bottom) names: take-delivery willingness, not premium yield, selects the underlying"
reading_id: "11_risk_management"
summary: "When a commenter offered 8%+ monthly put premiums on 陌陌/老虎 (30 June 2022), the author refused ('不敢買呀！'), having just called 阿里健康-type names 都沒底 (no bottom). His filter is not premium level — he collects ~6-7%/month on Tencent/Alibaba gladly — but willingness to take delivery at the strike with full cash and no leverage. A one-off illustration, not an avoid list."
tags: ["xueqiu-2022h1", "dated-levels", "sell-put", "take-delivery", "underlying-selection", "risk-management", "no-leverage"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2066:2065"
    chunk_hash: "2f4837a03c909210a4158c18c19961f5b03be3fdf2944f4f37c06d1f6a7b1301"
    page_range: [2066, 2066]
    quote: "阿里健康那些，我怎麼敢碰啊？都沒底"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p4041:4040"
    chunk_hash: "81b8f271262e35f168789b2548b5bd3398dbe826549e6cbbac72432fe013e5f5"
    page_range: [4041, 4041]
    quote: "不敢買呀！"
    edge_type: "supports"
card_hash: "3b82fc72cce4bc7971dd5902d8941bc7beec8d488048f6f21d0e509bf668751c"
---

## Dated State

All specific levels in this card come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. The dated levels: 8%+ monthly put premiums quoted by a commenter on 陌陌 (Momo) and 老虎 (Tiger Brokers); roughly 6-7%/month premium yields on the author's own Tencent/Alibaba option sales; and CNOOC (中海油), where he sold a HK$10-strike put for HK$0.48 and judged HK$8.5 very hard to break. All reflect June 2022 market conditions.

## Thesis

In one conversational thread (30 June 2022), when a commenter noted that names like 陌陌/老虎 carried 8%+ monthly put premiums, the author declined ('不敢買呀！'), having just said of 阿里健康-type names '我怎麼敢碰啊？都沒底' (how would I dare touch those — no bottom). His operative rule, stated in his own words and repeated, is willingness-to-own plus full cash — not an IV threshold: '我只sell put我願意以那個價錢接貨的公司'; '我sell put有多少錢就接多少的貨，絕不多賣'; '沒有準備好全額接貨的錢，不要sell put。會死人㗎！'. He happily collects similarly fat premiums (~6-7%/month — '你没算錯。不然的話我怎麼會賣得不亦樂乎') on Tencent/Alibaba, names he wants to own at the strike; where he can see a valuation floor he says so explicitly (中海油: '$8.5是很難跌破的…概率上對我有利'). Note: 陌陌/老虎/嘉年华 were named by a commenter, not the author, and the refusal was a point-in-time reaction in a single exchange — not an author-maintained avoid list (the corpus later shows him engaging with Momo options). Treat this as a one-off illustration of his take-delivery rule, not a heavily repeated standalone doctrine.

## The 30 June 2022 exchange, attributed precisely

Inside a thread about his option premium yields, the author told commenter 复利的魅力: "阿里健康那些，我怎麼敢碰啊？都沒底" — "names like Ali Health, how would I dare touch them? They have no bottom." The commenter then pushed names with even fatter premiums — 陌陌、老虎之类的，都8个点以上 (Momo, Tiger and the like, all above 8 points a month) — and the author's entire reply was "不敢買呀！" — "I don't dare buy!" Two attribution points matter for using this correctly. First, 陌陌/老虎 (and later 嘉年华) were the commenter's examples, not names the author put forward; the only name the author himself raised was 阿里健康. Second, the refusal was a point-in-time reaction within one exchange, and the corpus later shows him engaging with Momo options — so this must not be read as a maintained blacklist of tickers. It is an illustration of how he reacts when offered yield on a name he does not want to own.

## The discriminator: take delivery with full cash, not an IV cutoff

The premium level itself does not separate what he sells from what he refuses. In the same thread he confirms his own Tencent/Alibaba premiums run about 6-7% a month and that he sells them happily: "你没算錯。不然的話我怎麼會賣得不亦樂乎" — "you didn't miscalculate; otherwise how could I be selling with such relish." What separates the two cases is whether he wants the shares at the strike and can pay for them in full: "我只sell put我願意以那個價錢接貨的公司" (I only sell puts on companies I am willing to take delivery of at that price) and "我sell put有多少錢就接多少的貨，絕不多賣" (I sell only as many puts as my cash can take delivery of — never oversell). Beneath this sits his hard caveat, fully collateralized always: "沒有準備好全額接貨的錢，不要sell put。會死人㗎！" — without the full cash ready to take delivery, do not sell the put; it can kill you. No leverage is the floor of the whole program — "用融資是找死" (using margin financing is courting death) — and he explicitly opposes ordinary investors touching any derivatives at all. Where he can articulate why a strike is safe, he does so in valuation terms, as with CNOOC: "$8.5是很難跌破的…概率上對我有利" — given rich earnings and payout even at middling oil prices, HK$8.5 is very hard to break, so the odds favor him as the put seller. Note what he does not say: he never states that high implied volatility itself signals danger — that reading would sit oddly beside the fat Tencent/Alibaba premiums he collects gladly. The dividing line is willingness to own at the strike, with the money in hand.

## See Also

For surrounding context: deriv-implied-volatility explains why option premiums scale with implied volatility (context only — the author's own discriminator is take-delivery willingness, not the IV level); rm-neglected-tail-risk and rm-risk-type-taxonomy give the textbook risk vocabulary adjacent to his 都沒底 instinct, though the generalized tail-risk theory is not his framing. Within this deck, equity-index-volatility-skew-negative covers how markets price downside protection, and hk-stock-option-american-style-assignment together with hk-covered-call-shares-as-margin-collateral cover the assignment and collateral mechanics that make his full-cash rule operative.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p2066:2065` — 狗不叫, author reply c247154739 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p4041:4040` — 狗不叫, author reply c247157237 (post 224083814), verbatim ★AUTHOR words
