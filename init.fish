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
    # 명령어를 연산자(&&, ||, |, ;)로 분리
    set -l commands (string replace -a -r '\s*(&&|\|\||[|;])\s*' \n -- $argv[1])
    for cmd_line in $commands
        set -l tokens (string split ' ' -- (string trim -- $cmd_line))
        # 환경변수 지정 구문(KEY=VALUE) 건너뛰기
        set -l program_name ""
        for token in $tokens
            if test -z "$token"
                continue
            end
            if not string match -q '*=*' -- $token
                set program_name $token
                break
            end
        end
        if test -z "$program_name"
            continue
        end
        if __vltl_check $program_name
            # Available
            continue
        end
        # 한국어가 포함되어 있는지 확인
        if not vltl has-korean $program_name
            # 한국어가 없으면 변환 및 alias 등록 안 함
            continue
        end
        set -l eng_name (vltl convert $program_name)
        # 변환된 영어 명령어가 존재하는지 확인
        if not __vltl_check $eng_name
            # 변환된 명령어가 존재하지 않으면 alias 등록 안 함
            continue
        end
        # Register alias
        alias $program_name=$eng_name #<kor -> eng command>
        echo "vltl: New alias ($program_name -> $eng_name)"
        # Switch IME to English
        vltl switch-to-english
    end
end
