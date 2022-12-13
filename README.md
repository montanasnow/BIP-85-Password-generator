# BIP85 Password Generator

Generate a password of a given length and index based on an BIP32 Root Key and the proposal to modify BIP85 god at https://github.com/scgbckbone/bips/blob/passwords/bip-0085.mediawiki#PWD. This has been implemented by Coldcard https://coldcard.com/docs/bip85-passwords. Other BIP85 implementations are available at https://iancoleman.io/bip39/. 

The derivation path is m/83696968'/707764'/{pwd_len}'/{index}' Base64 encode the all 64 bytes of entropy. Remove any spaces or new lines inserted by Base64 encoding process. Slice base64 result string on index 0 to `pwd_len`. This slice is the password. As `pwd_len` is limited to 86, passwords will not contain padding.

## Usage
Supply a BIP32 Root Key or BIP-39 mnemonic phrase
* `bip85_password xprv...`
* `bip85_password 'mnemonic phrase'`

Options:
* -L password length (default 21)
* -I index (default 0)

The password length should be between 20 and 86 per the spec, but the minimum length is not enforced. The index must be less than 2,147,483,648 (2^31)

Example:
* `bip85_password xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb -L 15 -I 20 `
* `bip85_password 'wife shiver author away frog air rough vanish fantasy frozen noodle athlete pioneer citizen symptom firm much faith extend rare axis garment kiwi clarify'-L 15 -I 20 `

Generates password m0i9JxW/0X7EJ2a

Based on https://coldcard.com/docs/bip85-passwords and https://github.com/scgbckbone/bips/blob/passwords/bip-0085.mediawiki#PWD and https://github.com/scgbckbone/btc-hd-wallet/blob/master/tests/test_bip85.py

## Tests
BIP32 Root Key: `xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb` (from https://github.com/scgbckbone/bips/blob/passwords/bip-0085.mediawiki#PWD)

Password length 20, index 0 
`RrH7uVI0XlpddCbiuYV+`

Password length 21, index 0
`dKLoepugzdVJvdL56ogNV`

`Password length 24, index 0
`vtV6sdNQTKpuefUMOHOKwUp1`

Password length 32, index 1234 (from https://github.com/scgbckbone/btc-hd-wallet/blob/master/tests/test_bip85.py)
`mBhJgXCJd6IpdOu1cc/D1wU+5sxj/1tK`

Password length 64, index 1234
`HBqosVLBhKneX8ZCZgLdvmA8biOdUV2S/AteE5Rs8sMT0pfG3aItk/IrHGEpY9um`

Password length 86, index 1234
`7n3VQ63qjgY6OJBQxqWYToNRfzzN5J8DwN1D8JqlZfnsF+1LdPXG3gkOXighX4iKyKip8nRIhVVVObh/G41F7g`

BIP-39 mnemonic: `wife shiver author away frog air rough vanish fantasy frozen noodle athlete pioneer citizen symptom firm much faith extend rare axis garment kiwi clarify` 
From https://coldcard.com/docs/bip85-passwords. Coldcard only generates passwords with length 21). The mnemonic is equivalent to the BIP32 Root Key above.

Password length 21, index 0 `m/83696968'/707764'/21'/0'`
`BSdrypS+J4Wr1q8DWjbFE`

Password length 21, index 8 
`U4RD0R0A0RjpHOFtwnv9k`