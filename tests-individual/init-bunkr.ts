import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { VaultProgram } from "../target/types/vault_program";
import { getAccount, createMint, mintTo, getOrCreateAssociatedTokenAccount, createSetAuthorityInstruction, AuthorityType, getMint, transfer, closeAccount, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PROGRAM_ADDRESS as MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { mintTestNFT, getAccountData } from "../tests/utils";
import { BN, min } from "bn.js";
import { assert } from "chai";
import { USDC_TOKEN_ACCOUNT, USDC_TOKEN_MINT } from "../tests/constants";
import { createHash } from "crypto";
import { SHA256 } from "crypto-js"
import { AuthenticationObject, HashTuple, hashTupleBeet, InitBunkrData } from "../src/generated";
import { BigNumber } from "big-number";
import {
    ComputeBudgetProgram,
    PublicKey,
    Transaction
} from "@solana/web3.js";
//import { MerkleTree, createHash as merkleCreateHash } from '@guildofweavers/merkle';
import { MerkleTree } from "merkletreejs";
import { createHashChain, createMerkleTree, generateTotpObject, readfileData, writeFileData, calculatePreImage, createMerkleProofPath } from "../tests/fileCreation"



describe("Init Bunkr", () => {
    // Configure the client to use the local cluster.
    const providerAnchor = anchor.AnchorProvider.env();
    anchor.setProvider(providerAnchor);
    const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

    const wallet = anchor.Wallet.local();
    console.log("wallet: ", wallet.publicKey.toBase58());


    it("Initialize Bunkr Account", async () => {
        const { link, otps, initTime } = generateTotpObject(Math.pow(2, 20));
        console.log("🚀 ~ file: vault-program.ts:182 ~ it ~ link:", link)
        const tree = new MerkleTree(otps, SHA256)
        const root = tree.getRoot();
        const passwordHash = createHashChain("PASSWORD", Math.pow(2, 20));
        const finalPasswordHash = createHash("sha256").update(Buffer.concat([createHash("sha256").update(Buffer.from("PASSWORD")).digest(), Buffer.from("FINAL")])).digest();
        const resetHash = createHashChain("RESETPASSWORD", Math.pow(2, 20));
        const finalResetHash = createHash("sha256").update(Buffer.concat([createHash("sha256").update(Buffer.from("RESETPASSWORD")).digest(), Buffer.from("FINAL")])).digest();

        const initBunkrData: InitBunkrData = {
            name: "Test Bunkr",
            initTime: initTime,
            root: [...root],
            initialHash: [...passwordHash],
            finalHash: [...finalPasswordHash],
            initialResetHash: [...resetHash],
            finalResetHash: [...finalResetHash],
            shadowDriveSpace: "BpkFF4TDHUpyX8wana3a9NUd6xZXoC4RN1c4nv1PtaDm"
        }

        writeFileData("test.bin", Buffer.concat(otps));

        const bunkrAccount = findProgramAddressSync([Buffer.from("bunkr"), wallet.payer.publicKey.toBuffer()], program.programId)[0]
        const tx = await program.methods.initBunkr(initBunkrData)
            .accounts({
                bunkr: bunkrAccount,
                reimbursementWallet: new web3.PublicKey("B93EYGbkngEgApWYjtrLxXR5T4w28B3ErZeENStkCsx8")
            })
            .signers([wallet.payer])
            .rpc()
            .catch(console.log);



        console.log("Your transaction signature", tx);

        const data = await program.account.bunkr.fetch(bunkrAccount);
        console.log("🚀 ~ file: vault-program.ts:307 ~ it ~ data:", data)
    });

});


