---
schema_version: "cacg.v0"
id: "qm-volatility-model-garch-multivariate"
title: "Volatility-Model Estimation (ARCH / GARCH + Multivariate)"
reading_id: "reading_01_qm"
summary: "framing how conditional-variance models (ARCH and GARCH univariate; multivariate-vol covariance estimators) supply the time-varying volatility input that a CB-arb practitioner feeds into vega-sensitivity computation on the convertible's underlying-equity vol surface"
tags: ["concept", "garch", "multivariate"]
citations:
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p001:0000"
    chunk_hash: "620fd5334a40ae4fa9687eac778e84691a3939228d2cfaf8510458445d192ec5"
    page_range: [1, 1]
    quote: "CHAPTER 3 Conditional Heteroscedastic Models The objective of this chapter is to study some statistical methods and"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p001:0001"
    chunk_hash: "7f901481b98295ae4a7b022b595d4ef19fcdaa520d4d95c8715d180e1935ac80"
    page_range: [1, 2]
    quote: "Volatility has many other financial applications. As discussed in Chapter 7, volatility modeling provides a simple"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p002:0002"
    chunk_hash: "db04781e1997a6d1a54347040e05628806f578d032c02e6a459f0dc67b79b372"
    page_range: [2, 3]
    quote: "If intraday data of the stock, such as 10-minute returns, are available, then one can estimate the daily volatility"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p003:0003"
    chunk_hash: "c02388ce00ed7ab5000d683863e87fdb149954c062babb502d8943c266f4b9ff"
    page_range: [3, 4]
    quote: "Second, volatility evolves over time in a continuous manner—that is, volatility jumps are rare"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p004:0004"
    chunk_hash: "257b745bfefeae3444d45efbceab8dbac160057c7df1ee0d6a66502a18ee1fdd"
    page_range: [4, 4]
    quote: "Figure 3.1a shows the sample ACF of the return, which suggests no significant serial correlations except for a minor"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p004:0005"
    chunk_hash: "7b6d63ccb6ea284fa51ee09356d58a53ebff0abe7495dc29326702a8f9f5e593"
    page_range: [4, 5]
    quote: "The explanatory variables xt in Eq. (3.3) are flexible. For example, a dummy variable can be used for the Mondays to"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p005:0006"
    chunk_hash: "e7d01385baa1320df050d0b50044c1242fc7be418f677c8e39ce956cd65fa938"
    page_range: [5, 5]
    quote: "Use the residuals of the mean equation to test for ARCH effects. 3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p005:0007"
    chunk_hash: "f6320b646a0907291c21064eba3ca6e8d6444986abbfeb9fff458bce3455afc9"
    page_range: [5, 6]
    quote: "Specifically, the null hypothesis is Ho : α1 =···= αm = 0. Let SSR0 ="
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p006:0008"
    chunk_hash: "c556d8f1008554f144ca3120f849fac9747f25aa6a0ac0ba29a4a4e96c198879"
    page_range: [6, 7]
    quote: "The basic idea of ARCH models is that (a) the shock at of an asset return is serially uncorrelated, but dependent, and"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p008:0009"
    chunk_hash: "d008dea32b614bcc8a7491785e397745c3f301cdf48d43f7c7508e3cc22e04f4"
    page_range: [8, 9]
    quote: "104 CONDITIONAL HETEROSCEDASTIC MODELS ACF −0.10 0.0 0.05 0.10 (a) Sample ACF Lag PACF −0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p009:0010"
    chunk_hash: "a1b4ae9966d1c9e9e09ceacad641231f495764afee098f746407b82d183cc916"
    page_range: [9, 10]
    quote: "Third, in some applications, we need higher order moments of at to exist and, hence, α1 must also satisfy some"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p009:0011"
    chunk_hash: "f04552ac464d395e0e2050ea357d34470c40fefbc6d81a02d82e276bcf25f704"
    page_range: [9, 10]
    quote: "The condition αi ≥ 0 in Eq. (3.5) can be relaxed. It is a condition to ensure that the conditional variance σ 2 t 106"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p010:0012"
    chunk_hash: "9d290db8fe042818c46bcae9bf2002b840e37d75c50ac01fe8533fbeda89342d"
    page_range: [10, 11]
    quote: "It gives no indication about what causes such behavior to occur. 4"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p011:0013"
    chunk_hash: "cc6425cd7daf16027e57847410e518588d00242b768dea175aaf934b2da22dd6"
    page_range: [11, 12]
    quote: "Because {ηt} are not identically distributed, the least squares estimates of the prior model are consistent, but not"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p012:0014"
    chunk_hash: "c8a82a7b4d00c9837047f8821073d50c8547d128095d4d0153bcc56ae43a9252"
    page_range: [12, 12]
    quote: "t distribution with v degrees of freedom. Then Var(xv) = v/(v − 2) for v> 2, and we use t = xv/ √v/(v − 2)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p012:0015"
    chunk_hash: "422b30375d43fb80b7391f5f4807f5f44241aa32ccc5f6248164906c2f5f123d"
    page_range: [12, 13]
    quote: "This distribution reduces to a Gaussian distribution if v = 2 and it has heavy tails when v< 2"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p013:0016"
    chunk_hash: "f03aebf3276de668cd1d07832df216fdd760a1dd136590e42dcc37a90fbbe097"
    page_range: [13, 14]
    quote: "This is confirmed by the ARCH effect test shown in Section 3.3.1, and we proceed to identify the order of an ARCH model"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p015:0017"
    chunk_hash: "6b62bb5a6dc369e500e6e44abeb177a65dc8934b30d5ffc1d2737122bdce36c9"
    page_range: [15, 15]
    quote: "THE ARCH MODEL 111 Null Hypothesis: no autocorrelation Test Statistics: Test Stat 13.7820 p.value 0.1832 Dist"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p015:0018"
    chunk_hash: "bffcda30483de1dd9c960b9ca4d1b112b15447a95615b6d24ec4530fc6792f3a"
    page_range: [15, 16]
    quote: "The estimated degrees of freedom is 6.16 with standard error 1.65"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p016:0019"
    chunk_hash: "6727bfbe73614d72995148abf1a376e3b40901c48522b99bc3114f3d6af4060f"
    page_range: [16, 17]
    quote: "Finally, a more appropriate conditional heteroscedastic model for this data set is a GARCH(1,1) model, which is"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p017:0020"
    chunk_hash: "e47c55e1985385edc124211fc3edbe4e616016a266859f9b7597a3d8c54ea3bb"
    page_range: [17, 18]
    quote: "Model checking, using the standardized residual a˜t , indicates that the model is adequate. 3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p018:0021"
    chunk_hash: "1f1a5f3b9ae0b0a523b49b3c97d6378694665a6b5c6147a348c6c2ac585bada7"
    page_range: [18, 19]
    quote: "The αi and βj are referred to as ARCH and GARCH parameters, respectively"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p019:0022"
    chunk_hash: "2abc0b75cbecced388922272f6fa5191f8a5c86914651aab58743776f8873177"
    page_range: [19, 19]
    quote: "THE GARCH MODEL 115 Consequently, similar to ARCH models, the tail distribution of a GARCH(1,1) process is heavier than"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p019:0023"
    chunk_hash: "882a5bebe1c1f0e6e21bec366da5b78caeb11bd873e0eacbc8b2cca347e5bfd8"
    page_range: [19, 20]
    quote: "→∞ provided that α1 + β1 < 1. Consequently, the multistep ahead volatility forecasts of a GARCH(1,1) model converge to"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p020:0024"
    chunk_hash: "940b771da08acd20900110945e8e21f45acef2fb5d02bacae7e48946935fa346"
    page_range: [20, 20]
    quote: "Figure 3.6 shows the sample ACF of rt and the sample PACF of r2 t"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p020:0025"
    chunk_hash: "48e9fb4ef51e789944730fff052bf99fd977ec5f038a2eef196bfffc6ae2075f"
    page_range: [20, 21]
    quote: "A joint estimation of the AR(3)–GARCH(1,1) model gives rt = 0.0078 + 0.032rt−1 − 0.029rt−2 − 0.008rt−3 + at, σ 2 t = 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p021:0026"
    chunk_hash: "2fe8e50a72adf8f937021e6a4a32f6bd373b48b700ecc9d6fee5e9b431b4709c"
    page_range: [21, 22]
    quote: "Time series plot of the monthly excess returns of the S&P 500 index. ACF −0.10 0.0 0.05 (a) Lag Partial ACF −0.1 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p022:0027"
    chunk_hash: "b2df511fa9562dce63bd6caeb736a77cb2489aa79709b1b371d76088ab3a06b5"
    page_range: [22, 23]
    quote: "Both plots are based on the GARCH(1,1) model in Eq. (3.18). THE GARCH MODEL 119 ACF −0.2 −0.1 0.0 0.1 0.2 (a) ACF −0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p023:0028"
    chunk_hash: "28e8afbeff4ef773985f7f0e8d45d5bddc1063ee3abfb9a0f30d06d867dec322"
    page_range: [23, 24]
    quote: "The starting value σ 2 0 is fixed at either zero or the unconditional variance of at"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p024:0029"
    chunk_hash: "d43a39e5288e93185707b77a16b4aa4a3880b1a8d969f02522d045891e5e8461"
    page_range: [24, 25]
    quote: "Thus, the fitted GARCH(1,1) model with Student-t distribution is adequate"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p025:0030"
    chunk_hash: "f8cf151a0cc4b22e991ff8447215764504816b5be87d7fcef5858ef3c56109c5"
    page_range: [25, 26]
    quote: "But it is not an accurate estimate of σ 2 h+1 because a single observation of a random variable with a known mean value"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p026:0031"
    chunk_hash: "c17922f1855436128e8be055ddf4662f02eecd7685f4ecdd3cd08e6f5c5f927d"
    page_range: [26, 26]
    quote: "Furthermore, the fitted volatility series of the two-pass method is very close to that of Figure 3.7a. 3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p026:0032"
    chunk_hash: "0f0bf6a14ea90537715a2526467b74a549c5992e5859f3b8331307ef995c26bf"
    page_range: [26, 27]
    quote: "Nelson (1990) studies some probability properties of the volatility process σ 2 t under an IGARCH model"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p027:0033"
    chunk_hash: "95a1ea91e800017ea89561ed0d085b6f8fab8a5ea1c2b9ee0fc79cbe37ae8dd1"
    page_range: [27, 28]
    quote: "The parameter c is called the risk premium parameter. A positive c indicates that the return is positively related to"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p028:0034"
    chunk_hash: "bf0c20e6a8aba55ecdea0edcd05d091067eff21f0de3c21f9fa8fd74d5750414"
    page_range: [28, 29]
    quote: "In particular, to allow for asymmetric effects between positive and negative asset returns, he considers the weighted"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p029:0035"
    chunk_hash: "c4d43cd889f687c7e96616924493027d839d18b0b647460fe0fd1811dac1efb0"
    page_range: [29, 29]
    quote: "First, it uses logged conditional variance to relax the positiveness constraint of model coefficients"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p029:0036"
    chunk_hash: "935f4988c20a127dd5d5dacc3c47c5b0ac5012beeeaeaaff7d2a8b753b1e138d"
    page_range: [29, 30]
    quote: "The model is, therefore, nonlinear if θ = 0. Since negative shocks tend to have larger impacts, we expect θ to be"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p030:0037"
    chunk_hash: "b18d7c8ac261b6017ed027971172dc61f727b286a725ee9c26ead2c4e3f79aa0"
    page_range: [30, 31]
    quote: "Similar to a GARCH-M model, the parameter c in Eq. (3.28) is the risk premium parameter. Table 3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p031:0038"
    chunk_hash: "c5b7daff4ca30a3b92be5c0a0aa1ff1187bd32998d2b0f4cbd8434438c9b6595"
    page_range: [31, 31]
    quote: "The prior AR(1)–EGARCH(1,1) model is adequate. From the estimated volatility equation in (3.30) and using √2/π ≈ 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p031:0039"
    chunk_hash: "4aad0280fd72a2ceaa6fdbed4508e92b385c77948d5bfc2e77c18cf25fd981e3"
    page_range: [31, 32]
    quote: "The results are given below. S-Plus Demonstration Output edited. > ibm"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p032:0040"
    chunk_hash: "e84297c1eded1af64cb8a95ba0c028e58eacebf3f8508e1790c9964289d98f4f"
    page_range: [32, 33]
    quote: "Taking exponentials, the model becomes σ 2 t = σ 2α1 t−1 exp[(1 − α1)α0] exp[g( t−1)], g( t−1) = θ t−1 + γ(| t−1|−"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p033:0041"
    chunk_hash: "387d66d112a8a151fb6058e81acc67f07bc0704f49b32bf96ba07fa7a93c8c84"
    page_range: [33, 34]
    quote: "(θ − γ) can be obtained from most statistical packages. Alternatively, accurate approximations to these values can be"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p034:0042"
    chunk_hash: "abc9bb6cc73e74c9cdcfd1de3ecbfe60a89d1bab67161fcf6b5572c0174c91cc"
    page_range: [34, 35]
    quote: "Other threshold values can also be used; see Chapter 4 for the general concept of threshold models. Model (3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p035:0043"
    chunk_hash: "42fa6608e1593849a4b72bd72fff26da70460a9c0e7af5ef91dc99b0ad74babc"
    page_range: [35, 36]
    quote: "We mention the conditional heteroscedastic ARMA (CHARMA) model that uses random coefficients to produce conditional"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p036:0044"
    chunk_hash: "fa09f005c34bcbe28b653523799a465afb652c126eeaf9a78fea771f8bee320a"
    page_range: [36, 36]
    quote: "is a covariance matrix, which is non-negative definite, and σ 2 η is a variance, which is positive, we have σ 2 t ≥ σ"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p037:0045"
    chunk_hash: "9cf4aaf3deb675e41415e8606b663d725bad09ee62c43ae6b4c5fa03e6f18429"
    page_range: [37, 37]
    quote: "RANDOM COEFFICIENT AUTOREGRESSIVE MODELS 133 All of the estimates are now statistically significant at the 5% level"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p037:0046"
    chunk_hash: "57ae7d00faf6020ae7e2cf557281966dc95573c01afae1a0e8ea930cd0eaa794"
    page_range: [37, 38]
    quote: "The conditional mean and variance of the RCA model in Eq. (3"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p038:0047"
    chunk_hash: "0912bf537336d69ab368360d0fe4265729242667fde6ed3fd0e7a09abfaf0d74"
    page_range: [38, 39]
    quote: "The difficulty in estimating a SV model is understandable because for each shock at the model uses two innovations t"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p039:0048"
    chunk_hash: "69abd02882ce605afdcb10c4652889ddf340e1bc7b17a4f60d9b61b29abb9ebc"
    page_range: [39, 40]
    quote: "The extension to long-memory models in volatility study is motivated by the fact that the autocorrelation function of"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p040:0049"
    chunk_hash: "2720a5940ef0d19fbf5443db7b8c7035d49ae4938ab5b34b5b3a66ef274641a6"
    page_range: [40, 40]
    quote: "For applications, Ray and Tsay (2000) studied common long-memory components in daily stock volatilities of groups of"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p040:0050"
    chunk_hash: "496fe157becd7a91225badf8af7d6583d6b3cae7797b810d67b4da5fa4c0301a"
    page_range: [40, 41]
    quote: "The Ljung–Box statistics of the a˜ 2 t series show Q(10) = 2.89(0.98) and Q(20) = 7.26(0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p041:0051"
    chunk_hash: "ffd7c478bb3152f2c67b1ab773895a3fca7f0ab04102c6c23ebad2a2d6511385"
    page_range: [41, 42]
    quote: "uals a˜t = at /σt show Q(10) = 7.66(0.569) and Q(20) = 21.64(0.302)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p042:0052"
    chunk_hash: "1caec78eebd07ec06abd997a6af1fbc1a9fdb83e6c3d64954d65e480c14ae96a"
    page_range: [42, 43]
    quote: "The Ljung–Box statistics of the standardized residuals show Q(10) = 7.68 and Q(20) = 21"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p043:0053"
    chunk_hash: "cb5152972b910aa8e0aa4fa92a66d91bc5c506272c0057e6d2563896618402fa"
    page_range: [43, 43]
    quote: "Based on the standardized residuals a˜t = at /σt , we have Q(10) = 11.51(0.32) and Q(20) = 23.71(0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p043:0054"
    chunk_hash: "960b4f1a5062cf8f51a88213041e522b8b8e68a1b2250ecf04b33b1e59daaaad"
    page_range: [43, 44]
    quote: "The negative sign is understandable because it implies that using the lag-1 past return of IBM stock reduces the"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p044:0055"
    chunk_hash: "149fdce36e2ace0efeeab85c36a6b07f7dd693543119627adbb5e844ffa08a0c"
    page_range: [44, 45]
    quote: "Assuming that the conditional variance and covariance exist, we have Var(rm t |Ft−1) ="
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p045:0056"
    chunk_hash: "0c013e266b0df1fab479de43c86a9311cf0f1be56416101ed9e93fbf333765fa"
    page_range: [45, 45]
    quote: "Further research is needed to make this approach valuable. Example 3.6"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p045:0057"
    chunk_hash: "95e49af247d1043e5f87d473ec325ea33d23cc0c2efdd6f56c4d4cf9ba06507d"
    page_range: [45, 45]
    quote: "n i=1 r2 t,i, is called the realized volatility of rt ; see Andersen et al. (2001a, b)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p045:0058"
    chunk_hash: "0220475420d74948ca458a4ace2ce0aab50ba42c7f077211513da51e2bee8e68"
    page_range: [45, 46]
    quote: "Mathematically, realized volatility is a quadratic variation of rt and it assumes that {rt,i} n i=1 forms an iid"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p046:0059"
    chunk_hash: "5688a43ba308e0b6ea57abba8d6e5495ae6803a417df1a0cd858893c41f62cd9"
    page_range: [46, 47]
    quote: "Time plots of estimated monthly volatility for the log returns of the S&P 500 index from January 1980 to December 1999:"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p047:0060"
    chunk_hash: "ac2ddd8306f5c1f496d4c0ba460f86be804d7d23a1f6ef4efbca9bc47aa5572b"
    page_range: [47, 48]
    quote: "son (1980), Garman and Klass (1980), Rogers and Satchell (1991), and Yang and Zhang (2000) showed that one can use such"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p048:0061"
    chunk_hash: "11484fd7241d292eaac0a306d90e7299ddbc22291ab787f82c145b4cf9b2b883"
    page_range: [48, 49]
    quote: "A more precise, but complicated, estimator σˆ 2 4,t was also considered. However, it is close to σˆ 2 5,t"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p049:0062"
    chunk_hash: "20056b9b687b753e93fc855e62876cd15602a917b471c72226e6e86bddc5867a"
    page_range: [49, 49]
    quote: "The quantity Ht − Lt is called the range of the price in the tth day"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p049:0063"
    chunk_hash: "166050880cf53f63ba24003eb2620dca984d7cb1132a805d7a8534ac608a2087"
    page_range: [49, 50]
    quote: "Taking the square of the volatility model, we have σ 4 t = α2 0 + α2 1a4 t−1 + β2 1σ 4 t−1 + 2α0α1a2 t−1 + 2α0β1σ 2 t−1"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p050:0064"
    chunk_hash: "e890db10e873c5463b771257e5e6a08a9cfc0dae8492783f59d18487a630b77c"
    page_range: [50, 51]
    quote: "This result was obtained originally by George C. Tiao; see Bai, Russell, and Tiao (2003)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p051:0065"
    chunk_hash: "83cd0fb690335ed301867ca1b1cf8ebdae89fd34e5b3e0c55a5327162c1bc735"
    page_range: [51, 51]
    quote: "The excess kurtosis of at becomes Ka = [6 + (v + 1)K(g) a ]/[v − 4 − K(g) a ] provided that 1 − 2α2 1(v − 1)/(v − 4) −"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p052:0066"
    chunk_hash: "482f9444309f31e15a7b0c908a1a5f4fe21df543a6874b8967aa6c31ea04dc8f"
    page_range: [52, 53]
    quote: "148 CONDITIONAL HETEROSCEDASTIC MODELS nonlin mu a0 a1 b1 v frml at = rt(t)-mu frml gvar = a0+a1*at(t-1)**2+b1*h(t-1)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p053:0067"
    chunk_hash: "7622b9b22f58d497d557f6881cc0f0cd0985bcff091cc86db94294fcd0fcfc69"
    page_range: [53, 53]
    quote: "Build a GARCH model for the transformed series and compute 1-step to 5-step ahead volatility forecasts at the forecast"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p053:0068"
    chunk_hash: "f4ef5ed9c7252c16a90d9cca00483d12cef588cc80ea18e38e9413fe2c7fe914"
    page_range: [53, 54]
    quote: "Use the fitted model to compute 1-step to 5-step ahead volatility forecasts at the forecast origin h = 690. 3.8"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p054:0069"
    chunk_hash: "679e03976cf914036bc827b0b0b0a9d908e6ddae4d2941cf649bbaa5f75f9fbd"
    page_range: [54, 55]
    quote: "The file d-gmsp9303.txt contains the daily simple returns of GM stock and the S&P composite index from 1993 to 2003"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p055:0070"
    chunk_hash: "6a3991c71f8ef0c5ad2f2467b5d4dd18ff2f9adec6b298f5bd966cddbca24932"
    page_range: [55, 56]
    quote: "The AR(5) contains only lags 3 and 5. Denote the fitted volatility series by “spvol"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p056:0071"
    chunk_hash: "00071c57086262e887f4dc98a5e59b541410d088fad4a5224de98e0d15c24f24"
    page_range: [56, 56]
    quote: "Journal of Econometrics 114: 349–360. Bai, X., Russell, J. R., and Tiao, G. C. (2004)"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p056:0072"
    chunk_hash: "96f1b696c7c74f3e625861f0326ecfbf8b755a8c9ef4b22950df4d9653638622"
    page_range: [56, 57]
    quote: "J. (1980). On the estimation of security price volatilities from historical data. Journal of Business 53: 67–78"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p057:0073"
    chunk_hash: "ba6490e5772495851c6ba34ad89d1d13aec2a68fbb8012a3a5ea1f286f9b5552"
    page_range: [57, 58]
    quote: "Oxford University Press, Oxford, UK. Tsay, R. S. (1987). Conditional heteroscedastic time series models"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p058:0074"
    chunk_hash: "45a8e8042e9649386b9c16e847004b29e3563909b83131933aabac45c3dbe5ba"
    page_range: [58, 59]
    quote: "But such a mean function can be handled easily by the methods discussed in Chapter 2, and we do not discuss it here"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p059:0075"
    chunk_hash: "3255e74617c9d078489dbf440aa88b837fd84fb1e81fd81deb0f6b379840abbc"
    page_range: [59, 59]
    quote: "If g(.) is nonlinear, xt is said to be nonlinear in mean. If h("
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p059:0076"
    chunk_hash: "39278947db6ad257e84b42133222d77f36d5761c2653505c85aa34650d44107e"
    page_range: [59, 60]
    quote: "Finally, nonparametric and semiparametric methods such as kernel regression and artificial neural networks have also"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p060:0077"
    chunk_hash: "0c2497abe60cbfca9227964cbde5774d4483e50861282c2c3643d9bb62d4cffb"
    page_range: [60, 61]
    quote: "This model was introduced by Granger and Andersen (1978) and has been widely investigated"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p061:0078"
    chunk_hash: "dae724c9ade315b80dcc216ba6b537782e26fc19b62beb2db621a4326e4c3589"
    page_range: [61, 61]
    quote: "The only insignificant estimate is the coefficient of at−2. Define aˆt = Rt − 0.014 − 0.160Rt−1 + 0.014Rt−3 1 + 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p061:0079"
    chunk_hash: "6b0e2be6127cfc214ef76ac45991c241a3107ea3c3603b9c66972f512dd03adb"
    page_range: [61, 62]
    quote: "−1.5xt−1 + at if xt−1 < 0, 0.5xt−1 + at if xt−1 ≥ 0, (4.8) 158 NONLINEAR MODELS AND THEIR APPLICATIONS • • • • • • • •"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p062:0080"
    chunk_hash: "0609833e6756804aaaef1c1662bddb3aaca9cab9fa010e25733477f1f2a7d59c"
    page_range: [62, 63]
    quote: "Figure 4.1 shows the time plot of a simulated series of xt with 200 observations"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p062:0081"
    chunk_hash: "e588f9a0cc4807615102cd4c48ce245da8ecaac7cddfb5e57f27e0f79f2b3057"
    page_range: [62, 63]
    quote: "The weight for each regime is simply the probability that xt is in that regime under its stationary NONLINEAR MODELS"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p063:0082"
    chunk_hash: "6ec42b7fe1aaf54e17fc8b4f56efeedfe8fd298ce748793e2cf50cafe3a06b63"
    page_range: [63, 63]
    quote: "In recent years, there is increasing interest in TAR models and their applications; see, for instance, Hansen (1997),"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p063:0083"
    chunk_hash: "b5bed9d9ae6393b5a5a0de369bf5fd3cb19c68eacf3c866464a86d5eab510bf8"
    page_range: [63, 64]
    quote: "Using univariate ARIMA models, we obtain the model (1 − 1.18B + 0.33B2 )(1 − 0.51B12)yt = (1 − 1.17B + 0.48B2 )(1 − 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p064:0084"
    chunk_hash: "398df5311aa250f406e24704cee52294eef8854c2c0f44b1f0574c170ed46151"
    page_range: [64, 65]
    quote: "Time plot of monthly U.S. civilian unemployment rate, seasonally adjusted, from January 1948 to March 2004"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p065:0085"
    chunk_hash: "727bcbdebfd3d007401d9c0aab6a585c532496e5e7773e4c0829f38dc077d2b5"
    page_range: [65, 65]
    quote: "Consequently, model (4.11) is capable of describing the time-varying dynamics of the U.S. unemployment rate"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p065:0086"
    chunk_hash: "c69a279d2588a80fec6b824743d690ceaefdca59b43957dafafa882103976b54"
    page_range: [65, 66]
    quote: "All estimates are statistically significant at the 5% level. The Ljung–Box statistics of the standardized residuals"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p066:0087"
    chunk_hash: "1e86fd5371cead87bab281ad72f8ef4f7dbb6ae3a1d0586aa9c202aa45c7b232"
    page_range: [66, 67]
    quote: "More specifically, we consider an AR(2)–TAR–GARCH(1,1) model for the series and obtain rt = 0.033 − 0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p067:0088"
    chunk_hash: "ceefd7bcb40fdf1ed8c8f2dc7e884d944cdca8b708f49eefc982a881f63eb9ce"
    page_range: [67, 68]
    quote: "The thresholds {γj } are the discontinuity points of the conditional mean function µt"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p068:0089"
    chunk_hash: "7e0c650f12669583d3c5f618843fbfa9ff94d199c167f73f48c35f7a14593d53"
    page_range: [68, 68]
    quote: "164 NONLINEAR MODELS AND THEIR APPLICATIONS Example 4.4. To illustrate the application of STAR models in financial time"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p068:0090"
    chunk_hash: "0dc5c053555e14b5bcfd7dc481a107bd2602123d86a38f2643c19ad45e9adeef"
    page_range: [68, 69]
    quote: "The RATS program used is given in Appendix A. 4.1.4 Markov Switching Model The idea of using probability switching in"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p069:0091"
    chunk_hash: "6a101f25cc10edefe1cbe166357752887259f5ac12cf5c32941b5353f71790c4"
    page_range: [69, 69]
    quote: "Yet as long as xt−d is observed, the regime of xt is known in a SETAR model"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p069:0092"
    chunk_hash: "2bd88465dc27e81e7b0129635281845c7b0399aa3c52f8ee8df7270764296321"
    page_range: [69, 70]
    quote: "The data are seasonally adjusted and shown in Figure 4.4, where a horizontal line of zero growth is also given"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p070:0093"
    chunk_hash: "2d9ce246e3abc61bae99b0e615fcf8d67b7e82b26dfe74ab884e98aed0bb4713"
    page_range: [70, 71]
    quote: "Employing the MSA model in Eq. (4.18) with p = 4 and using a Markov chain Monte Carlo method, which is discussed in"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p070:0094"
    chunk_hash: "530c931f881975eb199ad752a008e3b1b3a15ce7035a0e35d1b8727a64926349"
    page_range: [70, 71]
    quote: "Finally, the estimated AR coefficients of xt−2 differ substantially between the two states, indicating that the"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p071:0095"
    chunk_hash: "ced95ab32b18d6c6f853b51517f62ec361f5905e0230ca9866be979ed9a17f17"
    page_range: [71, 72]
    quote: "For simplicity, consider the problem of estimating m(.) at a particular date for which X = x"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p072:0096"
    chunk_hash: "154ae3a9151443c3897a28c4142b1018cb7cf40c71013837b6e7736bd1facdd7"
    page_range: [72, 73]
    quote: "In Eq. (4.20), we assume that the weights sum to T . One can treat 1/T as part of the weights and make the weights sum"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p073:0097"
    chunk_hash: "39fdedb03e9ce21d1e2eca46cc662dc8667ed63e2076f7f083aabdfc2349dc06"
    page_range: [73, 74]
    quote: "In practice, many choices are available for the kernel K(x). However, theoretical and practical considerations lead to"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p074:0098"
    chunk_hash: "d7970b7b9c0dfbd7ca282055d3c6d93ab2d780feb452265bd16ac42f8e0dee8e"
    page_range: [74, 75]
    quote: "The first approach is the plug-in method, which is based on the asymptotic expansion of the mean integrated squared"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p075:0099"
    chunk_hash: "5c6a20f06f6b1b8cf8e58b4de05f6508ee7d16cf46f24f3d46adf58cbdbfd454"
    page_range: [75, 76]
    quote: "The function CV (h) is called the cross-validation function because it validates the ability of the smoother to predict"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p076:0100"
    chunk_hash: "23c576ae86305cbbdcf408562e350cdc30f7d8adf127c515da8df5a10050be21"
    page_range: [76, 77]
    quote: "172 NONLINEAR MODELS AND THEIR APPLICATIONS Consequently, we have aˆ = sT,2"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p077:0101"
    chunk_hash: "7db0463b88d767bd2a54e680d503637f84089e5b35c26c97b07e6e99e322312a"
    page_range: [77, 77]
    quote: "Time Series Application In time series analysis, the explanatory variables are often the lagged values of the series"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p077:0102"
    chunk_hash: "92fd4f3e1ed5893a3fa84b93faadb805fdcd7d30a14ab8856fa862ff01ae58ca"
    page_range: [77, 78]
    quote: "For simplicity, we use |yt| as a proxy of the volatility of xt"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p078:0103"
    chunk_hash: "a9c64aec3662cb8b0b902977d700c47e6e65311b7f525ac5f94796ad52724715"
    page_range: [78, 79]
    quote: "ple nonparametric methods can be helpful in understanding the dynamic structure of a financial time series"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p079:0104"
    chunk_hash: "e897483e9d2d337a5450c959a7557ab1b519a29f69f78a505f551ef1680f37a3"
    page_range: [79, 80]
    quote: "Estimation of conditional mean and volatility of weekly 3-month Treasury bill rate via a local smoothing method: (a) yt"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p080:0105"
    chunk_hash: "e71abcf2b85e5083d76e8eb1bd3a2fbd6ddf008707b23863e03f0fcdecb3cd29"
    page_range: [80, 81]
    quote: "mate f (.) would require p-dimensional smoothing, which is hard to do when p is large, especially if the number of data"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p081:0106"
    chunk_hash: "ec0570afc51d18432042cc98ec23186eaf13dc62b3fcf7bffdb3100b395b40cb"
    page_range: [81, 81]
    quote: "ods for nonlinear time series analysis have been considered by Kitagawa (1998) and the references therein"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p081:0107"
    chunk_hash: "b3330e00ae1bb48114ed04576974c2aacba1b1f998c91862789bc8752f137974"
    page_range: [81, 82]
    quote: "I N P U T O U T P U T Hidden Layer Figure 4.8. A feed-forward neural network with one hidden layer for univariate time"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p083:0108"
    chunk_hash: "174f9241d9529c530a2910d606299c84671f12ec2a14fdd8394cf57b6292d608"
    page_range: [83, 83]
    quote: "NONLINEAR MODELS 179 Combining the layers, the output of a feed-forward neural network can be written as o = fo  α0o"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p083:0109"
    chunk_hash: "2b720e7ca6611cb3bfc3204b44bdccd9e993016ec2ec3f87618cee8eb62aee27"
    page_range: [83, 84]
    quote: "They can approximate any continuous function uniformly on compact sets by increasing the number of nodes in the hidden"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p084:0110"
    chunk_hash: "77d6b26eee531278cfccb02442197a30ea79c2c1da42684d8a9853a3b5994515"
    page_range: [84, 85]
    quote: "To ensure the smoothness of the fitted function, some additional constraints can be added to the prior minimization"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p085:0111"
    chunk_hash: "3830a5c379cf425f3a0efbf494af62f02b6c0f953a4c34f544f68f45d335cb65"
    page_range: [85, 85]
    quote: "Forecast Comparison The monthly returns of IBM stock in 1998 and 1999 form the second subsample and are used to"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p085:0112"
    chunk_hash: "a07e3a146b385bc37e6816903a57effe7610d08fc072a1b40cfc6fcda531b9b6"
    page_range: [85, 86]
    quote: "We use eight input nodes consisting of the first four lagged values of both rt and dt and four nodes in the hidden"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p086:0113"
    chunk_hash: "e71faa13268e1266d15fce46b8a0efb923b1da4bf2a9c3846e7fd2806daab051"
    page_range: [86, 87]
    quote: "Histograms of the number of forecasting errors for the directional movements of monthly log returns of IBM stock"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p087:0114"
    chunk_hash: "8118e6ffab8746141610eeaa050a0d54e9bb4bb4d801cb485e3cf0d9a953f9e0"
    page_range: [87, 88]
    quote: "Because nonlinearity may occur in many ways, there exists no single test that dominates the others in detecting"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p088:0115"
    chunk_hash: "47541fbfe881a02ec0f0f60cfda511ac624224d96076db02b0125ec314394b1f"
    page_range: [88, 89]
    quote: "Bispectral Test This test can be used to test for linearity and Gaussianity"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p089:0116"
    chunk_hash: "df3f40851e336faa5a7d91bf169440d7d821a285c08f8d9109364f5e1d13a270"
    page_range: [89, 89]
    quote: "The statistic is, therefore, different from other test statistics discussed because the latter mainly focus on either"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p089:0117"
    chunk_hash: "557137567339d92858bf3c4dfc6c4470251444d650cbd7d0cb243f6a9dde8429"
    page_range: [89, 90]
    quote: "i<j Iδ (X∗ i ,X∗ j ), = 1, k, where T = T − + 1 and X∗ i = xi if = 1 and X∗ i = Xk i if = k"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p090:0118"
    chunk_hash: "d33d21b98c680f1a4fa4034aa5bc4aeacb4f476c5252b967c87d02393a525589"
    page_range: [90, 91]
    quote: "The test may be sensitive to the choices of δ and k, especially when k is large. 4.2"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p091:0119"
    chunk_hash: "3387a4db5d101e5dcb78cca976e49a7cdc4d3aa20d378a615cb3950ab9e01362"
    page_range: [91, 92]
    quote: "Remark. Because xˆk t for k = 2,...,s + 1 tend to be highly correlated with Xt−1 and among themselves, principal"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p092:0120"
    chunk_hash: "145a1fa76deebc35e28e4bfc5e9efa2f39eae8a519c5d9a1d712d6d40fd39edc"
    page_range: [92, 92]
    quote: "188 NONLINEAR MODELS AND THEIR APPLICATIONS g and T − p − g − 1, where g = p(p + 1)/2"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p092:0121"
    chunk_hash: "798cd1a313e462e54e6e14cea30e8e95077196c0cace9ec7c20bbdb33480934c"
    page_range: [92, 93]
    quote: "Let l1(r1; φˆ 1, σˆ 2 1 ; φˆ 2, σˆ 2 2 ) be the log likelihood function evaluated at the maximum likelihood estimates"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p093:0122"
    chunk_hash: "55a722badc7fab4b97a7e4ed6eed6443e2634a7257a056646f11d25d2b1c1f44"
    page_range: [93, 93]
    quote: "For a realization {xt} T t=1, xt−d can assume values {x1,...,xT −d }"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p093:0123"
    chunk_hash: "85087c2c05d165f7a80d7dcd3da35a82d30b782acd995963fba225bb4a0eb643"
    page_range: [93, 94]
    quote: "Step 5 . Consider the linear regression of the standardized predictive residual eˆ(m+j)+d = α0"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p094:0124"
    chunk_hash: "9527f73147111ce6dd809f1c22aeb9bbaa926e4e49c267eb1c10fe8af1f9bf67"
    page_range: [94, 94]
    quote: "The five series employed are as follows: 1. r1t : A simulated series of iid N(0, 1) with 500 observations. 2"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p094:0125"
    chunk_hash: "5aff1ff6bcf1d314a69bc45d5a656da1b673edf690faaa06248fea75c63d9a97"
    page_range: [94, 95]
    quote: "For the BDS test, we chose δ =ˆσa and δ = 1.5σˆa with k = 2,..., 5"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p095:0126"
    chunk_hash: "db717a6cb9c77f15a72eb312ebe6fc76300443b2c79fe2477e0900e820975fff"
    page_range: [95, 96]
    quote: "For general series, other tests of Section 4.2 apply. If nonlinearity is statistically significant, then one chooses a"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p096:0127"
    chunk_hash: "85895cecfe5fd051ebed209bd0bb998593bb8377c5937b6ced6fd33a0e30089a"
    page_range: [96, 97]
    quote: "In some cases, we may treat the estimated parameters as given. 4.4"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p096:0128"
    chunk_hash: "8b44b8e209393eb16f4339bd83939451b559f70f5e275e30e21c82a32be83ba3"
    page_range: [96, 97]
    quote: "In practice, the available data set is divided into two subsamples"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p097:0129"
    chunk_hash: "b762291577f9a4858fad53017512448d2ddd260554a560c28399daa8a04a0c6f"
    page_range: [97, 98]
    quote: "Under some mild conditions, χ2 has an asymptotic chi-squared distribution with 1 degree of freedom"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p098:0130"
    chunk_hash: "43b0e4081350d10525a1b84139ec9fd2004e2ccebea2f8df7d09463c0fa26188"
    page_range: [98, 98]
    quote: "Distributional Measure Practitioners recently began to assess forecasting performance of a model using its predictive"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p098:0131"
    chunk_hash: "05ad029fe4db260f9a6aad469cbceb0306b4771eaed6d37d71564acc3e1dea1f"
    page_range: [98, 99]
    quote: "The statistic can be used for both model checking and forecasting comparison. 4"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p099:0132"
    chunk_hash: "d4df31b405d5c4b49306ea7896386efc3b0ebc3af703eac941ba5fe70ccd4d47"
    page_range: [99, 100]
    quote: "This is a seasonal model even though the data were seasonally adjusted"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p100:0133"
    chunk_hash: "626ab94e3139809e7f53b18b1ebc7ece2b86a6d5c7cc3aca93ac8c93d49f52eb"
    page_range: [100, 101]
    quote: "Here the economy should be stable, and essentially the change in the rate follows a simple AR(1) model because the"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p101:0134"
    chunk_hash: "3d06b7fe1f0d4c819d7a82c4ea03caa0d356b69562820bed59da32482bd4d67a"
    page_range: [101, 101]
    quote: "This model implies that in the second state the unemployment rate xt has an upward trend with an AR(2) polynomial"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p101:0135"
    chunk_hash: "9641b78d243c201b86402dd036dbfa20c979b5ae834f6dab037517290efe466a"
    page_range: [101, 102]
    quote: "For forecast origins in economic contractions, the TAR model shows"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p103:0136"
    chunk_hash: "3c66d954c8f03b4dacb2b8f99f3626e94c0e18df15891a431429a786bd23d9a1"
    page_range: [103, 104]
    quote: "APPENDIX A: SOME RATS PROGRAMS FOR NONLINEAR VOLATILITY MODELS 199 industrial restructuring are most likely to occur"
    edge_type: "supports"
  - source_id: "qm_afts_trim"
    chunk_id: "qm_afts_trim:p104:0137"
    chunk_hash: "c9cbb1a0d2d93eb153d59225a0a543cd5c5c82c78797643517f0464685325e3b"
    page_range: [104, 104]
    quote: "A line starting with # denotes a comment. The data file is ‘m-ibmln.txt’. # load the data into S-Plus workspace"
    edge_type: "supports"
card_hash: "1e2d77b5dd76b6a54b4bf79e5d1d3c97d91dc547966f5068758f8a1d4bdea517"
---
framing how conditional-variance models (ARCH and GARCH univariate; multivariate-vol covariance estimators) supply the time-varying volatility input that a CB-arb practitioner feeds into vega-sensitivity computation on the convertible's underlying-equity vol surface

## Original Card (preserved verbatim)

## Intuition

The CB-arb practitioner who wants to size vega exposure on a
delta-hedged convertible needs a forward-looking estimate of the
underlying-equity return volatility (see
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md)
for the CB implied-vol-surface context and
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)
for the vega-attribution context). The univariate ARCH / GARCH
family is the canonical time-series toolkit: ARCH models the
conditional variance `σ_t²` as a linear combination of past squared
returns, and GARCH extends ARCH by adding an autoregressive lag in
the conditional variance itself, producing a parsimonious recursion
that captures the observed volatility clustering in financial-return
time-series. **Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

Tsay 3e Ch.3 motivates the ARCH model on financial-return time-
series that exhibit volatility clustering: large absolute returns
tend to be followed by large absolute returns and small by small. The
canonical illustrative example in §3.4.4 is the Deutsche mark / U.S.
dollar 10-minute exchange-rate returns (Figure 3.2), where the
empirical autocorrelation of squared returns is significantly non-
zero at the first several lags while the autocorrelation of raw
returns is near-zero — the diagnostic signature for ARCH effects in
the residual series. The CB-arb adaptation treats the underlying-
equity return series as the input; the resulting `σ_t²` forecast
feeds the vega-input cross-link at
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

```
<!-- primitive: garch-volatility-clustering source: _diagram_primitives.md -->
   |ε_t|  (absolute residual magnitude)
       ^
       |         IID / constant variance (no clustering)
       |    . . . . . . . . . . . . . . . . . . . . . .
       |    . . . . . . . . . . . . . . . . . . . . . .
       +────────────────────────────────────────────────> t
       |
       |   ARCH/GARCH conditional-variance clustering
       |
       |              ■■■                ■■■■
       |              ■■■■              ■■■■■
       |   .  .  .   ■■■■■   .  .  .   ■■■■■■   .  .
       |   . .  . .  ■■■■■■   . . .   ■■■■■■■  . . .
       +────────────────────────────────────────────────> t
                    └─cluster─┘        └──cluster──┘

   E[ε_t² | F_{t−1}] = a_0 + a_1·ε_{t−1}² + ...  (ARCH/GARCH)
   Large shock at t−1 inflates conditional variance at t.
```

## Definition

An ARCH(m) model specifies the conditional variance of an innovation
series `ε_t` (typically the residuals from a conditional-mean model
such as an AR or ARMA fit) as `σ_t² = α_0 + α_1·ε_{t-1}² + α_2·
ε_{t-2}² + ... + α_m·ε_{t-m}²` with `α_0 > 0` and `α_i ≥ 0` for
`i = 1, ..., m` for non-negativity of `σ_t²`, plus the
weak-stationarity condition `Σ_{i=1}^{m} α_i < 1` for finite
unconditional variance. The CB-arb consumer of the ARCH output is
the vega-input cross-link at
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

A GARCH(m, s) model extends ARCH by adding autoregressive lags in
the conditional variance itself. The conditional-variance recursion
is given by the expression `σ_t² = α_0 + Σ α_i·ε_{t-i}² + Σ β_j·σ_{t-j}²`
where the first summation runs over the `m` ARCH lags and the second
runs over the `s` GARCH lags, subject to the usual non-negativity
constraints on `α_0`, the `α_i`, and the `β_j`, plus the sum-less-
than-unity stationarity condition. The canonical practitioner default
for univariate equity-vol modelling is the GARCH(1, 1) special case
because it fits most empirical clustering patterns with two slope
parameters and a constant, and its persistence statistic is the
half-life proxy that gates the CB-arb's vega-deployment horizon (the
practitioner choice of horizon is documented at
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The multivariate generalisation in Tsay's Multivariate Time Series
Analysis Ch.10 extends the univariate `σ_t²` recursion to a
conditional covariance matrix `Σ_t` of a vector return series. The
BEKK and DCC (Dynamic Conditional Correlation) parameterisations are
the practitioner-standard parameterisations: BEKK writes `Σ_t = C·Cᵀ
+ A·ε_{t-1}·ε_{t-1}ᵀ·Aᵀ + B·Σ_{t-1}·Bᵀ` with positive-definiteness
guaranteed by construction; DCC separates volatility (univariate
GARCH per asset) from correlation (a separate scalar DCC recursion
on standardized residuals). The CB-arb application is cross-issuer
gamma attribution when the CB book holds positions in multiple
convertibles whose underlying-equity correlations matter (see
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)).
**Source:**
01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.

## Mathematical Reasoning

The ARCH(1) conditional-variance recursion (source ASSERTS) is `σ_t² = α_0 + α_1·ε_{t-1}²` where `α_0 > 0` and `α_1 ≥ 0`. The unconditional variance is `Var(ε_t) = α_0 / (1 − α_1)` (finite iff `α_1 < 1`); the kurtosis of `ε_t` exceeds 3 when `3·α_1² < 1`, which is the algebraic source of the fat-tailed marginal distribution that ARCH models reproduce. Tsay 3e Ch.3 §3.4 establishes both results without proof beyond the algebraic manipulation of conditional moments. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The GARCH(1, 1) recursion (source ASSERTS) is `σ_t² = α_0 + α_1·ε_{t-1}² + β_1·σ_{t-1}²` with `α_0 > 0`, `α_1, β_1 ≥ 0`, and `α_1 + β_1 < 1` for weak stationarity. Tsay §3.5 establishes the recursion form and the stationarity condition without further proof beyond the algebraic manipulation of conditional moments. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The unconditional variance under the GARCH(1, 1) stationarity assumption is the practitioner-canonical `Var(ε_t) = α_0 / (1 − α_1 − β_1)`. The persistence parameter is the sum `α_1 + β_1`, which controls the half-life of a volatility shock; a value close to unity indicates that volatility shocks decay slowly. The boundary case where `α_1 + β_1` equals unity is the so-called IGARCH limit, which has infinite unconditional variance and a unit-root in conditional variance — the practitioner-folkloric interpretation that is consistent with the algebraic forms. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The maximum-likelihood estimator (source ASSERTS) for the GARCH parameter vector `θ = (α_0, α_1, β_1)` maximises the conditional Gaussian log-likelihood `ℓ(θ) = − (1/2) · Σ_t [ln(σ_t²(θ)) + ε_t² / σ_t²(θ)]` where `σ_t²(θ)` is computed by the GARCH(1, 1) recursion. Quasi-maximum-likelihood estimation (QMLE) substitutes the Gaussian density for the true error distribution and remains consistent and asymptotically normal under mild regularity conditions; the sandwich-form standard errors are the robust-to-misspecification covariance estimator. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The multivariate DCC parameterisation (source ASSERTS) decomposes the time-varying conditional covariance as `Σ_t = D_t · R_t · D_t` where `D_t = diag(σ_{1,t}, ..., σ_{n,t})` is the diagonal matrix of univariate GARCH-fitted volatilities and `R_t` is the time-varying correlation matrix evolved by a scalar DCC recursion on the standardised residuals `z_{i,t} = ε_{i,t} / σ_{i,t}`. The estimation proceeds in two stages: first fit univariate GARCH per asset; then estimate the scalar DCC parameters from the standardised residuals. Tsay Multivariate Ch.10 establishes the two-stage estimator and its quasi-likelihood form without proof beyond the conditional-Gaussian algebra. **Source:** 01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.

## See Also

- [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md) — the sibling factor-construction card whose ridge / lasso / boosting machinery sits upstream of the vol-forecasting step here; the vol forecast feeds into the factor-model regression's residual variance assumption
- [`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md) — the CB implied-vol-surface card that consumes the GARCH univariate `σ_t²` output as the historical-vol benchmark for the CB underlying-equity vol input to vega
- [`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md) — the CB Greeks card that consumes the multivariate `Σ_t` cross-asset covariance estimate for cross-issuer gamma attribution when the CB book holds multiple convertibles whose underlying-equity correlations matter

## Escalate to Raw When

Open Tsay 3e directly or the more rigorous multivariate-vol
references when any of the criteria below applies. **Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

- The vol-model needs stochastic-volatility depth beyond GARCH
  (latent-volatility-process modelling, particle-filter estimation)
  — Tsay 3e Ch.3 §3.12 introduces the stochastic-volatility model
  at intuition depth only; deeper state-space machinery (Tsay
  Ch.11+) is out of scope per the v7+ CB-arb extension boundary
  discipline (see frontmatter `Out of scope:`). **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The vol-model needs jump-diffusion or realized-vol high-frequency
  estimation — out of scope; consult the relevant raw references
  if needed. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The multivariate vol-model needs BEKK identifiability conditions
  or copula-vol depth beyond Ch.10 DCC — out of scope under the v7+
  CB-arb extension policy. **Source:**
  01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.
- The CB-arb vol input needs implied-vol surface construction
  rather than historical-vol estimation — route to the
  [`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md)
  card. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The CB-arb vega computation needs the option-pricing surface
  (Black-Scholes / Black implied-vol mapping) — route to the
  [`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)
  card. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
