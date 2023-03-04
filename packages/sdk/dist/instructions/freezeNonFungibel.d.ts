import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
export declare const freezeNonFungibelStruct: beet.BeetArgsStruct<{
    instructionDiscriminator: number[];
}>;
export type FreezeNonFungibelInstructionAccounts = {
    tokenAccount: web3.PublicKey;
    tokenMint: web3.PublicKey;
    tokenMintEdition: web3.PublicKey;
    signer: web3.PublicKey;
    delegate: web3.PublicKey;
    tokenProgram?: web3.PublicKey;
    tokenMetadataProgram: web3.PublicKey;
    anchorRemainingAccounts?: web3.AccountMeta[];
};
export declare const freezeNonFungibelInstructionDiscriminator: number[];
export declare function createFreezeNonFungibelInstruction(accounts: FreezeNonFungibelInstructionAccounts, programId?: web3.PublicKey): web3.TransactionInstruction;
