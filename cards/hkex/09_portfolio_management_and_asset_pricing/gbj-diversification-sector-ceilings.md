---
schema_version: "cacg.v0"
id: "gbj-diversification-sector-ceilings"
title: "Sector position ceilings: spread within the industry, don't herd into the consensus favorite"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "He caps each industry at an explicit weight ceiling, trimming China Mobile and adding China Telecom \"to control the industry's upper-limit position\" rather than over-weighting one name. He sarcastically recounts the 2022 herd urging him to drop Alibaba sell-puts and pile everything into Tencent, and says his non-stock allocation is cash plus arbitrage positions plus A-shares."
tags: ["xueqiu-2022h1", "diversification", "concentration-risk", "position-sizing", "sector-ceiling", "portfolio-construction", "hk-stocks"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p5275:5274"
    chunk_hash: "c6b2a24f250402c08598003daafd2302452139698d16e735d46fc29401304ef0"
    page_range: [5275, 5275]
    quote: "上個月還有很多人勸我sell put阿里不如都押700，反正就是阿里巴巴十萬個不好，騰訊地位不可動搖"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p5280:5279"
    chunk_hash: "0a2376a5eeed4c9ab0c770132980dd0a4ace49437597c5c9116e4346d604cced"
    page_range: [5280, 5280]
    quote: "加了728，控制行業上限倉位。"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p7086:7085"
    chunk_hash: "3dff233321bba2ac618385beff813b3e2da3840d00b49f0824a9d78a315b2183"
    page_range: [7086, 7086]
    quote: "現金+套利交易倉位+ A股"
    edge_type: "supports"
card_hash: "09111dd1111db0f81203a109cda79f46dba7413e5388692cca3f808f8b9ed855"
---

## Thesis

In a thread (post 224598323) the author sarcastically recounts that the prior month many people urged him to stop sell-putting Alibaba and instead pile everything into 700 (Tencent), on the view that "Alibaba is a hundred-times bad and Tencent's position is unshakeable" — implicitly rejecting that herd-into-the-consensus-favorite approach. (Note: the accompanying "diversify, be a long-term investor / now everyone is dumbfounded" lines in that post are a forwarded comment from another user, 陈达美股投资, not the author, and the author does not himself assert that Tencent fell and Alibaba rose.) The author does explicitly manage industry concentration via a position ceiling: when asked why he cut China Mobile (941), he replied that he added China Telecom (728) "to control the industry's upper-limit position" — i.e. he spreads weight within a sector to keep any one name inside its allotted cap rather than over-weighting the leader. Separately, when asked what his non-stock allocation consists of, he answered: cash + arbitrage positions + A-shares.

## The herd episode: "just pile it all into 700"

In July 2022 the author reposted a lament by 陈达美股投资 and added his own parallel anecdote: 「上個月還有很多人勸我sell put阿里不如都押700，反正就是阿里巴巴十萬個不好，騰訊地位不可動搖」 — "Last month plenty of people were still urging me that rather than sell-putting Alibaba I should just bet everything on 700 [Tencent] — the line being that Alibaba had a hundred thousand things wrong with it and Tencent's position was unshakeable." Read the attribution carefully: the explicit exhortation in that post — "sounded reasonable, now everyone is dumbfounded; diversify, be a long-term investor" — belongs to the reposted commenter 陈达美股投资, not to the author. What the author himself supplies is the sarcastic recounting of the crowd's advice; his rejection of herding into the consensus favorite is carried by tone and by his choice to forward that comment, never stated first-person as a rule. Treat this leg as an inferred stance, not a quoted principle. Note also that the sell-put practice under discussion is, throughout his corpus, fully collateralized and run with no leverage — he sells a put only against 全額接貨的錢, the full cash to take delivery — so the crowd's debate was about which name to concentrate in, never about adding leveraged risk.

## The explicit mechanism: an industry weight ceiling

The one leg he does state in his own words as an operating rule sits in the same thread. When @Lhaj asked why he had trimmed China Mobile (941), he answered: 「加了728，控制行業上限倉位。」 — "Added 728 [China Telecom, 00728], to control the industry's upper-limit position." The mechanics: the industry (here telecom) carries an explicit maximum weight in his book. Rather than letting the leading carrier absorb the whole sector budget, he trims the larger holding and adds a second name in the same industry, spreading the fixed sector allocation across carriers so that no single name breaches the ceiling. The cap binds at the industry level; the within-sector spread is how he stays under it without giving up the sector exposure.

## What sits outside the stock book

In a much later thread (running to 2026-03, beyond the 2022-H1 core of the corpus), @mn02rx asked whether the apparent 30%-plus of his portfolio not in the main stock lines was still cash or simply A-share weight. He answered: 「現金+套利交易倉位+ A股」 — "cash + arbitrage-trade positions + A-shares." He attaches no label to this bucket; it is a plain factual answer about what the non-stock allocation contains. The observable point is only that the residual is itself spread across three sleeves — cash, arbitrage positions, and A-shares — rather than parked in a single pile.

## See Also

For the textbook treatment of why spreading weight across names reduces single-name exposure, see pm-diversification-and-correlation and pm-systematic-vs-idiosyncratic-risk; his explicit industry cap is a live instance of the constraint framing in pm-portfolio-constraints. The fully collateralized sell-put context behind the herd episode runs on the mechanics covered in hk-stock-option-american-style-assignment and hk-covered-call-shares-as-margin-collateral.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p5275:5274` — 狗不叫, post 224598323, verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p5280:5279` — 狗不叫, author reply c247917450 (post 224598323), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p7086:7085` — 狗不叫, author reply c398436995 (post 366413082), verbatim ★AUTHOR words
