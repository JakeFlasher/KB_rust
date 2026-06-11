---
schema_version: "cacg.v0"
id: "gbj-calls-against-base-are-pocket-money"
title: "Calls against the base are pocket money on volatility — small size, leg chosen by tolerance"
reading_id: "07_derivatives_and_volatility"
summary: "For 狗不叫, calls sold against a core holding are pocket money (零用錢) on volatility, never a return engine: small size only (e.g. 1/4 of his CNOOC position), explicit reluctance to part with the shares, and a leg-selection rule — if you fear losing your shares don't sell calls, if you fear taking delivery don't sell puts — all inside his fully collateralized, no-leverage option-writing program."
tags: ["xueqiu-2022h1", "dated-levels", "option-selling", "sell-call", "position-sizing", "cnooc", "premium-income"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1458:1457"
    chunk_hash: "fc3c77bef6aa2e10320a94c4c2e91cb4422ab529031729e933a9fa021aae31e7"
    page_range: [1458, 1458]
    quote: "我只是sell call賺點波動的零用錢，沒想到把家用都蝕精光。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2202:2201"
    chunk_hash: "5495a094112e6fd630a5cd8152b655fcf8ac7ead8d13d65ac1f078c9b34df742"
    page_range: [2202, 2202]
    quote: "擔心弄丟籌碼就別sell call，擔心接貨就別sell put！你有看到我sell call保險和電信么？"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3572:3571"
    chunk_hash: "42953b8cf5ef95dbf1c8cbd4a8f142fd9926af82d6a54a577e8091c1fd9f7815"
    page_range: [3572, 3572]
    quote: "sell call 了1/4$中国海洋石油(00883)$ ，七月份，10.5元，收0.47元期權金。"
    edge_type: "supports"
  - source_id: "hkex_stock_options_info_sheet"
    chunk_id: "hkex_stock_options_info_sheet:p002:0002"
    chunk_hash: "98443f597118122d13b89eb3b783d3c52383fa6bed42bd3389168aa5bec04f81"
    page_range: [2, 2]
    quote: "Options can be exercised at any time up to 6:45 pm on any business day up to and including the last trading day"
    edge_type: "supports"
card_hash: "01dec62fb19d7c376fbfaf366f89a1be92bf12d4d9cf6d5dd7207011e3208e31"
---

## Dated State

All specific levels in this card come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and record his trades at the time — they are NOT durable recommendations. Dated levels: CNOOC (00883) July-2022 calls struck at HK$10.5 sold for HK$0.47 of premium on 1/4 of his position; earlier pre-ex-dividend CNOOC calls struck at HK$12 and HK$12.5; and his reference to CNOOC earning roughly 900億港幣 (≈HK$90bn) in 2022-H1.

## Thesis

He sells covered calls only modestly — to collect a little premium on volatility ('零用錢') against a core holding he is reluctant to give up for its earnings/dividends, explicitly stating he does not intend to sell calls in large size ('我也沒有打算大量賣，也有不捨，但我尊重概率') and capping quantity (e.g. only 1/4 of his CNOOC position). His stated discipline is leg-selection by tolerance: 'if you're worried about losing your shares don't sell calls; if you're worried about being assigned don't sell puts' (擔心弄丟籌碼就別sell call，擔心接貨就別sell put). Note: his '沒想到把家用都蝕精光' line is a wry, self-deprecating remark in a post that actually reassures (his sold-call shares returned to his account), not a stated doctrine that covered calls 'wipe out gains' — the card should not present it as a general risk principle.

## Pocket Money on Volatility, Not the Return Engine

「我只是sell call賺點波動的零用錢，沒想到把家用都蝕精光。」 — "I only sell calls to earn a bit of pocket money (零用錢) off volatility — and then I went and nearly lost the household budget." The second clause is a wry, self-deprecating aside, not a risk doctrine: the post it sits in is actually reassuring. The HK$12 and HK$12.5 calls he had sold before CNOOC's ex-dividend date went unexercised and the shares returned to his account, and he asks rhetorically how he could ever lightly part with a stock earning around HK$90bn in a single half-year. The framing establishes a hierarchy: the holding is kept for its earnings and dividends; the call premium is incidental income skimmed off volatility. One mechanical note for HK practitioners: HK stock options are American-style — exercisable on any business day up to and including the last trading day — so a short call written through an ex-dividend window genuinely can be assigned early. His calls survived because exercise made no economic sense for the buyer while the option still carried time value above intrinsic, not because the position was protected from assignment.

## Pick the Leg by the Outcome You Can Tolerate

「擔心弄丟籌碼就別sell call，擔心接貨就別sell put！你有看到我sell call保險和電信么？」 — "If you're worried about losing your chips, don't sell calls; if you're worried about taking delivery, don't sell puts! Have you seen me selling calls on my insurers or telecom?" The selection rule is tolerance-based: write only the leg whose worst realistic outcome you can live with. He applies it to his own book — the names he is unwilling to risk having called away (his insurance and telecom holdings) simply get no calls written against them at all.

## Small Size, Full Collateral

「sell call 了1/4$中国海洋石油(00883)$ ，七月份，10.5元，收0.47元期權金。」 — the trade log itself shows the sizing discipline: calls sold on only one quarter of his CNOOC position (July expiry, HK$10.5 strike, HK$0.47 premium collected). In the same thread he made the intent explicit: 「我也沒有打算大量賣，也有不捨，但我尊重概率」 — "I have no plan to sell calls in size; I am reluctant too — but I respect the probabilities." Reluctance caps the quantity; probability justifies the small slice he does write. All of this sits inside his base-position-protected program: every call is written against shares he holds in full — fully collateralized, with no leverage — and he opposes ordinary investors touching derivatives in the first place.

## See Also

For the payoff anatomy of a written call and where its premium comes from, see deriv-option-payoff-anatomy and deriv-vega-and-theta; his reluctance to lose the shares is an earnings-power judgment in the spirit of eq-intrinsic-value. On the HK mechanics behind this card: hk-stock-option-american-style-assignment (why an ex-dividend window is the assignment window), hk-covered-call-shares-as-margin-collateral (the underlying shares can stand in for cash margin on the written call), hk-short-call-assignment-settlement-timing, and hk-special-dividend-option-contract-adjustment.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p1458:1457` — 狗不叫, post 222987869, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2202:2201` — 狗不叫, post 223121311, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3572:3571` — 狗不叫, post 223857399, verbatim ★AUTHOR words
- `hkex_stock_options_info_sheet:p002:0002` — grounding snapshot `hkex_stock_options_info_sheet`
