#!/bin/bash

unset LINERA_WALLET
unset LINERA_STORAGE

BLUE='\033[1;34m'
YELLOW='\033[1;33m'
LIGHTGREEN='\033[1;32m'
NC='\033[0m'

function print() {
  echo -e $1$2$3$NC
}


LOG_FILE=$HOME/linera-project/linera.log
print $'\U01F4AB' $YELLOW " Running lienra net, log in $LOG_FILE ..."
linera net up 2>&1 | sh -c 'exec cat' > $LOG_FILE &

while true; do
  [ ! -f $LOG_FILE ] && sleep 3 && continue
  LINERA_WALLET_ENV=`grep "export LINERA_WALLET" $LOG_FILE | sed 's/"//g'`
  LINERA_STORAGE_ENV=`grep "export LINERA_STORAGE" $LOG_FILE | sed 's/"//g'`
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
print $'\U01f499' $LIGHTGREEN " Feed application deployed"
echo -e "    Bytecode ID:    $BLUE$feed_bid$NC"
echo -e "    Application ID: $BLUE$feed_appid$NC"

sed -i "s/feedAppID =.*/feedAppID = \"$feed_appid\"/g" webui/src/const/index.ts
sed -i "s/creditAppID =.*/creditAppID = \"$credit_appid\"/g" webui/src/const/index.ts
sed -i "s/marketAppID =.*/marketAppID = \"$market_appid\"/g" webui/src/const/index.ts

function cleanup() {
  rm -rf `dirname $LINERA_WALLET`
}

trap cleanup INT
read -p "  Press any key to exit"
print $'\U01f499' $LIGHTGREEN " Exit ..."

cleanup

