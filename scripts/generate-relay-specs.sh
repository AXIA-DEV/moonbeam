#!/bin/bash
set -e
source scripts/_init_var.sh

if [ -z "$AXIA_VERSION" ]; then
  AXIA_VERSION="sha-`egrep -o '/axia.*#([^\"]*)' Cargo.lock | \
    head -1 | sed 's/.*#//' |  cut -c1-8`"
fi

echo "Using AXIA revision #${AXIA_VERSION}"

echo "=================== BetaNet-Local ==================="
docker run -it -v $(pwd)/build:/build purestake/moonbase-relay-testnet:$AXIA_VERSION \
  /usr/local/bin/axia \
    build-spec \
      --chain betanet-local \
      -lerror \
      --disable-default-bootnode \
      --raw \
    > $ROCOCO_LOCAL_RAW_SPEC
echo $ROCOCO_LOCAL_RAW_SPEC generated