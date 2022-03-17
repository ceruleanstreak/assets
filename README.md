** Use this json in Settings | Developer: 

{"Address": "MultiAddress", "LookupSource": "MultiAddress"}

Source: https://github.com/centrifuge/go-substrate-rpc-client/issues/133 


** Launching: 

./target/release/node-template purge-chain --base-path /tmp/bob --chain local -y

./target/release/node-template purge-chain 

./target/release/node-template --help 

./target/release/node-template --dev

./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json


This only works in linux: 
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json

This is how we launch it on a remote server: 

./target/release/node-template \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --base-path /tmp/node \
    --port 30333 \
    --ws-port 9944 \
    --rpc-port 9933 \
    --rpc-cors all \
    --validator \
    --ws-external \
    --rpc-external \
    --rpc-methods=Unsafe \
    --prometheus-external \
    --name immers-node


This is how we connect to a remote server: 

./target/release/node-template --chain local --base-path /tmp/bob --bob --port 30334  --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"  --validator --bootnodes /ip4/109.248.175.119/tcp/30333/p2p/12D3KooWB3JPDSEpyPLowpRQsR4e1fLZToNrJuJL3oiJetMkfuku 



./target/release/node-template --chain=local --base-path /tmp/validator1 --alice --port 30333 --ws-port 9944 --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a 

./target/release/node-template --chain=local --base-path /tmp/validator2 --bob --port 30334 --ws-port 9945 --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 


./target/release/node-template --chain local --base-path /tmp/validator3 --alice --port 30333 --ws-port 9945 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator


./target/release/node-template --chain local --base-path /tmp/bob  --bob --port 30334 --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp