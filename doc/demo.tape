# VHS documentation
#
# Output:
#   Output <path>.gif               Create a GIF output at the given <path>
#   Output <path>.mp4               Create an MP4 output at the given <path>
#   Output <path>.webm              Create a WebM output at the given <path>
#
# Require:
#   Require <string>                Ensure a program is on the $PATH to proceed
#
# Settings:
#   Set FontSize <number>           Set the font size of the terminal
#   Set FontFamily <string>         Set the font family of the terminal
#   Set Height <number>             Set the height of the terminal
#   Set Width <number>              Set the width of the terminal
#   Set LetterSpacing <float>       Set the font letter spacing (tracking)
#   Set LineHeight <float>          Set the font line height
#   Set LoopOffset <float>%         Set the starting frame offset for the GIF loop
#   Set Theme <json|string>         Set the theme of the terminal
#   Set Padding <number>            Set the padding of the terminal
#   Set Framerate <number>          Set the framerate of the recording
#   Set PlaybackSpeed <float>       Set the playback speed of the recording
#   Set MarginFill <file|#000000>   Set the file or color the margin will be filled with.
#   Set Margin <number>             Set the size of the margin. Has no effect if MarginFill isn't set.
#   Set BorderRadius <number>       Set terminal border radius, in pixels.
#   Set WindowBar <string>          Set window bar type. (one of: Rings, RingsRight, Colorful, ColorfulRight)
#   Set WindowBarSize <number>      Set window bar size, in pixels. Default is 40.
#   Set TypingSpeed <time>          Set the typing speed of the terminal. Default is 50ms.
#
# Sleep:
#   Sleep <time>                    Sleep for a set amount of <time> in seconds
#
# Type:
#   Type[@<time>] "<characters>"    Type <characters> into the terminal with a
#                                   <time> delay between each character
#
# Keys:
#   Escape[@<time>] [number]        Press the Escape key
#   Backspace[@<time>] [number]     Press the Backspace key
#   Delete[@<time>] [number]        Press the Delete key
#   Insert[@<time>] [number]        Press the Insert key
#   Down[@<time>] [number]          Press the Down key
#   Enter[@<time>] [number]         Press the Enter key
#   Space[@<time>] [number]         Press the Space key
#   Tab[@<time>] [number]           Press the Tab key
#   Left[@<time>] [number]          Press the Left Arrow key
#   Right[@<time>] [number]         Press the Right Arrow key
#   Up[@<time>] [number]            Press the Up Arrow key
#   Down[@<time>] [number]          Press the Down Arrow key
#   PageUp[@<time>] [number]        Press the Page Up key
#   PageDown[@<time>] [number]      Press the Page Down key
#   Ctrl+<key>                      Press the Control key + <key> (e.g. Ctrl+C)
#
# Display:
#   Hide                            Hide the subsequent commands from the output
#   Show                            Show the subsequent commands in the output

Output demo.gif

Require echo

Set Shell "bash"

# This gives terminal 50 heigth and 100 wide
Set FontSize 12 
Set Width 900
Set Height 840

#Type "stty size" Sleep 100ms Enter

# Setup split screen, hide tmux bar with host name and simple prompt without user name
# zsh does not like comment lines and do not use history, so use sh
Hide
Type "tmux" Enter
Sleep 100ms
Type "tmux split-window -l 40" Sleep 100ms Enter
Type "/bin/sh" Enter
Type "export PS1='> '" Sleep 100ms Enter
Type "reset" Sleep 100ms Enter
Ctrl+B
Type ":set status off" Enter
Ctrl+B Type "o" Sleep 100ms
Type "/bin/sh" Enter
Type "export PS1=''" Sleep 100ms Enter
Type "reset" Sleep 100ms Enter

# Ready to show the windows
Show

# Currently we are in the top pane
Sleep 1s
Type "# run the demo" Sleep 100ms Enter

Ctrl+B Type "o" Sleep 100ms
Type "cargo run -q --example demo --features=termion,formatter" Enter

Sleep 2

Ctrl+B Type "o" Sleep 100ms
Type "# <States> at top are four tabs" Enter
Type "# In the middle is a smart widget with a target selector on the left" Enter
Type "# In the bottom left is a filtered tui logger widget" Enter
Type "# Bottom right is unfiltered tui logger widget with custom formatter ... MAYDAY" Enter
Type "# At the bottom is a progress task" Enter
Ctrl+B Type "o" Sleep 100ms

Sleep 1

Ctrl+B Type "o" Sleep 100ms
Type "# select the target <background-task2> with cursor down/up" Enter
Ctrl+B Type "o" Sleep 100ms

Sleep 5
Down Sleep 1
Down Sleep 1
Sleep 1 

Ctrl+B Type "o" Sleep 100ms
Type "# focus on this target with f" Enter
Ctrl+B Type "o" Sleep 100ms

Type "f"
Sleep 5

Ctrl+B Type "o" Sleep 100ms
Type "# focus on the previous target with Cursor up" Enter
Ctrl+B Type "o" Sleep 100ms

Up Sleep 1

Ctrl+B Type "o" Sleep 100ms
Type "# Remove focus with f" Enter
Ctrl+B Type "o" Sleep 100ms

Type "f"
Sleep 5

Ctrl+B Type "o" Sleep 100ms
Type "# For all targets only show Errors using Cursor Left/Down" Enter
Ctrl+B Type "o" Sleep 100ms

Up Sleep 100ms
Left Sleep 100ms
Left Sleep 100ms
Down Sleep 100ms
Left Sleep 100ms
Left Sleep 100ms
Down Sleep 100ms
Left Sleep 100ms
Left Sleep 100ms
Down Sleep 100ms
Left Sleep 100ms
Left Sleep 100ms
Down Sleep 100ms
Left Sleep 100ms
Left Sleep 100ms

Sleep 5

Ctrl+B Type "o" Sleep 100ms
Type "# Disable recording of target trace with -" Enter
Ctrl+B Type "o" Sleep 100ms

Type "-" Sleep 100ms
Up
Type "-" Sleep 100ms
Up
Type "-" Sleep 100ms
Up
Type "-" Sleep 100ms
Up
Type "-" Sleep 100ms


Ctrl+B Type "o" Sleep 100ms
Type "# Hide/Unhide the selector pane with h" Enter
Ctrl+B Type "o" Sleep 100ms

Type "h"
Sleep 3 
Type "h"
Sleep 3

Ctrl+B Type "o" Sleep 100ms
Type "# Switch to another Tab - independent Widget" Enter
Type "# Notice the disabled recording of target trace" Enter
Ctrl+B Type "o" Sleep 100ms

Tab
Sleep 5

Ctrl+B Type "o" Sleep 100ms
Type "# Quit with q" Enter
Ctrl+B Type "o" Sleep 100ms

Type "q"
Sleep 2