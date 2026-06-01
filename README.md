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
<p align="center" style="font-style: italic">
Create a simple script using bunsh, see the output and then iterate by pressing the up-arrow!
</p>

#### How to use

1. Run bunsh to create a script
2. Edit your ts script
3. Quit your editor; your script is run automatically
4. Press ↑ to iterate on the same script

It replicates the feel of a repl (fast iteration), but you come away with a clean script

#### Features

- Automatically edits shell history so you can iterate on scripts created with `bunsh` (supports fish, zsh, bash and the atuin shell history manager)
- Get typescript lsp if your editor (from $EDITOR) supports it (automatically copies the `src/tsconfig.json` from this repo)
- Includes a prelude: `import { $ } from "bun"` (if you wish to change this, you can clone the repo and edit src/default_imports.ts)
- Saves scripts to `~/.local/share/bunsh` timestamped so you can copy them (you can also give them custom names)
  - (technically it follows the XDG home directory spec)

### Installation

*Before you start, you'll need*
- [Bun](https://bun.com/docs/installation) (if you don't like bun you can tweak the code to swap it out for `python`, `node` or your preferred scripting setup)
- [rust/cargo](https://rust-lang.org/tools/install/)
   - (just to compile it for your system; sadly I don't have a prebuilt vesion yet)

---

1. Run `cargo install bunsh` to install `bunsh`
2. Eval `bunsh init <your_shell_eg_zsh>` in your shell config. This allow bunsh to automatically edit history (the first [Feature](#Features) in the list)
```sh
# If your shell is zsh (default on macos), add this to your ~/.zshrc
eval "$(bunsh init zsh)"

# bash: add to ~/.bashrc
eval "$(bunsh init bash)"

# fish: add to ~/.config/fish/config.fish
bunsh init fish | source
```
3. (optional) Eval `bunsh completions <shell>` for shell completions
```sh
# If your shell is zsh (default on macos), add this to your ~/.zshrc
eval "$(bunsh completions zsh)"

# bash: add to ~/.bashrc
eval "$(bunsh completions bash)"

# fish: add to ~/.config/fish/config.fish
bunsh completions fish | source
```

### Feedback

Feel free to make github issues. Other things I'd like to know:
- do you want this tool to generalize to other tools besides bun without having to fork it (e.g., python, rust-script etc)? I think it could take a config file
