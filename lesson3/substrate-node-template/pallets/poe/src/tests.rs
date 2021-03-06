// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// Test cases for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4, 5, 6];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

// Test cases for revoke_claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// Test cases for transfer_claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(Proofs::<Test>::get(&claim), (2, system::Module::<Test>::block_number()))
    })
}

#[test]
fn transfer_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn bid_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::bid_claim(Origin::signed(1), claim.clone(), 5));
        assert_eq!(Prices::<Test>::get(&claim), 5);
    })
}

#[test]
fn bid_claim_failed_with_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::bid_claim(Origin::signed(1), claim.clone(), 5),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn bid_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::bid_claim(Origin::signed(2), claim.clone(), 5),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn buy_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        let _ = PoeModule::bid_claim(Origin::signed(1), claim.clone(), 5);

        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
        let _ = PoeModule::buy_claim(Origin::signed(2), claim.clone(), 6);
        assert_eq!(Proofs::<Test>::get(&claim), (2, system::Module::<Test>::block_number()));
        assert_eq!(Prices::<Test>::get(&claim), 6);
    })
}

#[test]
fn buy_claim_failed_with_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 5),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn buy_claim_failed_with_not_for_bidding() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 5),
            Error::<Test>::ClaimNotForBidding
        );
    })
}

#[test]
fn buy_claim_failed_with_insufficient_price() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        let _ = PoeModule::bid_claim(Origin::signed(1), claim.clone(), 5);

        assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 4),
            Error::<Test>::InsufficientPrice
        );
    })
}

#[test]
fn buy_claim_failed_with_account_balance_not_enough() {
   new_test_ext().execute_with(|| {
       let claim = vec![0, 1];
       let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
       let _ = PoeModule::bid_claim(Origin::signed(1), claim.clone(), 5);

       assert_noop!(
            PoeModule::buy_claim(Origin::signed(2), claim.clone(), 30),
            Error::<Test>::AccountBalanceNotEnough
       );
   })
}