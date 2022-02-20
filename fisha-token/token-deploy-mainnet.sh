set -e

NETWORK=mainnet
SUFFIX=near
export NODE_ENV=$NETWORK

OWNER=ioustamora.$SUFFIX
MASTER_ACC=fisha-token.$SUFFIX
CONTRACT_ACC=$MASTER_ACC

FISHA_CONTRACT=fisha-game.$SUFFIX

## delete acc
#echo "Delete $CONTRACT_ACC? are you sure? Ctrl-C to cancel"
#read input
#near delete $CONTRACT_ACC $MASTER_ACC
#near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC
near deploy $CONTRACT_ACC ../res/fisha_token.wasm --masterAccount $MASTER_ACC
#near call $CONTRACT_ACC new "{\"owner_id\":\"$OWNER\"}" --accountId $MASTER_ACC
#near call $CONTRACT_ACC add_minter "{\"account_id\":\"$FISHA_CONTRACT\"}" --accountId $OWNER --depositYocto 1


## redeploy code only
#near deploy $CONTRACT_ACC ./res/fisha_token.wasm --masterAccount $MASTER_ACC

#save last deployment  (to be able to recover state/tokens)
cp ../res/fisha_token.wasm ../res/mainnet/fisha_token.`date +%F.%T`.wasm
date +%F.%T
