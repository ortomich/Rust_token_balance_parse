
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Token {
    address: String,
    name: String
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

async fn getter228(connect: &str, my_address: &str, token_file: &[u8]) -> web3::Result<()> {

    let websocket = web3::transports::WebSocket::new(connect).await?;
    let web3s = web3::Web3::new(websocket);

    let data = fs::read_to_string("src/addr.json").expect("Unable to read file");
    // let p: Token = serde_json::from_str(&data).unwrap();
    // println!("{}", data);
    // println!("--------------------------");

    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    // println!("Serialized: {}", json);
    let mut v = json.to_string();
    v.pop();
    v.pop();
    v.remove(0);
    v.remove(0);

    // это готовый массив с адрессами монет
    let split: Vec<&str> = v.split("},{").collect();


    // в этом блоке цикл, который пробегается по адрессам контрактов и делает запросы
    for i in split {
        let arr_i: Vec<&str> = i.split("\"").collect();        

        let erc20_address = Address::from_str(arr_i[3]).unwrap();
        let token_contract =
            Contract::from_json(web3s.eth(), erc20_address, token_file).unwrap();

        // let token_name: String = token_contract
        //     .query("name", (), None, Options::default(), None)
        //     .await
        //     .unwrap();

        // let total_supply: U256 = token_contract
        //     .query("totalSupply", (), None, Options::default(), None)
        //     .await
        //     .unwrap();

        // println!("Token name: {}, total supply: {}", token_name, total_supply);
        
        let balance_erc20: U256 = token_contract
            .query("balanceOf", Address::from_str(my_address).unwrap(), None, Options::default(), None)
            .await
            .unwrap();

        println!("Your {} balance: {}", arr_i[7], wei_to_eth(balance_erc20));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> web3::Result<()> {

    let thread1 = tokio::spawn(async move {
        getter228(
            "wss://eth-mainnet.alchemyapi.io/v2/xxxxxxxxxxxxxxxxxxxxxxxx",
            "here_is_your_address",
            include_bytes!("erc20_abi.json")
        ).await;
    });

    // let thread2 = tokio::spawn(async move {
    //     getter228(
    //         "wss://eth-mainnet.alchemyapi.io/v2/xxxxxxxxxxxxxxxxxxxxxxxx",
    //         "here_is_your_address",
    //         include_bytes!("erc20_abi.json")
    //     ).await;
    // });

    thread1.await.unwrap();
    // thread2.await.unwrap();

    Ok(())
}