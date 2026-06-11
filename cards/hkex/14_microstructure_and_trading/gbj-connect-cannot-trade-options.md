---
schema_version: "cacg.v0"
id: "gbj-connect-cannot-trade-options"
title: "Stock Connect (港股通) cannot trade HK options"
reading_id: "14_microstructure_and_trading"
summary: "港股通 (Stock Connect) accounts have no access to HK options: asked directly, the author answered with one word, 沒法. To write calls against shares held outside the options account, he advises simply transferring the stock into the options account. His timing for the Connect dividend-dodge buyback rests on two years of observation and statistics, not a formula."
tags: ["xueqiu-2022h1", "stock-connect", "options-access", "account-mechanics", "dividend-dodge", "hk-options", "microstructure"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p320:0319"
    chunk_hash: "7a88db24e7bd584864feafb48ab5d35500b9dff907a97a1384d5633793de4fec"
    page_range: [320, 320]
    quote: "沒法"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p349:0348"
    chunk_hash: "d4ec8fdbbc151a45b668e78a7d19d2e9ed09b79f7eb833637cf8fbdaf471eb00"
    page_range: [349, 349]
    quote: "你把股票轉到期權戶口就可以了"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3761:3760"
    chunk_hash: "ac72d358cad8d8062ca044af9abb7e2d519d3467f1023a7c53ed31aa85d8ebb5"
    page_range: [3761, 3761]
    quote: "這兩年的觀察和統計"
    edge_type: "supports"
  - source_id: "hkex_stock_connect_explained"
    chunk_id: "hkex_stock_connect_explained:p009:0006"
    chunk_hash: "1b87ad84c57be8c76b57454786cd7cb77f48d9370a6343975972ba64f6c8f2aa"
    page_range: [9, 10]
    quote: "only A shares and ETFs are included in Shanghai and Shenzhen Connect. Other product types such as B shares, bonds, and other securities are not included"
    edge_type: "supports"
card_hash: "0c6a56e9d8a4d2ed033831b13d66634ec1ad7f3390773489e71fbe651ab9d98e"
---

## Thesis

港股通 (Stock Connect) accounts cannot trade HK options — the author confirmed '沒法' when asked '港股通没法做期权吧'. To run a covered call on shares held outside an options account, the author's advice was simply '你把股票轉到期權戶口就可以了' (transfer the underlying shares into your options account). When a commenter asked whether the Stock-Connect dividend-dodge ('逃权买回') buyback timing came from many years of observation or some inherent formula, the author answered it is from '這兩年的觀察和統計' — i.e. roughly the past two years of empirical observation and statistics, not a formula. (Note: the claim that HK platforms lack a one-click covered call, and that options therefore require a direct HK/international broker, originates from the commenter's question, not the author; treat as context, not as the author's asserted principle.)

## The Connect channel is a cash-equity pipe — 沒法

When a reader asked the author point-blank, 「财大，港股通没法做期权吧」 ("Stock Connect can't do options, right?"), his entire reply was 「沒法」 — "can't be done." That one word settles only the negative: the Southbound Connect channel gives a mainland account no route to HK stock options. He does not go on to prescribe which alternative account to open — the affirmative step is not stated in this exchange, so it should not be read into his answer. The official program design matches his one-word reply: HKEX's Stock Connect explainer states that "only A shares and ETFs are included in Shanghai and Shenzhen Connect. Other product types such as B shares, bonds, and other securities are not included" — that scope statement describes the Northbound leg, and the program in both directions is built as a channel for designated cash equities and ETFs, with derivatives simply not among the eligible products.

## Backing a written call: move the shares, not the order type

A commenter who had sold a call against shares already in his account asked whether he could just wait for exercise day and deliver, adding in passing that his HK platform seemed to have no direct interface for selling a call against held stock — that user-interface observation is the commenter's own, and the author neither confirmed nor denied it. The author's reply was purely mechanical: 「你把股票轉到期權戶口就可以了」 — "just transfer the stock into your options account, and that's it." Once the underlying sits in the same account as the short call, the position is the author's standard practice of base-position-protected, fully collateralized option selling: the shares are physically in place to be delivered into assignment, so the written call is never naked.

## Two years of observation, not a formula

On a separate thread a commenter asked whether the buyback timing of the Connect-holder dividend dodge — '逃权买回', selling ahead of the ex-date to step around the dividend event and repurchasing afterwards — was 「通过多年观察确定的，还是有什么内涵在里边」 (determined through many years of observation, or carrying some inherent logic). The author's answer narrows both framings: 「這兩年的觀察和統計」 — "these two years' observation and statistics." The window is two years, not many, and the basis is empirical counting, not any formula. A practitioner should therefore treat the timing as a short-window empirical regularity that needs ongoing re-validation, not as a structural law of the market.

## See Also

For the official scope behind 沒法, see hk-stock-connect-excludes-derivatives; for what happens once the shares sit in the options account, see hk-covered-call-shares-as-margin-collateral and hk-short-call-assignment-settlement-timing; for why the 逃权买回 trade exists for Connect holders at all, see hk-mainland-dividend-withholding-cdta. On the CFA side, mt-institutional-setting-market-types and mt-execution-systems-quote-vs-order-driven cover how venue and account structure determine which order types and instruments are even executable, and mt-competition-within-among-markets covers why product scope differs across competing access channels.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p320:0319` — 狗不叫, author reply c245245932 (post 222375639), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p349:0348` — 狗不叫, author reply c248103841 (post 222375639), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3761:3760` — 狗不叫, author reply c247127787 (post 223990393), verbatim ★AUTHOR words
- `hkex_stock_connect_explained:p009:0006` — grounding snapshot `hkex_stock_connect_explained`
