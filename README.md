# sethfip
a CLI utility that is used to upload a file to IPFS and store its hash in Ethereum smart contract

## Basic usage
`sethfip --account 0x084c7D6B56267b811748A1Af3b3973da95641f50 upload --input vitalik_rocks.jpg`

`sethfip --account 0x084c7D6B56267b811748A1Af3b3973da95641f50 download`

## Setting up dev environment
1. Run local blockchain with `ganache-cli`
2. Run IPFS node with `ipfs daemon`
3. `cd sethfip_core`
4. Migrate smart contract `truffle migrate`

## Weird name?
Yup, just mixed characters from "eth ipfs" untill it made some sense.
