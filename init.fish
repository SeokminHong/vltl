function __vltl_check -S
    set -l program_name $argv[1]
    if type -q $program_name
        # Installed
        return 0
    end
    if abbr -q $program_name
        # Abbreviation
        return 0
    end

    return 1
end

function __vltl -S --on-event fish_preexec
    set -l program_name (string split ' ' $argv[1])[1]
    if __vltl_check $program_name -eq 0
        # Available
        return 0
    end
    set -l eng_name (vltl convert $program_name)
    # Register alias
    alias $program_name=$eng_name #<kor -> eng command>
    echo "vltl: New alias ($program_name -> $eng_name)"
    # Switch IME to English
    vltl switch-to-english
end
