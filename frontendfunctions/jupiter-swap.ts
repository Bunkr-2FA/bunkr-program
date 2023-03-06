import { Connection, Keypair, VersionedTransaction } from '@solana/web3.js';
import fetch from 'cross-fetch';
import { Wallet } from '@project-serum/anchor';
import bs58 from 'bs58';

const SHDW_PER_MB = 0.00025;

type Token = {
    mint: string;
    decimals: number;
}

// It is recommended that you use your own RPC endpoint.
// This RPC endpoint is only for demonstration purposes so that this example will run.
const connection = new Connection('https://rpc.helius.xyz/?api-key=c977f917-c416-4986-8045-463b1364224b', 'confirmed');

const wallet = new Wallet(Keypair.fromSecretKey(bs58.decode("2SQYJcYik486KXkwGdkGtNpVUgNUQpbgvBUBYK7Y1FF4jVNEAJninMqnjJ6qJJbMWMbQ6P62f4M2YzRuQqg4EDtx")));

const USDC: Token = {
    mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    decimals: 6
}

const SOL: Token = {
    mint: "So11111111111111111111111111111111111111112",
    decimals: 9

}

const SHDW: Token = {
    mint: "SHDWyBxihqiCj6YekG2GUr7wqKLeLAMK1gHZck9pL6y",
    decimals: 9
}

async function getUSDCIn(storageSizeMB: number): Promise<number> {
    const amountOut = storageSizeMB * SHDW_PER_MB;
    const data = await (await fetch(`https://price.jup.ag/v4/price?ids=SHDW&vsToken=USDC`)).json();
    const price = data.data.SHDW.price;
    const amountIn = price * amountOut;

    return amountIn;
}
async function getSOLIn(storageSizeMB: number): Promise<number> {
    const amountOut = storageSizeMB * SHDW_PER_MB;
    const data = await (await fetch(`https://price.jup.ag/v4/price?ids=SHDW&vsToken=SOL`)).json();
    const price = data.data.SHDW.price;
    const amountIn = price * amountOut;

    return amountIn;
}



// (async () => {
//     const usdc = await getUSDCIn(34);
//     const sol = await getSOLIn(34);

//     const url = `https://quote-api.jup.ag/v4/quote?inputMint=${SOL.mint}\&outputMint=${SHDW.mint}\&amount=${Math.ceil(sol * Math.pow(10, SOL.decimals))}\&slippageBps=50`
//     console.log("🚀 ~ file: jupiter-swap.ts:62 ~ url:", url)


//     const { data } = await (
//         await fetch(url)
//     ).json();

//     const routes = data;


//     // get serialized transactions for the swap
//     const transactions = await (
//         await fetch('https://quote-api.jup.ag/v4/swap', {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/json'
//             },
//             body: JSON.stringify({
//                 // route from /quote api
//                 route: routes[0],
//                 // user public key to be used for the swap
//                 userPublicKey: wallet.publicKey.toString(),
//                 // auto wrap and unwrap SOL. default is true
//                 wrapUnwrapSOL: true,
//                 // feeAccount is optional. Use if you want to charge a fee.  feeBps must have been passed in /quote API.
//                 // This is the ATA account for the output token where the fee will be sent to. If you are swapping from SOL->USDC then this would be the USDC ATA you want to collect the fee.
//                 // feeAccount: "fee_account_public_key"  
//             })
//         })
//     ).json();

//     const { swapTransaction } = transactions;
//     const swapTransactionBuffer = Buffer.from(swapTransaction, 'base64');
//     const transaction = VersionedTransaction.deserialize(swapTransactionBuffer)
//     transaction.sign([wallet.payer]);

//     // Execute the transaction
//     const rawTransaction = transaction.serialize()
//     const txid = await connection.sendRawTransaction(rawTransaction, {
//         skipPreflight: true,
//         maxRetries: 2
//     });
//     await connection.confirmTransaction(txid);
//     console.log(`https://solscan.io/tx/${txid}`);



// })();


