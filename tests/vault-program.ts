import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { VaultProgram } from "../target/types/vault_program";
import { getAccount, createMint, mintTo, getOrCreateAssociatedTokenAccount, createSetAuthorityInstruction, AuthorityType, getMint, transfer, closeAccount, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PROGRAM_ADDRESS as MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { mintTestNFT, getAccountData } from "./utils";
import { BN, min } from "bn.js";
import { assert } from "chai";
import { USDC_TOKEN_ACCOUNT, USDC_TOKEN_MINT } from "./constants";
import { createHash } from "crypto";
import { SHA256 } from "crypto-js"
import { HashTuple, hashTupleBeet, InitBunkrData } from "../src/generated";
import { BigNumber } from "big-number";
import {
  ComputeBudgetProgram,
  Transaction
} from "@solana/web3.js";
//import { MerkleTree, createHash as merkleCreateHash } from '@guildofweavers/merkle';
import { MerkleTree } from "merkletreejs";
import { createHashChain, createMerkleTree, generateTotpObject, readfileData, writeFileData, calculatePreImage, createMerkleProofPath } from "./fileCreation"



describe("vault-program", () => {
  // Configure the client to use the local cluster.
  const providerAnchor = anchor.AnchorProvider.env();
  anchor.setProvider(providerAnchor);
  const program = anchor.workspace.VaultProgram as Program<VaultProgram>;

  const wallet = anchor.Wallet.local();
  console.log("wallet: ", wallet.publicKey.toBase58());

  let mint, edition, ata, testVault;




  // it("Create Mint & Freeze NonFungible Token", async () => {

  //   const mintObject = await mintTestNFT(anchor.getProvider().connection, wallet.payer)

  //   if (!mintObject?.mint || !mintObject?.ata || !mintObject?.edition) return

  //   mint = mintObject.mint;
  //   ata = mintObject.ata;
  //   edition = mintObject.edition;

  //   console.log("mint: ", mint.toBase58());


  //   testVault = findProgramAddressSync(
  //     [Buffer.from("testvault"),
  //     wallet.publicKey.toBuffer(),],
  //     program.programId,
  //   )[0];

  //   console.log("testVault: ", testVault.toBase58());



  //   const tx = await program.methods.freezeNonFungible().accounts({
  //     tokenAccount: ata,
  //     tokenMint: mint,
  //     delegate: testVault,
  //     signer: wallet.payer.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
  //     tokenMintEdition: edition
  //   })
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log)

  //   console.log("Your transaction signature", tx);
  //   const accountData = await getAccountData(anchor.getProvider().connection, ata);
  //   assert(accountData.isFrozen, "Account is not frozen");
  //   assert(accountData.delegate.toBase58() === testVault.toBase58(), "Delegate is not set to vault");
  //   console.log("Delegate: ", accountData.delegate.toBase58());


  //   // const tx = await program.methods.initialize().rpc();
  //   // console.log("Your transaction signature", tx);
  // });
  // it("Thaw NonFungible Token", async () => {

  //   const tx = await program.methods.thawNonFungible().accounts({
  //     tokenAccount: ata,
  //     tokenMint: mint,
  //     delegate: testVault,
  //     signer: wallet.payer.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
  //     tokenMintEdition: edition
  //   })
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log)

  //   console.log("Your transaction signature", tx);
  //   const accountData = await getAccountData(anchor.getProvider().connection, ata);
  //   assert(!accountData.isFrozen, "Account is not frozen");
  //   console.log("delegate: ", accountData.delegate);
  //   assert(!accountData.delegate, "Delegate still set");

  // });
  // it("Lock Fungible Token", async () => {

  //   const vaultAta = getAssociatedTokenAddressSync(USDC_TOKEN_MINT, testVault, true)
  //   console.log("vaultAta: ", vaultAta.toBase58());

  //   const decimals = await (await getMint(anchor.getProvider().connection, USDC_TOKEN_MINT)).decimals;
  //   console.log("decimals: ", decimals);
  //   const origin_before = await (await getAccount(anchor.getProvider().connection, USDC_TOKEN_ACCOUNT)).amount

  //   let vaultBefore
  //   const vaultAccount = await getAccount(anchor.getProvider().connection, vaultAta).catch(console.log)

  //   vaultBefore = vaultAccount ? vaultAccount.amount : 0

  //   const tx = await program.methods.lockFungible(5).accounts({
  //     fromAssociatedTokenAccount: USDC_TOKEN_ACCOUNT,
  //     toAssociatedTokenAccount: getAssociatedTokenAddressSync(USDC_TOKEN_MINT, testVault, true),
  //     tokenMint: USDC_TOKEN_MINT,
  //     signer: wallet.payer.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //     vault: testVault,
  //     systemProgram: anchor.web3.SystemProgram.programId
  //   })
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log)

  //   console.log("Your transaction signature", tx);
  //   const origin_after = await (await getAccount(anchor.getProvider().connection, USDC_TOKEN_ACCOUNT)).amount;
  //   const vault_after = await (await getAccount(anchor.getProvider().connection, vaultAta)).amount;
  //   assert(origin_after + BigInt(5 * Math.pow(10, decimals)) == origin_before, "Origin account balance is not correct");
  //   assert(vault_after == vaultBefore + BigInt(5 * Math.pow(10, decimals)), "Vault account balance is not correct");
  // });
  // it("Unlock Fungible Token", async () => {




  //   const vaultAta = getAssociatedTokenAddressSync(USDC_TOKEN_MINT, testVault, true)
  //   console.log("vaultAta: ", vaultAta.toBase58());
  //   const decimals = await (await getMint(anchor.getProvider().connection, USDC_TOKEN_MINT)).decimals;

  //   console.log("decimals: ", decimals);
  //   let destinationBefore

  //   const destinationAccount = await getAccount(anchor.getProvider().connection, USDC_TOKEN_ACCOUNT).catch(console.log);

  //   destinationBefore = destinationAccount ? destinationAccount.amount : BigInt(0)

  //   const vault_before = await (await getAccount(anchor.getProvider().connection, vaultAta)).amount;

  //   const tx = await program.methods.unlockFungible(5).accounts({
  //     toAssociatedTokenAccount: getAssociatedTokenAddressSync(USDC_TOKEN_MINT, wallet.publicKey),
  //     fromAssociatedTokenAccount: getAssociatedTokenAddressSync(USDC_TOKEN_MINT, testVault, true),
  //     tokenMint: USDC_TOKEN_MINT,
  //     signer: wallet.payer.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //     vault: testVault,
  //     systemProgram: anchor.web3.SystemProgram.programId
  //   })
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log)

  //   console.log("Your transaction signature", tx);
  //   const destinationAfter = await (await getAccount(anchor.getProvider().connection, USDC_TOKEN_ACCOUNT)).amount;



  //   const vaultAfter = await (await getAccount(anchor.getProvider().connection, vaultAta)).amount;



  //   assert(destinationAfter - BigInt(5 * Math.pow(10, decimals)) == destinationBefore, "Origin account balance is not correct");
  //   assert(vaultAfter == vault_before - BigInt(5 * Math.pow(10, decimals)), "Vault account balance is not correct");
  // });



  // it("Test Hashing", async () => {
  //   const word = "hello";
  //   const amount = 5;

  //   //Building Merkle Tree
  //   const codes = Array.from({ length: Math.pow(2, 19) }, (_, i) => (createHash("sha256").update((i).toString()).digest()));
  //   const values = codes.map((c) => createHash("sha256").update(Buffer.concat([c, Buffer.from((2).toString())])).digest());
  //   // console.log("🚀 ~ file: vault-program.ts:187 ~ it ~ values:", values.map((v) => v.toString("hex")))

  //   const tree = new MerkleTree(values, SHA256)
  //   //console.log("🚀 ~ file: vault-program.ts:190 ~ it ~ tree:", tree.toString())
  //   const root = tree.getRoot();
  //   const proof = tree.getProof(values[20]);
  //   let proofPath: HashTuple[] = [];

  //   for (const element of proof) {
  //     let hashTupleObject: HashTuple = {
  //       hash: [],
  //       siblingIndex: 0
  //     };
  //     hashTupleObject.hash = [...element.data];
  //     if (element.position === "left") {
  //       hashTupleObject.siblingIndex = 0;
  //     }
  //     else if (element.position === "right") {
  //       hashTupleObject.siblingIndex = 1;
  //     }
  //     else throw new Error("Invalid position");
  //     proofPath.push(hashTupleObject);
  //   }
  //   // console.log(proof)
  //   // console.log("🚀 ~ file: vault-program.ts:189 ~ it ~ proofPath:", proofPath.map((v) => v));
  //   // console.log("tree: ", tree.toString());

  //   console.log("Finished building Merkle Tree")

  //   //console.log("🚀 ~ file: vault-program.ts:189 ~ it ~ proof:", proof.map((v) => v.data.toString("hex")))
  //   const tx = await program.methods.check(root, proofPath, codes[20])
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log)

  //   console.log("Your transaction signature", tx);
  //   // Hash Extend Test
  //   // const original_hash = createHash("sha256").update("hello").digest();
  //   // console.log("🚀 ~ file: vault-program.ts:200 ~ it ~ original_hash:", original_hash.toString("hex"))

  //   // const extended_hash = createHash("sha256").update(Buffer.concat([original_hash, Buffer.from((1).toString())])).digest();
  //   // console.log("🚀 ~ file: vault-program.ts:203 ~ it ~ extended_hash:", extended_hash.toString("hex"))

  //   // const tx = await program.methods.hashExtendTest(original_hash, extended_hash)
  //   //   .signers([wallet.payer])
  //   //   .rpc()
  //   //   .catch(console.log)

  //   // console.log("Your transaction signature", tx);
  //   //Hashing Logic
  //   //console.log("🚀 ~ file: vault-program.ts:182 ~ it ~ hash:", hash)
  //   // console.log("initialHash: ", hash.toString('hex'));
  //   // for (let i = 0; i < amount; i++) {
  //   //   hash = createHash('sha256').update(hash).digest()
  //   //   console.log("hash: ", hash.toString('hex'));
  //   // };
  //   // const hash_str = hash.toString('hex');
  //   // console.log("hash_str: ", hash_str);

  //   // Creating Hashtuples
  //   // let arr: HashTuple[] = [];

  //   // for (let i = 0; i < 30; i++) {
  //   //   let obj: HashTuple = {
  //   //     hash: [...hash],
  //   //     siblingIndex: 1
  //   //   };
  //   //   arr.push(obj);
  //   // }

  // });

  // it("Time test", async () => {
  //   const tx = await program.methods.timeTest().signers([wallet.payer]).rpc();
  //   console.log("Your transaction signature", tx);
  //   console.log("Time: ", Date.now() / 1000);
  // });

  // it("Initialize Bunkr Account", async () => {
  //   const { link, otps, initTime } = generateTotpObject(Math.pow(2, 20));
  //   console.log("🚀 ~ file: vault-program.ts:182 ~ it ~ link:", link)
  //   const tree = new MerkleTree(otps, SHA256)
  //   const root = tree.getRoot();
  //   const passwordHash = createHashChain("PASSWORD", Math.pow(2, 20));
  //   const finalPasswordHash = createHash("sha256").update(Buffer.concat([createHash("sha256").update(Buffer.from("PASSWORD")).digest(), Buffer.from("FINAL")])).digest();
  //   const resetHash = createHashChain("RESETPASSWORD", Math.pow(2, 20));
  //   const finalResetHash = createHash("sha256").update(Buffer.concat([createHash("sha256").update(Buffer.from("RESETPASSWORD")).digest(), Buffer.from("FINAL")])).digest();

  //   const initBunkrData: InitBunkrData = {
  //     name: "Test Bunkr",
  //     initTime: initTime,
  //     root: [...root],
  //     initialHash: [...passwordHash],
  //     finalHash: [...finalPasswordHash],
  //     initialResetHash: [...resetHash],
  //     finalResetHash: [...finalResetHash],
  //     shadowDriveSpace: "test.bin"
  //   }

  //   writeFileData("test.bin", Buffer.concat(otps));

  //   const bunkrAccount = findProgramAddressSync([Buffer.from("bunkr"), wallet.payer.publicKey.toBuffer()], program.programId)[0]
  //   const tx = await program.methods.initBunkr(initBunkrData)
  //     .accounts({
  //       bunkr: bunkrAccount,
  //     })
  //     .signers([wallet.payer])
  //     .rpc()
  //     .catch(console.log);

  //   // const tx = await program.methods.closeBunkr()
  //   //   .accounts({
  //   //     bunkr: bunkrAccount,
  //   //   })
  //   //   .signers([wallet.payer])
  //   //   .rpc()
  //   //   .catch(console.log);


  //   console.log("Your transaction signature", tx);

  //   const data = await program.account.bunkr.fetch(bunkrAccount);
  //   console.log("🚀 ~ file: vault-program.ts:307 ~ it ~ data:", data)



  // });

  it("Check current Bunkr", async () => {
    const bunkrAccount = findProgramAddressSync([Buffer.from("bunkr"), wallet.payer.publicKey.toBuffer()], program.programId)[0]
    const accountData = await program.account.bunkr.fetch(bunkrAccount);
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
    let code = "695483";
    console.log("🚀 ~ file: vault-program.ts:339 ~ it ~ integer:", integer)
    const proof = createMerkleProofPath(tree, integer, leaves)
    const otpHash = createHash("sha256").update(Buffer.from(code)).digest();
    const leaf = createHash("sha256").update(Buffer.concat([otpHash, Buffer.from(integer.toString())])).digest();
    console.log("🚀 ~ file: vault-program.ts:342 ~ it ~ leaf:", leaf.toString('hex'))
    const { hash, attempts } = calculatePreImage(Buffer.from(hashImage), "PASSWORD", Math.pow(2, 20));
    console.log("🚀 ~ file: vault-program.ts:346 ~ it ~ attempts:", attempts)

    const tx = await program.methods.testWithdraw(hash, otpHash, proof)
      .accounts({
        bunkr: bunkrAccount,
      }
      )
      .signers([wallet.payer])
      .rpc()
      .catch(console.log);

    console.log("Your transaction signature", tx);
  });
});
