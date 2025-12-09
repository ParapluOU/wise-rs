#!/bin/bash
# Download Wise API documentation JSON files

set -e

DOCS_DIR="$(dirname "$0")/../docs"
mkdir -p "$DOCS_DIR/api-reference" "$DOCS_DIR/guides"

download_json() {
    local url="$1"
    local path=$(echo "$url" | sed 's|https://docs.wise.com/page-data||; s|/data.json||')

    if [ -z "$path" ]; then
        return
    fi

    local dir="$DOCS_DIR$path"
    mkdir -p "$dir"

    echo "Downloading: $path"
    curl -s "$url" -o "$dir/data.json" 2>/dev/null || echo "Failed: $path"
}

export -f download_json
export DOCS_DIR

# Download in parallel (8 concurrent)
cat /tmp/wise-json-urls.txt | xargs -P 8 -I {} bash -c 'download_json "$@"' _ {}

echo "Done!"
