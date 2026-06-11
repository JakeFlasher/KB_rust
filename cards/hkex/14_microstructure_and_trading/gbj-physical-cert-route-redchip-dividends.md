---
schema_version: "cacg.v0"
id: "gbj-physical-cert-route-redchip-dividends"
title: "实物股票 Route: Withdrawing Red-Chips to Physical Certificates Escapes the 10% Dividend Tax"
reading_id: "14_microstructure_and_trading"
summary: "Red-chips taxed ~10% on dividends in street name at a HK broker become tax-exempt once withdrawn into physical certificates in the holder's own name, for mainland and HK holders alike. Converting an H-share gains nothing — the 10% still applies. The swap is done at the share registrar, can be delegated to a proxy if the broker permits, and dividends then pay by cheque or HK-bank credit."
tags: ["xueqiu-2022h1", "hong-kong", "dividend-tax", "red-chip", "physical-certificates", "share-registrar", "h-shares"]
citations:
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1548:1547"
    chunk_hash: "48b54663a28df04f51e425084ca0074410f2a57ff99daf9e8d174136c0935aec"
    page_range: [1548, 1548]
    quote: "762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1508:1507"
    chunk_hash: "ed185b3db3b33167d7e5ca10658fc531357da73d1c9028c725620d8c708498cf"
    page_range: [1508, 1508]
    quote: "所有股票都可以辦理，但是只有紅籌股才可以逃稅"
    edge_type: "supports"
  - source_id: "goubujiao_xueqiu_corpus"
    chunk_id: "goubujiao_xueqiu_corpus:p1514:1513"
    chunk_hash: "c8aebb139592de00a9254a878ff136a2111aec811e167a82af5e180f6cd49fb1"
    page_range: [1514, 1514]
    quote: "可以，但是依然要10%稅，所以轉成石鼓變得沒有意義"
    edge_type: "supports"
  - source_id: "ifec_physical_share_certificates"
    chunk_id: "ifec_physical_share_certificates:p001:0001"
    chunk_hash: "dd4f0ff13577b027d0ddbd43b8ebbf3e942a446be51f016a20b0e7eea715a39e"
    page_range: [1, 2]
    quote: "make sure that they are registered in your name, so that the share registrar can send you dividends"
    edge_type: "supports"
card_hash: "77589dc327577c7b10ea5c675b4cab25f290bf96ca4d17a1fae5eaa236a30aff"
---

## Thesis

For a red-chip otherwise taxed in street name, withdrawing to physical/registered shares (实物股票) held in your own name escapes the ~10% dividend tax — for any holder, mainland or HK. The route only works for red-chips; converting an H-share to physical still incurs the 10%, making it pointless. Conversion is done at the share registrar (generally requires being in HK) but can be delegated to a proxy if your broker permits; dividends on physical shares pay by cheque or HK-bank credit.

## The route: out of the nominee, into your own name

Asked which red-chips actually pay the dividend tax, the author scoped it himself: 「762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免」 — China Unicom (762), CNOOC (883) and China Mobile (941) held at a Hong Kong brokerage carry +10% on dividends, but withdrawing the shares into physical form ("提取實物") earns the exemption. In the same reply he named other red-chips (392, 934) that pay tax-free even in street name, which is why the route only matters for a red-chip *otherwise* taxed in nominee holding. The exemption is not residency-based: he stated explicitly that Hong Kong and mainland persons alike pay no tax once the shares are held physically in an individual's own name, and gave the rationale that a red-chip's parent company is incorporated in Hong Kong, where individual investors are by rule not taxed on dividends.

## Where it fails: H-shares

The conversion facility is universal, the tax escape is not: 「所有股票都可以辦理，但是只有紅籌股才可以逃稅」 — every stock can be processed into certificates, but only red-chips escape the tax. Asked whether China Telecom (728, an H-share) could be converted too, he answered 「可以，但是依然要10%稅，所以轉成石鼓變得沒有意義」 — you can, but the 10% tax still applies, so converting it to physical (石鼓 is his typo for 實物, physical shares) becomes meaningless. The decision rule is therefore: check the incorporation form first; the physical route changes nothing for an H-share's withholding.

## Mechanics at the registrar

The withdrawal is handled at the share registrar (過戶處), which as a default means showing up in Hong Kong; the workaround the author gave is delegation — if your brokerage permits you to authorize someone else to withdraw your position, the registrar side accepts a proxy acting for you. Once certificates are registered in your own name, dividends come from the registrar rather than through the broker: he confirmed both payment forms are available and selectable, but restricted to a Hong Kong bank account — i.e., cheque or direct credit to an HK bank. This rides on the standard registered-holder channel that official investor guidance describes for physical certificates: "make sure that they are registered in your name, so that the share registrar can send you dividends."

## See Also

For the custody backdrop — why brokerage-held shares are not in your name to begin with — see hk-ccass-nominee-vs-registered-shareholder; for the registrar-pays-the-registered-holder mechanics, hk-physical-certificate-dividend-registration; and for the treaty side of mainland dividend withholding rates, hk-mainland-dividend-withholding-cdta. On the CFA side, mt-institutional-setting-market-types covers how institutional structure (custody and registration form) shapes investor outcomes, fra-income-tax-accounting the treatment of withholding-type taxes, and eq-payout-policy-and-growth why after-tax dividend yield is the figure that matters for payout-driven holdings.

## Sources

Every Chinese quote above is the author's own verbatim wording from the ingested Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — see the Dated State / tags for its 2022-H1 weighting.

- `goubujiao_xueqiu_corpus:p1548:1547` — 狗不叫, author reply c248111703 (post 222999679), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p1508:1507` — 狗不叫, author reply c245571951 (post 222999679), verbatim ★AUTHOR words
- `goubujiao_xueqiu_corpus:p1514:1513` — 狗不叫, author reply c246170286 (post 222999679), verbatim ★AUTHOR words
- `ifec_physical_share_certificates:p001:0001` — grounding snapshot `ifec_physical_share_certificates`
