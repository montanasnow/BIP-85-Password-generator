# BIP85 Password Generator


## usage 

Generate passwords of a given length based on the propsal to modify BIP 85

Must supply an extended private key
* `bip85_password <XPRV>`

Options 
-L password length
-I index

Example,
* `bip85_password.exe xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb -L 15 -I 20 `

Generates password m0i9JxW/0X7EJ2a

Based on https://coldcard.com/docs/bip85-passwords and https://github.com/scgbckbone/bips/blob/passwords/bip-0085.mediawiki#PWD and https://github.com/scgbckbone/btc-hd-wallet/blob/master/tests/test_bip85.py

## tests
xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb

pwd-len=20, index=0
RrH7uVI0XlpddCbiuYV+

pwd-len=21, index=0
dKLoepugzdVJvdL56ogNV

vtV6sdNQTKpuefUMOHOKwUp1
pwd-len=24, index=0

pwd-len=32, index=1234
mBhJgXCJd6IpdOu1cc/D1wU+5sxj/1tK

pwd-len=64, index=1234
HBqosVLBhKneX8ZCZgLdvmA8biOdUV2S/AteE5Rs8sMT0pfG3aItk/IrHGEpY9um

pwd-len=86, index=1234
7n3VQ63qjgY6OJBQxqWYToNRfzzN5J8DwN1D8JqlZfnsF+1LdPXG3gkOXighX4iKyKip8nRIhVVVObh/G41F7g
