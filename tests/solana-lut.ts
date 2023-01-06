import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { findProgramAddressSync } from "@coral-xyz/anchor/dist/cjs/utils/pubkey";
import { assert } from "chai";
import { SolanaLut } from "../target/types/solana_lut";
import {
  compileBuildSignAndSendIxs,
  createLut,
  extendLut,
  sleep,
} from "./utils";

describe("Solana LUT", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaLut as Program<SolanaLut>;

  let lutPubkey: anchor.web3.PublicKey;
  let addresses: anchor.web3.PublicKey[];

  before(async () => {
    // Create address LUT
    lutPubkey = await createLut(provider);
    await sleep(1000);

    // Add pubkeys pda_1 to pda_30 to LUT
    addresses = [...Array(30).keys()].map(
      (i) =>
        findProgramAddressSync([Buffer.from(`${i + 1}`)], program.programId)[0]
    );
    await extendLut(provider, lutPubkey, addresses);
    await sleep(1000);
  });

  it(`Can't send a large tx with legacy compiling`, async () => {
    // The transaction has a context with +50 accounts, so this exceeds
    // the limit of 1232 bytes when serialized (that's about 30 or so accounts)
    try {
      await program.methods
        .initPdas()
        .accounts({ signer: provider.wallet.publicKey })
        .rpc();
    } catch (e) {
      assert.ok(e.toString().includes("Transaction too large"));
    }
  });

  it(`Can send the same tx with v0 compiling (using lookup table)`, async () => {
    // We have to init 50 accounts, so we need more compute budget
    let computeBudgetIx = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 400000,
    });

    // This is our custom instruction
    let ix = await program.methods
      .initPdas()
      .accounts({ signer: provider.wallet.publicKey })
      .instruction();

    let lut = (await provider.connection.getAddressLookupTable(lutPubkey))
      .value;
    console.log(`\tLUT has ${lut.state.addresses.length} addresses stored`);

    await compileBuildSignAndSendIxs(provider, [computeBudgetIx, ix], [lut]);
    await sleep(1000);

    // Indeed every PDA was initialized!
    for (let i = 0; i < addresses.length; i++) {
      let pda = await program.account.pda.fetch(addresses[i]);
      assert.equal(pda.value, i + 1);
    }
  });
});
