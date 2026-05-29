---
schema_version: "cacg.v0"
id: "qm-cb-arb-factor-construction"
title: "CB-Arb Factor Construction"
reading_id: "reading_01_qm"
summary: "framing how cross-sectional factor scoring is constructed for convertible-arbitrage idea generation — linear-model factor regressions, ridge / lasso shrinkage for high-dimensional feature spaces, and boosting (tree-ensemble) classifiers for cheapness / momentum / quality factors over a convertible universe"
tags: ["concept", "factor-models", "shrinkage"]
citations:
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p001:0000"
    chunk_hash: "58f15bacb14e7ff65c49da114dbd4c621e1386bbd7ed299ed0da99ce2e9469eb"
    page_range: [1, 2]
    quote: "This is page 43 Printer: Opaque this 3 Linear Methods for Regression 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p002:0001"
    chunk_hash: "796b19ebe3924c0027506f47bb6b94831de4104360e09d4732bcd687dbe28b15"
    page_range: [2, 3]
    quote: "For example, if G is a five-level factor input, we might create Xj , j = 1, . . . , 5, such that Xj = I(G = j)"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p003:0002"
    chunk_hash: "59d521a66cdb0d6bcfa9b77812af061a4989f57ef64c7a359b3ed88a4930d180"
    page_range: [3, 4]
    quote: "Linear least squares fitting with X ∈ IR2 . We seek the linear function of X that minimizes the sum of squared"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p004:0003"
    chunk_hash: "ecf1a3614e6bbd488ec4ce64c70c1e05b53af95a766aaaa5cb2a9a091fdf7bb1"
    page_range: [4, 5]
    quote: "The matrix H = X(XT X) −1XT appearing in equation (3.7) is sometimes called the “hat” matrix because it puts the hat on"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p005:0004"
    chunk_hash: "cdf7dfe17ee9d1af52e656d1591b338896955a84f25cb4830f4519937ce60b4f"
    page_range: [5, 5]
    quote: "Rank deficiencies can also occur in signal and image analysis, where the number of inputs p can exceed the number of"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p005:0005"
    chunk_hash: "2b9da506680cc20a638abd33539561d088fd8bed0fdc2976057cb3d8317a31a1"
    page_range: [5, 6]
    quote: "Also (N − p − 1)ˆσ 2 ∼ σ 2χ 2 N−p−1 , (3.11) a chi-squared distribution with N −p−1 degrees of freedom"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p006:0006"
    chunk_hash: "d53f65fb0a2f4b7e8feb72b3a3c8283f291d8fc02f80b8572c117ca826975f5c"
    page_range: [6, 7]
    quote: "For example, to test if a categorical variable with k levels can be excluded from a model, we need to test whether the"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p007:0007"
    chunk_hash: "279275d0b9db42eb931b1010c781d102577b16645f46857ce95ded06b0da52cd"
    page_range: [7, 8]
    quote: "Hence the standard practice of reporting βˆ ± 2 · se(βˆ) amounts to an"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p007:0008"
    chunk_hash: "976920bac42034b5f09d8967e75316663a3d0d9aeabbf8bba203f28740a54e7f"
    page_range: [7, 8]
    quote: "Figure 1.1 (page 3) of Chapter 1 is a scatterplot matrix showing every pairwise plot between the variables"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p008:0009"
    chunk_hash: "6547f57a8ac3de5ff03a0826c3f87623e4959c0e83540fca9088e67555998bb6"
    page_range: [8, 9]
    quote: "A Z-score greater than 2 in absolute value is approximately significant at the 5% level"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p009:0010"
    chunk_hash: "06b786f9d65d83c3ba24a899a327d94ec8febf8f6c657aa7a26a1eb9cfa7e62b"
    page_range: [9, 10]
    quote: "The least squares estimate of a T β is ˆθ = a T βˆ = a T (XT X) −1XT y. (3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p010:0011"
    chunk_hash: "6c0da5d6ef07f3afe3ff7cbfb726f6d1fe7116c05bd89a674f98d9cfd6afb283"
    page_range: [10, 11]
    quote: "From a more pragmatic point of view, most models are distortions of the truth, and hence are biased; picking the right"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p011:0012"
    chunk_hash: "5238c301c33470e126b09ddb6d03c7bb5acd9afcc032c9a1f8dfe235f8901cd9"
    page_range: [11, 11]
    quote: "Then we can write βˆ = hx, yi hx, xi , r = y − xβ. ˆ (3.26) As we will see, this simple univariate regression provides"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p011:0013"
    chunk_hash: "99cbe70491b0d7d576df9a8ab1ec1b0df2f16ddfc43daf27609bf4f9f1141b4e"
    page_range: [11, 12]
    quote: "Step 2 is just a simple univariate regression, using the orthogonal predictors 1 and z. Figure 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p012:0014"
    chunk_hash: "c4a2cec05883585e7be28d2c30d3950514cc73e2ae009972dd73aa40751658cc"
    page_range: [12, 13]
    quote: "Since zp alone involves xp (with coefficient 1), we see that the coefficient (3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p013:0015"
    chunk_hash: "0729715925e6a78767b40d4c51d7bde7925832722f28948fb7da9f54e17219b1"
    page_range: [13, 14]
    quote: "Algorithm 3.1 is known as the Gram–Schmidt procedure for multiple regression, and is also a useful numerical strategy"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p014:0016"
    chunk_hash: "9c05475ea8d5adb7ebf4a54d6bb264c974800555d850c834a4f492e2ea6fe3e6"
    page_range: [14, 15]
    quote: "A straightforward generalization of the univariate loss function (3.2) is RSS(B) = X K k=1 X N i=1 (yik − fk(xi))2 (3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p015:0017"
    chunk_hash: "7b6bb54bc75f941c64355ae9860ca0ffaef1fc21599a01355e6dd7664abf8782"
    page_range: [15, 15]
    quote: "tors, we often would like to determine a smaller subset that exhibit the strongest effects"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p015:0018"
    chunk_hash: "0572815471631d87ba87999bd936cf3d29704fe98789f9615f078add04ad8d73"
    page_range: [15, 16]
    quote: "The question of how to choose k involves the tradeoff between bias and variance, along with the more subjective desire"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p016:0019"
    chunk_hash: "198fa7ee134a52e2f6088e216fa9fdb2454e6b4103b072caed4c0e483696da94"
    page_range: [16, 17]
    quote: "With many candidate predictors, this might seem like a lot of computation; however, clever"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p017:0020"
    chunk_hash: "77423a6b01f25f0ba1b8a55887dbbeb2b1e2914d4d1913dbf15bd1b5fae4b4d8"
    page_range: [17, 18]
    quote: "Backward selection can only be used when N > p, while forward stepwise can always be used. Figure 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p018:0021"
    chunk_hash: "3e311fb3323eb96ab58314b5740a8c9768feb5372032a61e741e7ae92601b1e3"
    page_range: [18, 19]
    quote: "tercept equal to ¯y, and centered predictors with coefficients initially all 0"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p019:0022"
    chunk_hash: "d34e3a322b2ad9caaec08fcf97a6b03d7c4163c1850fd169ae33bb9d2e080625"
    page_range: [19, 19]
    quote: "The learning method is fit—for a range of values of the complexity parameter—to nine-tenths of the data, and the"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p019:0023"
    chunk_hash: "ff66f3da0fdf53cdaa1ff5fbefab29c8742528bd6f83807e1379f501414fb5ab"
    page_range: [19, 20]
    quote: "Shrinkage methods are more continuous, and don’t suffer as much from high variability. 3.4"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p020:0024"
    chunk_hash: "af59e96bd1a4c511a57eda34e736c4fd09042724d5d2d5f3e5b661cf01a23302"
    page_range: [20, 21]
    quote: "The estimates of prediction error and their standard errors were obtained by tenfold cross-validation; full details are"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p021:0025"
    chunk_hash: "cde2bf030b3a173aa2d0acc7af4ce94b08e3706577c03386f612faeb3e01d9a6"
    page_range: [21, 22]
    quote: "When there are many correlated variables in a linear regression model, their coefficients can become poorly determined"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p022:0026"
    chunk_hash: "5a448d447f040fdc3fb0d841511ebc2912630d338126eaa7f4c067ec14ffb355"
    page_range: [22, 22]
    quote: "This makes the problem nonsingular, even if XT X is not of full rank, and was the main motivation for ridge regression"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p022:0027"
    chunk_hash: "34e2d2cbd2b7ff9aaab92f4c981af58ca1a4f34192d6bc2fd9932066955c00e7"
    page_range: [22, 23]
    quote: "composition is extremely useful in the analysis of many statistical methods"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p023:0028"
    chunk_hash: "47778084c1f8a91cd15882ca833fc09f64586a7ec9edc7362e5cb9c35a9ee582"
    page_range: [23, 24]
    quote: "Profiles of ridge coefficients for the prostate cancer example, as the tuning parameter λ is varied"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p024:0029"
    chunk_hash: "edf5e47a6687b9154f09b074cb5740be3f89c6d9d07b2c420001afa395227732"
    page_range: [24, 24]
    quote: "What does a small value of d 2 j mean? The SVD of the centered matrix X is another way of expressing the principal"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p024:0030"
    chunk_hash: "044b6eda819f5821c5d539c27070ff0159f3c0e896cdbcbcf03928a5b4977e47"
    page_range: [24, 25]
    quote: "This sample variance is easily seen to be Var(z1) = Var(Xv1) = d 2 1 N , (3.49) and in fact z1 = Xv1 = u1d1"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p025:0031"
    chunk_hash: "c395d1edc5d2f18563971bf9167bcd53f942408a13252129f94e04fb2087c14e"
    page_range: [25, 26]
    quote: "Ridge regression projects y onto these components, and then shrinks the coefficients of the low– variance components"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p026:0032"
    chunk_hash: "f40946d46b447efca39169c170c6e598d58531ec2a678fe219fd832af52812b4"
    page_range: [26, 27]
    quote: "Note that df(λ) = p when λ = 0 (no regularization) and df(λ) → 0 as λ → ∞"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p027:0033"
    chunk_hash: "27e10b3050c08b5ae7493dcac626443376e1953cd9b292f32a1a501bc027de32"
    page_range: [27, 27]
    quote: "Because of the nature of the constraint, making t sufficiently small will cause some of the coefficients to be exactly"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p027:0034"
    chunk_hash: "cdd45217117c245dc06ef9ffa5f42a3f5ddbd5e897eb77f7a8141ee3ab507505"
    page_range: [27, 28]
    quote: "In the case of an orthonormal input matrix X the three procedures have explicit solutions"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p028:0035"
    chunk_hash: "982fbc591c1b741b7cbebf791c2bfa3a8d7f314498ab61425d998d77afc95657"
    page_range: [28, 29]
    quote: "A vertical line is drawn at s = 0.36, the value chosen by cross-validation. Compare Figure 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p029:0036"
    chunk_hash: "8ccf703ca4df11ca4ed2774ec9f2be5e9becb5386726289bee3ae0d1bc154f0d"
    page_range: [29, 30]
    quote: "Shown are contours of the error and constraint functions. The solid blue areas are the constraint regions |β1| + |β2| ≤"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p030:0037"
    chunk_hash: "f884882ed3c9edb8f4a711258f81fc93e3bf213af9909f672a74c9d54a0e2489"
    page_range: [30, 31]
    quote: "The case q = 1 (lasso) is the smallest q such that the constraint region is convex; non-convex constraint regions make"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p031:0038"
    chunk_hash: "37d7ff29e3871245e2f9271e6af1b1427d71baaac6a8ab2501231ad3a2eb58a5"
    page_range: [31, 32]
    quote: "Figure 3.13 compares the Lq penalty with q = 1.2 and the elastic-net penalty with α = 0"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p032:0039"
    chunk_hash: "3f5ac90f0f56368de87db4a12fadecbdd62dc0d829d2d168a543a0bbba5709f4"
    page_range: [32, 32]
    quote: "If p > N − 1, the LAR algorithm reaches a zero residual solution after N − 1 steps (the −1 is because we have centered"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p032:0040"
    chunk_hash: "e42812dc46bb103a4ac035cb05aa71a7426211512473ebff0655257b2a2a884a"
    page_range: [32, 33]
    quote: "The name “least angle” arises from a geometrical interpretation of this process; uk makes the smallest (and equal)"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p033:0041"
    chunk_hash: "785baa71044240615672a22e9526838f96f0250667e11354275494011e8452a4"
    page_range: [33, 34]
    quote: "Left panel shows the LAR coefficient profiles on the simulated data, as a function of the L1 arc length"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p034:0042"
    chunk_hash: "584760a594a9616f9515f563a3c3a98f7866a6a31a68d9be49f4cdf84a231689"
    page_range: [34, 35]
    quote: "We now give a heuristic argument for why these procedures are so similar"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p035:0043"
    chunk_hash: "17593aad3df02cc80f67fadd5d812a1482eafdd66308257f979093d03987e09f"
    page_range: [35, 35]
    quote: "Figure 3.16 compares LAR and lasso to forward stepwise and stagewise regression. The setup is the same as in Figure 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p035:0044"
    chunk_hash: "e92ee30f725ed9fefa9cb0c8bc564586a62466ed97940abce032a2b266b94f00"
    page_range: [35, 36]
    quote: "This makes intuitive sense: the harder that we fit to the data, the larger this covariance and hence df(yˆ)"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p036:0045"
    chunk_hash: "083d552a99eed8e8ccd743477ce7d34b350cbfbea2f777f420e6a91927b1b19b"
    page_range: [36, 37]
    quote: "These techniques are adaptive in a smoother way than best subset selection, and hence estimation of degrees of freedom"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p037:0046"
    chunk_hash: "8013f1d84a72bb06beeeed8c26267bcdb70b7e834ccec34ea7e1d4dbb4708a71"
    page_range: [37, 38]
    quote: "Note that if M = p, we would just get back the usual least squares estimates, since the columns of Z = UD span the"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p038:0047"
    chunk_hash: "fdded2e3a72b8674ba8d7a7808aa908bc92a54fdc15463fbddce0f8cf8c6ff8f"
    page_range: [38, 39]
    quote: "Hence in the construction of each zm, the inputs are weighted by the strength of their univariate effect on y 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p039:0048"
    chunk_hash: "d93b97605d7a3dc044f83e04538f6da6372840d30656be31bacafbcbe492ff99"
    page_range: [39, 40]
    quote: "cients can be recovered from the sequence of PLS transformations"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p040:0049"
    chunk_hash: "d435bfc59fb099891ab52e1141a66f8a664034de667ac523f5e926642819321c"
    page_range: [40, 40]
    quote: "ple with two correlated inputs X1 and X2, with correlation ρ"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p040:0050"
    chunk_hash: "f43db45fc33eaff3518986ec08454715d8102ae9245169cdfbd430477ea6b955"
    page_range: [40, 41]
    quote: "To summarize, PLS, PCR and ridge regression tend to behave similarly"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p041:0051"
    chunk_hash: "bb0421c3b7387c446dfc616e43d7fb25d3c821fff539fba1df07f92c7cb2949f"
    page_range: [41, 42]
    quote: "Coefficient profiles from different methods for a simple problem: two inputs with correlation ±0"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p042:0052"
    chunk_hash: "795beaafa25e3f648c08f3db2ff30baa0a8d314b20400e74e54ff9d37a1ae2c1"
    page_range: [42, 43]
    quote: "Note that at most M = min(K, p) directions can be found. The leading canonical response variates are those linear"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p043:0053"
    chunk_hash: "22218248c7f312e3b3465af4787560a274b752dcec5aa7a5cf470d6d21d2a76e"
    page_range: [43, 44]
    quote: "Although a better estimate of Σ would be (Y−XBˆ ) T (Y−XBˆ )/(N −pK), one can show that the solution remains the same"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p044:0054"
    chunk_hash: "74e63d653e04d6bfb0294c81d81c9ff5c592422b54ae39391ef935d4a497399b"
    page_range: [44, 45]
    quote: "In addition, L1 regularization has taken on a life of its own, leading to the development of the field compressed"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p045:0055"
    chunk_hash: "ba3fbb0176c0dc7ec22f6822ce6e5e230fe5a9e59b6162f15ad75a6409c4ae8f"
    page_range: [45, 46]
    quote: "Coefficient profiles for the prostate data. The left panel shows incremental forward stagewise regression with step"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p046:0056"
    chunk_hash: "2978995613130acd10d1a2e8e1331e1ec673c932ea0a61ac8aa6f92a23c549f9"
    page_range: [46, 46]
    quote: "88 3. Linear Methods for Regression Efron originally thought that the LAR Algorithm 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p046:0057"
    chunk_hash: "51c921671d18d3d694a2c22cafccf9e7f8891963068d099962ebc736e9b66d12"
    page_range: [46, 47]
    quote: "efficient path is discouraged from changing directions too often"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p047:0058"
    chunk_hash: "8068dea2762b2836f3cbfefca702668c58a6639833f2f6ca3629ce63ba6c9bd1"
    page_range: [47, 48]
    quote: "lently as minβ||XT (y − Xβ)||∞ subject to ||β||1 ≤ t. (3.79) Here || · ||∞ denotes the L∞ norm, the maximum absolute"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p048:0059"
    chunk_hash: "0673ad8a5289abc5853ef1a480739a5c7c099855011fb185d23a250fa6df7fa6"
    page_range: [48, 49]
    quote: "Hence it can achieve a smaller maximum than the lasso, but in the process a curious phenomenon can occur"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p048:0060"
    chunk_hash: "0c5f372fc09472407e4ba0742bab99f5c6f9d273c5d165b9fe73f590c8506def"
    page_range: [48, 49]
    quote: "That is, for some values of λ, an entire group of predictors may drop out of the model. This procedure 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p049:0061"
    chunk_hash: "f2a778bb123247d48400a42decc431f6e59c9050ce3cdf5b06f1a0f79ee501cb"
    page_range: [49, 50]
    quote: "ficients for the columns of XSc on XS are not too large, that is, the “good” variables S are not too highly correlated"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p050:0062"
    chunk_hash: "5c400707cedf4df1ab2d4e301565653b3f0aad59ccd4475620e0749ee856d2b4"
    page_range: [50, 50]
    quote: "The second term in square-braces reduces the amount of shrinkage in the lasso for larger values of β, with ultimately"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p050:0063"
    chunk_hash: "d424987f6c105b77ed6feaedf9b1479bebc7bd562cd4a6f3d7649c4ee446681b"
    page_range: [50, 51]
    quote: "Suppose the predictors are all standardized to have mean zero and unit norm"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p051:0064"
    chunk_hash: "485256c61b9c490984212744d07307fe412641fa0ab70c6c0ca8013fec885c49"
    page_range: [51, 52]
    quote: "The same kind of algorithm can be applied to the elastic net, the grouped lasso and many other models in which the"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p052:0065"
    chunk_hash: "397dcb263883a6b24afe95f90ae75dd73155e96126c8ffe28453431979f41b40"
    page_range: [52, 53]
    quote: "Partial least squares was introduced by Wold (1975). Comparisons of shrinkage methods may be found in Copas (1983) and"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p053:0066"
    chunk_hash: "efae3b3abe8f4ad559ae5a4fe31de621743975a8798b8481d499978542bfbd78"
    page_range: [53, 53]
    quote: "acterize the solution to this modified criterion. Show that a similar result holds for the lasso. Ex. 3"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p053:0067"
    chunk_hash: "9cb7aaf1f9975180538fd5f9435d84638b8372eb1a64ced62e0f9c3ccbc863a2"
    page_range: [53, 54]
    quote: "Describe an efficient procedure for doing this. Ex. 3.10 Backward stepwise regression"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p054:0068"
    chunk_hash: "2ba4d7b8bec4309bc9828ed36d79f2e1e0a39e09efd3d97ef4c95b3d7752982b"
    page_range: [54, 55]
    quote: "Ex. 3.19 Show that kβˆridgek increases as its tuning parameter λ → 0"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p055:0069"
    chunk_hash: "0555d06e30e9cb41c694ef466e6766deea1e89d7374dc893f4e3c0584f6a2ec5"
    page_range: [55, 56]
    quote: "Let βˆ be the least-squares coefficient of y on X, and let u(α) = αXβˆ for α ∈ [0, 1] be the vector that moves a"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p056:0070"
    chunk_hash: "cbb9beb745991d0680094d7db4fc08db34e3adf22138335ef24b78f8a5ab25e4"
    page_range: [56, 56]
    quote: "98 3. Linear Methods for Regression variable, linearly adjusted for all the variables currently in the model"
    edge_type: "supports"
  - source_id: "qm_eslii_ch3_trim"
    chunk_id: "qm_eslii_ch3_trim:p056:0071"
    chunk_hash: "3afbf55a8b460f9e15ac83e332e9c557783c5ce8d686c7cde37dc864398824f1"
    page_range: [56, 57]
    quote: "Assuming the predictors are standardized, relate λ to the correlation between the jth predictor and the current"
    edge_type: "supports"
card_hash: "da94f494848eae7aaade6131c35e8b1a8cdbff7cdf4bf6b37fe5b9da8ba707de"
---
framing how cross-sectional factor scoring is constructed for convertible-arbitrage idea generation — linear-model factor regressions, ridge / lasso shrinkage for high-dimensional feature spaces, and boosting (tree-ensemble) classifiers for cheapness / momentum / quality factors over a convertible universe

## Original Card (preserved verbatim)

## Intuition

The practitioner-quoted CB-arb relative-value screens described in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
(adjusted conversion premium, credit/equity ratio comparison,
implied-vs-realized vol gap, parity-spread to bond-floor) consume a
cross-sectional ranker that scores each convertible on a universe
by its deviation from an expected fair-value benchmark. The
statistical-learning toolkit provides two layers of machinery for the
ranker-construction step: (i) the linear regression of issuer-specific
features on a target return / mispricing label, and (ii) shrinkage of
the coefficient vector via ridge or lasso penalties when the feature
dimension is high relative to the issuer count. The ESL framing
decomposes each step into a symbolic estimator and a
regularization-versus-bias trade-off without prescribing any
particular weighting scheme, which is what makes the toolkit reusable
across cross-sections (see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the CB-specific feature set). **Source:**
01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

Boosting (ESL Ch.10) provides a third layer when the linear-loading
specification is too restrictive: the algorithm composes a sequence
of weak classifiers and combines them into a strong cross-sectional
ranker. ESL Ch.10 develops boosting as a general cross-sectional
classification toolkit anchored on synthetic data and standard
machine-learning benchmark sets, not on a CB-arb-specific example.
The CB-arb adaptation (treating each convertible as an issuer-level
observation, stacking cheapness / momentum / quality features as the
predictor matrix) is documented in the CB vertical's practitioner
literature; the cross-sectional ranker output feeds the screens in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

```
<!-- primitive: regression-scatter-and-fit source: _diagram_primitives.md -->
   y
    ^                                       .
    |                                   .  ŷ = b̂_0 + b̂_1·x
    |                              .   /
    |                         .       /  .
    |                    .           / .
    |               .       .       /
    |          .                .  /
    |     .            .          /  .
    |          .          .      /
    |  .             .          /     .
    |       .                  /  .
    | b̂_0  ___________________/
    |                        /
    +-----------------------+----------------------> x
```

## Definition

A cross-sectional factor model is the linear specification
`Y_i = b_0 + b_1 · x_{1i} + b_2 · x_{2i} + ... + b_k · x_{ki} + ε_i`
where `Y_i` is the realized return or mispricing label on observation
`i`, `x_{ji}` is the value of the `j`-th factor on observation `i`,
`(b_1, ..., b_k)` are the loading coefficients to be estimated, and
`ε_i` is the residual. The OLS estimator `b̂ = (XᵀX)⁻¹ XᵀY` estimates
the loading coefficients under the classical Gauss-Markov assumptions
(see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the convertible-issuer cross-section instance: `Y_i` is a
parity-spread or implied-vs-realized vol gap label on convertible
`i`, `x_{ji}` is one of the four practitioner-quoted CB-arb factor
families on convertible `i`). **Source:**
01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.

When the feature count `k` is large relative to the observation count
`n`, OLS becomes ill-conditioned and the coefficient estimates
inflate; the ridge estimator augments the objective with an L2
penalty `min_b Σ_i (Y_i − x_iᵀ b)² + λ · Σ_j b_j²` and the lasso
estimator augments with an L1 penalty `min_b Σ_i (Y_i − x_iᵀ b)² +
λ · Σ_j |b_j|`. The L1 penalty has the additional effect of forcing
some coefficients exactly to zero, performing simultaneous
regularization and feature selection. The hyperparameter `λ` is held
abstract here; the sibling signal-validation card will cross-reference
the K-fold cross-validation discipline that selects it
out-of-sample. The CB-arb application of high-dimensional feature
spaces (issuer-level cheapness scores aggregated across multiple
look-back windows) is documented at the practitioner cross-link
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

The boosting estimator generalises beyond linear loadings by composing
a sequence of weak classifiers `(h_1, h_2, ..., h_M)` and combining
them into a strong cross-sectional ranker
`F_M(x) = Σ_{m=1}^{M} α_m · h_m(x)` where each round `m` re-weights
the observations that the prior rounds misclassified. The CB-arb
construction step uses boosting as a classifier for the cheapness /
momentum / quality factor families when the linear-loading
specification is too restrictive; the CB-specific feature engineering
and screen-output interpretation live in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

## Mathematical Reasoning

The OLS estimator (source ASSERTS) is the closed-form minimiser of
the sum of squared residuals `Σ_i (Y_i − x_iᵀ b)²` over the
parameter vector `b ∈ ℝ^{k+1}`. The first-order conditions yield the
normal equations `XᵀX b̂ = XᵀY`, whose solution is `b̂ = (XᵀX)⁻¹ XᵀY`
when `(XᵀX)` is invertible. The Gauss-Markov assumptions guarantee
unbiasedness `E[b̂] = b` and minimum variance `Var(b̂) = σ² (XᵀX)⁻¹`
among linear unbiased estimators; the classical Wald inference toolkit
extends to the cross-sectional factor setting unchanged. **Source:**
01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.

The ridge estimator (source ASSERTS) modifies the OLS first-order
conditions to `(XᵀX + λ · I) b̂_{ridge} = XᵀY`, whose closed-form
solution is `b̂_{ridge} = (XᵀX + λ · I)⁻¹ XᵀY`. The `λ · I` term
inflates the diagonal of the cross-product matrix and is what
guarantees invertibility even when `k > n` or when the predictor
columns are nearly collinear. The lasso estimator has no closed-form
solution because the L1 penalty is non-differentiable at zero;
coordinate-descent and least-angle-regression algorithms compute the
solution path over `λ`. The shrinkage-and-selection effect is the
key property that makes lasso useful for high-dimensional CB-arb
feature spaces. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

The boosting estimator (source DECOMPOSES) builds the strong ranker
`F_M(x)` additively: at round `m`, the weak learner `h_m` is fitted
to the residuals of the prior ensemble `F_{m-1}(x)`, then combined
with a step-size coefficient `α_m` whose value depends on the chosen
loss function. AdaBoost uses an exponential loss `L(y, F) = exp(−y F)`
that yields `α_m = (1/2) · ln((1 − err_m) / err_m)` where `err_m`
is the weighted error of `h_m`; gradient boosting generalises this
to any differentiable loss by fitting `h_m` to the negative gradient
of the loss at `F_{m-1}`. The CB-arb adaptation (whether to set
classification targets — binary buy/sell signal — or regression
targets — continuous expected-return score — for the cheapness /
momentum / quality labels) is a practitioner-engineering choice
documented at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

The Wooldridge Intro treatment (source ASSERTS) of the cross-section
regression frames the same OLS machinery at undergraduate depth with
explicit dummy-variable encoding for categorical features (industry,
country, credit-rating bucket). The dummy-variable trap warns against
perfect multicollinearity when the dummy set spans the categorical
attribute completely; the standard remedy is to omit one category as
the baseline. CB-arb factor models that include issuer fixed effects
via dummies inherit this trap structure unchanged (see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the CB-specific issuer / industry / credit-rating bucket
typology). **Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.176-235.

## See Also

- [`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md) — the four practitioner-quoted CB-arb relative-value screens (adjusted conversion premium, credit/equity ratio comparison, implied-vs-realized vol gap, parity-spread to bond-floor metric) that consume the factor-construction step's cross-sectional ranker output as input
- [`pm-multifactor-asset-pricing-intuition`](../09_portfolio_management_and_asset_pricing/pm-multifactor-asset-pricing-intuition.md) — the multifactor asset-pricing framing for portfolio-level factor exposure interpretation; the CB-arb factor construction here is an asset-class-specific application of the same conceptual surface

## Escalate to Raw When

Open ESL 2e directly, Greene 8e directly, or the more rigorous
econometric / ML references when any of the criteria below applies.
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

- The factor feature space includes non-numeric kernel similarity
  measures (text-derived sentiment vectors, graph-based issuer
  similarity scores) — kernel methods are out of scope per the v7+
  CB-arb extension boundary discipline (see `Out of scope:`
  frontmatter for the chapter-level boundary specification).
  **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
- The factor model needs deep neural-network architecture or random
  forests beyond boosting — both are out of scope under the v7+
  CB-arb extension policy (see `Out of scope:` frontmatter for the
  chapter-level boundary specification); consult the relevant raw
  references only if the practitioner deliberately steps outside the
  CB-arb pilot scope. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.
- The factor specification requires unsupervised clustering or
  self-organising maps to discover latent issuer groups — clustering
  is out of scope (see `Out of scope:` frontmatter); route to the
  appropriate unsupervised-learning reference if the use case truly
  requires it. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
- The factor model needs panel-data fixed/random-effects estimation
  to control for issuer-level unobserved heterogeneity — that
  machinery is in the sibling `qm-panel-cb-factor-inference.md` card
  (see `Out of scope:` frontmatter for the panel-data deferral).
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.
- The hyperparameter `λ` for shrinkage / boosting needs
  out-of-sample validation — that discipline is in the sibling
  `qm-signal-validation-oos-discipline.md` card (see `Out of scope:`
  frontmatter for the signal-validation deferral).
  **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
