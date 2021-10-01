# Solana Learning Notes

* Smart contracts on solana are called programs
* Programs can be written in any language that supports LLVM backend. Programs are stored in BPF bytecode format.
* Programs are stateless. Programs cannot store any data. Accounts are used to store state.
* Programs are themselves stored in accounts. These accounts are specially marked as "executables".
* Each account can hold `data` and `SOL`.
* Every account has an owner and only the owner can debit and change account's data.
* Accounts can only owned by Programs.
* All Executable Accounts are owned by BPF Loader.
* All Basic Accounts are owned by System Program.
* Even Though, System Program owns all the basic accounts. It can debit only when the transaction is signed by the private key of that account.
* BPF Loader and System Program are owned by NativeLoader.
* In theory, programs have full autonomy over the accounts they own. It is up to the program's creator to limit this autonomy and up to the users of the program to verify the program's creator has really done so
* All accounts to be read or written to must be passed into the entrypoint function
* Entrypoint structure of a program depends on the BPF Loader being used.

### General Program Structure

```
.
├─ src
│  ├─ lib.rs -> registering modules
│  ├─ entrypoint.rs -> entrypoint to the program
│  ├─ instruction.rs -> program API, (de)serializing instruction data
│  ├─ processor.rs -> program logic
│  ├─ state.rs -> program objects, (de)serializing state
│  ├─ error.rs -> program specific errors
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ Xargo.toml
```

- instructions.rs defines the API.

---

### Token Program

- A Token account is required to hold tokens created by Token Program.
- Owner of a Token *asssociated* account is stored in the token owner attribute.
- **token account owner attribute** is different from **account owner**.
- **account owner** is stored in internal information of solana account.
- **token account owner attribute** is stored in the userspace storage of that account along with other attributes. userspace storage is the **data** field of a account. As the owner of token account is stored in it's userspace (not internal information) the private key of that account doesn't have much of a use.
- The token account owner attribute will be set to Owner's address allowing the owner to transfer the tokens present in that account using their private key.

```rust=
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If is_some, this is a native token, and the value logs the rent-exempt reserve. An Account
    /// is required to be rent-exempt, so the value is used by the Processor to ensure that wrapped
    /// SOL accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
}
```

- Every Token Account references the mint account which is used to check which token this particular account holds.
- All the Token Accounts and Mint accounts are owned by Token Program (has permission to change `data` and `SOL` amount).


---

- Solana has sysvars that are parameters of the Solana cluster you are on. These sysvars can be accessed through accounts and store parameters such as what the current fee or rent is. As of solana-program version 1.6.5, sysvars can also be accessed without being passed into the entrypoint as an account.

- Rent is deducted from an account's balance according to their space requirements regularly. An account can, however, be made rent-exempt if its balance is higher than some threshold that depends on the space it's consuming

- Program Derived Address (PDAs) are public keys which are derived from the Program Public Key (`program_id`) and seeds. PDAs are not present on ed25519 curve as a result they don't have private keys.
- When including a signed account in a program call, in all CPIs including that account made by that program inside the current instruction, the account will also be signed, i.e. the signature is extended to the CPIs

- There can be several instructions (ix) inside one transaction (tx) in Solana. These instructions are executed out synchronously and the tx as a whole is executed atomically. These instructions can call different programs.
- The system program is responsible for allocating account space and assigning (internal - not user space) account ownership


- Instructions may depend on previous instructions inside the same transaction
- Commitment settings give downstream developers ways to query the network which differ in finality likelihood

- when a program calls invoke_signed, the runtime uses those seeds and the program id of the calling program to recreate the PDA and if it matches one of the given accounts inside invoke_signed's arguments, that account's signed property will be set to true
- If an account has no balance left, it will be purged from memory by the runtime after the transaction
- In any call to a program that is of the "close" kind, i.e. where you set an account's lamports to zero so it's removed from memory after the transaction, make sure to either clear the data field or leave the data in a state that would be OK to be recovered by a subsequent transaction
- "closing" instructions must set the data field properly, even if the intent is to have the account be purged from memory after the transaction
