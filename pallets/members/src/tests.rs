use crate::{mock::*, Error};
use frame_support::{assert_ok, assert_noop, error::BadOrigin};

#[test]
fn add_member() {
	new_test_ext().execute_with(|| {
		assert_ok!(Members::add_member(Origin::root(), 2, 3));
		assert_eq!(Members::member_list_by_club(2, 3), true);
	});
}

#[test]
fn remove_member() {
	new_test_ext().execute_with(|| {
		assert_ok!(Members::add_member(Origin::root(), 2, 3));
		assert_ok!(Members::remove_member(Origin::root(), 2, 3));
		assert_eq!(Members::member_list_by_club(2, 3), false);
	});
}

#[test]
fn remove_nonexistant_member() {
	new_test_ext().execute_with(|| {
		assert_noop!(Members::remove_member(Origin::root(), 5, 3), Error::<Test>::NotAvailable);
	});
}

#[test]
fn add_duplicate_member() {
	new_test_ext().execute_with(|| {
		assert_ok!(Members::add_member(Origin::root(), 2, 3));
		assert_noop!(Members::add_member(Origin::root(), 2, 3), Error::<Test>::Duplicate);
	});
}

#[test]
fn add_duplicate_member_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(Members::add_member(Origin::signed(1), 2, 3), BadOrigin);
	});
}

#[test]
fn remove_member_non_root() {
	new_test_ext().execute_with(|| {
		assert_ok!(Members::add_member(Origin::root(), 2, 3));
		assert_noop!(Members::remove_member(Origin::signed(1), 2, 3), BadOrigin);
	});
}
