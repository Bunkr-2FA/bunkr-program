import * as anchor from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { VaultProgram } from "../target/types/vault_program";
import { BUNKR_PROGRAM_ID } from "./constants";
import { SystemProgram, sendAndConfirmTransaction } from "@solana/web3.js";
import {
    MINT_SIZE,
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    createInitializeMintInstruction,
    createMintToCheckedInstruction,
    getAssociatedTokenAddress,
    getMinimumBalanceForRentExemptMint,
    getAccount
} from "@solana/spl-token";
import {
    createCreateMasterEditionInstruction,
    createCreateMetadataAccountInstruction,
    PROGRAM_ID as MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";


export const getProgram = () => {
    const provider = anchor.getProvider();

    const program = new anchor.Program<VaultProgram>(
        require("../target/idl/vault_program.json"),
        BUNKR_PROGRAM_ID,
        provider
    );

    return program;
};

export const getTestVault = (
    authority: anchor.web3.PublicKey,
) => {
    return findProgramAddressSync(
        [Buffer.from("testvault"),
        authority.toBuffer(),],
        getProgram().programId,
    )[0];
};

export const mintTestNFT = async (
    connection: anchor.web3.Connection,
    feePayer: anchor.web3.Keypair
): Promise<{ mint: anchor.web3.PublicKey, ata: anchor.web3.PublicKey, edition: anchor.web3.PublicKey }> => {
    const mint = anchor.web3.Keypair.generate();

    const ata = await getAssociatedTokenAddress(
        mint.publicKey,
        feePayer.publicKey
    );

    const tokenMetadataPubkey = await getMetadataPDA(mint.publicKey);

    const masterEditionPubkey = await getMasterEditionPDA(mint.publicKey);

    const tx = new anchor.web3.Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey,
            newAccountPubkey: mint.publicKey,
            lamports: await getMinimumBalanceForRentExemptMint(connection),
            space: MINT_SIZE,
            programId: TOKEN_PROGRAM_ID,
        }),
        createInitializeMintInstruction(
            mint.publicKey,
            0,
            feePayer.publicKey,
            feePayer.publicKey
        ),
        createAssociatedTokenAccountInstruction(
            feePayer.publicKey,
            ata,
            feePayer.publicKey,
            mint.publicKey
        ),
        createMintToCheckedInstruction(
            mint.publicKey,
            ata,
            feePayer.publicKey,
            1,
            0
        ),
        createCreateMetadataAccountInstruction(
            {
                metadata: tokenMetadataPubkey,
                mint: mint.publicKey,
                mintAuthority: feePayer.publicKey,
                payer: feePayer.publicKey,
                updateAuthority: feePayer.publicKey,
            },
            {
                createMetadataAccountArgs: {
                    data: {
                        name: "Test",
                        symbol: "TEST",
                        uri: "https://arweave.net/omJ0Rdesa2Zfq35mmIsabbXQyz8mgLmxN_Y-LODCgBw",
                        sellerFeeBasisPoints: 100,
                        creators: [
                            {
                                address: feePayer.publicKey,
                                verified: true,
                                share: 100,
                            },
                        ],
                    },
                    isMutable: true,
                },
            }
        ),
        createCreateMasterEditionInstruction(
            {
                edition: masterEditionPubkey,
                mint: mint.publicKey,
                updateAuthority: feePayer.publicKey,
                mintAuthority: feePayer.publicKey,
                payer: feePayer.publicKey,
                metadata: tokenMetadataPubkey,
            },
            {
                createMasterEditionArgs: {
                    maxSupply: 0,
                },
            }
        )
    );

    console.log("Minting NFT...");

    await sendAndConfirmTransaction(connection, tx, [feePayer, mint]);

    return { mint: mint.publicKey, ata: ata, edition: masterEditionPubkey };
};

async function getMetadataPDA(mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> {
    const [publicKey] = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
        ],
        MPL_TOKEN_METADATA_PROGRAM_ID
    );
    return publicKey;
}

async function getMasterEditionPDA(
    mint: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> {
    const [publicKey] = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
        ],
        MPL_TOKEN_METADATA_PROGRAM_ID
    );
    return publicKey;
}

export const getAccountData = async (connection: anchor.web3.Connection, pubkey: anchor.web3.PublicKey) => {
    const accountInfo = await getAccount(connection, pubkey);
    if (accountInfo === null) {
        throw "Error: cannot find the account";
    }
    return accountInfo;

}