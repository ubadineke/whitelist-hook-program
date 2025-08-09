# Permit Hook Whitelist Governance (Solana / Anchor)

A Solana Anchor program for community-governed whitelisting of transfer hooks used by downstream protocols (e.g., AMMs). The flow is simple:

- Propose a hook program ID with an audit hash
- Vote with RAY token balance as weight, during a 7‑day window
- Finalize after voting ends; approved hooks are added to the whitelist
- Downstream programs call a lightweight `check_hook` to verify a hook is whitelisted

## Quick Start

- Install prerequisites
  - Rust + cargo, Solana CLI, Anchor (0.31.x), Node.js (>=18), Yarn
- Configure localnet
  - `solana config set -u localnet`
  - `solana-keygen new` (if you need a local keypair)
- Install JS deps: `yarn install`
- Build program: `anchor build`
- Run tests: `anchor test`

Program ID (localnet): `FLCeHJtrs6ENYehB6BC3TctxHUPqzsquBGqhJHgQnyE3`

## Architecture

- Program crate: `programs/permit-hook`
- Language: Rust (Anchor 0.31)
- Testing: TypeScript (Mocha) under `tests/`

### Accounts

- `Whitelist`

  - `admin: Pubkey` — the initializer/administrator
  - `hooks: Vec<Pubkey>` — approved hook program IDs (max_len = 10)
  - `proposals: Vec<u64>` — IDs of active/past proposals (max_len = 20)
  - `vote_threshold: u64` — minimum “for” votes needed for approval (in raw RAY units)
  - PDA: seeds = `[b"whitelist"]`

- `HookProposal`
  - `hook_id: Pubkey` — candidate hook program ID
  - `audit_hash: [u8; 32]` — hash of an external audit/report
  - `votes_for: u64`, `votes_against: u64`
  - `proposer: Pubkey`
  - `active: bool`
  - `id: u64`
  - `created_at: i64` — unix timestamp
  - PDA: seeds = `[b"proposal", proposal_id.to_le_bytes()]`

### Constants and Events

- `RAY_TOKEN_MINT`: `4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R`
- Event `HookApproved { hook_id, proposal_id, votes_for, votes_against }`

## Instruction Set

Note: Instruction handler names below reflect the source files. Anchor clients camelCase method names from these (e.g., `initializeWhitelist`, `proposeHook`, ...).

1. Initialize whitelist

- Accounts: `admin (signer, payer)`, `whitelist (PDA)`, `system_program`
- Effect: creates `Whitelist`, sets `admin`, clears lists, and sets a default `vote_threshold`
- Default threshold in code: `1_000_000_000` raw RAY units (equals 1 RAY if 9 decimals)

2. Propose a hook

- Args: `proposal_id: u64`, `hook_id: Pubkey`, `audit_hash: [u8; 32]`
- Accounts: `proposal (PDA)`, `whitelist (mut)`, `proposer (signer, payer)`, `system_program`
- Effect: creates a `HookProposal`, adds `proposal_id` to `whitelist.proposals`
- Voting window begins at `created_at` and lasts 7 days

3. Vote on a proposal

- Args: `proposal_id: u64`, `vote_for: bool`
- Accounts: `proposal (mut)`, `voter_token_account (SPL token account)`, `voter (signer)`
- Constraints:
  - `voter_token_account.mint == RAY_TOKEN_MINT`
  - `voter_token_account.owner == voter`
- Effect: adds the token account balance to `votes_for` or `votes_against`
- Guardrails: proposal must be `active` and within the 7‑day voting window

4. Finalize a proposal

- Args: `proposal_id: u64`
- Accounts: `whitelist (mut)`, `proposal (mut)`
- Allowed only after the 7‑day window ends
- If `votes_for > votes_against` AND `votes_for >= whitelist.vote_threshold`, then:
  - `hook_id` is pushed into `whitelist.hooks`
  - `HookApproved` event is emitted
- Marks proposal `active = false`

5. Check hook (for downstream programs)

- Args: `hook_id: Pubkey`
- Accounts: `whitelist`
- Effect: verifies `hook_id` exists in `whitelist.hooks`; otherwise errors with `HookNotWhitelisted`

### Errors

- `Unauthorized`
- `HookNotWhitelisted`
- `ProposalInactive`
- `VotingPeriodEnded`
- `VotingPeriodActive`
- `InvalidTokenAccount`
- `InvalidTokenOwner`

## Governance Flow

1. Initialize the whitelist PDA once
2. Propose a new hook: provide the hook program ID and an `audit_hash`
3. Token holders vote during the 7‑day period using their RAY token account balance as weight
4. After the period, any party may finalize; if thresholds pass, the hook is whitelisted
5. Downstream protocols call `check_hook` to enforce the whitelist

Time window: 7 days from `proposal.created_at`.

## Example (TypeScript, Anchor client)

The example below shows the PDA derivations and method calls expected by the program. Adjust names to match your generated IDL camelCase methods.

```ts
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.permitHook; // as Program<PermitHook>

// 1) Initialize whitelist
const [whitelistPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("whitelist")],
  program.programId
);
await program.methods
  .initializeWhitelist()
  .accounts({
    admin: provider.wallet.publicKey,
    whitelist: whitelistPda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

// 2) Propose a hook
const proposalId = new anchor.BN(1);
const [proposalPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("proposal"), proposalId.toArrayLike(Buffer, "le", 8)],
  program.programId
);
const hookProgramId = new PublicKey("HooK1111111111111111111111111111111111111");
const auditHash = Buffer.alloc(32); // fill with your hash
await program.methods
  .proposeHook(proposalId, hookProgramId, [...auditHash])
  .accounts({
    proposal: proposalPda,
    whitelist: whitelistPda,
    proposer: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

// 3) Vote (for)
const voterTokenAccount = new PublicKey("<your RAY associated token account>");
await program.methods
  .voteHook(proposalId, true)
  .accounts({
    proposal: proposalPda,
    voterTokenAccount,
    voter: provider.wallet.publicKey,
  })
  .rpc();

// 4) Finalize (after 7 days)
await program.methods
  .finalizeProposal(proposalId)
  .accounts({
    whitelist: whitelistPda,
    proposal: proposalPda,
  })
  .rpc();

// 5) Check hook (downstream)
await program.methods
  .checkHook(hookProgramId)
  .accounts({ whitelist: whitelistPda })
  .rpc();
```

## Build, Deploy, Test

- Build: `anchor build`
- Localnet deploy: `anchor deploy`
- Test: `anchor test` (uses `ts-mocha`, see `Anchor.toml` scripts)

`Anchor.toml` is configured for `localnet` with wallet `~/.config/solana/id.json` and yarn as the package manager.

## Directory Layout

- `programs/permit-hook/src`
  - `states.rs` — `Whitelist`, `HookProposal`
  - `instructions/` — instruction handlers
  - `errors.rs`, `events.rs`, `consts.rs`
  - `lib.rs` — program module (entrypoints)
- `tests/` — mocha tests
- `migrations/` — deployment scripts (optional)
- `Anchor.toml` — Anchor config
- `Cargo.toml`, `Cargo.lock` — Rust crate config
- `package.json`, `tsconfig.json`, `yarn.lock` — TypeScript tooling

## Notes and Caveats

- Voting weight is the current RAY token account balance at the time of each vote; repeat votes are not constrained in-code (no per-voter tracking)
- Default `vote_threshold` is a placeholder; change to suit your governance scale
- The 7‑day window is enforced via on-chain timestamps and is not configurable in this version
- `check_hook` is intentionally lightweight for downstream integration
- Security: ensure `audit_hash` is derived from a verifiable source; consider adding admin controls or proposal filters as needed

## License

ISC (see `package.json`).
