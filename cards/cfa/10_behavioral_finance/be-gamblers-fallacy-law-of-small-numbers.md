---
schema_version: "cacg.v0"
id: "be-gamblers-fallacy-law-of-small-numbers"
title: "Gambler's Fallacy and the Law of Small Numbers"
reading_id: "10_behavioral_finance"
summary: "The gambler's fallacy (a signal reduces the chance of repeating) follows from the Law of Small Numbers (small samples seen as representative); Rabin's urn-without-replacement model; the hot-hand bias as mirror image."
tags: ["behavioral-finance", "representativeness", "random-sequences", "belief-biases"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p082:0131"
    chunk_hash: "8478c0524ba38da4003fec81d0d165642993897e74c46eb6931d8e00a066e5ab"
    page_range: [83, 83]
    quote: "fallacy (GF) refers to the mistaken belief that, in a sequence of signals known to be i.i.d., observing one signal reduces the likelihood of next observing that same signal."
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p084:0135"
    chunk_hash: "8efa422c6eedcbc7099d41e512ecb868b3d03c2fa6efbeae0d228faa670dfa4e"
    page_range: [85, 85]
    quote: "view a sample randomly drawn from a population as highly representative, that is, similar to the population in all essential"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p085:0136"
    chunk_hash: "e07fc78b85e9d46e924a54ae37e64a9b5d1db480ab432b4b017c3faeb434dd8c"
    page_range: [85, 85]
    quote: "she forms beliefs as if the signals are drawn without replacement from an urn of finite size M"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p087:0140"
    chunk_hash: "38fa83a2a7cd3c99a0da8f0ba3f09b9012f9b77e23695e268477bec652f32c47"
    page_range: [87, 87]
    quote: "to someone who suffers from the GF, an i.i.d. process looks like it has too many streaks, so a belief in the hot hand arises to explain the apparent excess of"
    edge_type: "supports"
card_hash: "4cedfefd2719cf07f1a45a3ba3eb14f59ea7f5b81623581601a09b171f11ec39"
---
# Gambler's Fallacy and the Law of Small Numbers

## Intuition
After a coin lands heads several times, many people feel a tail is "due." This is the gambler's fallacy: in a process known to be i.i.d., observing a signal is felt to lower the chance of seeing the same signal next. It is one of the oldest documented belief biases, observed among gamblers, lottery players, roulette spinners, and even — strikingly — in professional decisions like asylum-court rulings and loan reviews, where decision-making is negatively autocorrelated after controlling for case quality.
**Source:** Benjamin (2019) Ch.2 §2.1 pp.83-84.

Tversky and Kahneman explained the fallacy with the Law of Small Numbers: people treat a small sample as highly representative of the population from which it is drawn, a tongue-in-cheek name for the mistaken belief that the Law of Large Numbers also applies to small samples. After a streak of heads, a tail is felt to be needed to "balance" the sequence and preserve the appearance of fairness. The same intuition implies people should over-infer from small samples.
**Source:** Benjamin (2019) Ch.2 §2.1 p.85.

## Definition
**Gambler's fallacy (GF)** is the mistaken belief that, in a sequence of signals known to be i.i.d., observing one signal reduces the likelihood of next observing that same signal.
**Source:** Benjamin (2019) Ch.2 §2.1 p.83.

**Law of Small Numbers (LSN)** is the belief that a randomly drawn sample is highly representative — similar to the population in all essential characteristics — regardless of sample size; equivalently, the belief that the Law of Large Numbers applies to small samples.
**Source:** Benjamin (2019) Ch.2 §2.1 p.85.

**Hot-hand bias** is a mistaken belief that a random process has more of a "hot hand" (positive autocorrelation, continuing streaks) than it truly does — the mirror image of the GF for human-performance settings.
**Source:** Benjamin (2019) Ch.2 §2.2 p.87.

## Mathematical Reasoning
Rabin's formal model of the LSN: signals are truly i.i.d. with `a` signals having rate `θ` and `b` signals rate `1 − θ`. Because the agent believes in the LSN, she forms beliefs *as if* the signals are drawn *without replacement* from a finite urn of size `M` containing `θ·M` copies of `a`. This directly generates the GF: after an `a` is drawn, the urn holds one fewer `a`, so the perceived probability the next signal is `a` is `(θ·M − 1)/(M − 1)`, which is strictly smaller than `θ`.
**Source:** Benjamin (2019) Ch.2 §2.1 p.85.

When the true rate is unknown and must be inferred, the same urn logic produces *over-inference*: an agent who sees a streak `aa` treats it as stronger evidence for a high-`θ` state than it truly is. With two states `A` (`θ_A`) and `B` (`θ_B`), the perceived likelihood ratio for `aa` is `π(aa|B)/π(aa|A) < (θ_B/θ_A)^2`, so a short streak is read as especially diagnostic. This over-inference, applied across many sources, makes the agent exaggerate cross-source variation in rates — for example concluding fund managers must be a mix of skilled and unskilled when in truth all are average.
**Source:** Benjamin (2019) Ch.2 §2.1 p.86.

The hot-hand bias arises as the mirror image. Rabin and Vayanos show that an agent who dogmatically believes (per the GF) that one component of a process is negatively autocorrelated, but puts small positive prior on a hot state, will — after observing an i.i.d. process long enough — come to believe in a hot state to explain the apparent excess of streaks: "to someone who suffers from the GF, an i.i.d. process looks like it has too many streaks, so a belief in the hot hand arises to explain the apparent excess of streaks." Applied to stock returns, this combined GF/hot-hand belief explains why investors expect returns to be partially predictable.
**Source:** Benjamin (2019) Ch.2 §2.2 p.87.

## See Also
- [be-representativeness-strength-vs-weight](./be-representativeness-strength-vs-weight.md#intuition) — the Griffin-Tversky framework where representativeness drives over/underreaction.
- [be-representativeness-conjunction-base-rate](./be-representativeness-conjunction-base-rate.md#intuition) — other representativeness-driven errors (conjunction fallacy, base-rate neglect).
- [be-diagnostic-expectations](./be-diagnostic-expectations.md#intuition) — representativeness-as-diagnosticity asset-pricing models.

## Escalate to Raw When
- You need the Miller-Sanjurjo statistical correction that re-opened the basketball hot-hand debate. **Source:** Benjamin (2019) Ch.2 §2.2 pp.89-90.
- You need the Rabin-Vayanos generalization beyond the finite-urn model or the Benjamin-Moore-Rabin coin-flip experiments distinguishing consecutive vs non-consecutive locations. **Source:** Benjamin (2019) Ch.2 §2.1 pp.86-87.
