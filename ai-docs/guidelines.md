# Rust Project Guidelines

## Document Version

- Version: 1.0
- Date: 2024-11-17

## Purpose

This document serves as the central reference for all architectural decisions, coding standards, and best practices for our Rust project.

## Table of Contents

- [Rust Project Guidelines](#rust-project-guidelines)
  - [Document Version](#document-version)
  - [Purpose](#purpose)
  - [Table of Contents](#table-of-contents)
  - [Architecture Decisions](#architecture-decisions)
    - [Core Architecture Principles](#core-architecture-principles)
    - [Technology Choices](#technology-choices)
  - [Error Handling Patterns](#error-handling-patterns)
    - [Rust vs C# Error Handling](#rust-vs-c-error-handling)
    - [Project Error Handling Guidelines](#project-error-handling-guidelines)
  - [Code Style \& Standards](#code-style--standards)
  - [Project Structure](#project-structure)
  - [Testing Strategy](#testing-strategy)
  - [Documentation Requirements](#documentation-requirements)

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
   - Configurable via TOML settings
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

```text
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
