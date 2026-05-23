#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

display_name() {
    case "$1" in
        chunithm_intl) echo "CHUNITHM (International)" ;;
        chunithm_jp)   echo "CHUNITHM (Japan)" ;;
        maimai_intl)   echo "maimai DX (International)" ;;
        maimai_jp)     echo "maimai DX (Japan)" ;;
        ongeki)        echo "O.N.G.E.K.I." ;;
        polarischord)  echo "Polaris Chord" ;;
        popnmusic)     echo "pop'n music" ;;
        soundvoltex)   echo "SOUND VOLTEX" ;;
        *)             echo "$1" ;;
    esac
}

game_url() {
    case "$1" in
        chunithm_intl) echo "https://chunithm.sega.com/" ;;
        chunithm_jp)   echo "https://chunithm.sega.jp/" ;;
        maimai_intl)   echo "https://maimai.sega.com/" ;;
        maimai_jp)     echo "https://maimai.sega.jp/" ;;
        ongeki)        echo "https://ongeki.sega.jp/" ;;
        polarischord)  echo "https://p.eagate.573.jp/game/polarischord/pc/" ;;
        popnmusic)     echo "https://p.eagate.573.jp/game/popn/popn29/" ;;
        soundvoltex)   echo "https://p.eagate.573.jp/game/sdvx/vi/" ;;
        *)             echo "" ;;
    esac
}

{
    echo "<!-- SONG_DB_START -->"
    echo "| Game | Songs | Last Updated | Database |"
    echo "|---|---:|:---:|:---:|"
    for toml in $(ls data/*/music.toml | sort); do
        game=$(echo "$toml" | cut -d/ -f2)
        count=$(grep -m 1 '^count = ' "$toml" | awk '{print $3}')
        updated=$(grep -m 1 '^last_updated = ' "$toml" | awk -F'"' '{print $2}' | cut -dT -f1)
        name=$(display_name "$game")
        url=$(game_url "$game")
        echo "| [$name]($url) | $count | $updated | [View]($toml) |"
    done
    echo "<!-- SONG_DB_END -->"
} > /tmp/song_table.txt

awk '
    /<!-- SONG_DB_START -->/ { skip=1; while ((getline line < "/tmp/song_table.txt") > 0) print line; next }
    /<!-- SONG_DB_END -->/ { skip=0; next }
    !skip { print }
' README.md > /tmp/README.tmp && mv /tmp/README.tmp README.md

npx prettier --write README.md
