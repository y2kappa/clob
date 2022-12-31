#!/bin/bash
while getopts t:p:c: flag
do
    case "${flag}" in
        c) CLUSTER=${OPTARG};;
        t) TESTING=${OPTARG};;
    esac
done

export PROGRAM_NAMES=( clob )

echo "Integration tests mode: $TESTING";
echo "Cluster: $CLUSTER";

if [ -z "$PROGRAM" ]; then
    echo -e "[${ORN}WARNING${NC}] No program specified. Building all the programs = (${PROGRAM_NAMES[*]})"
else
    PROGRAM_NAMES=($PROGRAM)
fi

FEATURE_FLAGS="--features $CLUSTER"

if [[ ("$TESTING" == "yes")  && ($CLUSTER == "localnet" || $CLUSTER == "devnet") ]]; then
    FEATURE_FLAGS="$FEATURE_FLAGS integration_tests skip_price_validation edition2021"
fi

echo "FEATURE_FLAGS: $FEATURE_FLAGS"

if [[ ($CLUSTER == "localnet") && ! -d ./keys/$CLUSTER ]]; then
    echo -e "[${ORN}WARNING${NC}] On $CLUSTER and keys directory is not present. Generating new keys!"
    new_local_keypairs $CLUSTER
fi

mkdir -p target/deploy/
KEYS_DIR=keys/$CLUSTER
OWNER_KP=$KEYS_DIR/owner.json

for PROGRAM in "${PROGRAM_NAMES[@]}"
do
    if [[ $CLUSTER == "localnet" ]]; then
        PROGRAM_KP=$KEYS_DIR/$PROGRAM.json
        cp $PROGRAM_KP target/deploy/$PROGRAM-keypair.json
    fi
    if ! anchor build \
        -p $PROGRAM \
        --provider.wallet $KEYS_DIR/$OWNER_KP \
        --provider.cluster $CLUSTER \
        -- $FEATURE_FLAGS; then
        echo -e "[${RED}ERROR${NC}] Program '$PROGRAM.so' building failed !!"
    else
        echo -e "[${GRN}SUCCESS${NC}] Program '$PROGRAM.so' built successfully !!"
    fi
done