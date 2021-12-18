
set -e
set -x

# wait until assets are copied
function get_assets() {
    while [ ! -f "$OL_PROOF_FILE" ]; do
            echo "$OL_PROOF_FILE does not exist. Sleep ..."
            sleep 1m
    done
}

# args
is_validator="$1"
is_debug="$2"
upstream_url="$3"

echo "validator: $is_validator, debug: $is_debug"

# 0L binaries
if [[ -z "${OL_BIN}" ]]; then
    export OL_BIN=/ol-bin
fi

echo "Bin dir: $OL_BIN"
export PATH="$OL_BIN":$PATH


if [[ -z "${OL_HOME_DIR}" ]]; then
    export OL_HOME_DIR=/root/.0L
fi
echo "0L home dir: $OL_HOME_DIR"
export NODE_ENV=prod

mkdir -p "$OL_HOME_DIR"

while [ "$is_debug" == "true" ]; do
    echo 'In debug mode sleep 10'
    sleep 10
done

OL_PROOF_FILE="$OL_HOME_DIR"/vdf_proofs/proof_0.json

# wait until assets are copied
get_assets

cd "$OL_HOME_DIR"
# wait until main node starts
sleep 1m
# start web monitor
"$OL_BIN"/ol serve -c 

