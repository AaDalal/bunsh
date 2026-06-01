# Fish wrapper for bunsh
# See shell_inits/README.md
function bunsh
    set -l bin (command -s bunsh)
    or set bin (status dirname)/target/debug/bunsh

    # Capture stderr to a temp file so we can parse out the script path
    # without losing real error messages.
    set -l tmpfile (mktemp)
    command bunsh $argv 2>$tmpfile
    set -l exit_code $status

    # Walk through captured stderr: extract the script path marker,
    # and forward everything else back to stderr as normal.
    set -l script_path
    while read -l line
        if string match -q '__bunsh_script__:*' $line
            set script_path (string replace '__bunsh_script__:' '' $line)
        else
            echo $line >&2
        end
    end <$tmpfile
    rm -f $tmpfile

    # If we got a script path, add `bunsh run <filename>` to history
    # so up-arrow hits the specific script. The original `bunsh` entry
    # is left alone — it's harmless further back in history.
    if test -n "$script_path"
        set -l filename (basename $script_path)
        set -l new_cmd "bunsh run $filename"

        # Atuin: add a completed entry for the rewritten command.
        if command -q atuin
            set -l atuin_id (atuin history start -- $new_cmd)
            atuin history end --exit $exit_code -- $atuin_id 2>/dev/null
        end

        # Native fish history: fish has no `history add` builtin, so we
        # write directly to the history file and merge it into memory.
        set -l histfile (set -q XDG_DATA_HOME && echo $XDG_DATA_HOME || echo ~/.local/share)/fish/fish_history
        echo "- cmd: $new_cmd" >>$histfile
        echo "  when: "(date +%s) >>$histfile
        builtin history merge
    end

    return $exit_code
end
