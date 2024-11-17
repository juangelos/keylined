# Keylined

## Product name

The name of the product shall be "Keylined".

## Product description

A cool little app written in Rust that gives you an instant input text box and an open microphone to be able to execute commands to use your computer.

### Unnegotiable priorities

- It needs to be instant and very, very fast: show the command line instantaneously when you press a key or invoke with voice
- It should rely on existing, FAST native mechanisms and APIs of the operating system to run commands
- It should be extensible by using operating system shell mechanisms, eg. add commands by having scripts, built-in commands, pipes, etc.

## Use cases

The whole idea is to be able to command the whole computer with just a key and writing a short command, so that anything
the computer can do or execute is just 2 or 3 keystrokes away. The input box will have auto-completion, so all you
should need is the key to open the command line (instantaneous) and then you can type the command's first letters and
press Enter.

## Features

- one particular feature will be to have a set of folders that the tool will scan and continuously watch for executable commands (eg. .sh, .ps1, .cmd, .py, .csx, etc.), and will add them to the autocomplete list, and be able to execute them with their proper shell or interpreter ("handler") (eg. .sh will be executed with bash, .py will be executed with python, etc.). We should find a convention for the tool to be able to read from the scripts a short description of the command, so that the tool can show it in the autocomplete list as a tooltip.
- if a command has parameters, they should be able to be typed in the input box and the tool will know how to pass them to the appropriate handler to execute the command with the parameters.

## Target audience

The main target audience is intermediate users of the computer, who want to streamline their work and be able to use the computer with speed and efficiency. For this audience we should have some sort of online repository of commands that they can search and import. A secondary audience is computer experts who want to be able to use the computer with speed and efficiency, and are able to write their own commands and scripts to share with the community. I should be easy for them to share their commands and scripts with the community.

## Technology and architecture

The code will be written in Rust to be as lighting fast as possible.
A library that is very fast and flexible to accommodate the possible design configuration should be chosen. For example, it should allow to configure the font, colors, etc., ideally by specifying with CSS styles. (settings will be "line-font-family", "line-font-size", "line-font-weight", "line-line-height", etc.) but it should by no means use any kind of inline browser or webview because those are extremely slow compared with the velocity we want (almost instantaneous).
A library needs to be chosen for voice-recognition that doesn't require much compute to detect a "call sign" like "hey KeyLine", after which full voice-recognition can be enabled to listen to the user at length.

## UX

The UX is entirely simple: a horizontal rectangle ("box") that contains a simple text input with initially just one line. The principal and main goal of the UI is that it is blazingly fast. The initial height of the box is the font line height (configurable, default 1em) plus any configurable line spacing (default 0). The default is a big font-size (default 36), and so that it shows prominently in the screen. All the aspects of the design are configurable. Once invoked with a global keyboard shortcut, is always visible on the screen, and that is always in focus, until the user presses Enter, and then it hides instantly again unless there's an error related to the input, in which case, by default the border can blink in red for 2 seconds (behavior a), or it can be configured to (behavior B) change to a custom color and stay in it until the user resumes editing by pressing any key, and then the command executes. A tray icon is also displayed, and clicking on it brings up the command line again.

When the command line is visible, the user can type commands, and the text input is automatically focused, so that the user can type commands without having to click on it. While the user is typing, the current text is evaluated to check if it matches one of the possible commands. Once it matches and the syntax is valid, the text changes to a lighter color to indicate so. The box can expand vertically downwards adding the height corresponding to another font line height when the command the user is typing is too long to fit on the box, or when the user explicitly presses a configurable key (default Alt-Enter) to expand it. The box can have a minimal border, and the user can drag the box around the screen.

The text editing experience can have two modes (configurable: "editMode" setting): Emacs (for linux users) or Windows (the default), which should be familiar to a Windows user (the user can type commands, and the text input is automatically focused, so that the user can type commands without having to click on it. The user can also use the arrow keys to navigate through the text, and the backspace key to delete characters. Control Arrow Keys jump from word to word. The user can also use the mouse to select text, or Shift key to select text, etc.). Some inspiration can be gathered from the PSReadLine module of PowerShell.

## Configuration

The configuration system uses TOML files for both commands and settings, located in the program's "config" directory.

### Command Configuration

Commands are defined in individual TOML files within the "config/commands" directory. Each command file contains:

```toml
name = "command-name"           # Command identifier (lowercase letters, numbers, hyphens)
description = "Command desc"    # Brief description of what the command does
executable = "program.exe"      # Program to execute
icon = "📝"                     # Emoji icon to represent the command

[permissions]
elevation = "user"             # Required privileges: "user", "admin", or "system"
requires_confirmation = false  # Whether to prompt for confirmation

[validation]
timeout = 30000               # Maximum execution time in milliseconds
```

### Settings Configuration

Application settings are defined in "config/settings.toml", controlling UI appearance and behavior:

```toml
[ui]
font_size = 36                # Default font size in pixels
font_family = "Segoe UI"      # Font family to use
font_weight = "normal"        # Font weight: "normal" or "bold"
line_height = 1.2            # Line height multiplier
border_color = "#000000"     # Border color in hex
background_color = "#FFFFFF" # Background color in hex
text_color = "#000000"      # Text color in hex

[behavior]
edit_mode = "windows"        # Text editing mode: "windows" or "emacs"
error_display = "blink"      # Error display: "blink" or "stay"
error_timeout = 2000        # Error display duration in milliseconds
auto_hide = true            # Hide after command execution

[shortcuts]
activate = "Alt+Space"      # Global shortcut to show command line
expand = "Alt+Enter"        # Shortcut to expand input box

[voice]
wake_word = "hey keyline"   # Wake word for voice activation
recognition_mode = "cloud"  # Voice recognition mode: "cloud", "local", "hybrid"
sensitivity = 0.5          # Wake word detection sensitivity
```

Both command and settings files are validated against schemas to ensure correct configuration. The system supports hot-reloading of configurations, allowing changes to take effect without restarting the application.
