---
schema_version: "cacg.v0"
id: "gbj-hk-option-costs-and-lot-mechanics"
title: "HK option mechanics: check the share count (股數) every time; stamp duty on exercise, not expiry"
reading_id: "14_microstructure_and_trading"
summary: "HK stock option contract mechanics from 狗不叫's practice: the per-contract share count is not fixed (500, 1000, and 1105 all appear in his threads), so check 股數 before every trade; special dividends — not ordinary ones — reset both strike and multiplier and can create odd lots (碎股); exercise incurs stamp duty while expiry avoids it; and his terse counter-estimate of HK exercise cost is roughly 0.3%."
tags: ["xueqiu-2022h1", "dated-levels", "hk-options", "contract-multiplier", "stamp-duty", "exercise-assignment", "trading-costs", "corporate-actions"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p255:0254"
    chunk_hash: "8df8c04c8b3bb57e41692625cf88334d28ac6fc8e51728a5f330a631eccf9471"
    page_range: [255, 255]
    quote: "每次一定看淸楚股數！"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p3747:3746"
    chunk_hash: "4d73dab985a527267778e9d1f1a8b0c72dafa4294a62f93052d9a79359e33e38"
    page_range: [3747, 3747]
    quote: "一張1000股哦，別弄錯"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1859:1858"
    chunk_hash: "cb0d3168bf15db16a20c92d2c7ea29b0e58b9eb9fd21550ca28b126a3f170da0"
    page_range: [1859, 1859]
    quote: "行權一樣要交印花稅，但是沒有行權就省下了"
    edge_type: "supports"
  - source_id: "ird_stamp_duty_hk_stock_rates"
    chunk_id: "ird_stamp_duty_hk_stock_rates:p001:0000"
    chunk_hash: "d81b02baacc9403654857e74d5000bb8f0b3f3c0debd74d30618d2bd185e69b3"
    page_range: [1, 1]
    quote: "Contract Note for sale or purchase of any Hong Kong stock 0.1% of the amount of the consideration or of its value on every sold note and every bought note"
    edge_type: "supports"
card_hash: "6b1690a1aa29358bff85549e669ecb67da3ea632b51dff54caeb31f91e259a7c"
---

## Dated State

All specific strikes and levels below come from the author's 2022-H1 Xueqiu corpus snapshot (a handful of utterances run to 2026-03) and are NOT durable recommendations. Dated levels named in this card: a CNOOC (00883) call sold at a 12.5 strike, adjusted after a special dividend to an 11.31 strike with 1105 shares per contract; a China Mobile put at a 49 strike carrying a 500-share multiplier; and Tencent (00700) sell-put strikes of 360 and 340 in a roll-down cost comparison.

## Thesis

On HK equity options, the author asserts the exercise/assignment cost is about 0.3% — a terse one-word reply pushing back on a commenter's "nearly 1%" estimate, with no methodology given. Contract size is NOT fixed and must be checked every time (每次一定看淸楚股數): the author warns that many people wrongly assume one contract is 100 shares; in one exchange he tells a follower a contract is 1000 shares (context-bound to that trade), a commenter's 500-share multiplier for a China Mobile put goes uncorrected, and the author's own CNOOC example shows a special dividend adjusting the strike to 11.31 with the multiplier reset to 1105 shares per contract, creating odd lots (碎股) — only special dividends trigger this adjustment; ordinary dividends do not. Exercise incurs stamp duty, but letting an option expire unexercised avoids it; the author cites this saved stamp duty as one cost advantage of selling options over transacting in the underlying (the broader "premium-selling is cheaper than trading stock" framing is the card's synthesis, not his stated thesis).

## Always Verify the Contract Share Count (股數)

The portable discipline is verification, not any single number. When a follower copied him into a China Mobile put, his immediate reply was 「每次一定看淸楚股數！」 — "every single time, look carefully at the share count!" The follower then read the contract specification and reported a 500-share multiplier on that contract, and the author elsewhere notes that many people wrongly assume one contract is 100 shares. When another follower announced he would simply follow the author's trades, the author warned 「一張1000股哦，別弄錯」 — "one contract is 1000 shares, don't get that wrong." That 1000 is context-bound to the specific contract being copied, not a universal HK convention: across his own threads the multiplier shows up as 500, 1000, and 1105 shares per contract. Before selling any HK stock option, open the contract specification and confirm the share count; never infer it from another name, another month, or a US-market habit.

## Special Dividends Reset Strike and Multiplier

His CNOOC position shows why the count drifts. He sold a call at the 12.5 strike; the stock then went ex on a special dividend, and the contract was adjusted to an 11.31 strike with the multiplier reset to 1105 shares per contract — total contract value stayed roughly equal (12.5 x 1000 versus 11.31 x 1105), but the adjusted position generated odd lots (碎股, odd-lot share parcels he called a nuisance). He is explicit that only special dividends trigger this adjustment: ordinary dividends leave the strike and multiplier untouched, and an option exercised after a regular ex-date still settles at its original strike. After any special-dividend event on an underlying you have options on, re-check both the strike and the 股數 before assuming anything about your position.

## Stamp Duty: Exercise Pays, Expiry Saves

Answering a follower who hoped being assigned would also dodge stamp duty, he wrote 「行權一樣要交印花稅，但是沒有行權就省下了」 — "exercise still incurs stamp duty, but with no exercise that cost is saved." Exercise or assignment is a real stock transfer, so HK stock stamp duty (0.1% of the consideration on each of the bought and sold notes, per the IRD schedule) applies just as it would to trading the shares outright; an option that expires unexercised involves no transfer and no stamp duty. He applies the same arithmetic when rolling: moving a Tencent sell put from the 360 strike down to 340 means that, if assigned, both commission and stamp duty are paid on the smaller consideration. He repeatedly cites this saved stamp duty as a cost advantage of collecting premium over transacting in the underlying — though the broader "premium-selling is cheaper than trading stock" claim is a synthesis from these remarks, not a thesis he stated in those words. On all-in exercise cost, when a commenter guessed HK exercise fees run nearly 1%, his entire reply was the single token "0.3%" — no methodology, conditions, or fee breakdown — so treat it as a practitioner's rough counter-estimate to check against your own broker's fee schedule, not a quotable figure. Note the setting: the stamp-duty remark sits in the thread of his risk-disclaimer post, where he insists option selling stay fully collateralized (no call sold without 100% of the underlying shares, no put sold without the full cash to take delivery — no leverage) and states that he opposes ordinary investors touching derivatives at all.

## See Also

For the official mechanics behind these habits, see hk-stock-transfer-stamp-duty-per-side (the 0.1%-per-side IRD rate), hk-special-dividend-option-contract-adjustment (the exchange's adjustment rule for special dividends and other capital events), and hk-stock-option-american-style-assignment together with hk-short-call-assignment-settlement-timing (assignment can arrive any day, which makes the share-count and strike re-check urgent rather than optional). On the cfa side, mt-effective-cost-trade-benchmark and mt-implementation-shortfall give the standard framework for the all-in trading-cost comparisons he is making, and mt-order-types-market-limit-stop covers the surrounding order-mechanics vocabulary.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p255:0254` — 狗不叫, author reply c244839614 (post 222375639), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p3747:3746` — 狗不叫, author reply c247010174 (post 223990393), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p1859:1858` — 狗不叫, author reply c245756661 (post 223114673), verbatim ★AUTHOR words
- `ird_stamp_duty_hk_stock_rates:p001:0000` — grounding snapshot `ird_stamp_duty_hk_stock_rates`
