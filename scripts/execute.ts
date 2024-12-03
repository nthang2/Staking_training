import { Secp256k1HdWallet } from "@cosmjs/amino";
import { Decimal } from "@cosmjs/math";
import { CosmWasmClient, SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import * as dotenv from "dotenv";
dotenv.config();

const getTxAPI = "https://lcd.orai.io/cosmos/"

const rpcEndpoint = "https://rpc.orai.io:443/";
const chainID = "Oraichain"
const mnemonic = process.env.MNEMONIC_ADMIN!;

const CONTRACT_ADDRESS = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
const OCH_TOKEN = process.env.OCH_TOKEN!

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


async function sendStake(contractAddress: string, amount: String) {
    const client = await getClient();
    const wallet = await getWallet();
    const senderAddress = (await wallet.getAccounts())[0].address;

    const stakeMsg = {
        user_send_stake: {
            amount_stake: amount
        }
    }

    const fee = "auto"
    try {
        const result = await client.execute(senderAddress, contractAddress, stakeMsg, fee, "Sending stake");
        console.log("Transaction successful!", result.transactionHash);
        console.log("Sending stakeMsg_true:", JSON.stringify(stakeMsg));
    } catch (error) {
        console.log("Error: ", error);
        console.log("Sending stakeMsg:", JSON.stringify(stakeMsg));
    }
}

async function increaseAllowance(amount: String) {
    const client = await getClient();
    const wallet = await getWallet();
    const senderAddress = (await wallet.getAccounts())[0].address;

    const increaseAllowanceMsg = {
        increase_allowance: {
            spender: CONTRACT_ADDRESS,
            amount: amount
        }
    }

    try {
        const result = await client.execute(senderAddress, OCH_TOKEN, increaseAllowanceMsg, "auto", "Increase allowance");
        console.log("Transaction successful!", result.transactionHash);
        console.log("Increase allowanceMsg_true:", JSON.stringify(increaseAllowanceMsg));
    } catch (error) {
        console.log("Error: ", error);
        console.log("Increase allowanceMsg:", JSON.stringify(increaseAllowanceMsg));
    }
}

async function withdrawStake(contractAddress: string, amount: String) {
    const client = await getClient();
    const wallet = await getWallet();
    const senderAddress = (await wallet.getAccounts())[0].address;
    
        const withdrawMsg = {
            user_withdraw_stake: {
                amount_withdraw: amount
            }
        }
    
        const fee = "auto"
        try {
            const result = await client.execute(senderAddress, contractAddress, withdrawMsg, fee, "Withdraw stake");
            console.log("Transaction successful!", result.transactionHash);
            console.log("Withdraw stakeMsg_true:", JSON.stringify(withdrawMsg));
        } catch (error) {
            console.log("Error: ", error);
            console.log("Withdraw stakeMsg:", JSON.stringify(withdrawMsg));
        }
}

async function claimReward(contractAddress: string) {
    const client = await getClient();
    const wallet = await getWallet();
    const senderAddress = (await wallet.getAccounts())[0].address;

    const claimRewardMsg = {
        user_claim_reward: {}
    }

    const fee = "auto"

    try {
        const result = await client.execute(senderAddress, contractAddress, claimRewardMsg, fee, "Claim reward");
        console.log("Transaction successful!", result.transactionHash);
        console.log("Claim rewardMsg_true:", JSON.stringify(claimRewardMsg));
    } catch (error) {
        console.log("Error: ", error);
        console.log("Claim rewardMsg:", JSON.stringify(claimRewardMsg));
    }
}

async function setRewardPerSecond(amount: String) {
    const client = await getClient();
    const wallet = await getWallet();
    const senderAddress = (await wallet.getAccounts())[0].address;

    const rewardPerSecondMsg = {
        admin_set_reward_per_second: {
            new_reward_per_second: amount
        }
    }

    const fee = "auto"

    try {
        const result = await client.execute(senderAddress, CONTRACT_ADDRESS, rewardPerSecondMsg, fee, "Set reward per second");
        console.log("Transaction successful!", result.transactionHash);
        console.log("Set reward per secondMsg_true:", JSON.stringify(rewardPerSecondMsg));
    } catch (error) {
        console.log("Error: ", error);
        console.log("Set reward per secondMsg:", JSON.stringify(rewardPerSecondMsg));
    }
}

async function main() {
    const contractAddress = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
    const amount: String = "1110"
    //increaseAllowance("1000000")
    //sendStake(contractAddress, amount)
    //withdrawStake(contractAddress, amount)
    //claimReward(contractAddress)
    setRewardPerSecond("1")

    
    

}
main();