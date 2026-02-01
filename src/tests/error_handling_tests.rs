//! Error handling tests for Ralph CLI
//!
//! This module tests the error handling mechanisms to ensure all error types
//! are correctly caught, converted, and displayed.

use std::io;

// Import the types from error module
use crate::error::{RalphError, RalphResult};

/// Test that RalphError::Io correctly stores and displays IO errors
#[test]
fn test_io_error_conversion() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");

    // Convert to RalphError
    let ralph_err: RalphError = io_err.into();

    // Verify it's an Io variant
    match ralph_err {
        RalphError::Io(e) => {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
            assert!(e.to_string().contains("file not found"));
        }
        _ => panic!("Expected RalphError::Io variant"),
    }
}

/// Test that RalphError::Io displays correctly
#[test]
fn test_io_error_display() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let ralph_err: RalphError = io_err.into();

    let display = format!("{}", ralph_err);
    assert!(display.starts_with("IO error:"));
    assert!(display.contains("access denied"));
}

/// Test different IO error kinds conversion
#[test]
fn test_io_error_kinds() {
    let error_kinds = vec![
        io::ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::ConnectionRefused,
        io::ErrorKind::ConnectionReset,
        io::ErrorKind::ConnectionAborted,
        io::ErrorKind::NotConnected,
        io::ErrorKind::AddrInUse,
        io::ErrorKind::AddrNotAvailable,
        io::ErrorKind::BrokenPipe,
        io::ErrorKind::AlreadyExists,
        io::ErrorKind::WouldBlock,
        io::ErrorKind::InvalidInput,
        io::ErrorKind::InvalidData,
        io::ErrorKind::TimedOut,
        io::ErrorKind::WriteZero,
        io::ErrorKind::Interrupted,
        io::ErrorKind::Other,
        io::ErrorKind::UnexpectedEof,
        io::ErrorKind::OutOfMemory,
    ];

    for kind in error_kinds {
        let io_err = io::Error::new(kind, "test error");
        let ralph_err: RalphError = io_err.into();

        match ralph_err {
            RalphError::Io(e) => assert_eq!(e.kind(), kind),
            _ => panic!("Expected RalphError::Io variant for {:?}", kind),
        }
    }
}

/// Test that RalphError implements std::error::Error trait
#[test]
fn test_error_trait_implementation() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
    let ralph_err: RalphError = io_err.into();

    // Verify it implements Error trait
    let _: &dyn std::error::Error = &ralph_err;
}

/// Test that RalphError::Other correctly stores and displays custom messages
#[test]
fn test_other_error_creation() {
    let message = "Custom error message".to_string();
    let ralph_err = RalphError::Other(message.clone());

    match &ralph_err {
        RalphError::Other(s) => assert_eq!(s, &message),
        _ => panic!("Expected RalphError::Other variant"),
    }

    let display = format!("{}", ralph_err);
    assert_eq!(display, message);
}

/// Test RalphError::Other with various message types
#[test]
fn test_other_error_various_messages() {
    let test_messages: Vec<String> = vec![
        "Simple message".to_string(),
        "Message with special chars: !@#$%^&*()".to_string(),
        "Message with unicode: 你好世界 🎉".to_string(),
        "Very long message ".repeat(100),
        "".to_string(), // Empty message
        "Single".to_string(),
    ];

    for msg in test_messages {
        let ralph_err = RalphError::Other(msg.clone());
        let display = format!("{}", ralph_err);
        assert_eq!(display, msg);
    }
}

/// Test that RalphResult type alias works correctly with Ok values
#[test]
fn test_ralph_result_ok() {
    fn returns_ok() -> RalphResult<i32> {
        Ok(42)
    }

    let result = returns_ok();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

/// Test that RalphResult type alias works correctly with Err values
#[test]
fn test_ralph_result_err() {
    fn returns_err() -> RalphResult<i32> {
        Err(RalphError::Other("test error".to_string()))
    }

    let result = returns_err();
    assert!(result.is_err());

    match result {
        Err(RalphError::Other(s)) => assert_eq!(s, "test error"),
        _ => panic!("Expected Err variant"),
    }
}

/// Test RalphResult with IO error
#[test]
fn test_ralph_result_io_err() {
    fn returns_io_err() -> RalphResult<String> {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "missing file");
        Err(io_err.into())
    }

    let result = returns_io_err();
    assert!(result.is_err());

    match result {
        Err(RalphError::Io(e)) => {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
        }
        _ => panic!("Expected RalphError::Io variant"),
    }
}

/// Test file not found error handling
#[test]
fn test_file_not_found_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "config.toml not found");
    let ralph_err: RalphError = io_err.into();

    let display = format!("{}", ralph_err);
    assert!(display.contains("IO error:"));
    assert!(display.contains("config.toml not found"));

    match ralph_err {
        RalphError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        _ => panic!("Expected Io error"),
    }
}

/// Test permission denied error handling
#[test]
fn test_permission_denied_error() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "cannot write to directory");
    let ralph_err: RalphError = io_err.into();

    let display = format!("{}", ralph_err);
    assert!(display.contains("IO error:"));
    assert!(display.contains("cannot write to directory"));

    match ralph_err {
        RalphError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::PermissionDenied),
        _ => panic!("Expected Io error"),
    }
}

/// Test error chaining with source
#[test]
fn test_error_source() {
    let io_err = io::Error::other("underlying io error");
    let ralph_err = RalphError::Io(io_err);

    // RalphError doesn't implement source() to return the underlying error
    // but we can verify the error trait is implemented
    let err_ref: &dyn std::error::Error = &ralph_err;
    assert!(err_ref.source().is_none()); // Our implementation doesn't chain
}

// Note: Debug trait tests removed - they test derive macro functionality

/// Test error conversion in a real-world scenario
#[test]
fn test_error_conversion_in_function() {
    fn may_fail(succeed: bool) -> RalphResult<String> {
        if succeed {
            Ok("success".to_string())
        } else {
            let io_err = io::Error::other("operation failed");
            Err(io_err.into())
        }
    }

    // Test success case
    let result = may_fail(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");

    // Test failure case
    let result = may_fail(false);
    assert!(result.is_err());
    match result {
        Err(RalphError::Io(e)) => {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        }
        _ => panic!("Expected Io error"),
    }
}

/// Test the ? operator with RalphResult
#[test]
fn test_question_mark_operator() {
    fn inner_operation() -> io::Result<String> {
        Err(io::Error::new(io::ErrorKind::NotFound, "inner error"))
    }

    fn outer_operation() -> RalphResult<String> {
        let result = inner_operation()?;
        Ok(result)
    }

    let result = outer_operation();
    assert!(result.is_err());

    match result {
        Err(RalphError::Io(e)) => {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
            assert!(e.to_string().contains("inner error"));
        }
        _ => panic!("Expected Io error from ? operator"),
    }
}

/// Test error message formatting with special characters
#[test]
fn test_error_special_characters() {
    let special_messages = vec![
        "Error: file/path\nwith\nnewlines",
        "Error: tab\there",
        "Error: \"quoted\" text",
        "Error: 'single' quotes",
        "Error: backslash \\ path",
    ];

    for msg in special_messages {
        let ralph_err = RalphError::Other(msg.to_string());
        let display = format!("{}", ralph_err);
        assert_eq!(display, msg);
    }
}

// Note: Thread safety and boundary tests removed - they test derive macros and edge cases

/// Test that io::ErrorKind is properly preserved through conversion
#[test]
fn test_error_kind_preservation() {
    let test_cases = vec![
        (io::ErrorKind::NotFound, "file not found"),
        (io::ErrorKind::PermissionDenied, "permission denied"),
        (io::ErrorKind::AlreadyExists, "already exists"),
        (io::ErrorKind::InvalidInput, "invalid input"),
    ];

    for (kind, msg) in test_cases {
        let io_err = io::Error::new(kind, msg);
        let ralph_err: RalphError = io_err.into();

        match ralph_err {
            RalphError::Io(e) => {
                assert_eq!(e.kind(), kind, "ErrorKind not preserved for {:?}", kind);
            }
            _ => panic!("Expected Io variant for {:?}", kind),
        }
    }
}

/// Test error downcasting (if we were to implement it)
#[test]
fn test_error_trait_object() {
    let io_err = io::Error::other("test");
    let ralph_err: RalphError = io_err.into();

    // As a trait object
    let err: Box<dyn std::error::Error> = Box::new(ralph_err);
    let display = format!("{}", err);
    assert!(display.contains("IO error:"));
}
