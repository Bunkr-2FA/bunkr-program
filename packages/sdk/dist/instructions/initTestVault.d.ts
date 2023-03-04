import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
export declare const initTestVaultStruct: beet.BeetArgsStruct<{
    instructionDiscriminator: number[];
}>;
export type InitTestVaultInstructionAccounts = {
    testVault: web3.PublicKey;
    signer: web3.PublicKey;
    systemProgram?: web3.PublicKey;
    anchorRemainingAccounts?: web3.AccountMeta[];
};
export declare const initTestVaultInstructionDiscriminator: number[];
export declare function createInitTestVaultInstruction(accounts: InitTestVaultInstructionAccounts, programId?: web3.PublicKey): web3.TransactionInstruction;
