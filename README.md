# Solana Lookup Table

An example showing how to use Solana Lookup Tables to "compress" public keys together with Anchor.

# Run

Install [Anchor](https://www.anchor-lang.com/docs/installation).

Make sure to have a Solana keypair generated (`solana address` should output something).  
If you don't, create one with `solana-keygen new`.

Then:

```bash
git clone https://github.com/GuidoDipietro/solana-lut.git
yarn
yarn add ts-mocha
anchor test
```
