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
import { AuthenticationObject, ResetRootObject } from "../src/generated";
import { BigNumber } from "big-number";
import {
    ComputeBudgetProgram,
    PublicKey,
    Transaction
} from "@solana/web3.js";
//import { MerkleTree, createHash as merkleCreateHash } from '@guildofweavers/merkle';
import { MerkleTree } from "merkletreejs";
import { createHashChain, createMerkleTree, generateTotpObject, readfileData, writeFileData, calculatePreImage, createMerkleProofPath } from "../tests/fileCreation"



describe("Confirm Bunkr Root", () => {
    // Configure the client to use the local cluster.
    const providerAnchor = anchor.AnchorProvider.env();
    anchor.setProvider(providerAnchor);
    const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

    const wallet = anchor.Wallet.local();
    console.log("wallet: ", wallet.publicKey.toBase58());


    it("Confirm Bunkr Root", async () => {

        const bunkrAccount = findProgramAddressSync([Buffer.from("bunkr"), wallet.payer.publicKey.toBuffer()], program.programId)[0]
        let accountData = await program.account.bunkr.fetch(bunkrAccount);
        const initTime = accountData.initTime;
        const hashImage = accountData.currentHash;

        const integer = Math.floor(((Date.now() / 1000) - initTime) / 30);
        const data = readfileData("test.bin");
        let leaves: Buffer[] = [];
        for (let i = 0; i < data.length; i += 32) {
            leaves.push(data.subarray(i, i + 32));
        }
        const tree = new MerkleTree(leaves, SHA256);
        const root = tree.getRoot();
        console.log("🚀 ~ file: vault-program.ts:333 ~ it ~ root:", root.toString("hex"));
        const onchainRoot = accountData.root;
        let code = "426153";
        console.log("🚀 ~ file: vault-program.ts:339 ~ it ~ integer:", integer)
        const proof = createMerkleProofPath(tree, integer, leaves)
        const otpHash = createHash("sha256").update(Buffer.from(code)).digest();
        const leaf = createHash("sha256").update(Buffer.concat([otpHash, Buffer.from(integer.toString())])).digest();
        console.log("🚀 ~ file: vault-program.ts:342 ~ it ~ leaf:", leaf.toString('hex'))
        const { hash, attempts } = calculatePreImage(Buffer.from(hashImage), "PASSWORD", Math.pow(2, 20));
        console.log("🚀 ~ file: vault-program.ts:346 ~ it ~ attempts:", attempts)
        const authObject: AuthenticationObject = {
            passwordHash: [...hash],
            otpHash: [...otpHash],
            proofPath: proof,
        }

        const tx = await program.methods.confirmRoot(authObject)
            .accounts({
                bunkr: bunkrAccount,
            }
            )
            .signers([wallet.payer])
            .rpc()
            .catch(console.log);

        console.log("Your transaction signature", tx);

        accountData = await program.account.bunkr.fetch(bunkrAccount);
        console.log("🚀 ~ file: vault-program.ts:307 ~ it ~ data:", accountData)
    });

});


