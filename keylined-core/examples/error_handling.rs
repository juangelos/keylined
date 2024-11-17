use keyline_core::prelude::*;

fn main() -> Result<()> {
    // Example of handling different error types
    let result = do_something_fallible()?;
    println!("Operation succeeded: {}", result);
    
    // Example of error conversion
    if let Err(e) = do_something_with_io() {
        println!("Operation failed: {}", e);
    }

    Ok(())
}

fn do_something_fallible() -> Result<String> {
    // Example of creating and returning our custom error
    if false {
        return Err(Error::Config("Something went wrong".into()));
    }
    Ok("Success!".into())
}

fn do_something_with_io() -> Result<()> {
    // Example of converting std::io::Error to our Error type
    std::fs::read_to_string("nonexistent.txt")?;
    Ok(())
}
