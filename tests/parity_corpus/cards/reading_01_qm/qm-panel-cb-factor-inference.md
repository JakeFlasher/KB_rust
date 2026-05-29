---
schema_version: "cacg.v0"
id: "qm-panel-cb-factor-inference"
title: "Panel-Data CB-Arb Factor Inference"
reading_id: "reading_01_qm"
summary: "framing the panel-data inference discipline that gates CB-arb cross-issuer factor scoring — pooled OLS vs unit-fixed-effects vs random-effects estimators, the within transformation that sweeps issuer-specific intercepts out, and the Hausman specification test that adjudicates fixed-vs-random under unobserved-issuer-heterogeneity bias"
tags: ["concept", "factor-models", "panel-data"]
card_edges:
  - target: "qm-signal-validation-oos-discipline"
    edge_type: "extends"
citations:
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p001:0000"
    chunk_hash: "83a10bb86646580cedd46efe608803e7886588c99789a18497c422d8117e61d7"
    page_range: [1, 1]
    quote: "413 11 MODELS FOR PANEL DATA § 11.1 INTRODUCTION Data sets that combine time series and cross sections are common in"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p001:0001"
    chunk_hash: "e4045c8ffa6a9262614f9895f632b850fe24f76e1da5e039e2b906ed10d837ff"
    page_range: [1, 2]
    quote: "In Section 11.8, we consider sources of endogeneity in the random effects model, including a model of the sort"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p002:0002"
    chunk_hash: "de4e72283c8356b8fc46b800e837459c60216f986744ae1ef1b156be7c3214a7"
    page_range: [2, 2]
    quote: "Two very famous ones are the National Longitudinal Survey of Labor Market Experience (NLS, www.bls.gov/nls/ nlsdoc"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p002:0003"
    chunk_hash: "4970154dba599e6c2578d1ad0615125c9ddd89b80c6297986b318314e86843bd"
    page_range: [2, 3]
    quote: "They are typically modeled as specific to the period in which they occur and are not carried across periods within a"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p003:0004"
    chunk_hash: "17d94089b7c1fb3f017d4c733b481bcaacfc37487515755258c25764be936b82"
    page_range: [3, 3]
    quote: "The basic framework for this discussion is a regression model of the form yit = xit = B + zi = A + eit = xit = B + ci +"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p003:0005"
    chunk_hash: "f7aaeabab89733a7c3e3199dfd31f67f6b3dae90f538633c644f921a9b632d99"
    page_range: [3, 4]
    quote: "If E[yit xi1, c, xiT, ci ] = E[yit xit, ci ] = xit = B + ci , then E[eit xit, ci ] = 0. 3 For example, Riphahn et al"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p004:0006"
    chunk_hash: "7bf4f58830d8b57e97d9ac72310d61e6f6469f871df5ed16f75aeadde95114a8"
    page_range: [4, 5]
    quote: "Broadly, they can be arranged as follows: 1. Pooled Regression: If zi contains only a constant term, then ordinary"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p005:0007"
    chunk_hash: "fae6c48990a5f3ab0a88d4602b6902ada1f7c15fa543a1c28d75de2d18fb0d8c"
    page_range: [5, 5]
    quote: "Random Parameters: The random effects model can be viewed as a regression model with a random constant term"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p005:0008"
    chunk_hash: "3b06f2164536970db3588f1551d9d2d41437242c0e52c5e56f952a31527ecf21"
    page_range: [5, 6]
    quote: "We will examine all of these in the chapters to follow. In some cases, such as the models for count data in Chapter 18,"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p006:0009"
    chunk_hash: "fcb0111c5cbda1d2cec2863abc82285f7ca680d6ddba34657d88fc07958ff3bf"
    page_range: [6, 6]
    quote: "This is a quarterly data set drawn from 1987 to 1993 in which individuals are interviewed five times"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p006:0010"
    chunk_hash: "d04d847dd8bbcb316d16f38dd1ba2ed57cdec1bbcc4d5a57a17d2ef2a7b04030"
    page_range: [6, 7]
    quote: "Individuals may appear for only a subset of the waves. In general, if the attrition is systematically related to the"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p007:0011"
    chunk_hash: "7bf7b058b1053c14517c30f385fcf7676687badeaf03a38191f83d52a222fb08"
    page_range: [7, 7]
    quote: "The results at this step included those in Table 11.2 (extracted from their Table IX)"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p007:0012"
    chunk_hash: "66f967159920cfc0956d1e63e54ef41136b1390c5619329a85d78145768a0877"
    page_range: [7, 8]
    quote: "The results below show the influence of the sample treatment on one of the estimated coefficients in the full model"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p008:0013"
    chunk_hash: "d5e5b0165aea4658ed4bc6924868abfc80b7c53d4cd69180d176eb497de7e3c2"
    page_range: [8, 9]
    quote: "This setup is an application of Heckman’s (1979) sample selection framework. (See Section 19.5"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p008:0014"
    chunk_hash: "f13e4bf624e67ebd566f0e257d4d0b8a6276c72af957e5aed4dd393a3ad172a5"
    page_range: [8, 9]
    quote: "TABLE 11.4 Attrition from the Medicine in Australia Balancing Employment and Life Data CHAPTER 11 ✦ Models For Panel"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p009:0015"
    chunk_hash: "a1b03692e87a2340cec5f94e0eab24a365590e4554b673cbbc2e7249e68f35d9"
    page_range: [9, 10]
    quote: "A second approach that loosens the bivariate normality assumption is based on a copula model (Section 12.2"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p010:0016"
    chunk_hash: "96a6259df9d9085efcb019dbfb82b6a15a416f6a821b499c86874bfb0290b917"
    page_range: [10, 10]
    quote: "The development to follow is structured so that the distinction between balanced and unbalanced panels, beyond the"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p010:0017"
    chunk_hash: "5e6675a2820d10222eefd83a4723fb1fd4551b0ec027ac9d3f446560f52decb4"
    page_range: [10, 11]
    quote: "In this case, the x’s will surely be correlated across observations, at least within observational units"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p011:0018"
    chunk_hash: "7a3c98d475e297b81de002d9590e46587b7ead0c5b4cd4a968c532ec992d8d9b"
    page_range: [11, 12]
    quote: "In this form, if the remaining assumptions of the classical model are met (zero conditional mean of eit,"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p012:0019"
    chunk_hash: "4d5c3965624ddbb6827849a90ea2a77792383ba4efed9813050857ae54e7d250"
    page_range: [12, 12]
    quote: "Stack the Ti observations for individual i in a single equation, yi = Xi B + wi , where B now includes the constant"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p012:0020"
    chunk_hash: "b186868e49a1c96d5575e33d8b5ca92e35fedccddd5799c31d1322caa4d56624"
    page_range: [12, 13]
    quote: "This result provides the counterpart to (9-12). As before, the center matrix must be estimated"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p013:0021"
    chunk_hash: "489144d44fc39f52a4f2afc65c4eac0e6e7ad111d04b2478cafb0034c506b30a"
    page_range: [13, 13]
    quote: "The estimator is Est.Asy.Var[b] = 1 R aR r = 1(br - b)(br - b)= . Example 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p013:0022"
    chunk_hash: "98e1db9df3c46c5ad510244deb9515503a365d47a82378ca1518985f8f02161d"
    page_range: [13, 14]
    quote: "However, they do report linear least squares estimates of the fixed effects model, which are simple least squares using"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p014:0023"
    chunk_hash: "206f8890c3e00ef57e24c233fa379779ad22f74a342034dc00882c8dce234b8b"
    page_range: [14, 15]
    quote: "Each of these is likely to induce correlation across observations that resembles the random (or fixed) effects we have"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p015:0024"
    chunk_hash: "ef1c61702e9127d21bfd42839aa42008e99c023891820b929782ec9109857e53"
    page_range: [15, 15]
    quote: "It is worth noting the Moulton bias might create the impression that the correction of the standard errors always"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p015:0025"
    chunk_hash: "3370ab5211d64fb645f6479a4782eece3b98c8a1a1d3ff100c14ba91199b778c"
    page_range: [15, 16]
    quote: "Many further refinements for more complex samples—consider the test scores example—have been suggested"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p016:0026"
    chunk_hash: "e21f71cf68b6b088ec2effb9c201e85921c31226664e2437839061b3644df28f"
    page_range: [16, 16]
    quote: "If the data-generating mechanism were strictly consistent with the random effects model, the answer would clearly be"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p016:0027"
    chunk_hash: "fee5913de6dc9e611d1603a61685c9116ce6a6ba4c3a1dc3510012546054614f"
    page_range: [16, 17]
    quote: "The loss of information that occurs through the averaging might be relatively small, though in principle the"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p017:0028"
    chunk_hash: "57be124d476e4990270a8f243d2f07139be46411fabe727621db0cb872a6d3e3"
    page_range: [17, 18]
    quote: "Example 11.5 Robust Estimators of the Wage Equation Table 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p018:0029"
    chunk_hash: "956768aa0686969b530e15798ce2902e7f1cd1942d01a54c265fd56722dcc61a"
    page_range: [18, 18]
    quote: "Of course, this is not helpful for the application in the example because the impact of Ed on ln Wage was the primary"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p018:0030"
    chunk_hash: "2d72f697872dbd21aa7b303340f0c7095d2ef580ccd6b4d484e43309dabef120"
    page_range: [18, 19]
    quote: "The treatment effect would be E[∆yi (∆xi = 0)] = u, which is precisely the constant term in the first difference"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p019:0031"
    chunk_hash: "bffcfaa575a0ee16a0caa29b3f763fb390aafec7b249c9916c52a578f4513284"
    page_range: [19, 20]
    quote: "In (11-5a), the moments would accumulate variation about the overall means, y and x, and we would use the total sums of"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p019:0032"
    chunk_hash: "6c77fa3a05b6e53f219e3e37da55d756dc078e7efde4cf3a836134d55f334486"
    page_range: [19, 20]
    quote: "An alternative estimator would be the between-groups estimator, 432 PART II ✦ Generalized Regression Model and Equation"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p020:0033"
    chunk_hash: "dad19b25a012acf913af9aa072bccece5ea5cda622d158da42c8352142787be4"
    page_range: [20, 21]
    quote: "The WHO data used in Example 6.22 is an unbalanced panel data set—we used only one year of the data in Example 6.22"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p021:0034"
    chunk_hash: "625d50ee65faa094240b8301bc6dc7a082b03cc4ff4e8ee463c4430238776072"
    page_range: [21, 21]
    quote: "Table 11.8 lists the decomposition of the variation in the variables used in the WHO studies"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p021:0035"
    chunk_hash: "5bb2121ddc95d19e61d09ec9cc3591d054e46d3bf6cf1b9a08e47dcd777069f3"
    page_range: [21, 22]
    quote: "A study on the topic is Cornwell and Schmidt (1984). We will examine this case in Section 11.4.6"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p022:0036"
    chunk_hash: "844db25626aba7affec06a2866d38429e55ab603c0fe790b9ca997ab2e7bb810"
    page_range: [22, 23]
    quote: "Because MD is symmetric and idempotent, bLSDV = [(X′MD)(MDX)] -1 [(X′MD)(MDy)]"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p023:0037"
    chunk_hash: "3270b36676836fe4a055a8f7a77544939c3ed28880b3b0316d3c135fde187727"
    page_range: [23, 23]
    quote: "This implies that for each i, ai = yi. - xi. = bLSDV. (11-16b) The appropriate estimator of the asymptotic covariance"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p023:0038"
    chunk_hash: "5bdf3b0ff7c5811ed96ceaf1dd916b0f5559e7dbb2ba2f9f22f74b522bc7065c"
    page_range: [23, 24]
    quote: "This is a case in which the MLE is biased, given (11-18) which gives the unbiased estimator"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p024:0039"
    chunk_hash: "5fa292f032106f933bfc67bec27d362ad958a9ec0c8e09e078aa1ba6f8fd7a90"
    page_range: [24, 24]
    quote: "The fixed effects formulation of the model will absorb the last four terms in the regression in ai"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p024:0040"
    chunk_hash: "700b9d7f3ffee1ef5b990fe89c58a9875fa27b182fd5793aa8b98bb46bbc6795"
    page_range: [24, 25]
    quote: "But M0 is idempotent, so X $ i = E $ i = X $ i Ei, and we have assumed that E[Ei Ei = X] = se 2 I"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p025:0041"
    chunk_hash: "3e39d359d14b79169a0e7ae375051644c12455376c4423a9c42b60eb78982e9e"
    page_range: [25, 25]
    quote: "If we are interested in differences across groups, then we can test the hypothesis that the constant terms are all"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p025:0042"
    chunk_hash: "043e2ddd8ac7dca91a86f5da5c4bdba263bfe4c1bf9765c2741d573d8fd71be0"
    page_range: [25, 26]
    quote: "The OLS and fixed effects estimates are presented in Table 11.9"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p026:0043"
    chunk_hash: "981f77fe61c18d0c031ffc5fb652e8ae4719b74756839d4904dfad474c06cd7b"
    page_range: [26, 27]
    quote: "The critical value from the F table would be less than 1.3, so the hypothesis of homogeneity is rejected. 11.4"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p027:0044"
    chunk_hash: "b2be52405eb22304283a72831e95eec3c0bd980d89ded5bc05630fb9b435c4c3"
    page_range: [27, 27]
    quote: "CHAPTER 11 ✦ Models For Panel Data 439 where a full n and T effects are included, but the restrictions a i ai = at dt ="
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p027:0045"
    chunk_hash: "e22845d76c533ef6b54d7992aab3dae126ffa9de15dac75220c5627b045c0f65"
    page_range: [27, 28]
    quote: "Example 11.8 Two-Way Fixed Effects with Unbalanced Panel Data The following experiment is done with the Cornwell and"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p028:0046"
    chunk_hash: "94467f72270df4b7094ca217740751055b2ad61ba06de991f0e552a60e5cec67"
    page_range: [28, 28]
    quote: "These are both straightforward to verify. For the exogeneity condition, let c denote the full set of common effects"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p028:0047"
    chunk_hash: "0397e2b5ca03225c672088a66af91ffc70d5ea0b01f9a82a23e1994aa2f42eb4"
    page_range: [28, 29]
    quote: "So, the group means qualify as a control function, as defined in Section 8.4.2"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p029:0048"
    chunk_hash: "220da5cce92f5cf175855971f54cc7e9f4e64b11c4b66e2a7efec6dea63140f8"
    page_range: [29, 30]
    quote: "In this fixed effects setting, the dummy variable approach of Section 11.4"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p030:0049"
    chunk_hash: "f00578d9f87513d78de751c4f55cd78e76f1b6094f8000422362748e88c28360"
    page_range: [30, 30]
    quote: "In this specification, we have simply respecified D to contain two or more sets of N columns"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p030:0050"
    chunk_hash: "40957a7fac07338d9dee1947747244f004b6057ffc7ad60411e17a2f1529bb9c"
    page_range: [30, 30]
    quote: "The asymptotic covariance matrices in (11-17) and (11-21) are computed as before"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p030:0051"
    chunk_hash: "68a5e53bd16a24b9451ca5fc82db23471dd9a0ae237e3c3780941c3363e15701"
    page_range: [30, 31]
    quote: "The time trend becomes the common effect. This can be treated as a fixed effects model"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p032:0052"
    chunk_hash: "4c3715ea96ff5d0e51884d8d77c5871a6c789670da1ed2552db0774353256a38"
    page_range: [32, 32]
    quote: "444 PART II ✦ Generalized Regression Model and Equation Systems fixed effects models"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p032:0053"
    chunk_hash: "2fadd9595f9215d7590139b0628cb26a8168845c0f3eafda87e2c75d60ed769e"
    page_range: [32, 33]
    quote: "This model might be viewed as applying only to the cross-sectional units in the study, not to additional ones outside"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p032:0054"
    chunk_hash: "a9716fdf973955668fa845a1fb262e9a347fd2166f3bdf513e905fb39bcc139b"
    page_range: [32, 33]
    quote: "We shall return to this issue later. See Mundlak (1978) for a methodological discussion of the distinction between"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p033:0055"
    chunk_hash: "ea2a8ff2a3957d9274aafb7de8fc2f474241a17fa6c3ac4e85ad3ee376624dc1"
    page_range: [33, 34]
    quote: "Because observations i and j are independent, the disturbance covariance matrix for the full nT observations is 𝛀 = D 𝚺"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p034:0056"
    chunk_hash: "c07715eb34664ca4cefcdd85b2289295b697ca8843f6c91312ef3d020cbd1da9"
    page_range: [34, 34]
    quote: "None of these is the preferred estimator in this setting because the GLS estimator will be more efficient than any of"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p034:0057"
    chunk_hash: "4efe6398c9a76c6c3e3cbf8475b268d4e9fd9f7eef1605a8008ee519c0d972a6"
    page_range: [34, 35]
    quote: "Unfortunately, unlike the others, this could be negative in a finite sample"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p035:0058"
    chunk_hash: "d9c19fb74f7dcde6c3ab50ec201ba538bbd7f0cbb509a6030afc73ff94aa6572"
    page_range: [35, 36]
    quote: "To the extent that l differs from one, we see that the inefficiency of ordinary least squares will follow from an"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p036:0059"
    chunk_hash: "67c9de988d798a7696526730610c7771e72bb32dbdb19714236639f1725a3009"
    page_range: [36, 36]
    quote: "Of course, this is unlikely, so as usual, we must first estimate the disturbance variances and then use an FGLS"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p036:0060"
    chunk_hash: "f4b019507f881e6c3599c847aa7a6c2228a7af8fd80d9a0c4818cb21f12dde19"
    page_range: [36, 37]
    quote: "The estimated parameters are the n means yi# and the K slopes"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p037:0061"
    chunk_hash: "7f52b8e4a12106ccd4e8b128ba9491c81d3b123c541715ab098434491289d101"
    page_range: [37, 38]
    quote: "A possible complication is that the estimator of su 2 can be negative in any of these cases"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p037:0062"
    chunk_hash: "0240b7fdf2b67ca454afebba346a709127b0e6041ffc8a9a349aaac7e5fed9f3"
    page_range: [37, 38]
    quote: "The practitioner is strongly advised to consult the program documentation for resolution"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p038:0063"
    chunk_hash: "7c4be3f1e71541ee840bef6fc8b791687199daaedd63bb59ffee36e3f47c8528"
    page_range: [38, 39]
    quote: "This is not so with an unbalanced panel. We will see in the example below, in this more general case, a distinct"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p039:0064"
    chunk_hash: "16abac0e35bc328235abf20add538abb144ea39ea256b76ca5b7264555566ee9"
    page_range: [39, 39]
    quote: "The inner double sums in the statistic sum the below diagonal terms in ei ei = which is one-half the sum of all the"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p039:0065"
    chunk_hash: "91f967a8f96ba7b7f2784f131cc4037725290fdb44db7b81bc0ca70e5c1e9dcc"
    page_range: [39, 40]
    quote: "The results for the two statistics are LM = 3497.02 and z2 = 179.66"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p040:0066"
    chunk_hash: "d94656b28490344a9307210199cfc2531fbdda2de15b8c04af178fe4975d090f"
    page_range: [40, 40]
    quote: "452 PART II ✦ Generalized Regression Model and Equation Systems Example 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p040:0067"
    chunk_hash: "5e4969f853df5fc901dc8dab040189fdd35347538805e878734905a3a227fb18"
    page_range: [40, 41]
    quote: "The assumption that the disturbances are equally correlated across periods regardless of how far apart the periods are"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p040:0068"
    chunk_hash: "52eef5935d848a0a34fb42e15d12665cf9aa66151ae9c0ef847f57910264380b"
    page_range: [40, 41]
    quote: "Although it can be shown that some consistency results will follow for T increasing, the typical panel data set is"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p041:0069"
    chunk_hash: "3c1d02b3ab5c85463bfd1e9eca0e39073eef13280660db1174e76c8b43bd4a9f"
    page_range: [41, 42]
    quote: "Some additional results for dynamic models are given by Bhargava and Sargan (1983)"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p042:0070"
    chunk_hash: "32765f25474bdaeea2e1b12368b57f531fd487153260fd5af5ef92fea232f307"
    page_range: [42, 43]
    quote: "Inserting this result in (11-43) produces the required covariance matrix for the test, Var[bFE - Bn RE] = Var[bFE] -"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p043:0071"
    chunk_hash: "84b4e9a01875907095065dfb5aecb46b1f1bf903caf93b22105e872f6455447b"
    page_range: [43, 43]
    quote: "An asymptotically equivalent test statistic is given by H′ = (bFE - bMEANS)′[Asy.Var[bFE] + Asy"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p043:0072"
    chunk_hash: "5eeb0a7bf94a82099176de01eb3c353c3c7a349e432c564fd71725abe3caf259"
    page_range: [43, 44]
    quote: "Note that the 27That is, “It makes no sense to report a fully robust variance matrix for FE and RE but then to compute"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p044:0073"
    chunk_hash: "ef8b57c3a54aba0be9077c1baa132408a7c7ee66f73df71d57cbfe208939ad6c"
    page_range: [44, 44]
    quote: "The pooled OLS estimator is fully robust and seems preferable"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p044:0074"
    chunk_hash: "d55ea12c5f9f2b69befdb669f24794e45aa49ad7cb552cecef5e5eb2fe7788f1"
    page_range: [44, 45]
    quote: "We recovered the subvector of the estimates at the right in Table 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p045:0075"
    chunk_hash: "c8ad47aa7dec27e913c6b5019449d4ff6f9340b7c4a0f3a565cd35a98b2ee1ec"
    page_range: [45, 45]
    quote: "We found some contradictory empirical evidence in Example 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p045:0076"
    chunk_hash: "f28a8fb45c6f511a64eb4007e0593b070b10948d2be39f612c1d5f5e9ef20153"
    page_range: [45, 46]
    quote: "The central feature of the fixed effects model in Section 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p046:0077"
    chunk_hash: "cb2430924e34ca8a3e54a6d9f0fa6250d0daf4d13f77dd42f3da81ac96cfb4a6"
    page_range: [46, 46]
    quote: "For purposes of this development, we will assume T = 3. The generalization will be obvious at the conclusion"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p046:0078"
    chunk_hash: "61f8453f6a0cdbf00789050184634816643dd3375468c5b6f806e8ef423a5df4"
    page_range: [46, 47]
    quote: "This does not affect the asymptotic properties of the FGLS estimator to be developed here, although it does have"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p047:0079"
    chunk_hash: "85f91bad219e74c34fd7ace087d9ba189a68db6565647e096ec5e421496da98f"
    page_range: [47, 47]
    quote: "For the first period, y1 = § y1,1 y2,1 f yn,1¥ = D 1 x1,1 x1,1 x1,2 x1,3 1 x2,1 x2,1 x2,2 x2,3 f f f f f 1 xn,1 xn,1"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p048:0080"
    chunk_hash: "590bfe235805429a6a8d9bc47b6939b7ace3f82447ac1c8814d00064abd3e7d9"
    page_range: [48, 48]
    quote: "460 PART II ✦ Generalized Regression Model and Equation Systems where TC = total cost, P = input price index, DIS ="
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p048:0081"
    chunk_hash: "4eaf6390ce8ba5b69ff2b0952c5730a5d3755b057abbaaa4e162aabbe271b53a"
    page_range: [48, 49]
    quote: "Therefore, the structural parameters of interest are (bD1, c, bD5), (gD1 c, gD5) (the coefficients on DIS) and (bO1, c,"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p049:0082"
    chunk_hash: "3c2eddc3ec1b429c8dc2eda4019d429d3712ec287975dfeddebe1989e20030e8"
    page_range: [49, 49]
    quote: "That is, we can compute the ordinary or feasible generalized least squares estimators and obtain an appropriate robust"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p049:0083"
    chunk_hash: "44aa351d050eaa605f88ac31737a361a5f914cf632ad1d4013dc266a222d5a82"
    page_range: [49, 50]
    quote: "The correct value is 0.0822. (Personal communication with the author.) TABLE 11"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p050:0084"
    chunk_hash: "22f1148ceadcad0d5980ec192bf607e67bf781b7b2682aab58391ed22aa5ace5"
    page_range: [50, 51]
    quote: "The results there would apply equally to clustered observations, as observed in Section 11.3.3"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p051:0085"
    chunk_hash: "e52489244d16fb2f3ae6a8599c1a115fc3e31ce09690871191b72f16d57918b6"
    page_range: [51, 51]
    quote: "The correlation across space is implied by the spatial autocorrelation structure, eit = la n j = 1 Wijejt + vt"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p051:0086"
    chunk_hash: "ea4183dafe5f16f7009b262033942add997f4d0ecf1dad74859ded393f9733a5"
    page_range: [51, 52]
    quote: "There is no natural residual-based estimator of l. Recent treatments of this model have added a normality assumption"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p052:0087"
    chunk_hash: "a02bdabdaef413500d2006c2e4a5197155a5a285fa9b5637adeaf3f0015593b0"
    page_range: [52, 53]
    quote: "A “pure space-recursive model” specifies that the autocorrelation pertains to neighbors in the previous period, yit ="
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p053:0088"
    chunk_hash: "4a8364c626c0434e03879b1ee9e497259fd77516a6c935c2220a8933ed66ba1c"
    page_range: [53, 53]
    quote: "The authors applied the method to analysis of a cross section of 1,000 residential sales in Anne Arundel County,"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p053:0089"
    chunk_hash: "c2239ff129bbd2125812bbe24851d48f5efed9a3d0145c8676e57bdec5016eb8"
    page_range: [53, 54]
    quote: "Test statistics for spatial autocorrelation based on the OLS residuals are shown in Table 11.14"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p054:0090"
    chunk_hash: "88211e63f9e2e9d664c359e7f6391defe43c3a7511b1e9549d36a87cf3a2e4d9"
    page_range: [54, 55]
    quote: "For each local authority, this model implies yit = gt + xit = B + ui + lΣj wij ejt + vit"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p054:0091"
    chunk_hash: "0815088060281600cae85a900266348db8db1039a6ad57a125fbc808c39857a3"
    page_range: [54, 55]
    quote: "The function f(.) is not identified. TABLE 11.15 Estimated Spatial Regression Models CHAPTER 11 ✦ Models For Panel Data"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p055:0092"
    chunk_hash: "42aafae9f3f0228f6917a143521fe247965740e9cbed0f491a0ad945cb305891"
    page_range: [55, 56]
    quote: "At this point, we can examine three major building blocks in this set of methods, a panel data counterpart to two-stage"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p056:0093"
    chunk_hash: "12af501994312508dab6fc9f63b9eba7f4d2633eba09ba8d39aa18ed9c74cfc6"
    page_range: [56, 56]
    quote: "468 PART II ✦ Generalized Regression Model and Equation Systems We can see from this expression that this computation"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p056:0094"
    chunk_hash: "67ce37d3f26e4f02b6153bec4feaf2fb8cc74287b5b456b045f42dfb79298f10"
    page_range: [56, 57]
    quote: "The steps follow the earlier prescription: 1. Use pooled 2SLS to compute Bn IV,Pooled and obtain residuals w"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p057:0095"
    chunk_hash: "cd2cf223088b23aee2cb6138dc12c9816245a66e521d845cb2015429e9390ec8"
    page_range: [57, 57]
    quote: "The variable ln Income is endogenous in the health equation. There is also a time-invariant variable, Female, in the"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p058:0096"
    chunk_hash: "b011f4e4bfb8e2610f064e67efdda978696939d31c79ef55f2fac43cd89b708b"
    page_range: [58, 58]
    quote: "470 PART II ✦ Generalized Regression Model and Equation Systems Their model is of the form yit = x1it = B1 + x2it = B2"
    edge_type: "supports"
  - source_id: "qm_greene_trim"
    chunk_id: "qm_greene_trim:p058:0097"
    chunk_hash: "98b1fcfda5998575bf0ea646f8d9c3620fd12ec0d7d1a78c5effbbaa6535ac53"
    page_range: [58, 58]
    quote: "By construction, any OLS or GLS estimators of this model are inconsistent when the model contains variables that are"
    edge_type: "supports"
card_hash: "51c785a0e3a3502d1ced9d1ccdfe6fe9d9603b99baec3f9de55a70873741ff7e"
---
framing the panel-data inference discipline that gates CB-arb cross-issuer factor scoring — pooled OLS vs unit-fixed-effects vs random-effects estimators, the within transformation that sweeps issuer-specific intercepts out, and the Hausman specification test that adjudicates fixed-vs-random under unobserved-issuer-heterogeneity bias

## Original Card (preserved verbatim)

## Intuition

A CB-arb cross-issuer factor signal must contend with the fact that
the conversion-premium, credit / equity ratio, and bond-floor distance
of any one issuer carry persistent, issuer-specific level effects that
are unrelated to the cross-sectional factor scores being tested. The
pooled-OLS estimator that the upstream factor-construction pipeline
(see [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md))
fits to the stacked `(x_{it}, y_{it})` panel inherits an
omitted-variable bias whenever those issuer-specific levels are
correlated with the regressors — the unit-fixed-effects estimator
removes that bias by transforming each observation into a deviation
from its own issuer mean, sweeping the unobserved intercept `α_i`
out of the score equation; the practitioner-quoted CB-arb
relative-value pipeline at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
benefits from this issuer-fixed-effects within-transformation
whenever the cross-sectional signal is estimated on data with
persistent issuer heterogeneity.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

Greene Ch.11.3 illustrates the panel-data machinery on a
manufacturing-productivity panel of firms observed over multiple
years, where the within estimator strips out time-invariant firm
characteristics (managerial capability, capital vintage) that would
otherwise confound the marginal-productivity slope; the same
fixed-effects-by-issuer machinery applies to controlling for
unobserved issuer heterogeneity in CB-arb relative-value factor
inference (the CB-arb application grounding is supplied by the
explicit cross-link to
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
above and the downstream signal-validation discipline at
[`qm-signal-validation-oos-discipline`](qm-signal-validation-oos-discipline.md)
gates which fixed-effects-estimated factor scores survive the
out-of-sample test).
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

```
<!-- primitive: panel-fixed-effects-decomp source: _diagram_primitives.md -->
   y_{it} = α_i + β · x_{it} + ε_{it}
   ───────────────────────────────────────────────────────
                       ▲          ▲           ▲
                       │          │           │
        unit-fixed-effect      common      idiosyncratic
        (issuer / firm /       slope       within-unit
         person α_i)           β            shock ε_{it}

   Within transformation (sweeps α_i out):
   (y_{it} − ȳ_i) = β · (x_{it} − x̄_i) + (ε_{it} − ε̄_i)

   ───────────────────────────────────────────────────────
        Unit i = 1    │    Unit i = 2    │    Unit i = 3
                      │                  │
        ●     ●       │      ●           │           ●
          ●           │   ●     ●        │     ●  ●
                      │                  │
        ─── α_1 ───   │   ─── α_2 ───   │   ─── α_3 ───
                      │                  │
        within-i β identified after subtracting α_i
```

## Definition

The panel-data regression `y_{it} = α_i + β · x_{it} + ε_{it}`
indexed over cross-sectional units `i ∈ {1, ..., N}` and time
periods `t ∈ {1, ..., T}` admits three canonical estimators of the
slope `β`: the pooled-OLS estimator
treats `α_i = α` constant across units and fits the stacked sample
directly; the unit-fixed-effects (within) estimator allows each `α_i`
to be a free parameter and either subtracts the within-unit mean
from both `y_{it}` and `x_{it}` (the within transformation) or
introduces `N` unit dummies (the LSDV equivalent); the random-effects
estimator treats `α_i` as a unit-specific random draw from a
distribution with `E[α_i | x_{it}] = 0` and uses the GLS-weighted
sample. Greene Ch.11.4 derives the algebra of the within
transformation and proves that the fixed-effects slope estimator is
unbiased and consistent under strict exogeneity `E[ε_{it} | x_{i,1},
..., x_{i,T}, α_i] = 0`, regardless of whether `α_i` is correlated
with `x_{it}`. The random-effects estimator is more efficient under
the additional zero-correlation assumption but inconsistent under
its violation.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Hausman specification test adjudicates the fixed-vs-random
choice by computing a quadratic-form statistic in the difference
between the two slope estimators, `H = (β̂_FE − β̂_RE)' · (V̂_FE −
V̂_RE)^{-1} · (β̂_FE − β̂_RE)`, which is asymptotically chi-square
distributed with degrees of freedom equal to the number of slope
parameters under the null of zero correlation between `α_i` and
`x_{it}`. Greene Ch.11.5 derives the variance-difference matrix
algebra and gives the null-distribution argument. Rejection of the
null is the operational signal that issuer-fixed-effects must be
retained for unbiased CB-arb factor-slope inference; the practitioner
application of this gate to the CB-arb cross-issuer factor pipeline
is documented at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
where the relative-value-screen scoring sits downstream of the
issuer-fixed-effects choice made here.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Wooldridge Intro 8e treatment reframes the same machinery at
undergraduate depth: the within (or "fixed effects") estimator is
introduced via first-differencing for the two-period case and via
the within-mean subtraction for the general T-period case; the
between estimator (cross-sectional regression on unit means) is
contrasted with the within estimator as the variance-decomposition
complement; the random-effects estimator is positioned as the GLS-
weighted combination of within and between variation under the
exogeneity-of-`α_i` assumption. Wooldridge Intro Ch.13-14 also
introduces the clustered-standard-errors correction that adjusts
the OLS variance estimator for within-unit serial correlation in
the residuals.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

## Mathematical Reasoning

The pooled-OLS estimator (source ASSERTS) of the stacked panel
`y_{it} = α + β · x_{it} + (α_i − α + ε_{it})` is biased and
inconsistent whenever `Cov(x_{it}, α_i) ≠ 0` because the composite
error `u_{it} = (α_i − α) + ε_{it}` is correlated with the
regressor. Greene derives the within (fixed-effects) estimator
`β̂_FE = [Σ_i Σ_t (x_{it} − x̄_i)(x_{it} − x̄_i)']^{-1} · Σ_i Σ_t
(x_{it} − x̄_i)(y_{it} − ȳ_i)` by least-squares minimisation of the
within-transformed residual sum of squares; the unbiasedness of
`β̂_FE` under strict exogeneity follows from the orthogonality of
the demeaned regressor with the demeaned idiosyncratic shock.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The random-effects estimator (source ASSERTS) imposes
`E[α_i | x_{it}] = 0` and `Var(α_i) = σ_α²` and applies GLS to the
composite-error specification with covariance matrix `Σ = σ_ε² · I_T
+ σ_α² · ι_T · ι_T'` where `ι_T` is the `T × 1` vector of ones; the
GLS transformation subtracts a fraction `θ` of the within-unit mean
from each observation, with `θ = 1 − √(σ_ε² / (σ_ε² + T · σ_α²))`.
The random-effects estimator is more efficient than fixed effects
under the zero-correlation assumption but inconsistent if the
assumption fails.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Hausman specification test (source ASSERTS) exploits the fact
that under the null of `Cov(α_i, x_{it}) = 0` both `β̂_FE` and
`β̂_RE` are consistent but `β̂_RE` is efficient, so the difference
`β̂_FE − β̂_RE` has variance `V̂_FE − V̂_RE` (Hausman's lemma); under
the alternative `Cov(α_i, x_{it}) ≠ 0`, `β̂_FE` remains consistent
while `β̂_RE` is inconsistent so the difference has nontrivial
probability limit. The statistic `H = (β̂_FE − β̂_RE)' · (V̂_FE −
V̂_RE)^{-1} · (β̂_FE − β̂_RE)` is asymptotically `χ²_k` under the
null, with `k` equal to the number of time-varying regressors.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The between estimator (source ASSERTS) regresses unit-time-averaged
responses `ȳ_i` on unit-time-averaged regressors `x̄_i` across the
`N` cross-sectional units, exploiting only cross-unit variation and
discarding the within-unit information. The OLS estimator of the
pooled panel decomposes as a matrix-weighted average of the within
and between estimators, with weights determined by the relative
within-unit and between-unit variation in the regressors. Wooldridge
Intro Ch.14 motivates the decomposition for the two-period case via
first-differencing.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

The clustered-standard-errors correction (source ASSERTS) replaces
the OLS variance estimator with a sandwich form that aggregates the
score contributions within each unit cluster before averaging
across clusters: `V̂_cluster = (X'X)^{-1} · (Σ_i X_i' · ε̂_i · ε̂_i'
· X_i) · (X'X)^{-1}`, where `X_i` is the `T × k` matrix of regressors
for unit `i` and `ε̂_i` is the `T × 1` residual vector. The correction
delivers consistent standard errors under arbitrary within-unit
serial correlation of the idiosyncratic shock, as long as the
number of clusters grows.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

## See Also

- [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md) — the upstream factor-construction step whose stacked-panel pooled-OLS estimator inherits the omitted-variable bias addressed here by the within (unit-fixed-effects) estimator
- [`qm-signal-validation-oos-discipline`](qm-signal-validation-oos-discipline.md) — the downstream out-of-sample validation discipline that gates which fixed-effects-estimated factor scores survive the K-fold cross-validation test
- [`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md) — the practitioner-quoted CB-arb cross-issuer relative-value screen whose factor-slope inference benefits from the issuer-fixed-effects within transformation when issuer-specific intercepts are correlated with the relative-value regressors

## Escalate to Raw When

Open Greene 8e or Wooldridge Intro 8e directly when any of the
criteria below applies. **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

- The panel inference must handle a dynamic-panel specification with
  a lagged dependent variable on the right-hand side — the
  Arellano-Bond / system-GMM machinery is out of scope per the v7+
  CB-arb extension boundary discipline (see frontmatter `Out of
  scope:` field for the chapter-level boundary specification).
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
- The panel response is binary, ordinal, or censored (logit / probit
  / Tobit panel) — the nonlinear panel-likelihood machinery is out
  of scope per the v7+ CB-arb extension boundary policy.
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
- The cross-sectional unit count is small (single-digit issuers)
  and asymptotic-cluster justification fails — bootstrap or
  small-cluster-corrected inference depth is out of scope.
  **Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.
- The deeper graduate-depth fixed/random-effects asymptotic theory
  is required (the would-be Wooldridge Cross/Panel 2e MIT Press 2010
  primary anchor) — the on-disk PDF is non-quotable per Critical
  Rule 4 (SCAN-quality OCR scan); the re-activation trigger is a
  clean publisher PDF acquisition. **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
