#!/bin/sh
rm -f *.cast

echo "set 100x50 in console"
echo "set PS1 to > in .profile" 
echo "do not use e.g. xterm"
sleep 5

tmux new-session -d -t tui

(
tmux split-window -l 40
sleep 1
tmux send-keys -t tui.0 "# run the demo" Enter
sleep 5
tmux send-keys -t tui.1 "cargo run --example demo"
sleep 1
tmux send-keys -t tui.1 Enter

while [ `ps ax|grep -v grep|grep -c demo` -eq 0 ]
do
	sleep 1
done

tmux send-keys -t tui.0 "# select the target <debug> with cursor down/up" Enter
sleep 5
tmux send-keys -t tui.1 Down
sleep 1
tmux send-keys -t tui.1 Down
sleep 1
tmux send-keys -t tui.1 Down
sleep 1
tmux send-keys -t tui.1 Up

tmux send-keys -t tui.0 "# focus on this target <debug> with f" Enter
sleep 5
tmux send-keys -t tui.1 f
sleep 1

tmux send-keys -t tui.0 "# focus on the previous target with Cursor up" Enter
sleep 5
tmux send-keys -t tui.1 Up
sleep 1

tmux send-keys -t tui.0 "# Remove focus with f" Enter
sleep 5
tmux send-keys -t tui.1 f
sleep 1

tmux send-keys -t tui.0 "# For all targets only show Errors using Cursor Left/Down" Enter
sleep 5
tmux send-keys -t tui.1 Up Up Left
sleep 1
tmux send-keys -t tui.1 Left
sleep 1
tmux send-keys -t tui.1 Left
sleep 1
tmux send-keys -t tui.1 Left
sleep 1
tmux send-keys -t tui.1 Down Left Left Left Left 
sleep 1
tmux send-keys -t tui.1 Down Left Left Left Left 
sleep 1
tmux send-keys -t tui.1 Down Left Left Left Left 
tmux send-keys -t tui.1 Down Left Left Left Left 
tmux send-keys -t tui.1 Down Left Left Left Left 
tmux send-keys -t tui.1 Down Left Left Left Left 
sleep 1

tmux send-keys -t tui.0 "# Disable recoring of target trace with -" Enter
sleep 5
tmux send-keys -t tui.1 Up
sleep 1
tmux send-keys -t tui.1 "-"
sleep 1
tmux send-keys -t tui.1 "-"
sleep 1
tmux send-keys -t tui.1 "-"
sleep 1
tmux send-keys -t tui.1 "-"
sleep 1

tmux send-keys -t tui.0 "# Hide the selector pane with h" Enter
sleep 5
tmux send-keys -t tui.1 h
sleep 1

tmux send-keys -t tui.0 "# Unhide the selector pane with h" Enter
sleep 5
tmux send-keys -t tui.1 h
sleep 1

tmux send-keys -t tui.0 "# Switch to another Tab - independent Widget" Enter
tmux send-keys -t tui.0 "# Notice the disabled recording of target trace" Enter
sleep 5
tmux send-keys -t tui.1 Tab
sleep 1


tmux send-keys -t tui.0 "# Quit with q" Enter
sleep 5
tmux send-keys -t tui.1 q

while [ `ps ax|grep -v grep|grep -c demo` -eq 1 ]
do
	sleep 1
done
tmux send-keys -t tui.1 exit Enter
tmux send-keys -t tui.0 exit Enter
)&


asciinema rec --command "tmux attach -t tui" tmux-$(date +%F--%H%M).cast

echo upload to https://dstein64.github.io/gifcast/
echo size small
echo font dejavu-sans-mono
# docker run --rm -v $PWD:/data asciinema/asciicast2gif -s 1 tmux*cast demo.gif
