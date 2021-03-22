# Utils

A subcrate called `utils` provides extra tools to help automation of `rosetta-cli` checks.

## Faucet

Interacts with the testnet Faucet to make sure that a prefunded account can be used during `check:construction`.

## Snapshot

Fetches a snapshot to be used to create a `bootstrap_balances.json` file with the balances of every single address in the ledger. Used when running `check:data` with pruning enabled.