---
schema_version: "cacg.v0"
id: "fa-etp-taxonomy-and-legal-structures"
title: "ETP Taxonomy: ETF vs ETN vs ETV/ETC & Legal Wrappers"
reading_id: "22_fund_level_arbitrage"
summary: "ETP is an umbrella; the legal wrapper (1940-Act fund vs 1933-Act note/trust) decides whether in-kind creation/redemption arbitrage holds. ETFs get exemptive relief to issue redeemable creation units; ETNs are unsecured issuer debt with a cash-only, issuer-gated issuance channel."
tags: ["etp-taxonomy", "etn", "exemptive-relief"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p321:0424"
    chunk_hash: "6ceb3ca40fdaf28e7e636bfdc99b89e5107e8e4ccbd6fd787f6015e3c4ea942d"
    page_range: [322, 322]
    quote: "Exchange-traded notes are senior, unsecured, unsubordinated debt securities"
    edge_type: "defines"
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p316:0417"
    chunk_hash: "a2b8b516901a12bdaa8b5702f6469d579e43fd6922677ba02a61bbd77b613ac4"
    page_range: [317, 317]
    quote: "ETF cannot operate without exemptive relief. Applying for such relief can take several months or even years, and may cost a tremendous amount of money in legal bills and other administrative costs."
    edge_type: "supports"
card_hash: "c4df8e620482c1d2320513127b59a08cc664bd01266e315355a835891c28d718"
---
# ETP Taxonomy: ETF vs ETN vs ETV/ETC & Legal Wrappers

## Intuition
"Exchange-traded product" (ETP) is an umbrella, not a structure. Everything that lists and trades intraday gets lumped under it, but the legal wrapper underneath determines what an investor actually owns and — crucially for arbitrage — whether a primary-market creation/redemption channel exists to anchor price to value. A true ETF is an investment company holding a portfolio of securities; its shares are claims on that portfolio. An exchange-traded note (ETN) holds nothing: it is a bare promise from a bank to pay an index return, so its "value" rides on the issuer's creditworthiness. ETV/ETC structures (grantor trusts, commodity pools, limited partnerships) sit in between, holding physical or derivative exposure but lacking the 1940-Act investor protections. Because each wrapper has different recourse, tax, and issuance plumbing, the same headline exposure can behave very differently when stress hits the arbitrage mechanism.

```
            ETP (umbrella term - trades intraday)
   ___________________|__________________________
  |                   |                          |
 ETF                ETV/ETC                      ETN
 1940 Act           1933 Act                     1933 Act
 owns a portfolio   owns trust/LP units          owns NOTHING
 redeemable in-kind cash/in-kind, no 1940         issuer-promise only
 creation units    protections                   cash-only, issuer-gated
  |                                                |
 in-kind arb band HOLDS                  arb band BREAKS if issuer
 (basket <-> shares)                     suspends new issuance / credit gaps
```

**Source:** Abner (2016) *The ETF Handbook* 2e §13 pp.311-322.

## Definition
- **ETP (exchange-traded product):** the catch-all umbrella covering every exchange-listed, intraday-tradable wrapper — ETFs, ETNs, and ETV/ETC trusts/partnerships alike. Correct but non-discriminating.
- **ETF:** an investment company with redeemable shares, registered under the Investment Company Act of 1940 (the "1940 Act"), holding a portfolio that backs the shares.
- **ETN (exchange-traded note):** senior, unsecured, unsubordinated debt registered under the Securities Act of 1933 — "senior, unsecured, unsubordinated debt securities" whose defining risk is that it "is backed only by the credit of the issuer." It conveys no ownership of assets, only an issuer promise to pay the benchmark return net of fees, and "can be created or redeemed for cash only" on a prospectus-defined schedule.
- **ETV / ETC (exchange-traded vehicle / commodity):** a 1933-Act trust or partnership unit (commodity or currency trust) whose holders "do not have the protections associated with ownership of shares in an investment company registered under the 1940 Act."
- **Wrapper split:** ETFs sit in the 1940-Act column; ETNs, grantor trusts, ETCs, and limited partnerships sit in the 1933-Act column.

**Source:** Abner (2016) *The ETF Handbook* 2e §13 pp.312-322.

## Mathematical Reasoning
Let the listed price be P, the value of the reference exposure be V (NAV for a fund, index level net of fees for a note), and let c capture round-trip arbitrage frictions. The creation/redemption mechanism keeps P pinned to V only while an authorized party can convert between shares and the underlying at will:

- **1940-Act ETF (in-kind channel open):** an AP can exchange the basket for creation units and vice versa, so any deviation |P - V| > c is closed by primary-market arbitrage. The band is two-sided and symmetric, which is exactly the arbitrage band extended in [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md).
- **ETN (issuance gate):** there is no portfolio to deliver; conversion is cash-only and issuer-controlled. If the issuer suspends new issuance, the creation side of the band is severed: P can run to a premium P > V with no force pulling it back, so the no-arbitrage relation degrades to a one-sided (or absent) bound. Add issuer credit: economic value is V_index x R(issuer), where R is a recovery/credit factor; default risk drives R < 1 even when the index is flat. Thus ETN value carries a term ETF shares do not.

Comparative statics: the looseness of the price-to-value band is increasing in (i) wrapper-specific frictions c, and (ii) the degree to which the creation channel is gated. The 1940-Act wrapper minimizes both via mandatory exemptive relief that authorizes redeemable creation units; the 1933-Act note wrapper maximizes the issuance-gate term.

**Source:** Abner (2016) *The ETF Handbook* 2e §13 pp.317-322.

## See Also
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — the two-sided no-arbitrage band that the open 1940-Act in-kind channel makes possible; this card explains the wrappers where that band weakens or breaks.
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the in-kind creation-unit plumbing that only the ETF wrapper supports.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — the ETN issuance suspension is a concrete instance of a severed creation channel.
- [`fa-in-kind-basket-design-and-fees`](./fa-in-kind-basket-design-and-fees.md) — basket design presumes the 1940-Act in-kind wrapper.

Legacy (other tree, prose only): the convertible-bond arbitrage cards on Chinese T+0 and CSDC settlement mechanics illustrate how a different legal-settlement wrapper reshapes the arbitrage band; behavioral-finance limits-of-arbitrage material gives the general theory for why structural gates let mispricings persist.

## Escalate to Raw When
Go to the raw handbook when you need the exact roster of exemptive-relief sections an ETF must obtain (e.g., the creation-unit and Section 12(d)(1) fund-of-funds carve-outs), the per-wrapper taxation rows of the ETP-umbrella exhibit (ordinary income vs the blended commodity-pool rate vs the collectibles rate for grantor trusts), the generic listing-standard thresholds for domestic/international/fixed-income index ETFs, or the full enumerated ETN risk list (call feature, index-sponsor discretion, CFTC interaction). Those worked structural details and concrete numeric thresholds live in the source and should not be reconstructed here.

**Source:** Abner (2016) *The ETF Handbook* 2e §13 pp.314-323.
