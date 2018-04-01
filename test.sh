#!/bin/bash

#
# Bash script for testing the timestamping.
#

set -e

# Base URL
BASE_TS_URL=http://127.0.0.1:8000/api/services/timestamping/v0
BASE_BC_URL=http://127.0.0.1:8000/api/services/blockchain/v0


# Exit status
STATUS=0

# Launches the service and waits until it starts listening
# on the TCP port 8000.
function launch-server {
    #cargo run &
    #CTR=0
    #MAXCTR=60
    #while [[ ( -z `lsof -iTCP -sTCP:LISTEN -n -P 2>/dev/null |  awk '{ if ($9 == "*:8000") { print $2 } }'` ) && ( $CTR -lt $MAXCTR ) ]]; do
    #  sleep 1
    #  CTR=$(( $CTR + 1 ))
    #done
    #if [[ $CTR == $MAXCTR ]]; then
    #    echo "Failed to launch the server; aborting"
    #    exit 1
    #fi
    rm -rf ./db/*
    ./run.sh 1

    echo "Waiting for server start..."
    sleep 10
}


# Kills whatever program is listening on the TCP port 8000
function kill-server {
    SERVER_PID=`lsof -iTCP -sTCP:LISTEN -n -P 2>/dev/null |  awk '{ if ($9 == "*:8000") { print $2 } }'`
    if [[ -n $SERVER_PID ]]; then
        kill -9 $SERVER_PID
    fi
}


# Creates a timestamp
# Arguments:
# - $1: filename with the transaction data
function create-timestamp {
    RESP=`curl -H "Content-Type: application/json" -X POST -d @$1 $BASE_TS_URL/timestamp/hash 2>/dev/null`
}


# Checks a response to an Exonum transaction.
#
# Arguments:
# - $1: expected start of the transaction hash returned by the server
function check-transaction {
    if [[ `echo $RESP | jq .data_hash` =~ ^\"$1 ]]; then
        echo "OK, got expected data hash $1"
	echo "Response"
	echo $RESP
    else
        echo "Unexpected response: $RESP"
        STATUS=1
    fi
    echo ""
}

function get-timestamp {
    RESP=`curl -H "Content-Type: application/json" -X GET $BASE_TS_URL/timestamp/$1 2>/dev/null`
}

function check-timestamp {
    if [[ `echo $RESP | jq .data_hash` =~ ^\"$1 ]]; then
        echo "OK, got expected transaction hash $1"
	echo "Response"
	echo $RESP

    else
        echo "Unexpected response: $RESP"
        STATUS=1
    fi
    echo ""
}


function get-height {
    RESP=`curl -H "Content-Type: application/json" -X GET $BASE_BC_URL/height 2>/dev/null`
    echo "Blockchain HEIGHT: " $RESP
    echo ""
}

function get-block {
    RESP=`curl -H "Content-Type: application/json" -X GET $BASE_BC_URL/block/$1 2>/dev/null`
    echo "Block: " $1
    echo $RESP
    echo ""
}

function get-tx-hash {
    RESP=`curl -H "Content-Type: application/json" -X POST -d @$1 $BASE_TS_URL/timestamp/hash 2>/dev/null`
    TX_HASH=`echo $RESP | jq --raw-output .tx_hash`
}


function get-tx {
    RESP=`curl -H "Content-Type: application/json" -X GET $BASE_BC_URL/transaction/$1 2>/dev/null`
    echo "Transaction: " $1
    echo $RESP
    echo ""
}


kill-server
launch-server

echo "Checking timestamping API"

echo "Creating a timestamp by 'examples/timestaping-req-1.json'"
create-timestamp examples/timestaping-req-1.json
check-transaction b32b3423e734f4ec4a9f73f934e314a87b49e48ab7fe97b7f84d606193571b60

echo "Creating a timestamp by 'examples/timestaping-req-2.json'"
create-timestamp examples/timestaping-req-2.json
check-transaction a92217670661fbb669142f67dc7810cd3afcd2691f4195410fb84c93e4c47926

echo "Creating a timestamp by 'examples/timestaping-req-3.json'"
create-timestamp examples/timestaping-req-3.json
check-transaction 6bb5b4df9228e8119164a397b00034ac33739485dee01fad933dadb3c6cf1be9

echo "Creating a timestamp by 'examples/timestaping-req-4.json'"
create-timestamp examples/timestaping-req-4.json
check-transaction 1cd143c7ed95be46f91ca9cd675f02de794378607cba12a07a1d1df6032ec904

echo "Creating a timestamp by 'examples/timestaping-req-5.json'"
create-timestamp examples/timestaping-req-5.json
check-transaction ef5725af0e304b092ad356083f94cd522a5d05b7360d7535dea12436ee77d4c4

get-height
get-block $RESP

echo "Checking timestamp b32b3423e734f4ec4a9f73f934e314a87b49e48ab7fe97b7f84d606193571b60"
get-timestamp b32b3423e734f4ec4a9f73f934e314a87b49e48ab7fe97b7f84d606193571b60
check-timestamp b32b3423e734f4ec4a9f73f934e314a87b49e48ab7fe97b7f84d606193571b60

echo "Checking timestamp a92217670661fbb669142f67dc7810cd3afcd2691f4195410fb84c93e4c47926"
get-timestamp a92217670661fbb669142f67dc7810cd3afcd2691f4195410fb84c93e4c47926
check-timestamp a92217670661fbb669142f67dc7810cd3afcd2691f4195410fb84c93e4c47926

echo "Checking timestamp 6bb5b4df9228e8119164a397b00034ac33739485dee01fad933dadb3c6cf1be9"
get-timestamp 6bb5b4df9228e8119164a397b00034ac33739485dee01fad933dadb3c6cf1be9
check-timestamp 6bb5b4df9228e8119164a397b00034ac33739485dee01fad933dadb3c6cf1be9

echo "Checking timestamp 1cd143c7ed95be46f91ca9cd675f02de794378607cba12a07a1d1df6032ec904"
get-timestamp 1cd143c7ed95be46f91ca9cd675f02de794378607cba12a07a1d1df6032ec904
check-timestamp 1cd143c7ed95be46f91ca9cd675f02de794378607cba12a07a1d1df6032ec904

echo "Checking timestamp ef5725af0e304b092ad356083f94cd522a5d05b7360d7535dea12436ee77d4c4"
get-timestamp ef5725af0e304b092ad356083f94cd522a5d05b7360d7535dea12436ee77d4c4
check-timestamp ef5725af0e304b092ad356083f94cd522a5d05b7360d7535dea12436ee77d4c4


echo ""
echo "Checking blockchain explorer API"
get-height
get-block $RESP

get-tx-hash examples/timestaping-req-1.json
get-tx $TX_HASH

get-tx-hash examples/timestaping-req-2.json
get-tx $TX_HASH

get-tx-hash examples/timestaping-req-3.json
get-tx $TX_HASH

get-tx-hash examples/timestaping-req-4.json
get-tx $TX_HASH

kill-server
exit $STATUS


























