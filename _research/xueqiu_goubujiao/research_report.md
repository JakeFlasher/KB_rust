The grounding-books report confirms the corpus, identifies the four methods, and documents which existing CFA folders cross-link versus where new sources are needed. I now have everything required. Let me write the comprehensive report.

The report will be returned as my final response (per the instructions not to write .md report files). It is the deliverable the orchestration script expects.

# 狗不叫 (财主) — Investment Wisdom Distillation for a CFA-Framework Knowledge Deck

Author: 狗不叫 (handle "财主"), Xueqiu user 2424206371. Corpus: `_research/xueqiu_goubujiao/corpus_full.md` (266 posts, 597 author replies, 23 batch files, 2020-10-16 → 2026-03-12; ~95% of posts cluster in 2022-06/07). Verbatim quotes below are exact substrings of `corpus_full.md` with their `post_id`; English is a gloss, not a translation for citation. This report seeds a `/humanize:gen-plan` plan for English knowledge-cards carrying Chinese verbatim evidence.

---

## 1. Executive summary

狗不叫 is a Hong Kong value-and-income investor whose durable edge is a **disciplined fusion of fundamental conviction with a strictly-collateralized option-selling overlay, executed inside an intimate command of HKEX plumbing and the HK/China dividend-tax code.**

His stance is **owner, not trader**. He buys cash-generative, high-dividend HK blue chips (oil majors, telecoms, insurers, coal/power SOEs, select internet names), holds through drawdowns judged on a **total-return (dividends-included)** basis, and defines "bull market" by per-share equity and dividend growth rather than price. Around that core he runs a **non-directional premium program** — selling puts only at strikes where he genuinely wants the shares and holds the full cash to take delivery, plus modest covered calls for "pocket-money" volatility income.

The whole apparatus is governed by **one hard bottom line: no leverage, ever.** He sizes puts to deliverable cash (often to incoming dividends, keeping the program self-financing), bounds every single-name worst case to a small fraction of the book, refuses fat premiums on names with "no floor," and avoids exotics (accumulators) whose maximum loss he cannot state. His second, rarer edge is **mechanical and fiscal**: HK options need a non-Connect broker; red-chips escape dividend tax via physical-certificate conversion while H-shares cannot; special vs ordinary dividends adjust strikes differently; and HK option liquidity lives in only a few mega-caps.

**Headline takeaways for an HKEX-specific deck:**
1. **The option program is base-position-protected, NOT "covered" in the textbook sense.** He calls his own sell-calls "半裸" (half-naked) — sold against shares he plans to *acquire* via put assignment (`222917670`/`223828800`). Every card touching the overlay must carry his two preconditions verbatim: never sell-call without 100% of the underlying; never sell-put without full cash to take delivery (`223114673`).
2. **He honestly attributes the overlay's contribution at ~1.9% of the book** (`223625482`) and disclaims that ordinary investors should touch derivatives at all (`223114673`). The overlay is a modest, experimental sleeve, not the engine — and the deck must not sanitize it into a tidy retail strategy.
3. **The genuinely unique, hard-to-find-elsewhere content is HK operational/fiscal knowledge** (red-chip 实物股票 tax route, 港股通 cannot trade options, special-dividend strike adjustment, ~0.3% exercise cost, 1000-share-but-verify lot, stamp-duty-on-exercise-not-expiry). This is the corpus's highest-value, lowest-redundancy yield versus the existing CFA cards.

**Important calibration:** of the ~50 candidate cards, the adversarial verification rated only **~12 fully `faithful`**; the majority are `overgeneralized` (dropped risk caveats, inflated recurrence, or commenter-words attributed to the author), 3 are outright `misattributed`/`weak_evidence` and should be excluded or heavily rewritten. The corpus is **conviction-rich but evidence-thin per claim** — many "principles" rest on a single 2022 reply, often duplicated by Xueqiu repost threading that inflates apparent recurrence.

---

## 2. Investment philosophy & method (the core repeatable framework)

### 2.1 The four methods (corpus-grounded)
- **M1 — Option-income overlay.** Sell cash-secured puts + modest covered calls on owned/wanted HK blue chips; roll for net credit; manage assignment; select strike/tenor by moneyness and theta-vs-delta; **100% collateral, never naked, never leveraged.**
- **M2 — Yield + patience + contrarian accumulation** of HK blue-chip dividend payers; total-return accounting; business-value (not price) definition of a bull market.
- **M3 — HK operational/fiscal mechanics** (option contract adjustment, Connect exclusion, withholding, stamp duty, certificate withdrawal, liquidity concentration).
- **M4 — Behavioral temperament** (holding mentality at bottoms, balanced book as backstop, inaction after research, owner-style AGM activism, skepticism toward denials/pumps).

### 2.2 The non-negotiable risk spine (the most-repeated, most load-bearing content)
This is the single most emphatic, highest-engagement declaration in the corpus (`223114673`, 👍149 💬370), explicitly written as a warning to deter imitators:

> 本人一向反對普通投資者觸碰任何的衍生工具，即便是sell call和sell put。 (`223114673`)
> *I have always opposed ordinary investors touching any derivative, even sell-call and sell-put.*

> 高危遊戲，手中沒有100%的正股，千萬不要sell call；同理，沒有準備好全額接貨的錢，不要sell put。會死人㗎！ (`223114673`)
> *High-danger game: if you don't hold 100% of the underlying, never sell calls; likewise, without the full cash ready to take delivery, don't sell puts. People die from this!*

> 不用杠桿是底線，忌貪！別動任何歪念！ (`223114673`, line 658)
> *No leverage is the bottom line; guard against greed; entertain no crooked ideas.*

> 用融資是找死 (`223114673`, line 680) — *Using margin is looking for death.*

**Authoring rule (critical):** this caveat must travel with *every* M1 card. Verification flagged "dropped risk warning" as the dominant failure mode across the deriv/pm/pa candidates.

### 2.3 The base-position framing (his own words contradict "covered")
> 我sell call sell put的原則是不弄丟底倉，至少在我認為的合理價以下不弄丟。 (`222436722`)
> *My principle in selling calls/puts is not to lose the base position (底倉) — at least not below what I consider fair value.*

> Sell call算是半裸吧？ (`223828800`/`222917670`) — *Sell-call counts as half-naked, I guess?*

He sells calls against shares he only intends to acquire via pending put assignment. **The deck should title the program "base-position-protected option selling around fair value," not "covered option selling."**

### 2.4 Non-directional, probability-respecting premium harvesting
> 和我之前sell call12元和sell put 10元一樣，不預判股價。 (`223866640`)
> *Same as my earlier sell-call at 12 and sell-put at 10 — I do not predict the stock price.*

The ideal outcome is a flat, range-bound underlying ("不死不活"), with edge from probabilities anchored to a **fundamental floor**, not a forecast.

### 2.5 Total-return, business-value temperament
> 只考慮到已收股息，我手中的所有保險股都是盈利的，更別說我還收了期權金。 (`223121311`)
> *Counting only dividends already received, all my insurance stocks are in profit — let alone the option premiums I've also collected.*

> 權益資產一直都處於牛市階段，我的持股組合每年的股東權益和派息都持續增長，怎麼可能不是呢？股價，就別提了 (`223625646`)
> *Equity assets have always been in a bull phase — my portfolio's per-share equity and dividends grow every year, how could it not be? Price — don't even mention it.*

---

## 3. Thematic playbook

Each subsection states the durable rule, then 1-2 verbatim quotes with `post_id`. Theme order follows the input themes.

### 3.1 Covered/cash-secured option-selling system
**Rules:** sell puts only at strikes you'd own at; keep strikes near the money (deep-OTM earns too little); set per-name price bands; roll down for net credit accounting for HK stamp duty/commission; roll deep-OTM near legs forward to free capital; run the wheel (assignment → sell calls); keep covered calls small.

> 我只sell put我願意以那個價錢接貨的公司 (`223114673`) — *I only sell puts on companies I'm willing to take delivery of at that price.*

> 平倉360元的sell put，虧9元每股，新開340元七月份sell put，收回11元。就這張期權而言，相當於騰訊七月份升回$340，能夠淨賺$2。 (`223114673`)
> *Close the 360 put at -9/share, open a July 340 put collecting 11; for this option, it's as if Tencent recovers to 340 and I net +2.*

> 騰訊$400以下，我願意繼續sell put... 120元以上，我可能就不願意繼續sell put了。 (`223114673`)
> *Below Tencent 400 I'll keep selling puts; above [Alibaba] 120 I probably won't.* (Note: dated mid-2022 personal levels, not durable rules; he noted a fast move immediately "challenged my bottom line.")

> 偏離行權價太遠，吃不了波幅的錢 (`223114673`) — *A strike too far from the money eats none of the volatility money.*

**Authoring caveat:** "弄丟" means losing the **share position (籌碼)**, NOT losing premium (he defines it himself, `223426897`). Several candidate cards misread this.

### 3.2 Risk discipline and the no-leverage bottom line
**Rules:** size to deliverable cash; never naked; never short (for ordinary people); avoid unbounded-loss structures; refuse no-floor names; admit errors.

> 我sell put有多少錢就接多少的貨，絕不多賣。 (`223114673`) — *I sell puts only to the extent of cash on hand to take delivery; I never oversell.*

> accumulator可是無底深淵，連自己最壞情況下會虧損多少也說不清賬。 (`223114673`)
> *An accumulator is a bottomless pit — you can't even state your worst-case loss.* (Single emphatic remark, duplicated by reposts; NOT a recurring theme.)

> 如果多手做空0.5%倉位，已經爆倉... 對於一般人，不做空保平安！ (`227101293`)
> *A multi-contract short of even a 0.5% position already blew up the account... for ordinary people, no shorting keeps you safe.* (The blow-up is leverage-driven.)

> 阿里健康那些，我怎麼敢碰啊？都沒底 (`223114673`) — *Ali Health and the like — how would I dare touch them? No bottom.* (Caveat: 陌陌/Momo is NOT a permanent avoid — he later sold Momo puts; 老虎/嘉年华 are commenters' words, per verification.)

### 3.3 HKEX mechanics: options, lots, fees, stamp duty, dividends
**Rules:** the buyer controls exercise; special dividends adjust strikes (ordinary do not); time expiry around ex-dates; no dividend arbitrage via short/borrow; ~0.3% exercise cost; verify lot size every time; stamp duty on exercise but not expiry.

> 由於是特別股息，行使價被調整為11.31元，每一張期權代表股數被調整為1105股... 但是這產生了一些碎股，真麻煩。 (`222375639`)
> *Because it's a special dividend, the strike was adjusted to 11.31 and each contract to 1105 shares... but this creates odd-lots, truly annoying.*

> 中國神華不是特別股息，如果除淨日之前行權人沒有行使期權，除淨后也是按照原來的價格行權，也就是$26。 (`223114673`)
> *Shenhua's wasn't a special dividend, so post-ex the option still exercises at the original $26.*

> 月底才到期，行不行使put的權利，以及決定什麼時候行駛，決定權在買家。我是賣家。 (`222436722`)
> *Expiry is month-end; whether and when to exercise the put is the buyer's decision. I'm the seller.*

> 除淨日股票在誰的手就是誰收息 (`223114673`) — *Whoever holds the share on the ex-date collects the dividend.*

> 行權一樣要交印花稅，但是沒有行權就省下了 (`223114673`) — *Exercise still pays stamp duty; not exercising saves it.* / 一張1000股哦，別弄錯 (`223990393`) — *One contract is 1000 shares, don't get it wrong.* / 每次一定看淸楚股數！ (`222375639`) — *Always check the share count!* (Multiplier varies: 500/1000/1105 all seen.)

### 3.4 HK/China dividend-tax engineering
**Rules:** red-chip vs H-share withholding; 实物股票 (physical-certificate) route escapes tax for red-chips only; after-tax yield drives peer selection; fund 分红 vs corporate 派息 incidence.

> 762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免 (`222999679`)
> *762/883/941 held at a HK brokerage incur +10%, but withdrawing physical certificates exempts them.*

> 所有股票都可以辦理，但是只有紅籌股才可以逃稅 (`222999679`)
> *Any stock can be converted, but only red-chips can escape the tax.* (Converting an H-share is pointless — 依然要10%稅.)

> 我自己會買$中国移动(00941)$ ，因為我持有941不用交股息稅，但728我要10%。如果是內資，應該$中国电信(00728)$ 更好吧？... 22年前瞻估值上也沒什麼差距了。 (`222762628`)
> *I'd buy Mobile because I pay no dividend tax on 941 but owe 10% on 728; for mainland investors Telecom may be better — presupposing valuation parity.*

> 派息是企業盈利所得分配，分紅分的是本來就屬於你的錢，ETF分紅與否，價值是沒有分別的。 (`224009519`)
> *派息 distributes corporate profit; 分红 returns money already yours — whether an ETF distributes is value-neutral.*

### 3.5 Sector frameworks (oil, telecom, insurance, internet, banks, utilities)
**Oil-producer (CNOOC) earnings model:**
> 即便實現油價低至60美元，盈利也有700億港元。未考慮暴利稅，未計算增產的影響，每1美元油價的上升，增厚約25-26億港幣利潤。 (`222825600`)
> *Even at $60 oil, profit is ~70bn HKD; before windfall tax and before production growth, each +$1/bbl adds ~2.5-2.6bn HKD.* (Caveat: the $40/$50/$60 "scenario table" `223087810` was retracted by the author as reversed — "我寫反了"; figures mix HKD and RMB across posts.)

**Telecom:**
> 依旧认为移动是现在市场上最好的defensive play，电信是最佳的增长与股息的结合。 (`223159123`)
> *Mobile is the best defensive play; Telecom is the best growth+dividend combination.*

> 電訊商就是賺流量錢的，阿里是技術提供方 (`223643275`) — *Carriers earn toll money on traffic; Alibaba is the technology provider.*

**Insurer (sum-of-parts + opaque-asset avoidance):**
> 我真的不知道他資產端到底投資了多少地產債和信托，還在不斷展期呢！ (`222402238`)
> *I genuinely don't know how much property debt and trusts sit on its asset side, still being rolled over!* (His reason for skipping Ping An.)

> 太保的財產險至少值600億元，它的壽險不可能不值兩千億元 (`222402238`)
> *CPIC's P&C is worth at least 60bn; its life can't be worth less than 200bn.*

**SOE coal/power payout logic:**
> 政府缺錢只要加大派息力度就可以了，大股東持股比例不低。 (`222825600`)
> *A cash-needy government can just raise the payout — the dominant shareholder's stake is large.* (Counter to windfall-tax fear; he still carries the tax as a live risk.)

**Internet moats / decline signals:**
> 由用戶產生即時內容這個行業，監管難度極高，反過來想，已經沒有資本願意介入與之進行同類型的競爭 (`222517599`)
> *Real-time UGC has extreme regulatory difficulty — inverted, no capital is willing to enter and compete.*

> 我只說盡量避免買營收下滑的股票，尤其是銷售量也是下滑的企業，背後往往代表着那間企業是行業競爭下的輸家。 (`223566493`)
> *I only say: try to avoid buying revenue-declining stocks, especially with declining sales volume — usually the loser in industry competition.*

### 3.6 Fundamental analysis (margins, balance sheet, underwriting)
> 作為成本加成定價方式的制造廠，如果原料成本下跌只會令毛利下降，疊加庫存降價，毛利率也會是降的。 (`222958791`)
> *For a cost-plus manufacturer, falling input costs only compress gross profit; with inventory markdowns, gross margin falls too.*

> 賬上的錢在子公司。按照國稅法，向海外母公司派息，要交10%稅... 只有90億元到賬... 直接影響利潤。 (`222760389`)
> *Cash sits in the subsidiary; paying it up to an overseas parent incurs 10% tax — 100bn transferred yields only 90bn, directly hitting profit.* (Reported cash ≠ distributable cash.)

> 6Apa和胰島素集采都有不確定性，所以全年業績我也不好判斷，0.8元應該會有 (`223200257`)
> *6-APA and insulin procurement are uncertain, so I can't judge full-year results well — 0.8 should be there.* (Floor EPS, not a point forecast.)

### 3.7 Macro and regime views
> 對股票投資者而言，通縮遠遠比通脹可怕，溫和的通脹可以解決很多麻煩事。 (`222869825`)
> *For an equity investor, deflation is far more frightening than inflation; mild inflation solves many problems.*

> 複利是我借給你100擔大米，一年後你還我120擔大米，而不是... 還給我的錢只夠買80擔大米。 (`224276051`)
> *Compounding is lending 100 piculs of rice and getting 120 back — not lending enough to buy 100 and getting money that buys only 80.*

> 加息收緊市場資金之時，一般都是不產生收益的資產優先被拋棄……比特幣不例外 (`222524028`)
> *When rate hikes tighten liquidity, non-yielding assets are dumped first — Bitcoin no exception.*

> 站在資源和社會配套設施的角度，絕大多數國家，新能源汽車不太可能佔存量市場超過30%。 (`222763326`)
> *On resource and infrastructure grounds, in most countries EVs are unlikely to exceed ~30% of the existing vehicle stock.* (He marks this to evidence — `224025817`: his anti-EV view "似乎是錯的," seems wrong.)

> 大牛市還會有的，比當年條件優勝的地方有更低的存貸利率，更高的企業回報率，以及市場缺乏其他好的投資渠道。 (`350177302`)
> *Big bull markets will recur — conditions superior to before: lower rates, higher corporate returns, lack of other channels.*

### 3.8 Behavioral discipline and portfolio construction
> 这么多年，我一直觉得持股心态最重要……尤其是底部，很容易跌着跌着信心都跌没了。结果是该加仓的时候跑了去减仓 (`222402238`)
> *All these years I've felt holding mentality matters most — especially at bottoms, confidence erodes until you cut exactly when you should add.*

> 在投資中最難做到的是什麼？... 當你研究完一只股票，甚至是一整個行業，最後卻什麼也不做。... 應該有能力成為大師級投資者。 (`224442723`)
> *The hardest thing in investing? Researching a stock — even a whole industry — then doing nothing. Such a person has the makings of a master.*

> 我參加股東大會從來沒把自己當小股東，倒是把坐在我對面的几位董事都當成我自己的員工了。 (`222942928`)
> *At AGMs I never see myself as a small shareholder — I treat the directors across from me as my own employees.*

> 我看了傳聞，將信將疑。想想當年$中国华融(02799)$ 也強調公司一切經營正常 (`224245810`)
> *I read the rumor, half-believing; recall that Huarong too stressed "operations entirely normal."*

> 大股東的優勢在於它可以大折價私有化，小股東的優勢在於我們可以拍拍屁股就走人。真心勸喻這種傻事少做 (`224150431`)
> *The big holder can privatize at a steep discount; the small holder's edge is the ability to walk away. I sincerely advise: do this foolish thing less.* (His own ~3%-of-float stake took three years to exit.)

> 加了728，控制行業上限倉位。 (`224598323`) — *Added 728 to control the sector's position ceiling.* (Caveat: the "diversify, everyone's dumbfounded" vindication line in that post is a forwarded comment from 陈达美股投资, NOT the author.)

---

## 4. Card-candidate inventory mapped to existing frameworks

Only candidates whose verdict is **faithful**, or whose **corrected_summary salvages** them, are listed as authorable. Recurrence (`rec`) is the corrected/realistic count where verification deflated it. Crosslinks are the verified-existing targets.

### 4.1 Authorable cards (faithful or salvageable)

| Proposed card title | Framework / reading_folder | Evidence | Verdict | Rec | See-Also |
|---|---|---|---|---|---|
| Base-position-protected option selling around fair value (retitle from "covered") | deriv / 07 | strong | overgeneralized→salvage | 3 | deriv-option-payoff-anatomy; eq-intrinsic-value |
| Sell puts only on companies you'd own at the strike (cash-secured, bounded downside) | deriv / 07 | strong | **faithful** | 7 | deriv-option-payoff-anatomy; rm-risk-objectives-and-tolerance; eq-intrinsic-value |
| Selling premium harvests vol/time-value, not direction | deriv / 07 | strong | overgeneralized→salvage | 5 | deriv-vega-and-theta; deriv-implied-volatility; deriv-greeks-overview |
| Keep sold-put strikes near the money; deep-OTM earns too little | deriv / 07 | strong | overgeneralized→salvage | 4 | deriv-delta-and-hedging; deriv-vega-and-theta; deriv-option-payoff-anatomy |
| Per-name sell-put price bands (date-stamped personal levels) | deriv / 07 | strong | overgeneralized→salvage | 3 | deriv-option-payoff-anatomy; eq-intrinsic-value; rm-risk-objectives-and-tolerance |
| Rolling sold puts down to a lower strike for net credit | deriv / 07 | strong | overgeneralized→salvage | 4 | deriv-option-payoff-anatomy; mt-effective-cost-trade-benchmark; deriv-delta-and-hedging |
| Roll deep-OTM near legs forward to free capital (delta-vs-theta) | deriv / 07 | strong | **faithful** | 4 | deriv-vega-and-theta; deriv-greeks-overview; deriv-delta-and-hedging |
| The wheel: assignment then reverse into (half-naked) calls — one-off edge case | deriv / 07 | moderate | overgeneralized→salvage | 1 | deriv-option-payoff-anatomy; deriv-delta-and-hedging; rm-risk-type-taxonomy |
| Covered calls are pocket-money on volatility, not a return engine | deriv / 07 | strong | overgeneralized→salvage | 4 | deriv-option-payoff-anatomy; deriv-vega-and-theta; eq-intrinsic-value |
| Option exercise mechanics: buyer controls; OTM/ITM; extrinsic value | deriv / 07 | strong | overgeneralized→salvage | 3 | deriv-option-payoff-anatomy; deriv-no-arbitrage-bounds; deriv-put-call-parity |
| Strike adjustment: special dividends adjust, ordinary do not | deriv / 07 | strong | **faithful** | 5 | deriv-option-payoff-anatomy; deriv-put-call-parity; cb-conversion-feature-mechanics |
| Dividend-aware option timing: ex-date, T+1 assignment, who collects | deriv / 07 | strong | overgeneralized→salvage | 4 | deriv-option-payoff-anatomy; deriv-put-call-parity; eq-payout-policy-and-growth |
| No-leverage bottom line: size sell-puts to cash, never oversell | rm / 11 | strong | **faithful** | 6 | rm-risk-objectives-and-tolerance; rm-risk-type-taxonomy; pm-portfolio-constraints |
| Ordinary investors shouldn't touch derivatives; never short, never naked | rm / 11 | strong | **faithful** | 2 | rm-risk-objectives-and-tolerance; rm-risk-type-taxonomy; mt-market-manipulation-bluffing |
| Refuse fat premiums on no-floor names (single conversational moment) | rm / 11 | moderate | overgeneralized→salvage | 1 | rm-risk-type-taxonomy; deriv-implied-volatility |
| Fundamental-floor-anchored strike selection (CNOOC ~8.5) | eq / 05 | moderate | weak_evidence→salvage | 1 | eq-intrinsic-value; eq-dividend-discount-models; deriv-option-payoff-anatomy |
| Honest attribution/sizing of the option sleeve (~1.9%) | pa / 15 | moderate | overgeneralized→salvage (fix post_id→223625482) | 1 | pa-contribution-analysis-benchmark-free; deriv-option-payoff-anatomy |
| Self-financing overlay: size sell-puts to dividend inflow (directional habit, not formula) | pm / 09 | moderate | overgeneralized→salvage | 2 | pm-portfolio-constraints; pm-rebalancing-mechanics; eq-payout-policy-and-growth |
| Sell-put as passive accumulation at strike-minus-premium | deriv / 07 | strong | overgeneralized→salvage | 2 | deriv-option-payoff-anatomy; pm-rebalancing-mechanics; deriv-put-call-parity |
| HK option liquidity concentrated in mega-caps; pick by liquidity (drop the "don't chase MM quote" subprinciple) | mt / 14 | strong | overgeneralized→salvage | 4 | mt-bid-ask-spread-immediacy-price; mt-liquidity-depth-immediacy-width; mt-empirical-determinants-illiquidity |
| HK option costs: ~0.3% exercise; verify lot size; stamp-duty mechanics | mt / 14 | strong | overgeneralized→salvage | 4 | mt-effective-cost-trade-benchmark; mt-implementation-shortfall; mt-order-types-market-limit-stop |
| Stock Connect (港股通) cannot trade HK options | mt / 14 | strong | overgeneralized→salvage (narrow "two years") | 3 | mt-execution-systems-quote-vs-order-driven; mt-institutional-setting-market-types |
| HK dividend tax: red-chip vs H-share withholding (holding-form-dependent) | mt / 14 | strong | overgeneralized→salvage | 1 | mt-institutional-setting-market-types; eq-payout-policy-and-growth; fra-income-tax-accounting |
| 实物股票 route: convert red-chips to physical to escape dividend tax | mt / 14 | strong | **faithful** | 4 | mt-institutional-setting-market-types; fra-income-tax-accounting; eq-payout-policy-and-growth |
| After-tax dividend yield drives peer selection (移动 vs 电信) | eq / 05 | moderate | overgeneralized→salvage | 1 | eq-dividend-discount-models; eq-payout-policy-and-growth; mt-institutional-setting-market-types |
| Fund 分红 vs corporate 派息: tax incidence + channel consistency | fra / 03 | strong | **faithful** | 3 | fra-income-tax-accounting; fa-etf-creation-redemption-mechanism; mt-institutional-setting-market-types |
| Total-return (dividend-inclusive) accounting through drawdowns | pa / 15 | strong | misattributed→salvage (fix citation 3; 明牌 is 3rd party's) | 2 | pa-true-twr-and-chain-linking; eq-payout-policy-and-growth; eq-dividend-discount-models |
| Business-value bull market: equity + dividend growth, not price | eq / 05 | strong | **faithful** | 2 | eq-payout-policy-and-growth; eq-intrinsic-value; pm-efficient-markets-and-anomalies |
| Holding mentality at bottoms; balanced book as backstop | be / 10 | thin→moderate | overgeneralized→salvage | 1 | be-loss-aversion-reference-dependence; be-investor-overreaction; pm-diversification-and-correlation |
| Discipline of inaction: research deeply, then do nothing | be / 10 | moderate | overgeneralized→salvage (drop "circle of interest") | 1 | be-overconfidence-bias; be-regret-aversion-status-quo-endowment; pm-active-vs-passive-decision |
| Skepticism toward denials, manipulation, engineered rallies | be / 10 | moderate | **faithful** | 2-3 | mt-market-manipulation-bluffing; cc-standard-ii-b-market-manipulation; fra-quality-of-financial-statements |
| AGM activism: owner mindset; press payout (single AGM, aspirational response) | be / 10 | strong | overgeneralized→salvage | 2 | be-limits-of-arbitrage; eq-payout-policy-and-growth |
| Read major-shareholder selling via cheapest-execution lens (Naspers/Tencent) | eq / 05 | moderate | **faithful** | 1 | mt-block-trader-upstairs-depth; mt-market-impact-price-concession; mt-order-flow-information-content |
| Small-cap minority liquidity trap; curate the shared book | eq / 05 | strong | **faithful** | 3 | mt-empirical-determinants-illiquidity; mt-liquidity-premium-asset-pricing; pm-portfolio-constraints |
| Diversification over concentration; sector position ceilings | pm / 09 | moderate | overgeneralized→salvage (vindication line is a repost) | 3 | pm-diversification-and-correlation; pm-portfolio-constraints; pm-systematic-vs-idiosyncratic-risk |
| Insurer analysis: avoid opaque asset sides; sum-of-parts; ownership taxonomy | eq / 05 | moderate | overgeneralized→salvage (太保/平安-specific) | 1-2 | eq-sum-of-parts-valuation; eq-intrinsic-value; fra-equity-risk-from-accounting |
| Oil-producer valuation: oil-price-to-earnings sensitivity | eq / 05 | strong | overgeneralized→salvage (retracted scenario; HKD/RMB) | 3 | eq-cyclicality-and-cycle-adjustment; eq-intrinsic-value; ec-commodity-price-forecasting |
| SOE windfall-tax vs payout; minority prefers dividends over capex | eq / 05 | strong | **faithful** | 4 | eq-payout-policy-and-growth; eq-intrinsic-value; ec-fiscal-policy-and-budget-deficits |
| Telecom sector framework: defensive vs growth; carriers vs cloud; upstream | eq / 05 | strong | **faithful** | 4 | eq-industry-and-sector-factor-models; eq-payout-policy-and-growth; fra-depreciation-and-amortization |
| Moats from regulatory difficulty, product-fit, customer-acquisition economics | eq / 05 | mixed | overgeneralized→salvage | 3 (1 each) | eq-industry-and-sector-factor-models; eq-quality-and-low-vol-factor-scoring; eq-intrinsic-value |
| Decline signals: revenue/volume drop, side-bets, youth, ESG buzzwords | eq / 05 | strong | **faithful** | 3 | eq-quality-and-low-vol-factor-scoring; fra-earnings-quality-and-sustainability; fra-reporting-quality-framework |
| Margin & cost-structure: cost-plus, spread-vs-margin, pass-through, procurement | fra / 03 | strong | **faithful** | 4 | fra-earnings-quality-and-sustainability; fra-financial-analysis-techniques; fra-inventory-cost-methods |
| Balance-sheet quality: distributable cash, repatriation tax, receivables, leverage | fra / 03 | strong | **faithful** (add seeds for sub 2/4) | 3 | fra-balance-sheet-foundations; fra-cash-flow-statement-mechanics; fra-credit-risk-from-accounting |
| Buybacks: scale relative to size; OEM read-through; cyclical valuation | eq / 05 | strong | overgeneralized→salvage (drop "tell"; add 224645625) | 3 | eq-share-count-and-per-share-effects; eq-cyclicality-and-cycle-adjustment; eq-pe-and-relative-valuation |
| Conservative, downside-anchored earnings underwriting under policy uncertainty | eq / 05 | strong | **faithful** | 3 | eq-intrinsic-value; eq-dcf-mechanics; fra-earnings-quality-and-sustainability |
| Deflation worse than inflation; real vs nominal returns | ec / 02 | strong | **faithful** | 4 | ec-monetary-policy-and-inflation; ec-aggregate-supply-demand-mechanics; pa-arithmetic-vs-geometric-excess-return |
| EV-penetration ceiling; treat unenacted policy as non-information | ec / 02 | moderate | overgeneralized→salvage (add 223301216) | 3 | ec-commodity-price-forecasting; be-belief-perseverance-biases; eq-cyclicality-and-cycle-adjustment |
| Bull-market preconditions and policy-regime contrast | ec / 02 | moderate | **faithful** | 1 | ec-monetary-fiscal-policy-mechanics-l1; ec-business-cycles-and-output-gaps; pm-efficient-markets-and-anomalies |
| China credit/banking: mortgage-bank quality; no securitized-subprime contagion | ec / 02 | moderate | overgeneralized→salvage (drop "now ending" as stated) | 1 | fra-credit-risk-from-accounting; fi-securitization-fundamentals; fi-mbs-prepayment-models |
| Contrarian entry on clustered bad news; relative-strength bottoming; HK seasonality | be / 10 | moderate | **faithful** | 2 | be-investor-overreaction; be-sentiment-vs-fundamentals; be-extrapolation-from-recent-data |
| Index-inclusion / Connect flow events: tradable but unpredictable | be / 10 | moderate | **faithful** | 2 | be-noise-trader-equilibrium; fa-etf-creation-redemption-arbitrage-band; mt-index-portfolio-markets-design |
| Deep-value/downside-limited names; prefer hedged cyclicals | eq / 05 | strong | overgeneralized→salvage (3 separate remarks, not 1 screen) | 3 (1 each) | eq-value-and-momentum-factor-scoring; eq-cyclicality-and-cycle-adjustment; eq-intrinsic-value |
| Holdco/NAV-discount: trap vs narrowing (path-dependent) | eq / 05 | moderate | overgeneralized→salvage (he declined the trade) | 2-4 | eq-sum-of-parts-valuation; fa-etf-vs-cef-premium-discount; be-limits-of-arbitrage |
| Governance & ownership-structure quality (3 separate one-offs) | eq / 05 | moderate | weak_evidence→salvage | 1 each | eq-quality-and-low-vol-factor-scoring; eq-sum-of-parts-valuation; eq-cross-sectional-multiples-distribution |
| Utilities/staples as crash-year safe harbor — conditional | eq / 05 | moderate | overgeneralized→salvage (顺价 is MAINLAND not HK) | 2 | eq-quality-and-low-vol-factor-scoring; eq-cyclicality-and-cycle-adjustment; eq-industry-and-sector-factor-models |
| Regulatory-risk discipline + platform-risk acceptance (drop the OPM seed) | rm / 11 | moderate | overgeneralized→salvage | 2 | rm-risk-type-taxonomy; rm-operational-risk-basics; cc-standard-i-a-b-knowledge-of-law-and-independence |
| Cool-off discipline; avoid wealth-destroyer vehicles (1.5% is one trade, not a cap) | be / 10 | moderate | overgeneralized→salvage | 2 | be-present-focused-preferences-taxonomy; be-self-control-mental-accounting; rm-risk-objectives-and-tolerance |
| Broker/listing-venue ops: relisting transfers, share-class consolidation, model-portfolio cap | mt / 14 | moderate | overgeneralized→salvage (drop "avoid being caught") | 2 | mt-institutional-setting-market-types; mt-execution-systems-quote-vs-order-driven |

### 4.2 Excluded / thin / needs-more-evidence (do NOT author as-is)

| Candidate | Framework | Reason |
|---|---|---|
| **Cloud sector thesis: hidden-asset optionality (Tianyi Cloud "free call option")** | eq / 05 | **misattributed** — the Tianyi-Cloud quote (`223159123`) is a verbatim repost of user @欲辨已忘言-'s IR-call write-up (everything after `//@欲辨已忘言-:`), NOT the author. The "hidden-asset optionality" pillar must be DROPPED. Salvageable remnant: author's own ease-of-adoption moat (`223631520`) + China-vs-US TAM analogy → fold into the "Moats" card instead. |
| **IR calls as due diligence; shareholder base as marketing asset** | eq / 05 | **misattributed** — same `223159123` repost; the IR-as-DD half is not the author's words. Only the one-line marketing-audience quip (`223372206`) is author-attributed (and "exploit" mischaracterizes an admiring tone). Too thin for a standalone card. |
| **Talk-your-book discipline + OPM incentive checks** | be / 10 | overgeneralized + fabricated detail ("OTM puts"); two unrelated single statements bundled; "managers" added. Keep only if rewritten to the corrected_summary and split. |
| **Engagement without overtrading; itchy-hands; extreme-return chasing** | be / 10 | overgeneralized — three unrelated one-liners welded; the "not a license to overtrade" caveat has NO textual support; "拉黑" likely means muting an account, not blacklisting a stock. |
| **No dividend arbitrage via short/borrow** | deriv / 07 | weak_evidence — rests on one substantive line + a one-word "是啊"; the no-arbitrage *conclusion* is the card's inference, not stated. |
| **Premium is the buyer's insurance cost; fat premium prices risk** | deriv / 07 | overgeneralized — the 6-7%/month figure and IV/"insurance premium" framing are commenters' words; the author ignores IV and reasons from fundamentals. Merge the genuine "refuse 8%+ no-floor names" content into the rm "refuse fat premiums" card. |
| **Sell-put underlying selection (Tencent vs Alibaba)** | deriv / 07 | overgeneralized — core claim sound but drops the cash-secured/no-leverage gating; downgrade evidence to moderate; can author only with caveats restored. |
| **Insider buying as confidence signal** | eq / 05 | overgeneralized — three scattered offhand remarks across three companies; the "magnitude-relative-to-income" framework and "one data point, not a thesis" are the card-writer's synthesis. Author with corrected_summary only. |
| **Avoid option-selling into earnings; auto-dealer profit model** | deriv / 07 | overgeneralized + fabricated prescription — the "avoid selling into earnings" rule is an inference from a trailing comma-ended remark; the auto-dealer note is unrelated and belongs under autos/industry, not deriv. Split and re-source. |
| **Sell on deteriorating leverage and admit the error** | rm / 11 | overgeneralized — a single retrospective "I misjudged the company" confession elevated to a sell *rule*; "misjudged" ≠ "deteriorating"; two unrelated topics fused. |
| **Profit-vs-growth trade-off + counterparty/receivables risk** | eq / 05 | weak_evidence — two unrelated single-shot remarks; each appears once; "margin" substituted for the author's "利潤." |
| **Float-adjusted turnover as a microstructure red flag** | mt / 14 | overgeneralized — author explicitly says the *meaning* is NOT understood and does not state it; one-off about one stock; any "red flag" reading is unsupported inference. |

---

## 5. HK-market operational knowledge (the uniquely-HK content not in the CFA cards)

This is the corpus's highest-value, lowest-redundancy material. The existing `cards/cfa/14_microstructure_and_trading/` covers *generic* venue mechanics (spreads, order types, impact) but is silent on every HK-specific item below. **New HK primary sources (HKEX contract-adjustment rules, IRD/SAT withholding rules, Stock Connect FAQ) are required to ground these as cards** — the corpus supplies the practitioner observation, not the rule citation.

1. **Stock Connect cannot trade HK options.**
   > 沒法 (`222375639`, answering "港股通没法做期权吧") — *Can't.* / 你把股票轉到期權戶口就可以了 (`222375639`) — *Transfer the stock into your options account.*

2. **Red-chip vs H-share dividend withholding, holding-form-dependent.**
   > 有一些不用交，有一些要交10%，比如392、934，完全不用交，762、883、941如果在香港證券行持有就要+10%，但是提取實物，就可以豁免 (`222375639`)
   > 因為是紅籌股，母公司在香港注冊，按例個人投資者不用交稅。 (`222375639`) — *Because it's a red-chip with HK-incorporated parent, individual investors are exempt by rule.* (China Mobile/941 is a red-chip; China Telecom/728 is an H-share — conversion is pointless for the latter.)

3. **实物股票 (physical-certificate) route — red-chips only.**
   > 可以，但是依然要10%稅，所以轉成石鼓變得沒有意義 (`222999679`) — *[For an H-share] you can convert, but it's still taxed 10%, so converting is meaningless.* (石鼓 = author's typo for 實物.) Done at the registrar (過戶處); generally requires being in HK; dividends on physical shares pay by cheque or HK bank account only.

4. **Special-dividend strike adjustment / odd-lots / verify lot size.** (Quotes in §3.3.) HK adjusts strike × multiplier so contract value ≈ constant; ordinary dividends do not adjust; multipliers of 500/1000/1105 all appear.

5. **Costs: ~0.3% exercise, stamp duty on exercise not expiry.** (Quotes in §3.3.) The ~0.3% is the author's terse correction of a commenter's "nearly 1%."

6. **Liquidity concentration + HK-vs-US microstructure.**
   > 阿里騰訊中海油還可以，其他的真的不怎麼樣 (`224093303`) — *Alibaba/Tencent/CNOOC are OK; others really aren't great.*
   > 港股期權我基本上掛中間價一定會成交，美股期權卻經常出現掛中間價沒有成交的情況。我估計是因為行使價太多以及到期日太多的原故，價差的損失也是我的一個考慮 (`222517599`)
   > *In HK, quoting the mid basically always fills; in US options the mid often doesn't — too many strikes/expiries fragment liquidity, and spread loss is one of my considerations.*

7. **Fund 派息/分红 tax + southbound 20% withholding.** (Quotes in §3.4; the 20% figure `224335900` is the author reposting official 基金互认 policy.)

8. **Broker/relisting ops.**
   > IB自動轉，其他公司要自己操作 (`366413082`) — *IB auto-transfers [on a relisting]; other firms you must handle yourself.* / 美的和海信轉到A股。 (`366413082`)

---

## 6. Faithfulness & attribution notes

**Method.** Each candidate's citation_seeds were adversarially checked for (a) verbatim-exact substring match against `corpus_full.md`, (b) author-attribution (text after `回复@user:` and OUTSIDE the `(to @user: "...")` parenthetical = author's words; text after a `//@user:` repost delimiter = the quoted person's words), and (c) correct `post_id` binding. I independently re-verified the three highest-stakes flags against the corpus (lines confirmed below).

**Verdict distribution (~50 candidates):** ~12 `faithful`; ~30 `overgeneralized` (almost all salvageable via corrected_summary); 3 `weak_evidence`; 3 `misattributed`.

**Dominant failure mode — dropped risk caveats.** The most common defect is omitting the author's `223114673` risk spine (no-leverage / 100%-underlying / full-cash-to-deliver / "會死人㗎" / "ordinary investors shouldn't touch derivatives"). Every M1 (deriv/pm/pa) card must restore it. Confirmed verbatim at line 654 / 658 / 680.

**Confirmed misattributions (must NOT be authored as the author's words):**
- **Tianyi Cloud "free call option" + IR-call professionalism** (`223159123`, line 806): the entire numbered IR write-up sits after `//@欲辨已忘言-:` — it is a reposted user's phone-call notes, added with **no author comment**. The corpus header tags it `[AUTHOR POST]` only because it's on his timeline. → Drop the cloud "hidden-asset optionality" pillar and the IR-as-DD card.
- **"同意最後一句" / 明牌 / "shareholder's due"** (line 913): the quote belongs to **post 223366919** (not the candidate's cited 223301216), and 明牌 / "分红那不是我该得的吗" are **inside the `(to @艾叶z: ...)` parenthetical** — i.e. a third party's words (管我财, quoted by @艾叶z). The author only said "同意最後一句" (I agree with the last sentence). → Fix the Total-Return card's third citation and do not present 明牌 as his coinage.
- **"1.9% attribution" post_id**: the quote lives in the **post body of 223625482** (line 1083) and as a reply (line 717), NOT in the cited 223114673 (which is the risk-disclaimer post). → Re-bind to 223625482.

**Other commenter-words-as-author flags (verified):** the "diversify / everyone's dumbfounded" vindication line (`224598323`) is a forward from 陈达美股投资; the "6-7%/month" and IV/"insurance premium" framings are commenters'; "delta>theta" originated with @希尓瑞斯; "look past car-maker hype" originated with @岗仁波齐; the OEM "Adidas" pairing and the "circle of interest" framing are interpolations.

**Inflated-recurrence flags (verified pattern).** Xueqiu repost threading re-surfaces the same author reply under multiple parent-quote views, producing duplicate lines (e.g. accumulator at 684/1574/1587/1599; 不用杠桿 at 658/2321/2327; 半裸 at 712/1277/1283). The accumulator, CNOOC-floor, 移动-vs-电信, insurer-SOTP, and red-chip-tax claims each rest on effectively **one** utterance, not the candidate's stated recurrence. The authoring phase should compute recurrence on **distinct utterances**, not raw grep hits.

**Jurisdiction flag:** the 顺价/保民生 gas pass-through (`228478086`) is a **mainland** China issue (中国燃气 00384), wrongly metadata-tagged "HK gas." Correct before authoring the utilities card.

---

## 7. Coverage, gaps & risks

**Time span (correction to the prompt's "2022-2026").** The corpus runs **2020-10-16 → 2026-03-12**, but is overwhelmingly a **2022-06/07 snapshot**: 254 of 266 posts are 2022, 2 in 2020, 1 in 2023, 8 in 2025, 4 in 2026. The price bands (Tencent <400, Alibaba <120), the CNOOC oil model, and the option program are all **dated mid-2022 tactical states**, not durable levels — cards must say so. The 2025-26 posts add only: bull-market preconditions (`350177302`), broker/relisting ops (`366413082`/`366414200`), telecom rotation, regulatory-warning discipline (`371440678`), and a few one-line passes (`379185257`).

**What the corpus does NOT cover (gaps for card-authoring):**
- **No quantitative methods (folder 01), no fixed income/credit-cards-level detail (06), no convertibles (08).** His macro/credit views are aphoristic, not modeled.
- **No worked option-pricing or Greeks math** — he uses theta/delta/IV *intuitively* (and ignores IV when commenters raise it). The deriv cards must crosslink *up* to the existing Hull-grounded theory cards (`deriv-greeks-overview`, `deriv-vega-and-theta`) rather than re-derive.
- **No general portfolio-optimization / factor-model framework** — diversification is behavioral, not mean-variance.
- **No HK rule citations** — he states practitioner observations (~0.3% cost, special-dividend adjustment, 10% withholding) without citing HKEX/IRD/SAT primary sources. The grounding-books report (`grounding_books_report.md`) confirms these are **hard gaps requiring NEW official HK sources** to ground as cards; the corpus is the *observation* layer, not the *authority* layer.

**Risks for the card-authoring phase:**
1. **CJK verbatim-citation extraction.** The deck model is English-card-with-Chinese-quote. The cards must carry the *exact* traditional/simplified mixed-script quotes (he writes mostly traditional with occasional simplified and typos: 行駛/行使, 石鼓/實物, 不死不話/不死不活). Any PDF/extraction round-trip risks STX/normalization corruption (cf. the AC-5 byte-equal Pdfium gotchas in project memory). Cite directly from `corpus_full.md` substrings; do not re-extract.
2. **Granularity.** Many themes collapse to one utterance; resist authoring near-duplicate cards (e.g. the seven sell-put sub-tactics could over-fragment). Prefer fewer, richer cards with multiple sub-principles over many thin ones.
3. **Caveat preservation = correctness.** Given the risk-spine omission pattern, the gen-plan should make "risk caveat present" a hard QA gate for every deriv/rm/pm/pa card.
4. **Author-attribution gate.** Make the `//@user:` repost-delimiter and `(to @user: "...")` parenthetical rules an explicit QA check; three candidates already failed it.
5. **Framework fit for non-CFA content.** HK tax/ops content lands in `mt/14` and `fra/03` by analogy, not by native CFA topicality — flag as `17_cross_cutting` candidates if the deck supports it, so they aren't mistaken for curriculum-derived cards.

---

## 8. Provenance index

- **Corpus:** `/home/jakeshea/knowledge_base_framework_discovery/_research/xueqiu_goubujiao/corpus_full.md` — 2509 lines; **266 posts** (`### POST <id> | <date> BJ | 👍 💬 🔁`), **597 author-reply markers** (`- ↩︎`), **266 `[AUTHOR POST]` blocks**. Date range 2020-10-16 → 2026-03-12.
- **Batch manifest:** `/home/jakeshea/knowledge_base_framework_discovery/_research/xueqiu_goubujiao/batch_manifest.json` — `n_posts:266, author_replies:597, n_batches:23, batch_size:12`.
- **Batches:** `/home/jakeshea/knowledge_base_framework_discovery/_research/xueqiu_goubujiao/batches/batch_01.md` … `batch_23.md` (12 posts each, last is 2).
- **Grounding-source analysis:** `/home/jakeshea/knowledge_base_framework_discovery/_research/xueqiu_goubujiao/grounding_books_report.md` (+ `grounding_books_raw.json`) — maps the four methods to existing CFA cards and identifies where NEW HK primary sources are mandatory (option-writing workflow; HKEX/IRD/Connect rules; value-investing doctrine; China-SOE institutional context).
- **Existing framework taxonomy (crosslink targets):** `/home/jakeshea/knowledge_base_framework_discovery/cards/cfa/` — 14 reading folders present: `01_quantitative_methods, 02_economics, 03_financial_reporting_analysis, 05_equity, 06_fixed_income_and_credit, 07_derivatives_and_volatility, 08_convertible_bonds, 09_portfolio_management_and_asset_pricing, 10_behavioral_finance, 11_risk_management, 14_microstructure_and_trading, 15_performance_and_attribution, 17_cross_cutting, 22_fund_level_arbitrage`. All candidate `reading_folder` values resolve to these.

**Independently re-verified in this pass:** risk-spine quotes at `corpus_full.md` lines 654/658/680 (post 223114673); 半裸 one-off at 712/1277/1283 (post 223828800); 1.9% attribution body at line 1083 → post **223625482** (not 223114673); 同意最後一句 at line 913 → post **223366919**, with 明牌 inside the `(to @艾叶z:)` parenthetical; Tianyi Cloud / IR text at line 806 → post 223159123, entirely after `//@欲辨已忘言-:` (reposted, not authored). Per-year post distribution: 2020×2, 2022×254, 2023×1, 2025×8, 2026×4.