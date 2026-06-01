# Zsh wrapper for bunsh — rewrites history to include the script path
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
        # Remove the current entry and add the rewritten one
        fc -W
        sed -i '' -e '$ d' "$HISTFILE" 2>/dev/null || sed -i '$ d' "$HISTFILE"
        print -s "bunsh run $filename"
        fc -R
    fi

    return "$exit_code"
}
