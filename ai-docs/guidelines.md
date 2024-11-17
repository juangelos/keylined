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
   - Instant command-line display on activation
   - Native OS mechanisms for command execution
   - No web-based technologies (no webview/browser components)
   - Minimal resource usage for voice recognition

2. **UI Architecture**
   - Native GUI implementation
   - Single-line command input as primary interface
   - Configurable via JSON settings
   - Tray icon integration

3. **Command Processing**
   - Real-time command validation
   - Extensible command system using OS shell mechanisms
   - Support for built-in commands, scripts, and pipes

4. **Configuration Architecture**
   - JSON-based configuration files
   - Separate configs for commands and settings
   - Located in `config/` directory
   - Hot-reloading capability

### Technology Choices
1. **GUI Framework Requirements**
   - Native performance
   - CSS-like styling capability (for fonts, colors, spacing)
   - Custom window management (borderless window, draggable)
   - Global keyboard shortcuts
   - Text input with rich editing capabilities:
     - Windows/Emacs keybinding modes
     - Selection and navigation
     - Multi-line support with dynamic height
   - System tray integration
   - Real-time visual feedback for command validation

2. **Voice Recognition Requirements**
   - Low-compute wake word detection
   - Two-phase voice recognition:
     a. Lightweight wake word detection ("hey KeyLine")
     b. Full command recognition
   - Minimal latency and resource usage

3. **Command Processing Requirements**
   - Real-time command validation
   - Support for:
     - Built-in commands
     - Shell scripts
     - Command piping
   - Error handling with visual feedback
   - Hot-reloading of command definitions

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
│   └── tray.rs         # System tray implementation
├── commands/           # Command processing
│   ├── parser.rs      # Command parsing and validation
│   └── executor.rs    # Command execution
├── config/            # Configuration handling
│   ├── settings.rs    # Settings management
│   └── commands.rs    # Command definitions
└── voice/             # Voice recognition
    ├── wake_word.rs   # Wake word detection
    └── recognition.rs # Full speech recognition

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
