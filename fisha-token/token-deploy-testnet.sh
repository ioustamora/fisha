set -ex
NETWORK=testnet
OWNER=ioustamora.$NETWORK
MASTER_ACC=fisha.game.$NETWORK
CONTRACT_ACC=token.$MASTER_ACC
# token.fisha.game.testnet
export NODE_ENV=$NETWORK

## delete acc
#echo "Delete $CONTRACT_ACC? are you sure? Ctrl-C to cancel"
#read input
#near delete $CONTRACT_ACC $MASTER_ACC
#near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC
near deploy $CONTRACT_ACC ../res/fisha_token.wasm --masterAccount $MASTER_ACC
#near call $CONTRACT_ACC new "{\"owner_id\":\"$OWNER\"}" --accountId $MASTER_ACC

set -e
#save last deployment  (to be able to recover state/tokens)
cp ../res/fisha_token.wasm ../res/testnet/fisha_token.`date +%F.%T`.wasm
#date +%F.%T
