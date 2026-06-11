---
schema_version: "cacg.v0"
id: "gbj-no-leverage-bottom-line"
title: "No leverage to take delivery: size sell-puts to deliverable cash, never oversell"
reading_id: "11_risk_management"
summary: "He sizes every sell-put to the cash he actually has ready to take full delivery — never more — and refuses to let assignment draw on margin: using financing to take delivery is 'looking for death' (用融資是找死). No leverage is his stated bottom line; once a name is already large and deliverable cash is exhausted, he stops selling puts on it."
tags: ["xueqiu-2022h1", "sell-put", "no-leverage", "position-sizing", "cash-secured", "risk-management", "options", "margin"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2116:2115"
    chunk_hash: "2db4d1509e64d1443b6c3823cbdaae6ce698db7f5bdf71f5ce7524696d8a8d00"
    page_range: [2116, 2116]
    quote: "用融資是找死"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p2098:2097"
    chunk_hash: "a64e072da3faed221ba5a4491502747eb3a9da082654067e6fc372c64600d7be"
    page_range: [2098, 2098]
    quote: "我sell put有多少錢就接多少的貨，絕不多賣。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p5510:5509"
    chunk_hash: "9a75be9e8e5900634f6657213f92a33761cf200c015965e6dd84e639c17c08a6"
    page_range: [5510, 5510]
    quote: "大鵝已經很多了，我也沒有錢接貨了"
    edge_type: "supports"
card_hash: "89931ca10b4acdf0c90abfc52a23d6dd4f595b6a9c509a9e1817e2f62e682510"
---

## Thesis

His hard bottom line is to sell puts only up to the cash he actually has to take delivery, never more, and never finance assignment with margin — using financing to take delivery is 'looking for death'.

## The Sizing Rule: Deliverable Cash Bounds the Put Book

The rule is mechanical, not aspirational: 「我sell put有多少錢就接多少的貨，絕不多賣。」 — "when I sell puts, however much money I have is how much stock I take delivery of — absolutely never sell more." The number of puts written is set by the cash prepared for full assignment, not by margin capacity, premium appetite, or conviction in the name. In the same reply he contrasts this with accumulators, which he rejects because their worst case cannot even be stated; a fully collateralized short put, by contrast, has a known, pre-funded worst case.

One practical refinement: "deliverable cash" means cash that will be available by assignment, not only cash idle in the account today. He declined to write more June-expiry puts but wrote a July-expiry one because a CNOOC dividend would land in mid-July, before expiry: 「我是沒錢接六月份的了，但是中海油七月中股息到賬，所以我才選擇sell put七月到期的」 — "I have no money to take June delivery, but CNOOC's dividend arrives mid-July, which is why I chose a sell put expiring in July." No incoming cash, no put.

## Financing Assignment Is "Looking for Death"

A reader asked whether, when fully invested, assignment would simply draw on margin financing automatically. His entire answer: 「用融資是找死」 — "using financing is looking for death." This is the cash-secured half of a symmetric pair he states in the post body itself: 「高危遊戲，手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！」 — this is a high-risk game: never sell a call without 100% of the underlying stock in hand; by the same token, never sell a put without the full cash to take delivery prepared — "it can kill." He restates the floor as a character rule, not merely a sizing rule: 「不用杠桿是底線，忌貪！別動任何歪念！」 — "not using leverage is the bottom line — guard against greed! Entertain no crooked ideas!" And he opens the very same post with 「本人一向反對普通投資者觸碰任何的衍生工具」 — he has always opposed ordinary investors touching any derivatives, sell calls and sell puts included; this discipline is how he runs a game he says most people should not play at all.

## When Capacity Is Used Up, Stop

Urged during the 2022 drawdown to concentrate on Tencent instead, he refused on capacity grounds alone: 「大鵝已經很多了，我也沒有錢接貨了」 — "the big goose (大鵝, his nickname for Tencent) is already a big holding, and I have no money left to take delivery." Once a name is already large and the deliverable cash is exhausted, the rule says stop selling puts on it — there is no conviction override and no topping up with financing.

## See Also

For the institutional framing of the same discipline, see rm-risk-objectives-and-tolerance (risk tolerance as a hard constraint on strategy), rm-risk-type-taxonomy (leverage as its own risk channel), and pm-portfolio-constraints (liquidity and capital constraints on sizing) in the cfa deck. On HK mechanics, hk-stock-option-american-style-assignment explains why delivery cash must be ready before expiry — assignment can arrive early; hk-short-call-assignment-settlement-timing covers how quickly settlement follows; and hk-covered-call-shares-as-margin-collateral details the collateral treatment on the sell-call half of his symmetric rule.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p2116:2115` — 狗不叫, author reply c247433103 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p2098:2097` — 狗不叫, author reply c247180488 (post 223114673), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p5510:5509` — 狗不叫, author reply c248219219 (post 224796566), verbatim ★AUTHOR words
