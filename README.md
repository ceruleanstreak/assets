** Use this json in Settings | Developer: 

{"Address": "MultiAddress", "LookupSource": "MultiAddress"}

Source: https://github.com/centrifuge/go-substrate-rpc-client/issues/133 

# Prepare for launch

### This is how you create customSpec.json and customSpecRaw.json (Linux only, local machine)
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json 
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json

### You may want to purge the chain before starting the node
./target/release/node-template purge-chain --chain=customSpecRaw.json 

# Launching: 
## Launch first cloud node: 
./target/release/node-template \
--chain=customSpecRaw.json \
--alice \
--validator \
--rpc-cors=all \
--rpc-methods=Unsafe \
--unsafe-rpc-external \
--unsafe-ws-external \
--no-mdns \
--telemetry-url 'wss://telemetry.polkadot.io/submit/ 1' \
--prometheus-external \
--name='Digital Ocean Cloud Node'
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \

It will tell you the public address which you'll paste in the bootnodes next

## Launch second node
./target/release/node-template \
--chain=customSpecRaw.json \
--telemetry-url 'wss://telemetry.polkadot.io/submit/ 1' \
--bob \
--validator \
--ws-external \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--unsafe-rpc-external \
--unsafe-ws-external \
--prometheus-external \
--name='Digital Ocean Second Node' \
--node-key 0000000000000000000000000000000000000000000000000000000000000002 \
--bootnodes=/ip4/137.184.162.190/tcp/30333/p2p/12D3KooWKo5oBjBbaE6qtH2jmw5pyL1Z6k9sFyiHKwTB7SjrCs6c

## Launch third node
./target/release/node-template \
--chain=customSpecRaw.json \
--telemetry-url 'wss://telemetry.polkadot.io/submit/ 1' \
--charlie \
--validator \
--ws-external \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--unsafe-rpc-external \
--unsafe-ws-external \
--prometheus-external \
--name='Immers Cloud Node' \
--node-key 0000000000000000000000000000000000000000000000000000000000000003 \
--bootnodes=/ip4/137.184.162.190/tcp/30333/p2p/12D3KooWKo5oBjBbaE6qtH2jmw5pyL1Z6k9sFyiHKwTB7SjrCs6c







./target/release/node-template --chain customSpecRaw.json --base-path /tmp/bob  --bob --port 30334 --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --bootnodes /ip4/109.248.175.119/tcp/30333/p2p/12D3KooWB3JPDSEpyPLowpRQsR4e1fLZToNrJuJL3oiJetMkfuku --rpc-methods Unsafe


This is how we connect to a remote server: 

./target/release/node-template --chain local --base-path /tmp/bob --bob --port 30334  --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"  --validator --bootnodes /ip4/109.248.175.119/tcp/30333/p2p/12D3KooWB3JPDSEpyPLowpRQsR4e1fLZToNrJuJL3oiJetMkfuku 



./target/release/node-template --chain=local --base-path /tmp/validator1 --alice --port 30333 --ws-port 9944 --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a 

./target/release/node-template --chain=local --base-path /tmp/validator2 --bob --port 30334 --ws-port 9945 --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 

./target/release/node-template --chain local --base-path /tmp/validator3 --alice --port 30333 --ws-port 9945 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator


./target/release/node-template --chain local --base-path /tmp/bob  --bob --port 30334 --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp