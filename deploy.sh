#!/bin/bash

unset LINERA_WALLET
unset LINERA_STORAGE

killall -15 linera > /dev/null 2>&1
killall -15 linera-proxy > /dev/null 2>&1
killall -15 linera-server > /dev/null 2>&1

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

function print() {
  echo -e $1$2$3$NC
}


NODE_LOG_FILE=$HOME/linera-project/linera.log
SERVICE_LOG_FILE=$HOME/linera-project/service_8080.log

print $'\U01F4AB' $YELLOW " Running lienra net, log in $NODE_LOG_FILE ..."
linera net up 2>&1 | sh -c 'exec cat' > $NODE_LOG_FILE &

while true; do
  [ ! -f $NODE_LOG_FILE ] && sleep 3 && continue
  LINERA_WALLET_ENV=`grep "export LINERA_WALLET" $NODE_LOG_FILE | sed 's/"//g'`
  LINERA_STORAGE_ENV=`grep "export LINERA_STORAGE" $NODE_LOG_FILE | sed 's/"//g'`
  print $'\U01F411' $LIGHTGREEN " Waiting linera net ..."
  [ -z "$LINERA_WALLET_ENV" -o -z "$LINERA_STORAGE_ENV" ] && sleep 3 && continue
  print $'\U01F411' $LIGHTGREEN " Linera net up ..."
  break
done

$LINERA_WALLET_ENV
$LINERA_STORAGE_ENV

while true; do
  print $'\U01F411' $LIGHTGREEN " Waiting linera database `dirname $LINERA_WALLET` ..."
  [ ! -f $LINERA_WALLET ] && sleep 3 && continue
  break
done

print $'\U01F4AB' $YELLOW " Deploying Credit application ..."
credit_bid=`linera publish-bytecode ./target/wasm32-unknown-unknown/release/credit_{contract,service}.wasm`
credit_appid=`linera create-application $credit_bid --json-argument '{"initial_supply":"99999999999999.0","amount_alive_ms":600000}'`
print $'\U01f499' $LIGHTGREEN " Credit application deployed"
echo -e "    Bytecode ID:    $BLUE$credit_bid$NC"
echo -e "    Application ID: $BLUE$credit_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Feed application ..."
feed_bid=`linera publish-bytecode ./target/wasm32-unknown-unknown/release/feed_{contract,service}.wasm`
feed_appid=`linera create-application $feed_bid --json-argument '{"react_interval_ms":60000}' --json-parameters "\"$credit_appid\"" --required-application-ids $credit_appid`
print $'\U01f499' $LIGHTGREEN " Feed application deployed"
echo -e "    Bytecode ID:    $BLUE$feed_bid$NC"
echo -e "    Application ID: $BLUE$feed_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Market application ..."
market_bid=`linera publish-bytecode ./target/wasm32-unknown-unknown/release/market_{contract,service}.wasm`
market_appid=`linera create-application $market_bid --json-argument '{"credits_per_linera":"30","max_credits_percent":30,"trade_fee_percent":3}' --json-parameters "\"$credit_appid\"" --required-application-ids $credit_appid`
print $'\U01f499' $LIGHTGREEN " Market application deployed"
echo -e "    Bytecode ID:    $BLUE$market_bid$NC"
echo -e "    Application ID: $BLUE$market_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Foundation application ..."
foundation_bid=`linera publish-bytecode ./target/wasm32-unknown-unknown/release/foundation_{contract,service}.wasm`
foundation_appid=`linera create-application $foundation_bid --json-argument '{"review_reward_percent":20,"review_reward_factor":20,"author_reward_percent":40,"author_reward_factor":20,"activity_reward_percent":10}'`
print $'\U01f499' $LIGHTGREEN " Foundation application deployed"
echo -e "    Bytecode ID:    $BLUE$foundation_bid$NC"
echo -e "    Application ID: $BLUE$foundation_appid$NC"

print $'\U01F4AB' $YELLOW " Deploying Review application ..."
review_bid=`linera publish-bytecode ./target/wasm32-unknown-unknown/release/review_{contract,service}.wasm`
review_appid=`linera create-application $review_bid --json-argument '{"content_approved_threshold":3,"content_rejected_threshold":2,"asset_approved_threshold":2,"asset_rejected_threshold":2,"reviewer_approved_threshold":2,"reviewer_rejected_threshold":2}' --json-parameters "{\"feed_app_id\":\"$feed_appid\",\"credit_app_id\":\"$credit_appid\"}" --required-application-ids $feed_appid --required-application-ids $credit_appid`
print $'\U01f499' $LIGHTGREEN " Review application deployed"
echo -e "    Bytecode ID:    $BLUE$review_bid$NC"
echo -e "    Application ID: $BLUE$review_appid$NC"


sed -i "s/feedApp =.*/feedApp = '$feed_appid',/g" webui/src/const/index.ts
sed -i "s/creditApp =.*/creditApp = '$credit_appid',/g" webui/src/const/index.ts
sed -i "s/marketApp =.*/marketApp = '$market_appid',/g" webui/src/const/index.ts
sed -i "s/reviewApp =.*/reviewApp = '$review_appid'/g" webui/src/const/index.ts

function run_new_service() {
  wallet_dir=`dirname $LINERA_WALLET`
  wallet=$wallet_dir/wallet_$1.json
  storage=rocksdb:$wallet_dir/linera$1.db
  print $'\U01f499' $LIGHTGREEN " Initialize wallet$1 ..."
  linera --wallet $wallet --storage $storage wallet init --genesis $wallet_dir/genesis.json
  print $'\U01f499' $LIGHTGREEN " Gen wallet2 pub key ..."
  pub_key=`linera --wallet $wallet --storage $storage keygen`
  print $'\U01f499' $LIGHTGREEN " Open wallet2 chain ..."
  effect_and_chain=`linera open-chain --to-public-key $pub_key`
  effect=$(echo "$effect_and_chain" | sed -n '1 p')
  chain_id=$(echo "$effect_and_chain" | sed -n '2 p')
  linera --wallet $wallet --storage $storage assign --key $pub_key --message-id $effect
  linera --wallet $wallet --storage $storage wallet show
  linera wallet show
  print $'\U01f499' $LIGHTGREEN " Run $2 service ..."
  LOG_FILE=`echo $SERVICE_LOG_FILE | sed "s/8080/$2/g"`
  linera --wallet $wallet --storage $storage service --port $2 > $LOG_FILE 2>&1 &
}

run_new_service 2 8081

print $'\U01f499' $LIGHTGREEN " Run 8080 service ..."
linera service > $SERVICE_LOG_FILE 2>&1 &


function cleanup() {
  rm -rf `dirname $LINERA_WALLET`
  killall -15 linera > /dev/null 2>&1
  killall -15 linera-proxy > /dev/null 2>&1
  killall -15 linera-server > /dev/null 2>&1
}

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

