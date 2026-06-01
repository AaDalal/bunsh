# Bash wrapper for bunsh — rewrites history to include the script path
bunsh() {
    local tmpfile
    tmpfile=$(mktemp)

    command bunsh "$@" 2>"$tmpfile"
    local exit_code=$?

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

    if [[ -n "$script_path" ]]; then
        local filename
        filename=$(basename "$script_path")
        # Remove the last history entry and replace it
        history -d -1 2>/dev/null || history -d $((HISTCMD - 1)) 2>/dev/null
        history -s "bunsh run $filename"
    fi

    return "$exit_code"
}
