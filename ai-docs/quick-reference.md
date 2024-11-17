# Keyline Quick Reference Card

## Common Development Tasks

### 1. Build & Run
```powershell
cargo build --release    # Optimized build
cargo run               # Development run
cargo test             # Run all tests
```

### 2. Debug Commands
```powershell
# Enable debug logging
$env:RUST_LOG="debug"
cargo run

# Run with performance tracing
cargo build --release
wpr -start CPU
.\target\release\keyline.exe
wpr -stop cpu_profile.etl
```

### 3. Common Code Patterns

#### Command Execution
```rust
// Execute a shell command
use std::process::Command;

fn execute_command(cmd: &str) -> Result<(), KeylineError> {
    Command::new("cmd")
        .args(["/C", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()?;
    Ok(())
}
```

#### UI Updates
```rust
// Update command line state
self.state.update(|state| {
    state.command_text = new_text;
    state.is_valid = validate_command(&new_text);
});
```

#### Error Handling
```rust
// Project standard error handling
fn process_command(cmd: &str) -> Result<(), KeylineError> {
    let result = execute_command(cmd)
        .map_err(|e| KeylineError::CommandError {
            cmd: cmd.to_string(),
            details: e.to_string(),
        })?;
    
    audit_log::log_command(cmd, &result)?;
    Ok(())
}
```

### 4. Configuration
```json
// config/settings.json
{
    "ui": {
        "font_size": 36,
        "font_family": "Segoe UI",
        "line_height": 1.2
    }
}
```

### 5. Debugging Checklist
- [ ] Check Windows Event Viewer
- [ ] Verify process privileges
- [ ] Monitor file access with ProcMon
- [ ] Review audit logs
- [ ] Check voice recognition status

### 6. Performance Optimization
- Use `cargo flamegraph` for profiling
- Enable release optimizations
- Monitor memory with Process Explorer
- Check Windows Performance Analyzer

### 7. Common Issues & Solutions
1. **Slow Startup**
   - Check initialization logging
   - Profile startup sequence
   - Verify file access patterns

2. **Voice Recognition**
   - Check microphone permissions
   - Verify audio device selection
   - Monitor CPU usage

3. **Command Execution**
   - Check process privileges
   - Verify PATH environment
   - Monitor process creation

### 8. Useful Links
- [Project Documentation](./guidelines.md)
- [Rust Windows Guidelines](https://docs.microsoft.com/en-us/windows/dev-environment/rust/)
- [Windows API Documentation](https://docs.microsoft.com/en-us/windows/win32/)
