#!/usr/bin/env bash

cd ~ || exit
rm -rf ~/.wasmd
rm -rf ~/wasmd
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd || exit
make install
wasmd init wte --chain-id=wte
wasmd keys add alice --keyring-backend=test
wasmd keys add bob --keyring-backend=test
wasmd keys add cecil --keyring-backend=test
wasmd keys add dave --keyring-backend=test
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account cecil "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account dave "1000000000000stake" --keyring-backend=test
wasmd genesis gentx alice "250000000stake" --chain-id=wte --amount="250000000stake" --keyring-backend=test
wasmd genesis collect-gentxs
wasmd start
