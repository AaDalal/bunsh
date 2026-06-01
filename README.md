<h1 align="center">
BUNSH
</h1>

<p align="center">
Easily iterate on js scripts so you can write less shell commands
</p>

<p align="center">
<a href="#how-to-use">How to use</a> &middot;
<a href="#features">Features</a> &middot;
<a href="#installation">Installation</a> &middot;
<a href="#feedback">Feedback</a>
</p>

<p align="center">
  <img src="demo.gif" alt="animated gif of using bunsh" width="80%" />
</p>

#### How to use

run bunsh to create a script ->
edit your ts script ->
quit your editor; your script is run automatically ->
press ↑ to iterate on the same script


it replicates the feel of a repl while maintaining scripts

#### Features

- Automatically edits history so you can easily edit the last script you used with `bunsh` (supports fish, zsh, bash and the atuin shell history manager)
- Get typescript lsp if your editor (from $EDITOR) supports it (automatically copies the `src/tsconfig.json` from this repo)
- Includes a prelude: `import { $ } from "bun"` (if you wish to change this, you can clone the repo and edit src/default_imports.ts)
- Saves scripts to `~/.local/share/bunsh` timestamped so you can copy them (you can also give them custom names)
  - (technically it follows the XDG home directory spec)

### Installation

*Dependencies*
- bun (if you don't like bun you can tweak the code to swap it out for `python`, `node` or your preferred scripting setup)
- rust/cargo (just to build it, sadly I don't have a prebuilt vesion yet)

1. Run `cargo install bunsh` to install `bunsh`
2. Eval `bunsh init <your_shell_eg_zsh>` in your shell config to allow bunsh to automatically edit history (the first [Feature](#Features))
```sh
# If your shell is zsh (default on macos)
eval "$(bunsh init zsh)"

# bash
eval "$(bunsh init bash)"

# fish
bunsh init fish | source
```
3. (optional) Eval `bunsh completions <shell>` for shell completions
```sh
# If your shell is zsh (default on macos)
eval "$(bunsh completions zsh)"

# bash
eval "$(bunsh completions bash)"

# fish
bunsh completions fish | source
```

### Feedback

Feel free to make github issues. Other things I'd like to know:
- do you want this tool to generalize to other tools besides bun without having to fork it (e.g., python, rust-script etc)? I think it could take a config file
