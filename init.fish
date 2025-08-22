function __vltl_check -S
    set -l program_name $argv[1]
    if type -q $program_name
        echo "$program_name is installed"
        return 0
    end
    if abbr -q $program_name
        echo "$program_name is an abbreviation for another command"
        return 0
    end

    return 1
end

function __vltl -S --on-event fish_preexec
    set -l program_name (string split ' ' $argv[1])[1]
    if __vltnl_check $program_name -eq 0
        echo "$program_name is available"
        return 0
    end
    echo "$program_name is not available"
end
