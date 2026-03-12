function __vltl_convert_and_expand
    set -l token (commandline --current-token)

    # $VLTL_PATH가 설정되어 있으면 해당 경로의 vltl을 사용, 아니면 PATH의 vltl 사용
    set -l __vltl_bin vltl
    if set -q VLTL_PATH
        set __vltl_bin $VLTL_PATH
    end

    if test -n "$token"; and $__vltl_bin has-korean "$token"
        set -l converted ($__vltl_bin convert "$token")
        if test -n "$converted"; and test "$converted" != "$token"
            commandline --current-token --replace -- "$converted"

            # 변환된 토큰에 대응하는 abbr이 있으면 한글 트리거로 자동 등록
            if abbr -q -- "$converted"
                __vltl_auto_register_abbr "$token" "$converted"
            end

            # Switch IME to English (only available on macOS)
            $__vltl_bin switch-to-english 2>/dev/null
        end
    end
end

function __vltl_auto_register_abbr
    set -l korean_trigger $argv[1]
    set -l english_trigger $argv[2]

    # 이미 한글 트리거에 대한 abbr이 있으면 중복 등록하지 않음
    if abbr -q -- "$korean_trigger"
        return
    end

    # 영어 트리거의 abbr 정의를 찾아 한글 트리거로 복제
    for def in (abbr --show)
        set -l parts (string split ' -- ' "$def")
        if test (count $parts) -lt 2
            continue
        end
        set -l after_separator $parts[2]
        set -l trigger_in_def (string split ' ' "$after_separator")[1]
        if test "$trigger_in_def" = "$english_trigger"
            set -l new_def (string replace -- "-- $english_trigger " "-- $korean_trigger " "$def")
            eval $new_def
            return
        end
    end
end

function __vltl_abbr_space
    __vltl_convert_and_expand
    commandline -f expand-abbr
    commandline -i ' '
end

function __vltl_abbr_enter
    __vltl_convert_and_expand
    commandline -f expand-abbr
    commandline -f execute
end

function __vltl_abbr_semicolon
    __vltl_convert_and_expand
    commandline -f expand-abbr
    commandline -i ';'
end

bind ' ' __vltl_abbr_space
bind \r __vltl_abbr_enter
bind \; __vltl_abbr_semicolon
