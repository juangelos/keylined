use keyline_core::prelude::*;

#[test]
fn test_error_conversion() {
    // Test converting IO errors to our Error type
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test error");
    let our_err: Error = io_err.into();
    
    match our_err {
        Error::Io(_) => (),
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_result_propagation() {
    fn inner_function() -> Result<()> {
        Err(Error::Config("test config error".into()))
    }

    let err = inner_function().unwrap_err();
    match err {
        Error::Config(_) => (),
        _ => panic!("Expected Config error variant"),
    }
}
