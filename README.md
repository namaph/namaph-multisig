# multisig-cpi
This is an example using the ['project-serum/multisig'](https://github.com/project-serum/multisig) program to execute an external transaction.
This attempts to demonstrate the multi-sig programs' capability to create and execute arbitrary transactions with multiple public keys (or potentially PDAs).

In its simplest terms, a DAO can be a multi-sig. Obviously things are lacking, but this might be a good starting point. Keep in mind if you are looking for a complete DAO solution, you will want to look into [spl-governance](https://github.com/solana-labs/solana-program-library/tree/master/governance). 
This repo was meant to be entirely educational for me to understand the multi-sig program and cross-program invocations. Please use at your own risk.


## How to run `anchor test`

You will need to have the 'multisig' program inside the `programs` folder.

```bash
git clone https://github.com/project-serum/multisig
cp -r multisig multisig-cpi/programs/
```

It was tested using this [`fca07d467003ac417df026736b27682a342efb79`](https://github.com/project-serum/multisig/tree/fca07d467003ac417df026736b27682a342efb79) commit.

*Please let me know if there is a better way to use the copy (reference) a program!*

After this, you may want to change `Anchor.toml` and `declare_id!()`(in both `lib.rs` in the programs) macros
to correspond to the generated program keypair.

The host program itself is left intentionally extremely simple! The `multisig-cpi.ts` test file may be more useful to decipher how it works.
