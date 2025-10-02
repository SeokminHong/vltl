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
    # 한국어가 포함되어 있는지 확인
    if not vltl has-korean $program_name
        # 한국어가 없으면 변환 및 alias 등록 안 함
        return 0
    end
    set -l eng_name (vltl convert $program_name)
    # 변환된 영어 명령어가 존재하는지 확인
    if not __vltl_check $eng_name -eq 0
        # 변환된 명령어가 존재하지 않으면 alias 등록 안 함
        return 0
    end
    # Register alias
    alias $program_name=$eng_name #<kor -> eng command>
    echo "vltl: New alias ($program_name -> $eng_name)"
    # Switch IME to English (only available on macOS)
    vltl switch-to-english 2>/dev/null
end
