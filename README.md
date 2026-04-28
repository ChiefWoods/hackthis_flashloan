# Flash loan exploit

## How the exploit works:

The vulnerable `flash_repay` logic scans the instruction sysvar and repays based on the visible
top-level `flash_loan` amount. The exploit places a harmless top-level `flash_loan(0)` first, then
calls `dummy::drain`, which does a CPI into `flash_loan` for the real vault amount. CPI calls are
not visible as top-level instructions in that scan, so `flash_repay` only charges the fee from the
`flash_loan(0)` and does not account for the large CPI loan. Result: the vault is drained except
for rent + fee.

## Steps to replicate the exploit:

1) Sync program id

```
anchor keys sync
```

2) Build the program

```
anchor build
```

3) Run the LiteSVM POC:

```
cargo test exploit -- --no-capture
```

4) Deploy the program:

```
anchor deploy -p dummy --provider.cluster d --program-keypair target/deploy/dummy-keypair.json --no-idl
```

5) Execute the exploit:

```
cargo run -p exploit
```

Example devnet exploit: [629ty29CgHEJtd2EoWjET5ibd3GBfQpr7VmunLvtGKnV9YYYic6b6xE25uMuMLQP5zgSCrYevPa9tijzzk4gNL7b](https://explorer.solana.com/tx/629ty29CgHEJtd2EoWjET5ibd3GBfQpr7VmunLvtGKnV9YYYic6b6xE25uMuMLQP5zgSCrYevPa9tijzzk4gNL7b?cluster=devnet)