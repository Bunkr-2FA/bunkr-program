import { SystemProgram, sendAndConfirmTransaction } from "@solana/web3.js";
import { Constants } from "../constants";
import * as programsUtils from "./programs.utils";
import { web3 } from "@project-serum/anchor";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  createMintToCheckedInstruction,
  getAssociatedTokenAddress,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import {
  createCreateMasterEditionInstruction,
  createCreateMetadataAccountInstruction,
  PROGRAM_ID as MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";

export const creatorsFromStakepool = async (
  poolAddress: string
): Promise<string[]> => {
  const stakePoolData = await programsUtils.getStakePool(
    Constants.connection,
    new web3.PublicKey(poolAddress)
  );
  return stakePoolData.parsed.requiresCreators.map((creator) => {
    return creator.toString();
  });
};

export const getBatchedMultipleAccounts = async (
  connection: web3.Connection,
  ids: web3.PublicKey[],
  config?: web3.GetMultipleAccountsConfig
): Promise<(web3.AccountInfo<Buffer | web3.ParsedAccountData> | null)[]> => {
  const batches = ids.reduce((acc, id, i) => {
    if (i % 100 === 0) {
      acc.push([]);
    }
    acc[acc.length - 1].push(id);
    return acc;
  }, [] as web3.PublicKey[][]);

  const batchAccounts = await Promise.all(
    batches.map(
      async (b) =>
        await connection.getMultipleAccountsInfo(
          b,
          config as unknown as web3.Commitment
        )
    )
  );

  return batchAccounts.flat();
};

export const mintTestNFT = async (
  connection: web3.Connection,
  feePayer: web3.Keypair
): Promise<Array<web3.PublicKey>> => {
  const mint = web3.Keypair.generate();

  const ata = await getAssociatedTokenAddress(
    mint.publicKey,
    feePayer.publicKey
  );

  const tokenMetadataPubkey = await getMetadataPDA(mint.publicKey);

  const masterEditionPubkey = await getMasterEditionPDA(mint.publicKey);

  const tx = new web3.Transaction().add(
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
            uri: "https://tkiouu5r6rofwnbuiatpq3rkh475kf5yeoewtzmbd7454rijczcq.arweave.net/mpDqU7H0XFs0NEAm-G4qPz_VF7gjiWnlgR_53kUJFkU",
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

  return [mint.publicKey, ata];
};

async function getMetadataPDA(mint: web3.PublicKey): Promise<web3.PublicKey> {
  const [publicKey] = await web3.PublicKey.findProgramAddress(
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
  mint: web3.PublicKey
): Promise<web3.PublicKey> {
  const [publicKey] = await web3.PublicKey.findProgramAddress(
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
