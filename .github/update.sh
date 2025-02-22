#!/usr/bin/env bash
# shellcheck disable=SC2086
url="https://github.com/osdev0/edk2-ovmf-nightly"
version=$(curl -s https://api.github.com/repos/osdev0/edk2-ovmf-nightly/releases/latest | jq -r '.tag_name')
commit_nightly=""

GREEN='\033[1;32m'
RESET='\033[0m'

ci=false
if echo "$@" | grep -qoE '(--ci)'; then
    ci=true

fi

only_check=false
if echo "$@" | grep -qoE '(--only-check)'; then
    only_check=true
fi

download_update() {
    declare -A urls
    echo -e "${GREEN}ovmf-$1-$arch$RESET: $download_url"
    for name in $(jq -r '.Nightly.[].sha256 | keys[]' ovmf_sources.json | sort -u); do
        download_url="$url/releases/latest/download/ovmf-$name-$1.fd"
        sha256=$(nix hash convert --hash-algo sha256 "$(nix-prefetch-url $download_url)")
        real_uri=$(echo $download_url | sed "s/download/$version/; s/latest/download/")
        jq --arg arch "$1" --arg name "$name" --arg version "$version" --arg sha256 "$sha256" \
            '(.["Nightly"][$arch]["sha256"][$name] = $sha256) | (.["Nightly"][$arch]["version"] = $version) ' \
            <ovmf_sources.json >ovmf_sources.json.tmp && mv ovmf_sources.json.tmp ovmf_sources.json
        urls["$real_uri"]=1
    done
    unique_urls_json=$(printf '"%s"\n' "${!urls[@]}" | jq -s '.')
    jq --arg arch "$1" --argjson url "$unique_urls_json" '(.["Nightly"][$arch]["url"] = $url)' <ovmf_sources.json >ovmf_sources.json.tmp && mv ovmf_sources.json.tmp ovmf_sources.json
    if $ci; then
        if [ "$(echo $version | cut -d'-' -f1)" = "nightly" ]; then
            if [ "$commit_nightly" = "" ]; then
                commit_nightly="$1"
            else
                commit_nightly="$commit_nightly && $1"
            fi
        fi
    fi

}

try() {
    if $only_check; then
        echo "should_update=true" >>"$GITHUB_OUTPUT"
        exit 0
    fi
    for arch in $(jq -r '.Nightly | keys[]' ovmf_sources.json); do
        download_update $arch
    done

}

set -e

try

if $only_check && $ci; then
    echo "should_update=false" >>"$GITHUB_OUTPUT"
fi

if ! git diff --exit-code >/dev/null; then
    init_message="Update ovmf_hashes"
    message="$init_message"

    if [ "$commit_nightly" != "" ]; then
        message="${message}_$(echo $version | cut -d'-' -f1) @ $commit_nightly to $version"
    fi

    echo "commit_message=$message" >>"$GITHUB_OUTPUT"
fi
