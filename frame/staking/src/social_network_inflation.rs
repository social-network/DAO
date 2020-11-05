use crate::EraIndex;
use sp_runtime::{Perbill, Percent, SaturatedConversion, traits::{AtLeast32BitUnsigned, Saturating}};

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `maximum-payout = 1.0003 * total-tokens (1 - 0.00005) ^ era-index`
/// `staker-payout = maximum_payout * 0.7`
pub fn compute_total_payout<N>(
    era_index: EraIndex,
    total_tokens: N,
) -> (N, N) where N: AtLeast32BitUnsigned + Clone {
    let k1 = Perbill::from_rational_approximation(233_278u128, 1_000_000_000u128);
    let k2 = Perbill::from_rational_approximation(999_950_000u128, 1_000_000_000u128)
        .saturating_pow(era_index.saturated_into());
    let maximum_payout = k2.mul_ceil(total_tokens.clone() + k1.mul_ceil(total_tokens));
    let staker_to_treasury_ratio = Percent::from_rational_approximation(7u32, 10u32);
    let staker_payout = staker_to_treasury_ratio.mul_floor(maximum_payout.clone());
    (staker_payout, maximum_payout)
}

#[cfg(test)]
mod test {
	#[test]
	fn calculation_is_sensible() {
        const TOTAL_TOKENS3: u128 = 77_777_777;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS3), (70_021_000, 100_030_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS3), (68_292_115, 97_560_165));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS3), (66_605_917, 95_151_311));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS3), (42_469_077, 60_670_110));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS3), (471_044, 672_920));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS3), (0, 0));

        const TOTAL_TOKENS4: u128 = 1_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS4), (700_210_000, 1_000_300_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS4), (682_921_149, 975_601_642));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS4), (666_059_177, 951_513_110));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS4), (424_690_768, 606_701_098));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS4), (4_710_437, 6_729_196));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS4), (0, 0));

        const TOTAL_TOKENS5: u128 = 10_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS5), (7_002_100_000, 10_003_000_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS5), (6_829_211_492, 9_756_016_418));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS5), (6_660_591_765, 9_515_131_094));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS5), (4_246_907_684, 6_067_010_978));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS5), (47_104_366, 67_291_952));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS5), (0, 0));
	}
}
