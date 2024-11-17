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
   - JSON-based configuration files
   - Separate configs for commands and settings
   - Located in `config/` directory
   - Hot-reloading capability

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
     - Audit logging to Windows Event Log
     - Configuration change detection
   - Execution isolation:
     - Separate desktop for elevated commands
     - Process job object containment
     - Network access controls
     - File system restrictions
   - Permission management:
     - UAC integration
     - Privilege elevation prompts
     - Token filtering

4. **Performance Requirements**
   - Instant UI display (<100ms) on activation
   - Native OS command execution
   - Minimal memory footprint
   - Efficient configuration loading

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
