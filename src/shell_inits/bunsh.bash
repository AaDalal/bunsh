# Bash wrapper for bunsh
# See shell_inits/README.md
bunsh() {
    local tmpfile
    tmpfile=$(mktemp)

    # Capture stderr to a temp file so we can parse out the script path
    # without losing real error messages.
    command bunsh "$@" 2>"$tmpfile"
    local exit_code=$?

    # Walk through captured stderr: extract the script path marker,
    # and forward everything else back to stderr as normal.
    local script_path=""
    while IFS= read -r line; do
        case "$line" in
            __bunsh_script__:*)
                script_path="${line#__bunsh_script__:}"
                ;;
            *)
                echo "$line" >&2
                ;;
        esac
    done <"$tmpfile"
    rm -f "$tmpfile"

    # If we got a script path, add `bunsh run <filename>` to history
    # so up-arrow hits the specific script. The original `bunsh` entry
    # is left alone — it's harmless further back in history.
    if [[ -n "$script_path" ]]; then
        local filename new_cmd
        filename=$(basename "$script_path")
        new_cmd="bunsh run $filename"

        # Atuin: add a completed entry for the rewritten command.
        if command -v atuin &>/dev/null; then
            local atuin_id
            atuin_id=$(atuin history start -- "$new_cmd")
            atuin history end --exit "$exit_code" -- "$atuin_id" 2>/dev/null
        fi

        # Native bash history:
        # `history -s` adds the rewritten entry to in-memory history.
        history -s "$new_cmd"
    fi

    return "$exit_code"
}
