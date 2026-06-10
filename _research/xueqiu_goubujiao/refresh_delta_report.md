# Corpus-refresh delta report (full-nested recrawl, re-judged 2026-06-11)

## What changed in the corpus

- Rebuilt `corpus_complete.md` from the crawler capture via the new committed builder
  (`sources/hkex/_registry/build_corpus_from_capture.py`); the old hand-built corpus had no producer script.
- Capture attested **INCOMPLETE** by the crawler (`meta.json`): timeline 266/277 posts (shortfall 11), 7 posts with comment shortfalls — recorded in `corpus_provenance.json`.
- 5,531 distinct comments (was 5,306): **+225 distinct, all commenter context — zero new 狗不叫 utterances** (the author layer is unchanged in both directions; 631 author-reply rows = 393 distinct ids).
- 4,235 comments now carry explicit nesting; 3 comments whose parents were previously un-crawled re-thread to their true depth.
- 242/266 post sections re-render byte-identical; the 24 differing sections are exactly the posts that gained rows.
- All AC-2/AC-3 gates re-run green: renderer determinism, corpus→PDF parity, ingest spike (189/192 bound + 3 principled attribution rejections — identical to the original run).

## Re-judgment of the 73 candidates (full-thread context)

All 73 candidates were independently re-judged against full nesting; every proposed verdict change was
adversarially refuted before acceptance. Result: **2 verdict changes (both upheld), 47 recurrence corrections.**

Verdict distribution: 21 faithful / 44 overgeneralized / 5 weak_evidence / 3 misattributed (counts unchanged; two titles swapped).

### overgeneralized → faithful: HK option liquidity is concentrated in a few mega-caps; pick market by liquidity not frequency

The original downgrade to overgeneralized rested entirely on the claim that the "don't chase a poor market-maker quote" element was "NOT supported by any of the three citation_seed quotes" and that "none of the cited posts (222517599, 223990393, 224093303) mention market makers (莊家) at all", with the only corpus MM-quote line (post 223114673) allegedly leaning opposite. That claim is false against the refreshed full-thread rendering of cited post 222517599: nested ★AUTHOR reply comment:c244934995 (answering comment:c244932108 "财主为啥不short短期的？") says "莊家買賣差價太大。剛好看到下個月的這只有個交易對手出價合適。" — the author…

### faithful → overgeneralized: Telecom sector framework: defensive vs growth, carriers vs cloud, upstream over downstream

The original pass kept 'faithful' by treating the missing carrier-cloud caveat as a MINOR temporal nuance, reading the toll-collector framing as the author's '2025 dismissive stance' versus a bullish 2022. The full nested corpus disproves that reading: dedup by comment id shows EVERY toll/tech-provider utterance is June 2022 — comment:c246500412 (2022-06-25, '肯定搞不過阿里雲和騰訊云，他們只負責漲過路費', reposted verbatim as post:223634416) and comment:c246526527 (2022-06-26, '電訊商就是賺流量錢的，阿里是技術提供方', the identical comment id placed under posts 223631520/223643275/223645343). There is no post-2022 dismissive cloud ut…

## Recurrence corrections (distinct author utterances, repost-duplicates collapsed)

47 of 73 counts were corrected (both directions — several were under-counted because the partial corpus
hid distinct author replies inside un-crawled threads):

- Sell puts only on companies you would happily own at the strike (cash-secured, b: 7 → 11
- Ordinary investors should not touch derivatives — and never short, never go nake: 2 → 5
- Avoid accumulators and complex multi-leg products with unbounded loss: 2 → 1
- Keep sold-put strikes near the money; deep-OTM puts earn too little: 4 → 5
- The wheel: take assignment then reverse into covered calls: 4 → 5
- Option exercise mechanics: who controls exercise, OTM vs ITM, early exercise: 3 → 5
- Option strike adjustment: special dividends adjust, ordinary dividends do not: 5 → 6
- Dividend-aware option timing: ex-date, assignment timing, and who collects the d: 4 → 8
- Sell-put underlying selection: choose for bounded downside and rich premium, not: 3 → 2
- Premium is the buyer's insurance cost and your edge — but a fat premium prices r: 3 → 6
- Refuse fat premiums on no-floor names: high IV is a warning, not free money: 3 → 4
- Fundamental-floor-anchored strike selection (oil-major case): 2 → 1
- Honest attribution and sizing of the option-income sleeve: 3 → 2
- Self-financing option overlay: size sell-puts to dividend inflow: 2 → 11
- HK option costs: ~0.3% exercise cost; lot size and stamp-duty mechanics: 4 → 10
- HK dividend tax: red-chip vs H-share withholding mechanics: 3 → 10
- 实物股票 route: convert red-chips to physical certificates to escape dividend tax: 4 → 9
- After-tax dividend yield drives peer selection (移动 vs 电信): 4 → 2
- Fund 分红 vs corporate 派息: dividend tax incidence and channel consistency: 3 → 5
- Total-return (dividend-inclusive) accounting through drawdowns: 4 → 3
- Business-value bull market: equity and dividend growth, not price: 2 → 4
- The discipline of inaction: research deeply, then often do nothing: 3 → 2
- Skepticism toward corporate denials, manipulation, and engineered rallies: 2 → 3
- AGM activism: owner mindset and pressing management on payout policy: 2 → 4
- Insider buying as a confidence/alignment signal: 3 → 2
- Read major-shareholder selling through the cheapest-execution lens: 1 → 2
- Small-cap minority liquidity trap; curate the shared book for liquidity: 3 → 4
- Insurer analysis: avoid opaque asset sides; sum-of-parts; ownership taxonomy: 3 → 5
- Oil-producer valuation: oil-price-to-earnings scenario sensitivity: 4 → 7
- SOE windfall-tax vs payout, and minority preference for dividends over growth ca: 4 → 5
- Telecom sector framework: defensive vs growth, carriers vs cloud, upstream over : 4 → 5
- Moats from regulatory difficulty, product-fit, and customer-acquisition economic: 3 → 4
- Decline signals: revenue/volume decline, side-bets, youth disengagement, ESG buz: 3 → 4
- Margin and cost-structure analysis: cost-plus, spread vs margin, pass-through, p: 4 → 5
- Balance-sheet quality: distributable cash, repatriation tax, receivables, levera: 3 → 6
- Buybacks: scale relative to size, and read-through screens: 3 → 5
- Oil macro: supply inelasticity, OPEC restraint, and refined-oil price controls: 3 → 4
- EV-penetration ceiling and treating unenacted policy as non-information: 3 → 4
- Bull-market preconditions and the policy-regime contrast: 1 → 2
- China credit/banking: mortgage-bank quality and no securitized-subprime contagio: 1 → 2
- Contrarian entry on clustered bad news; relative-strength bottoming and HK seaso: 2 → 3
- Regulatory-risk discipline and platform-risk acceptance in the China/HK environm: 2 → 4
- Sell on deteriorating leverage and admit the error: 2 → 3
- Cool-off discipline and avoiding wealth-destroyer vehicles: 2 → 5
- Broker/listing-venue operational mechanics: relisting transfers and share-class : 2 → 5
- Sell-put as passive accumulation at strike-minus-premium; price-conditioned cove: 2 → 4
- IR calls as due diligence and shareholder base as marketing asset: 2 → 1

## Where the new nested context mattered

47 candidates' bundles contained newly-crawled rows or full-nesting evidence the judges used
(mostly to *confirm* original readings with comment-id-level anchors; see `new_context_notes` per candidate
in `verifications_refresh.json`). The biggest single gain: post 379186770 (2026-03) grew 13 → 129 comments,
including the author's exchanges with @管我财 — context only, never citable as 狗不叫.

## Consumption

`verifications_refresh.json` supersedes `verifications.json` for card authoring: the AC-8 faithfulness
linter (`faithfulness_lint.py`, config `faithfulness_config.json`) reads it as the verdict/thesis/recurrence
authority. The original `verifications.json` is retained unmodified as the pre-refresh record.
