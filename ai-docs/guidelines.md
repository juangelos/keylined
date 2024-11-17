# Rust Project Guidelines

## Document Version
- Version: 1.0
- Date: 2024-11-17

## Purpose
This document serves as the central reference for all architectural decisions, coding standards, and best practices for our Rust project.

## Table of Contents
1. [Architecture Decisions](#architecture-decisions)
2. [Code Style & Standards](#code-style--standards)
3. [Project Structure](#project-structure)
4. [Testing Strategy](#testing-strategy)
5. [Documentation Requirements](#documentation-requirements)

## Architecture Decisions

### Core Architecture Principles
1. **Performance First**
   - Instant command-line display on activation (<100ms)
   - Windows-native mechanisms for command execution
   - No web-based technologies (no webview/browser components)
   - Minimal resource usage for voice recognition
   - Windows-optimized UI rendering

2. **UI Architecture**
   - Native GUI implementation
   - Single-line command input as primary interface
   - Configurable via JSON settings
   - Tray icon integration

3. **Command Processing & Security**
   - Real-time command validation
   - Extensible command system using OS shell mechanisms
   - Support for built-in commands, scripts, and pipes
   - Security model:
     - Command validation and sanitization
     - Permission levels (user, admin, system)
     - Execution isolation via job objects
     - Process resource limits
     - Command audit logging
     - Blocked command patterns
     - Shell injection prevention
   - Execution contexts:
     - Restricted: Limited to safe built-in commands
     - Standard: Regular user permissions
     - Elevated: Administrative commands
   - Security logging:
     - Command execution audit trail
     - Permission elevation events
     - Configuration changes
     - Failed command attempts

4. **Configuration Architecture**
   - TOML-based configuration files with schemas
   - Separate configs for commands and settings
   - Located in `config/` directory
   - Hot-reloading capability
   - Schema validation on load

### Technology Choices
1. **GUI Framework Requirements**
   - Use `iced` with native Windows backend for optimal performance
   - Styling through `iced`'s theme system:
     - Custom fonts via DirectWrite
     - Color schemes and spacing
     - Responsive layout engine
   - Native window management:
     - Borderless window implementation
     - Custom titlebar and dragging
     - Global hotkey registration
   - Windows shell integration via native APIs
   - Hardware-accelerated rendering using Direct2D/DirectWrite
   - Text input with rich editing capabilities:
     - Windows/Emacs keybinding modes
     - Selection and navigation
     - Multi-line support with dynamic height
   - System tray integration
   - Real-time visual feedback for command validation

2. **Voice Recognition Requirements**
   - Dual-mode voice recognition system:
     a. Pre-activation Mode:
        - Lightweight local wake word detection ("Hey KeyLine")
        - Minimal CPU/memory footprint (<2% CPU, <50MB RAM)
        - Uses Whisper.cpp or similar local lightweight model
        - Always running in background
     b. Command Mode:
        - Cloud-based full speech recognition
        - Activated via microphone button or wake word
        - Support for major cloud providers (Azure, AWS, GCP)
        - Fallback to local processing when offline
   - Error handling and feedback for both modes
   - Configurable activation phrases
   - Audio input device selection

3. **Command Processing Requirements**
   - Real-time command validation
   - Support for:
     - Built-in commands
     - Shell scripts
     - Command piping
   - Error handling with visual feedback
   - Hot-reloading of command definitions
   - Security requirements:
     - Command whitelist/blacklist system
     - Regular expression pattern matching for validation
     - Windows security token handling
     - Process creation with restricted tokens
     - Job object creation and management
     - Resource quota enforcement
     - Audit logging to structured log files
     - Log rotation and retention policies
     - Configuration change detection
   - Execution isolation:
     - Process sandboxing using platform-appropriate mechanisms
     - Network access controls via configuration
     - File system access restrictions
     - Resource usage limits (CPU, memory, file handles)
   - Permission management:
     - UAC integration
     - Privilege elevation prompts
     - Token filtering

4. **Audit Logging Requirements**
   - JSON-structured log files with:
     - Timestamp (UTC)
     - Command executed
     - User context
     - Permission level
     - Execution result
     - Resource usage
   - Log management:
     - Daily rotation
     - Compression of old logs
     - Configurable retention period
     - Size-based rotation
   - Log levels:
     - ERROR: Command failures
     - WARN: Permission elevation
     - INFO: Successful commands
     - DEBUG: Configuration changes
   - Optional log encryption
   - Log file location configurable
   
   Example log entry format:
   ```json
   {
     "timestamp": "2024-11-17T10:15:30.123Z",
     "level": "INFO",
     "event": "command_execution",
     "data": {
       "command": "notepad.exe",
       "user": "current_user",
       "permission_level": "standard",
       "execution_context": "shell",
       "process_id": 1234,
       "resource_usage": {
         "cpu_time_ms": 100,
         "peak_memory_kb": 1024
       }
     },
     "result": {
       "status": "success",
       "exit_code": 0
     }
   }
   ```

   Log file naming convention:
   - Active log: keyline-audit-YYYY-MM-DD.log
   - Compressed archives: keyline-audit-YYYY-MM-DD.log.gz
   - Location: %LOCALAPPDATA%/Keyline/logs/ (Windows)

5. **Performance Requirements**
   - Instant UI display (<100ms) on activation
   - Native OS command execution
   - Minimal memory footprint
   - Efficient configuration loading

## Getting Started Guide for .NET Developers

### Key Concepts for C# Developers

1. **Ownership & Borrowing** (vs Garbage Collection)
   ```csharp
   // C# - GC handles cleanup
   public void ProcessList(List<string> items) {
       var copy = items;  // Reference copy
       // Both variables can access the list
   }
   ```
   ```rust
   // Rust - Only one owner at a time
   fn process_list(items: Vec<String>) {
       let copy = &items;  // Borrowing reference
       // Original still owns the data
   } // items is dropped here
   ```

2. **Pattern Matching** (similar to C# switch expressions)
   ```csharp
   // C# pattern matching
   string GetMessage(object obj) => obj switch {
       string s => $"String: {s}",
       int i => $"Int: {i}",
       _ => "Unknown"
   };
   ```
   ```rust
   // Rust pattern matching
   fn get_message(obj: &Value) -> String {
       match obj {
           Value::String(s) => format!("String: {}", s),
           Value::Number(n) => format!("Int: {}", n),
           _ => "Unknown".to_string()
       }
   }
   ```

3. **Traits** (like C# interfaces)
   ```csharp
   // C# interface
   public interface ILogger {
       void Log(string message);
   }
   ```
   ```rust
   // Rust trait
   trait Logger {
       fn log(&self, message: &str);
   }
   ```

4. **Option<T>** (vs C# nullable types)
   ```csharp
   // C# nullable
   string? name = GetName();
   if (name != null) {
       Console.WriteLine(name);
   }
   ```
   ```rust
   // Rust Option
   let name: Option<String> = get_name();
   if let Some(name) = name {
       println!("{}", name);
   }
   ```

5. **Memory Safety**
   ```csharp
   // C# - possible null reference
   public void UseArray(int[] arr) {
       arr[0] = 42;  // Could throw NullReferenceException
   }
   ```
   ```rust
   // Rust - compiler prevents null references
   fn use_array(arr: &mut [i32]) {
       arr[0] = 42;  // Guaranteed to be safe if it compiles
   }
   ```

### Common Patterns

1. **Builder Pattern** (similar in both languages)
   ```rust
   // Rust builder
   let command = Command::new("notepad")
       .arg("file.txt")
       .current_dir("C:/temp")
       .spawn()?;
   ```

2. **Error Handling** (vs exceptions)
   ```rust
   // Rust prefers Result over exceptions
   fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
       if b == 0 {
           return Err("division by zero");
       }
       Ok(a / b)
   }
   ```

3. **Async/Await** (similar syntax, different runtime)
   ```rust
   // Rust async
   async fn fetch_data() -> Result<String, Error> {
       let response = client.get("https://api.example.com")
           .send()
           .await?;
       Ok(response.text().await?)
   }
   ```

### Windows Development & Debugging Tips
1. **IDE Setup**
   - Use VS Code with rust-analyzer extension
   - Install CodeLLDB or Microsoft C++ extensions
   - Configure tasks.json for build/test commands
   - Enable "Developer Mode" in Windows Settings

2. **Windows-Specific Debugging**
   - Use Windows Event Viewer for system-level issues
   - Enable Debug logging: `$env:RUST_LOG="debug"`
   - Use Process Monitor for file/registry access
   - Debug privilege elevation with:
     ```rust
     #[cfg(windows)]
     fn check_admin() -> bool {
         use windows_sys::Win32::Security::Authorization::IsUserAnAdmin;
         unsafe { IsUserAnAdmin() != 0 }
     }
     ```

3. **Common Windows Issues**
   - Path issues: Use `std::path::PathBuf` for Windows paths
   - Handle UTF-16 strings for WinAPI calls
   - Check file permissions with Process Monitor
   - Debug COM components with OleView

4. **Performance Profiling**
   - Use Windows Performance Recorder (WPR)
   - ETW tracing for system events
   - Process Explorer for real-time monitoring
   - Sample commands for our project:
     ```powershell
     # CPU profiling
     wpr -start CPU
     # run your test scenario
     wpr -stop cpu_profile.etl
     ```

5. **Project-Specific Tools**
   - Debug tray icon: Use Spy++ to monitor messages
   - Voice recognition: Windows Sound settings
   - Command execution: Process Monitor filters
     ```
     Process Name is keyline.exe
     Operation is Process Create
     ```

### Common Gotchas
1. No null values - use Option<T> instead
2. No inheritance - use traits for shared behavior
3. No exceptions - use Result<T,E> for error handling
4. No implicit type conversion
5. Strict mutability rules

## Error Handling Patterns

### Rust vs C# Error Handling
- Instead of C#'s try/catch blocks, Rust uses Result<T,E>
- Example comparison:

```csharp
// C# exception handling
try {
    var file = File.OpenRead("config.json");
    // ... use file
} catch (FileNotFoundException ex) {
    logger.LogError(ex);
    return null;
}
```

```rust
// Rust Result handling
let file = match File::open("config.json") {
    Ok(file) => file,
    Err(e) => {
        log::error!("{}", e);
        return None;
    }
};
```

### Project Error Handling Guidelines
1. **Custom Error Types**
   ```rust
   pub enum KeylineError {
       ConfigError(String),
       CommandError { cmd: String, details: String },
       LoggingError(std::io::Error),
   }
   ```

2. **Error Propagation**
   - Use the `?` operator (similar to C#'s await)
   - Chain Results with map_err() for context
   - Avoid unwrap() in production code

3. **Error Logging Strategy**
   - Log errors at their source
   - Include context and stack information
   - Use appropriate log levels

4. **User-Facing Errors**
   - Convert internal errors to user-friendly messages
   - Localize error messages
   - Include actionable information

## Code Style & Standards
- Follow the official Rust style guide
- Use `rustfmt` for consistent formatting
- Use `clippy` for linting
- Document all public APIs

## Project Structure
```
src/
├── main.rs              # Application entry point
├── ui/                  # UI components
│   ├── command_line.rs  # Main command line widget
│   ├── theme.rs         # Iced theme customization
│   ├── styles.rs        # Component-specific styles
│   └── tray.rs         # System tray implementation
├── commands/           # Command processing
│   ├── parser.rs      # Command parsing and validation
│   └── executor.rs    # Command execution
├── config/            # Configuration handling
│   ├── settings.rs    # Settings management
│   └── commands.rs    # Command definitions
├── logging/           # Audit logging system
│   ├── audit.rs       # Audit log implementation
│   ├── rotation.rs    # Log rotation and management
│   └── format.rs      # Log formatting and schemas
└── voice/             # Voice recognition
    ├── wake_word.rs   # Local wake word detection
    ├── cloud.rs      # Cloud speech recognition
    ├── offline.rs    # Offline fallback recognition
    └── common.rs     # Shared voice recognition types

config/
├── settings.json      # UI and behavior settings
└── commands.json      # Command definitions
```

## Testing Strategy
- Unit tests alongside implementation files
- Integration tests in separate tests directory
- Documentation tests for public APIs

## Documentation Requirements
- All public APIs must have documentation comments
- Update CHANGELOG.md for significant changes
- Maintain architecture decisions in this document
