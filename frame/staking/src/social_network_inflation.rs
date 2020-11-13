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
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_454_422, 77_792_032));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_451_699, 77_788_142));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_448_977, 77_784_253));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_446_254, 77_780_363));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_443_531, 77_776_474));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_440_809, 77_772_585));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_438_087, 77_768_697));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_435_365, 77_764_808));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_432_644, 77_760_920));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (54_429_922, 77_757_032));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (53_112_546, 75_875_066));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (51_801_147, 74_001_639));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (33_029_301, 47_184_716));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (2_710_540, 3_872_201));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (366_342, 523_347));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (134_423, 192_033));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (6_187, 8_839));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (5390000000, 7700000000));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));

        const TOTAL_ISSUANCE4: u128 = 1_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77795921, 1000233278));
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77792032, 1000183267));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77788142, 1000133257));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77784253, 1000083250));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77780363, 1000033245));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77776474, 999983242));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77772585, 999933243));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77768697, 999883245));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77764808, 999833250));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77760920, 999783258));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (77757032, 999733268));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (75_875_066, 975_536_568));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (74_001_639, 951_449_642));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (47_184_716, 606_660_630));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (3872201, 49785433));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (523_347, 6_728_747));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (192033, 2468995));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (8839, 113641));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (4744444443, 6777777777));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));

        const TOTAL_ISSUANCE5: u128 = 10_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77_795_921, 10_002_332_780));
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77792032, 10001832664));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77788142, 10001332567));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77784253, 10000832491));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77780363, 10000332444));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77776474, 9999832417));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77772585, 9999332421));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77768697, 9998832444));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77764808, 9998332498));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77760920, 9997832571));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (77757032, 9997332674));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (75_875_066, 9_755_365_672));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (74_001_639, 9_514_496_416));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (47_184_716, 6_066_606_296));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (3872201, 497854322));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (523_347, 67_287_464));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (192033, 24689949));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (8839, 1136406));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
	}
}
