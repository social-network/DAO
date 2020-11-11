use crate::EraIndex;
use sp_runtime::{Perbill, Percent, SaturatedConversion, traits::{AtLeast32BitUnsigned, Saturating, Zero}};

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `maximum-payout = 1.0003 * total-tokens (1 - 0.00005) ^ era-index`
/// `staker-payout = maximum_payout * 0.7`
pub fn compute_total_payout<N>(
    era_index: EraIndex,
    total_tokens: N,
    total_issuance: N,
) -> (N, N) where N: AtLeast32BitUnsigned + Clone {
    if era_index < 360_000 {
        // If era < 360,000 mint according to inflation formula
        let k1 = Perbill::from_rational_approximation(233_278u128, 1_000_000_000u128);
        let k2 = Perbill::from_rational_approximation(999_950_000u128, 1_000_000_000u128)
            .saturating_pow(era_index.saturated_into());
        let staker_payout = k2.mul_ceil(total_tokens.clone() + k1.mul_ceil(total_tokens));
        let maximum_payout = k2.mul_ceil(total_issuance.clone() + k1.mul_ceil(total_issuance));
        let staker_to_treasury_ratio = Percent::from_rational_approximation(7u32, 10u32);
        let staker_maximum = staker_to_treasury_ratio.mul_floor(maximum_payout.clone());
            if staker_payout > staker_maximum {
                (staker_maximum, maximum_payout)
            } else {
                (staker_payout, maximum_payout)
            }
    } else if era_index == 360_000 {
        let maximum_payout = 7_777_777_777u128.saturated_into::<N>().saturating_sub(total_issuance);
        let staker_to_treasury_ratio = Percent::from_rational_approximation(7u32, 10u32);
        let staker_maximum = staker_to_treasury_ratio.mul_floor(maximum_payout.clone());
        (staker_maximum, maximum_payout)
    } else {
        // If era > 360,000 no more minting
        let maximum_payout = Zero::zero();
        let staker_payout = Zero::zero();
        (staker_payout, maximum_payout)
    }
}

#[cfg(test)]
mod test {
	#[test]
	fn calculation_is_sensible() {
        const TOTAL_TOKENS: u128 = 77_777_777;
        const TOTAL_ISSUANCE3: u128 = 77_777_777;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_457_144, 77_795_921));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (53_112_546, 75_875_066));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (51_801_147, 74_001_639));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (33_029_301, 47_184_716));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (366_342, 523_347));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));

        const TOTAL_ISSUANCE4: u128 = 1_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77_795_921, 1_000_233_278));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (75_875_066, 975_536_568));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (74_001_639, 951_449_642));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (47_184_716, 606_660_630));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (523_347, 6_728_747));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));

        const TOTAL_ISSUANCE5: u128 = 10_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77_795_921, 10_002_332_780));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (75_875_066, 9_755_365_672));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (74_001_639, 9_514_496_416));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (47_184_716, 6_066_606_296));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (523_347, 67_287_464));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
	}
}
