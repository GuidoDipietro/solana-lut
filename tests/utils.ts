import { AnchorProvider, web3 } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

export const sleep = async (ms: number) =>
  new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Create a Lookup Table on the AddresLookupTableProgram
 * @param provider Anchor provider
 * @returns Lookup Table pubkey
 */
export const createLut = async (
  provider: AnchorProvider
): Promise<web3.PublicKey> => {
  // Get recent confirmed slot, LUT uses this as PDA seed
  let recentSlot = await provider.connection.getSlot({
    commitment: `confirmed`,
  });

  await sleep(1000);

  // Create instruction
  let [createLutIx, lutPubkey] =
    web3.AddressLookupTableProgram.createLookupTable({
      authority: provider.wallet.publicKey,
      payer: provider.wallet.publicKey,
      recentSlot,
    });

  // Sign and send instruction
  await compileBuildSignAndSendIxs(provider, [createLutIx]);

  return lutPubkey;
};

/**
 * Appends a list of addresses to an existing lookup table
 * @param provider Anchor provider
 * @param lutPubkey Address of lookup table to extend
 * @param addresses Addresses to append to lookup table
 */
export const extendLut = async (
  provider: AnchorProvider,
  lutPubkey: web3.PublicKey,
  addresses: web3.PublicKey[]
) => {
  let extendLutIx = web3.AddressLookupTableProgram.extendLookupTable({
    lookupTable: lutPubkey,
    authority: provider.wallet.publicKey,
    payer: provider.wallet.publicKey,
    addresses,
  });

  await compileBuildSignAndSendIxs(provider, [extendLutIx]);
};

/**
 * Given an array of instructions, compile a MessageV0, build transaction, sign, and send
 * @param provider Anchor provider
 * @param ixs Array of Transaction Instructions
 */
export const compileBuildSignAndSendIxs = async (
  provider: AnchorProvider,
  ixs: web3.TransactionInstruction[],
  luts?: web3.AddressLookupTableAccount[]
) => {
  // Compile message
  let blockhash = (await provider.connection.getLatestBlockhash(`confirmed`))
    .blockhash;
  let msg = new web3.TransactionMessage({
    payerKey: provider.wallet.publicKey,
    instructions: ixs,
    recentBlockhash: blockhash,
  }).compileToV0Message(luts);

  // Build and sign transaction
  let tx = new web3.VersionedTransaction(msg);
  tx.sign([(provider.wallet as NodeWallet).payer]);

  // Send
  await provider.connection.sendTransaction(tx);
};
