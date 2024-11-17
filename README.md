# Keylined

## Product description
An cool little app written in Rust that gives you an instant command line and an open microphone to to be able to execute commands to use your computer.

Unnegotiable priorities:
- it needs to be instant and very, very fast: show the command line instantaneously when you press a key or invoke with voice
- it should rely on existing, FAST native mechanisms and APIs of the operating system to run commands
- it should be extensible by using operating system shell mechanisms, eg. add commands by having scripts, built-in commands, pipes, etc.



## Technology and architecture:
The code will be written in Rust to be as lighting fast as possible.
A library that is very fast and flexible to accomodate the possible design configuration should be chosen. For example, it should allow to configure the font, colors, etc., ideally by specifying with CSS styles. (settings will be "line-font-family", "line-font-size", "line-font-weight", "line-line-height", etc.) but it should by no means use any kind of inline browser or webview because those are extremely slow compared with the velocity we want (almost instantaneous).
A library needs to be chosen for voice-recognition that doesnt require much compute to detect a "call sign" like "hey KeyLine", after which full voice-recognition can be enabled to listen to the user at length.

## UX
The UX is entirely simple: an horizontal rectangle ("box") that contains a simple text input with initially just one line. The principal and main goal of the UI is that it is blazingly fast. The initial height of the box is the the font line height (configurable, default 1em) plus any configurable line spacing (default 0). The default a big font-size (default 36), and so that it shows prominently in the screen. All the aspects of the design are configurable. Once invoked with a global keyboard shortcut, is always visible on the screen, and that is always in focus, until the user presses Enter, and then it hides instantly again unless there's an error related to the input, in which case, by default the border can blink in read for 2 seconds (behavior a), or it can be configured to (behavior B) change to a custom color and stay in it until the user resumes editing by pressing any key, and then the command executes. A tray icon is also displayed, and clicking on it brings up the command line again. 

When the command line is visible, the user can type commands, and the text input is automatically focused, so that the user can type commands without having to click on it. While the user is typing, the current text is evaluated to check if it matches one of the possible commands. Once it matches and the syntax is valid, the text changes to a lighter color to indicate so. The box can expand vertically downwards adding the height corresponding to another font line height when the command the user is typing is too long to fit on the box, or when the user explicitely presses a configurable key (default Alt-Enter) to expand it. The box can have a minimal border, and the user can drag the box around the screen.

The text editing experience can have two modes (configurable: "editMode" setting): Emacs (for linux users) or Windows (the default), which should be familiar to a Windows user (the user can type commands, and the text input is automatically focused, so that the user can type commands without having to click on it. The user can also use the arrow keys to navigate through the text, and the backspace key to delete characters. Control Arrow Keys jump from word to word. The user can also use the mouse to select text, or Shift key to select text, etc.), . Some inspiration can be gathered from the PSReadLine module of PowerShell.


## Configuration:
The configuration of some commands is done by editing a configuration file. The configuration file is a JSON file that contains a list of commands. Each command is a JSON object that contains a name, a description, a command, and an optional icon. The name is the name of the command, the description is the description of the command, the command is the command to execute, and the icon is the icon to display for the command. The icon is a unicode character. The configuration file is located in the program directory, subfolder "config", in a file called commands.json

The configuration of the settings (like the design, colors, fonts, etc.) is done by editing a configuration file. The configuration file is a JSON file that contains a list of settings. Each setting is a JSON object that contains a name, a description, and a value. The name is the name of the setting, the description is the description of the setting, and the value is the value of the setting. The configuration file is located in the program directory, subfolder "config", in a file called settings.json