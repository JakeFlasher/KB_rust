---
schema_version: "cacg.v0"
id: "gbj-special-dividend-strike-adjustment"
title: "HK option strike adjustment: special dividends adjust, ordinary dividends do not"
reading_id: "07_derivatives_and_volatility"
summary: "Only a special dividend adjusts an HK option: strike cut, per-contract share count raised, contract value roughly preserved, odd lots created (CNOOC 12.5 to 11.31, 1000 to 1105 shares). An ordinary dividend, however large, leaves the strike unchanged — Shenhua exercised at the original $26 post-ex. Verify a venue's adjustment policy before selling options across a big dividend."
tags: ["xueqiu-2022h1", "dated-levels", "hong-kong", "stock-options", "contract-adjustment", "special-dividend", "dividends", "option-writing"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p001:0000"
    chunk_hash: "cb6c87580777784f8f6fa5519b4c0ed4e3ce7688ab13425551fcf4f85c2fc7f0"
    page_range: [1, 1]
    quote: "由於是特別股息，行使價被調整為11.31元，每一張期權代表股數被調整為1105股，這比較有趣。派特別股息錢每張合約價值12.5X1000股，現在是11.31X1105股，合約價值是差不多的。但是這產生了一些碎股，真麻煩。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2048:2047"
    chunk_hash: "6a452d70d4a1a7ff04dc36c77514774cb115922c3bd76df1f3c18d9695dfa36e"
    page_range: [2048, 2048]
    quote: "中國神華不是特別股息，如果除淨日之前行權人沒有行使期權，除淨后也是按照原來的價格行權，也就是$26。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p4261:4260"
    chunk_hash: "014a367ea622448a00f5b492e2731802525c05080b49667ca1029dc992f1f798"
    page_range: [4261, 4261]
    quote: "我比較擔心萬一他大額派股息，但是又不屬於特別股息，不調整行權價，sell put可能會有麻煩"
    edge_type: "supports"
  - source_id: "hkex_contract_adjustments_chap08"
    chunk_id: "hkex_contract_adjustments_chap08:p001:0001"
    chunk_hash: "d1603450eed816af4fc4ccd77a616540443c89d240d18ae0fb32b532b8f0c4b9"
    page_range: [1, 2]
    quote: "the Exchange will not perform any capital adjustment on option positions unless the value of the payment is 2 per cent or more of the share's closing price on the day of the announcement"
    edge_type: "supports"
card_hash: "fde97ee1f811aa4fffa58e2105a402296d29974fb78ba304e923aa264d0a9e03"
---

## Dated State

All specific levels in this card are from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. Dated levels: CNOOC (00883) calls written at a 12.5 strike, adjusted to 11.31 with the per-contract share count raised from 1,000 to 1,105 after the June 2022 special dividend; China Shenhua (01088) June 2022 calls struck at $26, with the stock near 26 before and roughly 22.65 after the ex-date.

## Thesis

HK options/warrants are adjusted only for SPECIAL dividends — strike is cut and the share multiplier raised so contract value stays roughly constant (creating messy odd-lots) — while ordinary dividends leave the strike unchanged, so post-ex exercise of an ordinary-dividend option still settles at the original strike.

## Special dividend: strike cut, multiplier raised, value preserved

The author had sold CNOOC calls at 12.5 before the company's special dividend went ex, then reported the adjustment in detail: 「由於是特別股息，行使價被調整為11.31元，每一張期權代表股數被調整為1105股，這比較有趣。派特別股息錢每張合約價值12.5X1000股，現在是11.31X1105股，合約價值是差不多的。但是這產生了一些碎股，真麻煩。」 — because it was a special dividend, the strike was cut to 11.31 and each contract reset to represent 1,105 shares; one contract was worth 12.5 × 1,000 shares before the payout and 11.31 × 1,105 shares after, so contract value is about the same — but the rescaling threw off odd lots (碎股), which he found a real nuisance. The mechanics: the exchange rescales strike and share multiplier together so neither side of an open position gains or loses from the distribution itself; the residue is the odd-lot share amounts the writer later has to clean up.

This is the exchange's published rule, not the author's invention: under HKEX's contract-adjustment procedures an ordinary cash dividend triggers no capital adjustment, and even for special/extraordinary distributions "the Exchange will not perform any capital adjustment on option positions unless the value of the payment is 2 per cent or more of the share's closing price on the day of the announcement". The numeric threshold is HKEX's, not his — he never stated one.

## Ordinary dividend: no adjustment, post-ex exercise at the original strike

When a reader who had sold June China Shenhua calls at 26 asked whether post-ex exercise would key off the post-ex price (~22.65), he answered: 「中國神華不是特別股息，如果除淨日之前行權人沒有行使期權，除淨后也是按照原來的價格行權，也就是$26。」 — Shenhua's payout was not a special dividend, so if the holder has not exercised before the ex-date, exercise after the ex-date still settles at the original price, $26. A wording note: he literally said 不是特別股息 ("not a special dividend") rather than using the term "ordinary dividend"; that mapping is the gloss here, and the example actually sharpens the rule — Shenhua's regular payout was large (the stock dropped from about 26 to 22.65 across the ex-date), yet the strike did not move. For an ordinary dividend the strike simply does not move, no matter how large the payout; the option holder, not the contract terms, absorbs the ex-dividend drop.

## His caveat: confirm the adjustment policy before selling across a large dividend

Weighing puts on MOMO, a US-listed name, he flagged exactly this mechanism as the risk: 「我比較擔心萬一他大額派股息，但是又不屬於特別股息，不調整行權價，sell put可能會有麻煩」 — he was worried that a large payout that did not qualify as a special dividend would leave the strike unadjusted, and the short put could run into trouble, since an unadjusted put writer silently eats the ex-dividend drop. Commenters offered US broker anecdotes on both sides, but he asserted no US rule himself; his own position stayed at uncertainty. The working instruction that follows: before selling an option across a large announced dividend, confirm that venue's adjustment policy for that specific distribution — outside HK he treated the answer as unknown.

The caveat sits inside his hard risk frame of base-position-protected, fully collateralized option selling with no leverage. In the same thread that carried the Shenhua answer he declared 「手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！」 — never sell a call without 100% of the underlying stock in hand, never sell a put without the full cash to take delivery; it can kill you — and he is explicit that he 「一向反對普通投資者觸碰任何的衍生工具」, has always opposed ordinary investors touching any derivative.

## See Also

The official rule behind this card is in hk-special-dividend-option-contract-adjustment (HKEX's 2%-of-close threshold for special distributions). hk-stock-option-american-style-assignment explains why pre-ex early exercise is live in HK (stock options are American-style), and hk-short-call-assignment-settlement-timing covers what an assigned short writer faces at settlement. On the cfa side, deriv-option-payoff-anatomy covers the strike/payoff anatomy the adjustment is designed to preserve, deriv-put-call-parity shows how expected dividends enter option prices, and cb-conversion-feature-mechanics describes the analogous anti-dilution adjustments on convertible terms.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p001:0000` — 狗不叫, post 222375639, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2048:2047` — 狗不叫, author reply c247047700 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p4261:4260` — 狗不叫, post 224217701, verbatim ★AUTHOR words
- `hkex_contract_adjustments_chap08:p001:0001` — grounding snapshot `hkex_contract_adjustments_chap08`
