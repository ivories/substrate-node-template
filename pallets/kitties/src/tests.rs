use crate::mock::*;
use super::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        let lock_event = TestEvent::kitties_event(RawEvent::Created(1, 0));
        assert!(System::events().iter().any(|a| a.event == lock_event));
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_ok!(KittiesTest::transfer(Origin::signed(1), 2, 0));
        let lock_event = TestEvent::kitties_event(RawEvent::Transferred(1, 2, 0));
        assert!(System::events().iter().any(|a| a.event == lock_event));
    })
}

#[test]
fn transfer_kitty_failed_when_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_noop!(
            KittiesTest::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::RequireOwner
        );
    })
}

#[test]
fn transfer_kitty_failed_when_transfer_to_self() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_noop!(
            KittiesTest::transfer(Origin::signed(1), 1, 0),
            Error::<Test>::TransferToSelf
        );
    })
}

#[test]
fn transfer_kitty_failed_when_kitty_id_not_exist() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_noop!(
            KittiesTest::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::InvalidKittyId
        );
    })
}

#[test]
fn breed_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_ok!(KittiesTest::breed(Origin::signed(1), 0, 1));
        let lock_event = TestEvent::kitties_event(RawEvent::Created(1, 2));
        assert!(System::events().iter().any(|a| a.event == lock_event));
    })
}

#[test]
fn breed_kitty_failed_when_same_parent() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesTest::create(Origin::signed(1)));
        assert_noop!(
            KittiesTest::breed(Origin::signed(2), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
    })
}

#[test]
fn breed_kitty_failed_when_kitty_id_not_exist() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_noop!(
            KittiesTest::breed(Origin::signed(2), 1, 0),
            Error::<Test>::InvalidKittyId
        );
    })
}