# 명령어/abbreviation 캐시 파일 경로
set -g __vltl_cache_file ~/.cache/vltl/commands

# 캐시 갱신: 사용 가능한 명령어와 abbreviation 목록을 파일에 저장
function __vltl_refresh_cache
    set -l cache_dir (command dirname $__vltl_cache_file)
    command mkdir -p $cache_dir
    begin
        builtin --names
        functions --names
        for dir in $PATH
            if test -d $dir
                command ls $dir 2>/dev/null
            end
        end
        abbr --list
    end | command sort -u > $__vltl_cache_file
end

# 캐시가 stale한 경우 갱신
# $VLTL_CACHE_TTL (초 단위)로 캐시 유효 시간 설정 가능 (기본: 3600초 = 1시간)
function __vltl_refresh_cache_if_stale
    set -l ttl 3600
    if set -q VLTL_CACHE_TTL
        set ttl $VLTL_CACHE_TTL
    end

    if not test -f $__vltl_cache_file
        __vltl_refresh_cache
        return
    end

    # 파일 수정 시간 확인 (Linux: stat -c, macOS: stat -f)
    set -l mtime (command stat -c %Y $__vltl_cache_file 2>/dev/null; or command stat -f %m $__vltl_cache_file 2>/dev/null)
    if test -z "$mtime"
        __vltl_refresh_cache
        return
    end

    set -l now (command date +%s)
    if test (math $now - $mtime) -gt $ttl
        __vltl_refresh_cache
    end
end

function __vltl_convert_and_expand
    set -l token (commandline --current-token)

    # $VLTL_PATH가 설정되어 있으면 해당 경로의 vltl을 사용, 아니면 PATH의 vltl 사용
    set -l __vltl_bin vltl
    if set -q VLTL_PATH
        set __vltl_bin $VLTL_PATH
    end

    if test -n "$token"; and $__vltl_bin has-korean -- "$token"
        # 커서가 명령어 이름 위치에 있는지 AST로 확인
        set -l cmdline (commandline)
        set -l cursor_pos (commandline --cursor)
        if not $__vltl_bin is-command-position -- "$cmdline" "$cursor_pos"
            return
        end

        set -l converted ($__vltl_bin convert -- "$token")
        if test -n "$converted"; and test "$converted" != "$token"
            # 기본 변환이 존재하는 명령어/abbreviation인지 확인
            if type -q "$converted"; or abbr -q -- "$converted"
                commandline --current-token --replace -- "$converted"

                if abbr -q -- "$converted"
                    __vltl_auto_register_abbr "$token" "$converted"
                end

                $__vltl_bin switch-to-english 2>/dev/null
                return
            end

            # 대소문자 후보를 고려하여 캐시된 명령어에서 매칭 검색
            if test -f $__vltl_cache_file
                set -l match ($__vltl_bin find-command --first -- "$token" < $__vltl_cache_file)
                if test -n "$match"
                    commandline --current-token --replace -- "$match"

                    if abbr -q -- "$match"
                        __vltl_auto_register_abbr "$token" "$match"
                    end

                    $__vltl_bin switch-to-english 2>/dev/null
                end
            end
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
    # abbr --show 출력은 eval로 재실행 가능한 형식이므로 트리거만 교체하여 재실행
    for def in (abbr --show)
        # ' -- ' 기준으로 분리하여 트리거 위치 확인
        set -l parts (string split ' -- ' "$def")
        if test (count $parts) -lt 2
            continue
        end
        set -l after_separator $parts[2]
        set -l trigger_in_def (string split ' ' "$after_separator")[1]
        if test "$trigger_in_def" = "$english_trigger"
            # 트리거를 escape하여 eval 시 셸 메타문자 해석 방지
            set -l escaped_korean (string escape -- "$korean_trigger")
            set -l new_def (string replace -- "-- $english_trigger " "-- $escaped_korean " "$def")
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

# init 시점에 캐시가 stale하면 갱신
__vltl_refresh_cache_if_stale
