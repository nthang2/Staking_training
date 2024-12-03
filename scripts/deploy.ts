import * as dotenv from "dotenv";
import { Decimal } from "@cosmjs/math";
import axios from 'axios';
import * as fs from "fs";
import { Secp256k1HdWallet } from "@cosmjs/amino";
import { InstantiateResult, MigrateResult, SigningCosmWasmClient, UploadResult } from "@cosmjs/cosmwasm-stargate";


dotenv.config();
// This is your rpc endpoint
const getTxAPI = "https://lcd.orai.io/cosmos/"

const rpcEndpoint = "https://rpc.orai.io:443/";
const chainID = "Oraichain"
const mnemonic = process.env.MNEMONIC!;


async function getWallet(): Promise<Secp256k1HdWallet> {
    const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: "orai" });
    return wallet;
}

async function getClient(): Promise<SigningCosmWasmClient> {
    // Create a wallet
    const wallet = await getWallet();

    // Using
    const client = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        wallet,
        {
            gasPrice: {
                denom: "orai",
                amount: Decimal.fromUserInput("0.001", 6)
            }
        }
    );

    return client;
}
function ReadFile(path: string): Uint8Array {
    var file = fs.readFileSync(path);
    var buffer = new Uint8Array(file);
    return buffer
}

async function Upload(path: string): Promise<UploadResult> {
    // const query = await client.getTx("2D925C0F81EF1E26662B0A2A9277180CE853F9F07C60CA2F3E64E7F565A19F78")
    const wallet = await getWallet();
    const client = await getClient();

    const senderAddress = (await wallet.getAccounts())[0].address;
    const wasmCode = ReadFile(path)
    const fee = "auto"
    const memo: any = null
    // const fund = [coin(2, "orai")]
    // const res = await client.execute(senderAddress, contractAddress, msg, fee, memo, fund)
    const res = await client.upload(senderAddress, wasmCode, fee, memo)
    console.log(res)
    console.log(res.codeId)
    return res;
    
}

async function instantiate(codeId: number): Promise<InstantiateResult> {
    const wallet = await getWallet();
    const client = await getClient();
    const senderAddress = (await wallet.getAccounts())[0].address;

    const initMsg = {
        admin: "orai1etpatp0s4899uxda844q8074q29f7w664924h3",
        och_token: process.env.OCH_TOKEN,
        usdc_token: process.env.USDC_TOKEN,
        reward_per_second: "1"
    }

    const label = "staking"

    const res = await client.instantiate(senderAddress, codeId, initMsg, label, 'auto', { admin: senderAddress });
    console.log(res)
    return res;
}

async function migrate(contractAddress: string, newCodeId: number): Promise<MigrateResult> {
    const wallet = await getWallet();
    const client = await getClient();
    const senderAddress = (await wallet.getAccounts())[0].address;
    console.log("Hello");
    
    const migrateMsg = {}
    

    const res = await client.migrate(senderAddress, contractAddress, newCodeId, migrateMsg, 'auto');
    console.log(res)
    
    return res;

}


async function main() {
    
    const resUpload = await Upload("./artifacts/staking.wasm");
    // const resInitiate = await instantiate(resUpload.codeId);
    const resMigrate = await migrate("orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78", resUpload.codeId);
    // writeToEnvFile("COSMOS_BRIDGE", resInitiate.contractAddress);
    
}

main()


