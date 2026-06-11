---
schema_version: "cacg.v0"
id: "gbj-redchip-vs-hshare-withholding"
title: "Red-chip vs H-share dividend withholding: the 提取實物 (physical delivery) escape"
reading_id: "14_microstructure_and_trading"
summary: "Dividend withholding on HK-listed Chinese SOEs depends on issuer structure and holding form: some red-chips (392/934) pay nothing even in street name; others (762/883/941, incl. China Mobile) lose ~10% at a brokerage but become exempt after physical delivery into the holder's own name; a true H-share (China Telecom 728) pays ~10% regardless, so verify each ticker."
tags: ["xueqiu-2022h1", "hong-kong", "dividend-withholding", "red-chip", "h-share", "physical-certificates", "custody"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p314:0313"
    chunk_hash: "ec46545c6edcfe8b6fed1b8cc6a1ea69c70afbd9d2e86995ffe997b25e6aa0b2"
    page_range: [314, 314]
    quote: "有一些不用交，有一些要交10%，比如392、934，完全不用交，762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p340:0339"
    chunk_hash: "8291f83324d25e1fafb61a49213c884cbd2d9ef619f256e482290c3da365625c"
    page_range: [340, 340]
    quote: "因為是紅籌股，母公司在香港注冊，按例個人投資者不用交稅。"
    edge_type: "supports"
  - source_id: "ifec_physical_share_certificates"
    chunk_id: "ifec_physical_share_certificates:p001:0001"
    chunk_hash: "dd4f0ff13577b027d0ddbd43b8ebbf3e942a446be51f016a20b0e7eea715a39e"
    page_range: [1, 2]
    quote: "make sure that they are registered in your name, so that the share registrar can send you dividends"
    edge_type: "supports"
card_hash: "0d5150332266cd62eb3b938b7bb7d96fb778e74a852680b390d31e51e91c542c"
---

## Thesis

Whether a HK-listed Chinese SOE dividend is withheld depends on issuer structure (red-chip vs H-share) AND on how the shares are held, not on the H-share label alone. Per the author (POST 222375639 thread): some red-chips held at a HK brokerage in street name are fully exempt for individuals (e.g. 392/934), while others (e.g. 762/883/941, incl. China Mobile) incur ~10% withholding when held in street name — but taking physical delivery of the certificate ('提取實物', individual-name registration) exempts them. Crucially, this physical-delivery escape works only for red-chips (parent incorporated in HK — the author explicitly classifies China Mobile/941 as a red-chip, which is WHY converting it to physical removes the tax), whereas a true H-share like China Telecom (728) still pays ~10% even after conversion, making conversion pointless. Classification and the available exemption path are therefore ticker-specific — verify each name (the author's '移動是電訊不是' means China Mobile IS a red-chip and China Telecom is NOT, so the label is not obvious from the business). This is the author's personal brokerage experience, not formal tax guidance.

## The two-variable rule: what the issuer is, and how you hold it

Asked point-blank whether red-chips held in a Hong Kong brokerage account are all free of dividend tax, the author refused the blanket rule and answered ticker by ticker:

「有一些不用交，有一些要交10%，比如392、934，完全不用交，762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免」 — "Some pay nothing, some pay 10%. For example 392 (Beijing Enterprises) and 934 (Sinopec Kantons) pay nothing at all; 762 (China Unicom), 883 (CNOOC) and 941 (China Mobile) get +10% if held at a Hong Kong securities firm — but take physical delivery (提取實物) and they are exempted."

So even inside the red-chip family there are two tiers: names exempt in street name as-is, and names whose exemption only unlocks once the shares are withdrawn from the brokerage/nominee chain and registered in the individual investor's own name. He was explicit that the conversion itself is universally available but the tax benefit is not: 「所有股票都可以辦理，但是只有紅籌股才可以逃稅」 — "every stock can be processed [into a physical certificate], but only red-chips can escape the tax that way."

## Why the escape works for 941 — and fails for 728

When a commenter asked why China Mobile stops being taxed after conversion, the author gave the structural reason:

「因為是紅籌股，母公司在香港注冊，按例個人投資者不用交稅。」 — "Because it is a red-chip: the parent company is incorporated in Hong Kong, and by rule individual investors do not pay tax."

The classification is not guessable from the business. Asked whether China Mobile and China Telecom are both H-shares, he answered 「移動是電訊不是」 — compressed shorthand for "[China] Mobile IS [a red-chip]; [China] Telecom is NOT." China Mobile (941) has a HK-incorporated parent, so individual-name registration removes the withholding. China Telecom (728) is a true H-share with a mainland-incorporated issuer: he told another commenter the conversion can be done but 「依然要10%稅」 ("it still pays the 10% tax"), so converting 728 to physical achieves nothing.

Mechanically, the escape is a registration change. Shares at a brokerage sit in the clearing system under a nominee name; a certificate withdrawn from the system is registered in the holder's own name, and the registrar then pays the dividend to the individual directly — the state in which the red-chip individual-investor exemption applies. The IFEC's guidance on physical certificates makes the registration point explicitly: "make sure that they are registered in your name, so that the share registrar can send you dividends".

## Scope

All of this sits in one comment thread (2022-06-18 → 2022-07-07) in which the author re-stated his own brokerage and registrar experience to at least seven different interlocutors; it is confined to that thread in the corpus — practitioner detail from personal experience, not formal tax guidance and not corpus-wide doctrine. Treat each ticker's tier as a fact to verify with your broker and the share registrar for that specific name: some red-chips are exempt even in street name (a commenter named 越秀交通 and 冠德, and the author added 北控也是 — "Beijing Enterprises too"), others only after individual-name registration, and a true H-share has no escape path at all.

## See Also

For the custody machinery behind "street name", see hk-ccass-nominee-vs-registered-shareholder (electronic shares are registered to HKSCC Nominees, not to you) and hk-physical-certificate-dividend-registration (a withdrawn certificate must be registered in your own name for the registrar to pay you directly). hk-mainland-dividend-withholding-cdta covers the treaty side of withholding on mainland-incorporated issuers — the H-share leg of this card — and hk-stock-transfer-stamp-duty-per-side is a friction cost to weigh when moving positions. On the CFA side, mt-institutional-setting-market-types frames issuer and listing structures, eq-payout-policy-and-growth explains why withholding is material for payout-heavy SOEs, and fra-income-tax-accounting covers withholding versus effective tax rates.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p314:0313` — 狗不叫, author reply c248111703 (post 222375639), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p340:0339` — 狗不叫, author reply c246172648 (post 222375639), verbatim ★AUTHOR words
- `ifec_physical_share_certificates:p001:0001` — grounding snapshot `ifec_physical_share_certificates`
