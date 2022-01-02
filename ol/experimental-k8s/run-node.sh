
set -e
set -x

# wait until assets are copied
function get_assets() {
    while [ ! -f "$OL_PROOF_FILE" ]; do
            # check asset files
            if [ -f "$OL_ASSETS"/vdf_proofs/proof_0.json ]; then
                cp -r "$OL_ASSETS"/* "$OL_HOME_DIR"/
                # remove old web monitor files
                if [ -d "$OL_WEB_DIR" ]; then
                    rm -rf "$OL_WEB_DIR"
                fi
                break
            fi
            echo "$OL_PROOF_FILE does not exist. Sleep ..."
            sleep 1m
    done
}

# get current external ip of the node and update configs
function fix_ip() {
    # fix_ip
    my_ip=$(curl --silent 'https://api.ipify.org?format=json' | jq .ip | tr -d '"')
    echo "my_ip: $my_ip"
    pushd "$OL_HOME_DIR"
    curr_ip=$(grep "ip = " 0L.toml | awk '{print $3}' | tr -d '"')
    if [ "$my_ip" != "$curr_ip" ]; then
            crepl="find . -name \"0L.toml\" | xargs -I {} sed -i 's/ip = \"$curr_ip\"/ip = \"$my_ip\"/g'  {}"
            set +e
            eval $crepl
            set -e
    fi
    popd
}

# ol restore
function ol_restore() {
    "$OL_BIN"/ol restore
}

# fix peer ports to account for NodePort remap
function fix_ports() {
    # fix ports
    # 6179 -> 30179
    # 6180 -> 30180
    #
    pushd "$OL_HOME_DIR"
    for file in fullnode.node.yaml validator.node.yaml
    do
        if [ -f "$file" ]; then
            sed -Ei 's/listen_address: "(.+)6179"/listen_address: "\130179"/g' "$file"
            sed -Ei 's/listen_address: "(.+)6180"/listen_address: "\130180"/g' "$file"
            #sed -Ei 's/address: 127.0.0.1:8080/address: 0.0.0.0:8080/g' "$file"
            sed -Ei 's/127.0.0.1/0.0.0.0/g' "$file"
        fi
    done

    file=account.json
    if [ -f "$file" ]; then
        sed -Ei 's/("op_fullnode_network_addresses_string":)(".+)tcp\/6179/\1\2tcp\/30179/g' "$file"
        sed -Ei 's/("op_fullnode_network_addresses_string":)(".+)tcp\/6180/\1\2tcp\/30180/g' "$file"
    fi
    popd
}

function configure_ratelimit() {
    pushd "$OL_HOME_DIR"
    for file in fullnode.node.yaml validator.node.yaml
    do
        if [ -f "$file" ]; then
            if [ "$ratelimit_enabled" == "true" ]; then
              sed -Ei 's/rpc_ratelimit_enabled: (.+)/rpc_ratelimit_enabled: true/g' "$file"
              sed -Ei 's/fill_rate_tps: (.+)/fill_rate_tps: 0.5/g' "$file"
              sed -Ei 's/global_bucket_size: (.+)/global_bucket_size: 5/g' "$file"
            else
              sed -Ei 's/rpc_ratelimit_enabled: (.+)/rpc_ratelimit_enabled: false/g' "$file"
            fi
        fi
    done
    popd
}

#fix upstream node ip
function fix_upstream_ip() {
    pushd "$OL_HOME_DIR"
    # fix upstream node in 0L.toml
    if [ "$upstream_url" == http*]; then
        t="echo $upstream_url | sed -E 's/\//\\\\\//g'"
        et=$(eval $t)
        tt="sed -Ei 's/(upstream_nodes = )\[\"(.+)\"\]/\1\[\"$et\"\]/g' 0L.toml"
        eval $tt
    fi
    popd
}

# args
is_validator="$1"
is_debug="$2"
ratelimit_enabled="$3"
upstream_url="$4"

echo "validator: $is_validator, debug: $is_debug", ratelimit_enabled: "$ratelimit_enabled"

# 0L binaries
if [[ -z "${OL_BIN}" ]]; then
    export OL_BIN=/ol-bin
fi

echo "Bin dir: $OL_BIN"
export PATH="$OL_BIN":$PATH

# location of node assets, account.json, vdf_proofs, etc
# this is populated externally
if [[ -z "${OL_ASSETS}" ]]; then
    export OL_ASSETS=/assets
fi


echo "Run node as validator?: $is_validator"

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

# config file
OL_CFG_FILE="$OL_HOME_DIR"/0L.toml
if [ "$is_validator" == "true" ]; then
    OL_NODE_CFG_FILE="$OL_HOME_DIR"/validator.node.yaml
else
    OL_NODE_CFG_FILE="$OL_HOME_DIR"/fullnode.node.yaml
fi

# web monitor dir
OL_WEB_DIR="$OL_HOME_DIR"/web-monitor

OL_PROOF_FILE="$OL_HOME_DIR"/vdf_proofs/proof_0.json

# wait until assets are copied
get_assets

# copy fresh web monitor files
cp -r /web/web-monitor  "$OL_WEB_DIR"/

# update external ip of the node in configs
fix_ip

echo "$OL_PROOF_FILE exists. Start node ..."
ol_restore

# update peer ports to use NodePort remapping
fix_ports

# fix upstream node ip used by web mon
fix_upstream_ip

# configure ratelimit
configure_ratelimit

"$OL_BIN"/diem-node --config "$OL_NODE_CFG_FILE" &
#sleep 1m
#"$OL_BIN"/ol serve -c &
wait

