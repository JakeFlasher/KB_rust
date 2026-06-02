---
schema_version: "cacg.v0"
id: "be-regret-aversion-status-quo-endowment"
title: "Regret Aversion, Status-Quo, and Endowment"
reading_id: "10_behavioral_finance"
summary: "The emotional 'stickiness' cluster — regret aversion, status-quo bias, and the endowment effect — that combine to produce portfolio inertia: holding inherited or concentrated positions, avoiding decisive action, and demanding more to sell than to buy."
tags: ["behavioral-finance", "emotional-bias", "regret-aversion", "status-quo", "endowment-effect"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p246:0261"
    chunk_hash: "aeea7194f0f0f25b24857ee826a5cbfdb9a8ca03704923e3338128d918225843"
    page_range: [247, 247]
    quote: "actions because they fear that, in hindsight, whatever course they select will prove less than optimal."
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p267:0285"
    chunk_hash: "4a4e8d2003094118251074ad91aeaa725239e3d115c1f2803c915bfbd440f5ea"
    page_range: [268, 268]
    quote: "predisposes people facing an array of choice options to elect whatever option ratifies or extends the existing condition"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p158:0164"
    chunk_hash: "51c6ba4ef62dce1aeaaa46dafb2c760c7701a20a67fcec61edccfd4e0542da4e"
    page_range: [159, 159]
    quote: "People who exhibit endowment bias value an asset more when they hold property rights to it than when they"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p269:0287"
    chunk_hash: "bef4610a9c388c80d366e6038450f975b8924bf4b36e21489997e1bc76004d11"
    page_range: [269, 269]
    quote: "Loss aversion bias, endowment bias, and status quo bias often combine; and the result is an overall tendency to prefer things to stay as they are"
    edge_type: "supports"
card_hash: "b468a184457d3011e761f3db4184b1e998f4ee05b78bb2e3bdf3fba845e79f56"
---
# Regret Aversion, Status-Quo, and Endowment

## Intuition

Three of Pompian's emotional biases push in the same direction — toward *not acting* — and in practice they reinforce one another into portfolio inertia. Regret aversion makes people avoid decisive action because they fear that, in hindsight, whatever course they choose will prove less than optimal; the bias seeks to forestall the future pain of regret, so investors hold losing positions too long (to avoid admitting error) and stay out of recently fallen markets (to avoid being wrong again). Status-quo bias predisposes people to elect whatever option ratifies or extends the existing condition. Endowment bias makes people value an asset more once they own it than before.
**Source:** Pompian (2006) Ch.21 pp.227, Ch.23 pp.248, Ch.13 pp.139.

Pompian explicitly notes the cluster: loss aversion, endowment, and status-quo biases often combine, and the result is an overall tendency to prefer things to stay as they are, even when the calm comes at a cost. The canonical practitioner symptom is the inherited or concentrated stock position — the grandson who will not sell the bank stock he inherited even though his portfolio is dangerously underdiversified — driven by emotional attachment (status-quo/endowment), reluctance to admit the diversification was overdue (regret), and aversion to transaction costs. Because these are emotional, not cognitive, biases, the practitioner response leans toward adaptation, not correction.
**Source:** Pompian (2006) Ch.23 pp.249-250.

## Definition

**Regret aversion** is the emotional bias of avoiding decisive action for fear that the chosen course will, in hindsight, prove suboptimal; it spans errors of *commission* (regret from misguided action) and errors of *omission* (regret from misguided inaction), with commission regret felt more intensely.
**Source:** Pompian (2006) Ch.21 pp.227-228.

**Status-quo bias** (Samuelson-Zeckhauser, 1988) is the emotional predisposition to elect whichever option ratifies or extends the existing condition; it is a stronger form of inertia and implies a more intense "anchoring effect."
**Source:** Pompian (2006) Ch.23 pp.248-249.

**Endowment effect / endowment bias** is the tendency to value an asset more once one holds property rights to it; the minimum *willingness to accept* (WTA) to give it up exceeds the maximum *willingness to pay* (WTP) to acquire it, contrary to standard theory's prediction that WTP = WTA.
**Source:** Pompian (2006) Ch.13 pp.139-140.

## Mathematical Reasoning

The endowment effect is the cleanest formal anomaly: standard theory requires the willingness-to-pay to equal the willingness-to-accept, `WTP = WTA`, for the same good in the same liquid market. The endowment effect is the inequality `WTA > WTP`, because ownership "endows" the good with added value: removing a good from the endowment registers as a loss, while adding the same good registers as a (less heavily weighted) gain. Knetsch's mug-versus-candy experiment makes the asymmetry stark — when the mug was the endowed good, 89% kept it; when candy was the endowed good, 90% kept that; preferences tracked the endowment, not intrinsic desirability.
**Source:** Pompian (2006) Ch.13 pp.140-141.

The three biases are distinct but composable. Status-quo bias differs from endowment and loss aversion in that it does not depend on framing changes in losses versus gains; it is closer to pure inertia, the reluctance to leave a labeled "status quo" state. Endowment, by contrast, attaches added value to a held object regardless of label, and so by definition favors the status quo. Regret aversion supplies the dynamic motive — anticipated `E[regret]` of a commission error exceeds anticipated `E[regret]` of an omission error — so the inaction option is preferred whenever the agent decides on expected payoff *and* expected regret rather than payoff alone. When all three combine the net tendency is to prefer the present state even at a cost. (The source states the composition and the regret-theory framing in prose, without a unified parametric model.)
**Source:** Pompian (2006) Ch.23 pp.249, Ch.21 pp.228.

## See Also

- [be-cognitive-vs-emotional-bias-taxonomy](./be-cognitive-vs-emotional-bias-taxonomy.md#intuition) — parent: these are emotional biases, candidates for adaptation.
- [be-self-control-mental-accounting](./be-self-control-mental-accounting.md#intuition) — sibling emotional/self-control biases.
- [be-myopic-loss-aversion-equity-premium](./be-myopic-loss-aversion-equity-premium.md#intuition) — loss aversion, the fourth member of the inertia cluster.
- [be-regret-matching-foundations](./be-regret-matching-foundations.md#intuition) — the formal regret-minimization learning foundation.
- [be-reference-dependent-preferences-foundations](./be-reference-dependent-preferences-foundations.md#intuition) — reference-dependence underlying the WTA/WTP gap.

## Escalate to Raw When

- You need the Jim/Schmoogle commission-vs-omission worked illustration of regret asymmetry.
**Source:** Pompian (2006) Ch.21 pp.229.
- You need the New Jersey / Pennsylvania insurance-default natural experiment or the Samuelson-Zeckhauser inherited-portfolio study for status-quo evidence.
**Source:** Pompian (2006) Ch.23 pp.248-250.
- You need the Samuelson-Zeckhauser four-investment inheritance experiment that operationalizes endowment for inherited securities.
**Source:** Pompian (2006) Ch.13 pp.141.
