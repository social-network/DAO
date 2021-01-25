use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const ACCOUNT_ID: u64 = 1;
const ACCOUNT_ID2: u64 = 2;
const MISSION_ID: u32 = 11;
const MISSION_ID2: u32 = 12;
const GUARDIAN_ID: u64 = 101;
const GUARDIAN_ID2: u64 = 102;

#[test]
fn registration_and_unregistration_should_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(ValidatorRegistry::mission_of(ACCOUNT_ID), 0);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), Vec::<u64>::new());
        assert_ok!(ValidatorRegistry::register(
            Origin::signed(ACCOUNT_ID),
            MISSION_ID,
            GUARDIAN_ID
        ));
        assert_eq!(ValidatorRegistry::guardian_of(ACCOUNT_ID), GUARDIAN_ID);
        assert_eq!(ValidatorRegistry::mission_of(ACCOUNT_ID), MISSION_ID);
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), vec![GUARDIAN_ID]);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), vec![ACCOUNT_ID]);

        assert_ok!(ValidatorRegistry::register(
            Origin::signed(ACCOUNT_ID2),
            MISSION_ID,
            GUARDIAN_ID2
        ));
        assert_eq!(ValidatorRegistry::guardian_of(ACCOUNT_ID2), GUARDIAN_ID2);
        assert_eq!(ValidatorRegistry::mission_of(ACCOUNT_ID2), MISSION_ID);
        assert_eq!(
            ValidatorRegistry::guardians(MISSION_ID),
            vec![GUARDIAN_ID, GUARDIAN_ID2]
        );
        assert_eq!(
            ValidatorRegistry::validators(MISSION_ID),
            vec![ACCOUNT_ID, ACCOUNT_ID2]
        );

        assert_ok!(ValidatorRegistry::unregister(Origin::signed(ACCOUNT_ID2)));
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), vec![GUARDIAN_ID]);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), vec![ACCOUNT_ID]);
        assert_ok!(ValidatorRegistry::unregister(Origin::signed(ACCOUNT_ID)));
        assert_eq!(ValidatorRegistry::mission_of(ACCOUNT_ID), 0);
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), Vec::<u64>::new());
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), Vec::<u64>::new());
    });
}

#[test]
fn re_registration_should_not_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(ValidatorRegistry::register(
            Origin::signed(ACCOUNT_ID),
            MISSION_ID,
            GUARDIAN_ID
        ));
        assert_eq!(ValidatorRegistry::mission_of(ACCOUNT_ID), MISSION_ID);
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), vec![GUARDIAN_ID]);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), vec![ACCOUNT_ID]);

        assert_noop!(
            ValidatorRegistry::register(Origin::signed(ACCOUNT_ID), MISSION_ID2, GUARDIAN_ID),
            Error::<Test>::AlreadyRegistered
        );
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), vec![GUARDIAN_ID]);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), vec![ACCOUNT_ID]);

        assert_noop!(
            ValidatorRegistry::register(Origin::signed(ACCOUNT_ID2), MISSION_ID2, GUARDIAN_ID),
            Error::<Test>::GuardianAlreadyRegistered
        );
        assert_eq!(ValidatorRegistry::guardians(MISSION_ID), vec![GUARDIAN_ID]);
        assert_eq!(ValidatorRegistry::validators(MISSION_ID), vec![ACCOUNT_ID]);
    });
}

#[test]
fn registration_with_invalid_mission_id_should_not_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ValidatorRegistry::register(Origin::signed(ACCOUNT_ID), 0, GUARDIAN_ID),
            pallet_mission_tokens::Error::<Test>::InvalidMissionTokenId
        );
        assert_noop!(
            ValidatorRegistry::register(Origin::signed(ACCOUNT_ID), 18, GUARDIAN_ID),
            pallet_mission_tokens::Error::<Test>::InvalidMissionTokenId
        );
    });
}
