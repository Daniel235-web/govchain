use crate::{mock::*, Error, Event};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

// Test successful audit log entry creation
#[test]
fn it_works_for_log_activity() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create audit log entry
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(1),
            b"LOGIN".to_vec(),
            b"User logged into system".to_vec(),
            None,
            None
        ));
        
        // Check that the event was emitted correctly
        System::assert_last_event(
            Event::AuditEntryCreated {
                entry_id: 0,
                activity_type: BoundedVec::try_from(b"LOGIN".to_vec()).unwrap(),
                actor: 1,
                details: BoundedVec::try_from(b"User logged into system".to_vec()).unwrap(),
            }
            .into(),
        );
    });
}

// Test empty activity type is rejected
#[test]
fn log_activity_fails_for_empty_activity_type() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to create entry with empty activity type
        assert_noop!(
            AuditLogPallet::log_activity(
                RuntimeOrigin::signed(1),
                b"".to_vec(), // Empty activity type
                b"Some details".to_vec(),
                None,
                None
            ),
            Error::<Test>::InvalidActivityType
        );
    });
}

// Test empty details are rejected
#[test]
fn log_activity_fails_for_empty_details() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to create entry with empty details
        assert_noop!(
            AuditLogPallet::log_activity(
                RuntimeOrigin::signed(1),
                b"LOGIN".to_vec(),
                b"".to_vec(), // Empty details
                None,
                None
            ),
            Error::<Test>::InvalidDetails
        );
    });
}

// Test unsigned origin fails for log_activity
#[test]
fn log_activity_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            AuditLogPallet::log_activity(
                RuntimeOrigin::none(), // No signature
                b"LOGIN".to_vec(),
                b"Some details".to_vec(),
                None,
                None
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test successful get_audit_entry for existing entry
#[test]
fn it_works_for_get_audit_entry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create an audit entry
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(1),
            b"LOGIN".to_vec(),
            b"User logged in".to_vec(),
            None,
            None
        ));
        
        // Then retrieve it
        assert_ok!(AuditLogPallet::get_audit_entry(
            RuntimeOrigin::signed(2), // Different user can retrieve
            0 // Entry ID we just created
        ));
    });
}

// Test get_audit_entry fails for non-existent entry
#[test]
fn get_audit_entry_fails_for_non_existent_entry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to get entry that doesn't exist
        assert_noop!(
            AuditLogPallet::get_audit_entry(
                RuntimeOrigin::signed(1),
                999 // Non-existent entry ID
            ),
            Error::<Test>::EntryDoesNotExist
        );
    });
}

// Test multiple audit log entries have sequential IDs
#[test]
fn multiple_entries_have_sequential_ids() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create first entry - should get ID 0
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(1),
            b"CREATE".to_vec(),
            b"First entry".to_vec(),
            None,
            None
        ));
        
        // Create second entry - should get ID 1  
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(2),
            b"UPDATE".to_vec(),
            b"Second entry".to_vec(),
            None,
            None
        ));
        
        // Create third entry - should get ID 2
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(3),
            b"DELETE".to_vec(),
            b"Third entry".to_vec(),
            None,
            None
        ));
        
        // Verify we can retrieve all entries by their IDs
        assert_ok!(AuditLogPallet::get_audit_entry(RuntimeOrigin::signed(1), 0));
        assert_ok!(AuditLogPallet::get_audit_entry(RuntimeOrigin::signed(1), 1));
        assert_ok!(AuditLogPallet::get_audit_entry(RuntimeOrigin::signed(1), 2));
        
        // Entry 3 should not exist
        assert_noop!(
            AuditLogPallet::get_audit_entry(RuntimeOrigin::signed(1), 3),
            Error::<Test>::EntryDoesNotExist
        );
    });
}

// Test activity type that is too long is rejected
#[test]
fn log_activity_fails_for_too_long_activity_type() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create activity type that exceeds 50 bytes
        let long_activity_type = b"A".repeat(51);
        
        assert_noop!(
            AuditLogPallet::log_activity(
                RuntimeOrigin::signed(1),
                long_activity_type,
                b"Some details".to_vec(),
                None,
                None
            ),
            Error::<Test>::InvalidActivityType
        );
    });
}

// Test details that are too long are rejected
#[test]
fn log_activity_fails_for_too_long_details() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create details that exceed 500 bytes
        let long_details = b"X".repeat(501);
        
        assert_noop!(
            AuditLogPallet::log_activity(
                RuntimeOrigin::signed(1),
                b"LOGIN".to_vec(),
                long_details,
                None,
                None
            ),
            Error::<Test>::InvalidDetails
        );
    });
}

// Test audit log entry creation with related IDs
#[test]
fn it_works_for_log_activity_with_related_ids() {
    new_test_ext().execute_with(|| {
        System::set_block_number(5);
        
        // Create audit log entry with related proposal and wallet
        assert_ok!(AuditLogPallet::log_activity(
            RuntimeOrigin::signed(2),
            b"VOTE".to_vec(),
            b"User voted on proposal".to_vec(),
            Some(100),
            Some(42)
        ));
        
        // Verify entry exists and can be retrieved
        assert_ok!(AuditLogPallet::get_audit_entry(
            RuntimeOrigin::signed(1),
            0
        ));
        
        // Check that the event was emitted with correct data
        System::assert_last_event(
            Event::AuditEntryCreated {
                entry_id: 0,
                activity_type: BoundedVec::try_from(b"VOTE".to_vec()).unwrap(),
                actor: 2,
                details: BoundedVec::try_from(b"User voted on proposal".to_vec()).unwrap(),
            }
            .into(),
        );
    });
}

// Test unsigned origin fails for get_audit_entry
#[test]
fn get_audit_entry_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            AuditLogPallet::get_audit_entry(
                RuntimeOrigin::none(), // No signature
                0
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}