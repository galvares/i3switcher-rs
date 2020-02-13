# i3switcher-rs

**i3switcher-rs** is a hacky for the i3wm that provides _back and forth_ behavior between the two latest focused window. Interacts cyclically between the current and the last focused window, even if one is in another workspace.

Is similar to the behavior of `workspace back_and_forth`, but for window.

**OBS:** It is a `Rust` version of the i3switcher-py project [https://github.com/galvares/i3switcher-py] with some differences in how we use the IPC to capture and manage the focus. More elegant and faster than the python version.

## Requeriments

* rust >=1.9.0

## Crates

* i3ipc-rs
* regex

## Install

Using cargo to compile:
```
$ cargo build --release
```
then copy `target/release/i3switcher` to some $PATH directory:
```
$ cp target/release/i3switcher /usr/local/bin/i3switcher
```
and add in your i3wm config this lines:
```
set $alt Mod1
exec --no-startup-id /usr/local/bin/i3switcher
bindsym $alt+Tab exec true
```
that's it! Use Alt+TAB to switch between current and last focused window.
