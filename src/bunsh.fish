# Fish wrapper for bunsh — rewrites history to include the script path
function bunsh
    set -l bin (command -s bunsh)
    or set bin (status dirname)/target/debug/bunsh

    set -l tmpfile (mktemp)
    command bunsh $argv 2>$tmpfile
    set -l exit_code $status

    set -l script_path
    while read -l line
        if string match -q '__bunsh_script__:*' $line
            set script_path (string replace '__bunsh_script__:' '' $line)
        else
            echo $line >&2
        end
    end <$tmpfile
    rm -f $tmpfile

    # Rewrite history if we resolved a script path
    if test -n "$script_path"
        set -l filename (basename $script_path)
        builtin history delete --exact --case-sensitive -- (string join ' ' bunsh $argv)
        builtin history merge
        builtin history add -- "bunsh run $filename"
    end

    return $exit_code
end
