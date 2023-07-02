Zellij plugin for Gitpod, with `.gitpod.yml` tasks integration. <!-- Provides theme, resource meters, indicators, keybindings and menus. -->

<!-- Can be used locally as well for the `ui:theme` module, other [modules](#modules) will be disabled outside of Gitpod. -->

# Installation

<!-- You do not need to install it if you are using [dotsh](https://github.com/axonasif/dotsh). But you can always install it in your own way in case you don't want to use `dotsh`, continue reading in that case. -->

## Quickstart

If you want to try it out as is:

- Go to <https://gitpod.io/user/preferences> and scroll down.
- Set <https://github.com/axonasif/gitpod.zellij> as **Dotfiles - Repository URL** and click on `Save`.

## Custom setup

If you already have a dotfiles repository that you use on Gitpod, you can copy the contents from [install.sh](./install.sh) and append (while excluding the first shebang line) to your own [installation script](https://www.gitpod.io/docs/configure/user-settings/dotfiles#custom-installation-script).

# Customization

## Default editor

This plugin will spawn an editor in the first tab. It's not necessary for you to specify one, the plugin uses some heuristics to auto detect the available editor. But in case you want to override it, you can do so by creating a variable named `EDITOR` on <https://gitpod.io/user/variables> with a value (e.g. `nvim`) and `*/*` as scope.

# Roadmap

This plugin is minimal in its current form. The goal is to have similar features as [gitpod.tmux](https://github.com/axonasif/gitpod.tmux). If you are interested in any of the below features in particular, please [raise an issue](https://github.com/axonasif/gitpod.zellij/issues/new/choose) to let me know.

- [x] `.gitpod.yml` tasks integration
- [ ] Resource meters: CPU, DISK and MEMORY
- [ ] Custom keybindings
- [ ] Newly opened port notification
- [ ] Functions menu
  - [ ] Manage ports
  - [ ] Stop workspace
- [ ] Custom Gitpod theme
- [ ] Indicator for dotfiles installation progress
- [ ] Watch `.gitpod.yml` changes for prompting to run `gp validate`

# Development and contributing

[![Hack in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#github.com/axonasif/gitpod.zellij)

As in [.gitpod.yml](./.gitpod.yml), you will need to run the plugin in the background first and then start zellij: `cargo run & zellij`
