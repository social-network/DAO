use crate::EraIndex;
use sp_runtime::{Perbill, Percent, SaturatedConversion, traits::{AtLeast32BitUnsigned, Saturating}};

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `maximum-payout = 1.00042 * total-tokens (1 - 0.00000084) ^ era-index`
/// `staker-payout = maximum_payout * 0.7`
pub fn compute_total_payout<N>(
    era_index: EraIndex,
    total_tokens: N,
) -> (N, N) where N: AtLeast32BitUnsigned + Clone {
    let k1 = Perbill::from_rational_approximation(420_000u128, 1_000_000_000u128);
    let k2 = Perbill::from_rational_approximation(999_999_160u128, 1_000_000_000u128)
        .saturating_pow(era_index.saturated_into());
    let maximum_payout = k2.mul_ceil(total_tokens.clone() + k1.mul_ceil(total_tokens));
    let reward_coefficient = Percent::from_rational_approximation(7u32, 10u32);
    let staker_payout = reward_coefficient.mul_floor(maximum_payout.clone());
    (staker_payout, maximum_payout)
}

#[cfg(test)]
mod test {
	#[test]
	fn calculation_is_sensible() {
        const TOTAL_TOKENS: u128 = 10_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS), (7_003, 10_005));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS), (7_003, 10_005));
        assert_eq!(super::compute_total_payout(100u32, TOTAL_TOKENS), (7003, 10005));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS), (7_000, 10_001));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS), (6_997, 9_997));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS), (6_945, 9_922));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS), (6_439, 9_199));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS), (4_600, 6_572));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS2), (30_209, 43_156));  // it should not be
        assert_eq!(super::compute_total_payout(2_000_000u32, TOTAL_TOKENS2), (13_017, 18_597));  // it should not be
        assert_eq!(super::compute_total_payout(3_000_000u32, TOTAL_TOKENS2), (5_596, 7_995));    // it should not be


        const TOTAL_TOKENS2: u128 = 100_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS2), (70_029, 100_042));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS2), (70_029, 100_042));
        assert_eq!(super::compute_total_payout(100u32, TOTAL_TOKENS2), (70_023, 100_034));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS2), (70_000, 100_000));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS2), (69_970, 99_958));
        assert_eq!(super::compute_total_payout(5_000u32, TOTAL_TOKENS2), (69_736, 99_623));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS2), (69_443, 99_205));
        assert_eq!(super::compute_total_payout(50_000u32, TOTAL_TOKENS2), (67_147, 95_925));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS2), (64_383, 91_977));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS2), (45_998, 65_712));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS2), (30_209, 43_156));
        assert_eq!(super::compute_total_payout(2_000_000u32, TOTAL_TOKENS2), (13_017, 18_597));
        assert_eq!(super::compute_total_payout(3_000_000u32, TOTAL_TOKENS2), (5_596, 7_995));
        assert_eq!(super::compute_total_payout(4_000_000u32, TOTAL_TOKENS2), (2_392, 3_418));
        assert_eq!(super::compute_total_payout(5_000_000u32, TOTAL_TOKENS2), (1_009, 1_442));
        assert_eq!(super::compute_total_payout(6_000_000u32, TOTAL_TOKENS2), (412, 589));
        assert_eq!(super::compute_total_payout(7_000_000u32, TOTAL_TOKENS2), (155, 222));
        assert_eq!(super::compute_total_payout(8_000_000u32, TOTAL_TOKENS2), (49, 71));
        assert_eq!(super::compute_total_payout(9_000_000u32, TOTAL_TOKENS2), (0, 0));


        const TOTAL_TOKENS3: u128 = 100_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS3), (70_029_400, 100_042_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS3), (69_999_988, 99_999_983));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS3), (69_970_575, 99_957_965));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS3), (69_443_269, 99_204_671));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS3), (64_383_869, 91_976_956));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS3), (30_208_748, 43_155_355));
        assert_eq!(super::compute_total_payout(2_000_000u32, TOTAL_TOKENS3), (13_017_745, 18_596_779));
        assert_eq!(super::compute_total_payout(3_000_000u32, TOTAL_TOKENS3), (5_596_218, 7_994_598));
        assert_eq!(super::compute_total_payout(4_000_000u32, TOTAL_TOKENS3), (2_392_434, 3_417_764));
        assert_eq!(super::compute_total_payout(5_000_000u32, TOTAL_TOKENS3), (1_009_090, 1_441_558));
        assert_eq!(super::compute_total_payout(9_000_000u32, TOTAL_TOKENS3), (0, 0));
	}
}
