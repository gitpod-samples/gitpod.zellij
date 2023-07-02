#!/usr/bin/env bash

sudo bash <<'SCRIPT'
# Install latest static zellij
curl -L "https://github.com/zellij-org/zellij/releases/latest/download/zellij-x86_64-unknown-linux-musl.tar.gz" |
  tar -C /usr/bin -xzf - zellij

# Install gitpod.zellij plugin
plugin_path="/usr/bin/gitpod_zellij"
curl -L "https://github.com/axonasif/gitpod.zellij/releases/latest/download/gitpod_zellij-x86_64-unknown-linux-musl.tar.gz" |
  tar -C /usr/bin -xzf - gitpod_zellij
SCRIPT

# Auto start zellij on SSH or xtermjs
cat >>"$HOME/.bashrc" <<'SNIP'
if ! test -v ZELLIJ && ! pgrep -f "$HOME/.vscode-server/bin" 1>/dev/null && (test -v SSH_CONNECTION || test "$PPID" == "$(pgrep -f '/ide/xterm/bin/node /ide/xterm/index.cjs' | head -n1)"); then {
  gitpod_zellij & disown
  if ! zellij has-sessions 2>/dev/null; then {
    exec zellij -s gitpod
  } fi
    exec zellij attach
} fi
SNIP
