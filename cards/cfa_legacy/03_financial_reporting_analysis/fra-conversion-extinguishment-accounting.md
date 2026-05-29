---
schema_version: "cacg.v0"
id: "fra-conversion-extinguishment-accounting"
title: "Conversion and Extinguishment — Settlement Mechanics"
reading_id: "03_financial_reporting_analysis"
summary: "Walks through the issuer's accounting for conversion at maturity, early conversion, and repurchase / extinguishment of convertible debt under IFRS — book-value method on conversion (no gain/loss at maturity), gain/loss on liability component at repurchase, with the equity portion adjusted in Share Premium—Conversion Equity."
tags: ["financial-reporting", "conversion-extinguishment"]
citations:
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1256:1371"
    chunk_hash: "c9420073ae136f25f02619943bbd77736e1962942b562fcbd1bf8f735e6fef03"
    page_range: [1256, 1257]
    quote: "There is no gain or loss on conversion at maturity. The original amount allocated to equity (€194,374) is transferred to the Share Premium—Ordinary account"
    edge_type: "defines"
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1257:1372"
    chunk_hash: "9d32032967d748fa4b691ff674ecb1b8b9cdc6b14dde6855a7661b73a8c8ff46"
    page_range: [1257, 1257]
    quote: "The difference between the consideration allocated to the liability component and the carrying amount of the liability is recognized as a gain or loss"
    edge_type: "defines"
card_hash: "a269e1e9e725f831f071ef7f27d07a4acd7d445658cce25f6e1d9d676efcb5fb"
---
# Conversion and Extinguishment — Settlement Mechanics

## Intuition

A convertible bond on the issuer's balance sheet eventually
exits via ONE of four canonical paths, plus a fifth issuer-
initiated variant. Each path has distinct accounting
mechanics — specifically, distinct rules for whether a
gain/loss is recognized in profit or loss versus pushed
entirely into equity. The four canonical paths trace the
two binary choices: WHEN does settlement happen (at maturity
vs before), and WHAT happens (conversion to shares vs cash
redemption). The fifth variant — induced conversion — is an
issuer-side accelerant. **Source:** Kieso (2020) Ch.16 pp.1255-1258.

The key analytical takeaway: gain/loss recognition happens
ONLY on the pre-maturity-repurchase path; every other path
is gain/loss-neutral on the equity-classified leg, and
either gain/loss-neutral (at-maturity paths) or gain/loss-
relevant ONLY on the liability leg (pre-maturity
repurchase). The ORIGINAL equity component `E_0` always
lands in equity — sometimes transferred to Share Premium —
Ordinary (the conversion paths), sometimes reduced for a
fair-value adjustment (the pre-maturity repurchase path),
sometimes retained as Share Premium — Conversion Equity
(the maturity-repurchase path). **Source:** Kieso (2020)
Ch.16 pp.1255-1258.

```
+--------------------------------------------------+
|  Settlement-path summary (issuer-side)           |
+--------------------------------------------------+
|                                                  |
|  Path 1: maturity + repurchase  → no P&L         |
|  Path 2: maturity + conversion  → no P&L         |
|  Path 3: early    + conversion  → no P&L         |
|  Path 4: early    + repurchase  → P&L gain/loss  |
|  Path 5: induced conversion     → Conversion Exp |
|                                                  |
|  Only Paths 4 and 5 produce income-statement     |
|  line items. Paths 1, 2, 3 are P&L-neutral.      |
+--------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1255-1258.

## Definition

The five settlement paths each have a distinct journal-
entry pattern. **Source:** Kieso (2020) Ch.16 pp.1255-1258.

Path 1 — repurchase at maturity: the carrying amount has
amortized to face (see
[`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md)),
so there is no discount left. The journal entry derecognizes
the liability at face and records the cash outflow. The
equity component `E_0` stays in Share Premium — Conversion
Equity or is transferred within equity to Share Premium —
Ordinary. NO gain/loss recognized. **Source:** Kieso (2020)
Ch.16 pp.1255.

Path 2 — conversion at maturity: the holder elects
conversion at maturity. The book value method debits Bonds
Payable for face and Share Premium — Conversion Equity for
`E_0`, crediting Share Capital — Ordinary at `N · par` and
Share Premium — Ordinary for the residual `Face + E_0 −
N · par`. NO gain/loss recognized. **Source:** Kieso (2020)
Ch.16 pp.1255.

Path 3 — conversion before maturity: the holder converts at
some time `t < T`. The journal entry mirrors Path 2 but uses
`L_t` instead of Face. NO gain/loss recognized. The
unamortized gap `Face − L_t` (a discount when `L_t < Face`
in the common discount-issuance case, a premium of opposite
sign when `L_t > Face` in the mirror premium case) does NOT
get recognized as expense or gain at conversion; it simply
ceases to exist (the bond is gone). **Source:** Kieso (2020)
Ch.16 pp.1256.

Path 4 — repurchase before maturity: this is the ONLY path
that produces a gain/loss on the income statement. The
accounting bifurcates the repurchase amount into a
liability component and an equity component using the SAME
with-and-without method as at issuance — recomputed at the
prevailing market rate for a non-convertible bond with the
SHORTENED remaining maturity. The gain/loss on the
liability leg equals `L_t − L_repurch`. **Source:** Kieso
(2020) Ch.16 pp.1257.

Path 5 — induced conversion: the issuer offers a sweetener
(cash, additional shares, or other consideration) to
encourage early conversion. The sweetener is recognized as
Conversion Expense in the current period — NOT as a
reduction of equity raised. The IFRS rationale: the
sweetener pays the holder for SERVICE (converting earlier
than the holder would have otherwise chosen). **Source:**
Kieso (2020) Ch.16 pp.1258.

```
+----------------------------------------------------------+
|  Convertible bond settlement decision tree (issuer-side) |
+----------------------------------------------------------+
|                                                          |
|                       Settlement event                   |
|                              |                           |
|              +---------------+----------------+          |
|              |                                |          |
|         AT maturity                    BEFORE maturity   |
|              |                                |          |
|       +------+------+               +---------+--------+ |
|       |             |               |                  | |
|   conversion   repurchase      conversion         repurchase|
|       |             |               |                  | |
|   no gain/      no gain/       no gain/         gain/loss  |
|   loss          loss           loss             on liab leg|
|                                                  + equity  |
|                                                  adjustment|
|                                                            |
|  Plus: INDUCED conversion (issuer-side variant)            |
|     issuer offers sweetener → Conversion Expense           |
|     recognized in current period                           |
+----------------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1255-1258.

## Mathematical Reasoning

The journal-entry shapes for each path, with all amounts
symbolic. **Source:** Kieso (2020) Ch.16 pp.1255-1258.

Path 1 (repurchase at maturity): `Dr Bonds Payable Face /
Cr Cash Face`; carrying = Face at maturity; equity
component `E_0` stays in Share Premium — Conversion Equity
or is transferred within equity. No P&L line. **Source:**
Kieso (2020) Ch.16 pp.1255.

Path 2 (conversion at maturity): `Dr Bonds Payable Face,
Dr Share Premium — Conversion Equity E_0 / Cr Share Capital
— Ordinary (N · par), Cr Share Premium — Ordinary (Face +
E_0 − N · par)`; total equity increases by `Face + E_0` over
the bond's full life. No P&L line. **Source:** Kieso (2020)
Ch.16 pp.1255.

Path 3 (conversion before maturity): `Dr Bonds Payable L_t,
Dr Share Premium — Conversion Equity E_0 / Cr Share Capital
— Ordinary (N · par), Cr Share Premium — Ordinary (L_t +
E_0 − N · par)`; the equity uplift relative to Path 2 reflects
the gap between `L_t` and `Face` (less uplift than Path 2
when `L_t < Face` in the common discount-issuance case; more
uplift when `L_t > Face` in the mirror premium case). No P&L
line. **Source:** Kieso (2020) Ch.16 pp.1256.

Path 4 (repurchase before maturity): bifurcate the
repurchase price `P_repurchase = L_repurch + E_repurch`
where `L_repurch = PV(remaining coupons + face;
r_market_now; remaining maturity)` and `E_repurch =
P_repurchase − L_repurch`. Gain/Loss on liability leg =
`L_t − L_repurch` (gain if `L_t > L_repurch`; loss
otherwise). Equity adjustment is a within-equity
reclassification. P&L line: Loss on Repurchase (or Gain).
**Source:** Kieso (2020) Ch.16 pp.1257.

Path 5 (induced conversion): `Dr Conversion Expense S,
Dr Share Premium — Conversion Equity E_0, Dr Bonds Payable
L_t / Cr Share Capital — Ordinary (N · par), Cr Share
Premium — Ordinary (residual), Cr Cash (or Shares) S`;
where `S` is the fair value of the sweetener. P&L line:
Conversion Expense. **Source:** Kieso (2020) Ch.16 pp.1258.

The income-statement impact summary: Paths 1, 2, 3 are P&L-
neutral; Path 4 produces gain/loss on the liability leg
(driven by interest-rate and credit-spread movement between
issuance and repurchase); Path 5 produces Conversion
Expense equal to the sweetener fair value. **Source:**
Kieso (2020) Ch.16 pp.1255-1258.

Anchored-coverage discipline summary: induced conversion is
fully covered (Path 5); fixed-for-fixed classification
failures are covered upstream in
[`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md);
embedded-derivative FVTPL tracking is out-of-scope (Kieso
Ch.16 does not anchor); modification-vs-extinguishment
thresholds (IFRS 9 10% test for cash-flow change) are
deferred to a future repair round with a direct IFRS 9
anchor. **Source:** Kieso (2020) Ch.16 pp.1253-1258.

## See Also

- [`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md) — upstream IAS 32 split that creates `L_0`, `E_0`, and the bond's initial carrying amount
- [`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md) — upstream amortization recursion that produces the running carrying amount `L_t` consumed by all four settlement paths
- [`fra-non-current-liabilities`](./fra-non-current-liabilities.md) — general bond extinguishment context
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — general recognition framework for the Loss on Repurchase (Path 4) and Conversion Expense (Path 5) line items
- [`cb-bond-anatomy-and-cashflows`](../08_convertible_bonds/cb-bond-anatomy-and-cashflows.md) — investor-perspective cash-flow view
- [`cb-mandatory-vs-optional-conversion`](../08_convertible_bonds/cb-mandatory-vs-optional-conversion.md) — investor-perspective on the conversion-vs-redemption decision
- [`cb-call-and-put-protection`](../08_convertible_bonds/cb-call-and-put-protection.md) — investor-perspective on the call protection that drives issuer-side repurchase-vs-conversion choice

## Escalate to Raw When

Open Kieso Ch.16 (pp.1255-1258) directly when any of the
criteria below applies. **Source:** Kieso (2020) Ch.16 pp.1255-1258.

- the issuer's pre-maturity repurchase involves a term
  modification rather than a simple extinguishment, requiring
  application of the IFRS 9 modification-vs-extinguishment
  10% test (the current scope defers this). **Source:**
  Kieso (2020) Ch.16 pp.1257; cross-reference IFRS 9
  paragraph 3.3.
- the analyst needs the worked Roche Group numerical
  examples for Path 4 (Illustration 16.6 and 16.7 gain/loss
  arithmetic) for cross-check. **Source:** Kieso (2020)
  Ch.16 pp.1257.
- the convertible has a contingent-conversion feature
  (conversion contingent on a share-price trigger, time
  trigger, or other contingent event) that complicates the
  Path 2/3 mechanics. **Source:** Kieso (2020) Ch.16 pp.1255-1258.
- the issuer's induced-conversion sweetener is structured
  as additional shares rather than cash, requiring fair-
  value measurement of the additional shares at the
  inducement date. **Source:** Kieso (2020) Ch.16 pp.1258.
