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
	fn reward_coefficient_should_work() {
        const TOTAL_TOKENS: u128 = 100_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS), (70_029, 100_042));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS), (70_029, 100_042));
        assert_eq!(super::compute_total_payout(100u32, TOTAL_TOKENS), (70_023, 100_034));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS), (70_000, 100_000));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS), (69_970, 99_958));
        assert_eq!(super::compute_total_payout(5_000u32, TOTAL_TOKENS), (69_736, 99_623));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS), (69_443, 99_205));
        assert_eq!(super::compute_total_payout(50_000u32, TOTAL_TOKENS), (67_147, 95_925));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS), (64_383, 91_977));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS), (45_998, 65_712));
        assert_eq!(super::compute_total_payout(1_000_000u32, TOTAL_TOKENS), (30_209, 43_156));
	}
}
