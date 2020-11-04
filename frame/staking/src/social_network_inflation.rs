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


        const TOTAL_TOKENS4: u128 = 1_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS4), (700_294_000, 1_000_420_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS4), (699_999_876, 999_999_824));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS4), (699_705_753, 999_579_648));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS4), (694_432_692, 992_046_703));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS4), (643_838_685, 919_769_551));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS4), (302_087_481, 431_553_545));
        assert_eq!(super::compute_total_payout(2_000_000u32, TOTAL_TOKENS4), (130_177_452, 185_967_789));
        assert_eq!(super::compute_total_payout(3_000_000u32, TOTAL_TOKENS4), (55_962_182, 79_945_975));
        assert_eq!(super::compute_total_payout(4_000_000u32, TOTAL_TOKENS4), (23_924_348, 34_177_640));
        assert_eq!(super::compute_total_payout(5_000_000u32, TOTAL_TOKENS4), (10_090_906, 14_415_580));
        assert_eq!(super::compute_total_payout(9_000_000u32, TOTAL_TOKENS4), (0, 0));


        const TOTAL_TOKENS5: u128 = 10_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS5), (7_002_940_000, 10_004_200_000));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS5), (6_999_998_765, 9_999_998_236));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS5), (6_997_057_530, 9_995_796_472));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS5), (6_944_326_918, 9_920_467_027));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS5), (6_438_386_850, 919_769_5501));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS5), (3_020_874_810, 4_315_535_444));
        assert_eq!(super::compute_total_payout(2_000_000u32, TOTAL_TOKENS5), (1_301_774_520, 1_859_677_887));
        assert_eq!(super::compute_total_payout(3_000_000u32, TOTAL_TOKENS5), (559_621_820, 799_459_743));
        assert_eq!(super::compute_total_payout(4_000_000u32, TOTAL_TOKENS5), (239_243_477, 341_776_396));
        assert_eq!(super::compute_total_payout(5_000_000u32, TOTAL_TOKENS5), (100_909_053, 144_155_791));
        assert_eq!(super::compute_total_payout(9_000_000u32, TOTAL_TOKENS5), (0, 0));
	}
}
