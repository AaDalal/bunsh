## Shell Inits

When you run `bunsh` (no args) to create a script, it appends `bunsh <path/to/script.ts>`
to the history so you can click the ↑ and edit+run the script

How: the binary emits a machine-readable line on stderr:
```stdout
__bunsh_script__:/path/to/script.ts
```
This wrapper captures that line, strips it from visible output, and
adds another history entry that reads `bunsh run <filename>` instead
of bare `bunsh`. That way pressing ↑ re-runs the specific script.

This also works for atuin, if you use that.
