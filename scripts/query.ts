import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";

const getTxAPI = "https://lcd.orai.io/cosmos/"

const rpcEndpoint = "https://rpc.orai.io:443/";
const chainID = "Oraichain"
//const mnemonic = process.env.MNEMONIC!;


  

async function connectClient() {
    // Tạo một client kết nối đến blockchain
    const client = await CosmWasmClient.connect(rpcEndpoint);
    console.log("Connected to blockchain");
    return client;
}

async function queryRewardPerSecond() {
    const client = await connectClient();
    const contractAddress = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
    const queryMsg = { reward_per_second: {} };

    try {
        const response = await client.queryContractSmart(contractAddress, queryMsg);
        console.log("Reward per second: ", response.reward_per_second);
    } catch (error) {
        console.log("Error: ", error);
    }
    
}

async function queryTotalStaked() {
    const client = await connectClient();
    const contractAddress = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
    const queryMsg = { total_staked: {} };

    try {
        const response = await client.queryContractSmart(contractAddress, queryMsg);
        console.log("Total staked: ", response.total_staked);
    } catch (error) {
        console.log("Error: ", error);
    }
}

async function queryUserStaked(){
    const client = await connectClient();
    const contractAddress = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
    const queryMsg = 
    {
        user_staked: {
            user_address: "orai1tyl0wdg7shhv9m44kq34za5gqanu7uwfz2xvrc"
        }
    }

    try {
        const response = await client.queryContractSmart(contractAddress, queryMsg);
        console.log("User staked: ", response.user_staked);
    } catch (error) {
        console.log("Error: ", error);
    }

}

async function queryUserRewards(){
    const client = await connectClient();
    const contractAddress = "orai1yum9k33wzpr9ycyt6c6q2xzzmpsaplweeq0cyudukj0l52lu3fmq9ast78"
    const queryMsg = 
    {
        user_rewards: {
            user_address: "orai1tyl0wdg7shhv9m44kq34za5gqanu7uwfz2xvrc"
        }
    }

    try {
        const response = await client.queryContractSmart(contractAddress, queryMsg);
        console.log("User reward: ", response.user_rewards);
    } catch (error) {
        console.log("Error: ", error);
    }
}

async function main() {
    // await queryRewardPerSecond();
    // await queryTotalStaked();
    // await queryUserStaked();
    await queryUserRewards();
}
main();