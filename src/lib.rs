use once_cell::sync::Lazy;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, Bytes};
use web3::Web3;

static NO_CODE: Lazy<web3::types::Bytes> = Lazy::new(Bytes::default);

pub async fn binary_search_creation_block(web3: &Web3<Http>, address: Address) -> i32 {
    let mut low = 1;
    let mut high = get_latest_block(web3).await;
    let mut runs = 0;

    while low < high {
        let middle = (high + low) / 2;
        let code = contract_contains_code(web3, address, Some(middle)).await;
        // println!("h {} - m {} - l {} - {}", high, middle, low, code);
        match code {
            // contract is created, block is either `middle` or smaller than `middle`
            true => high = middle,
            // contract doesn't exist, block can't be `middle` and must be larger
            false => low = middle + 1,
        }
        runs += 1;
    }
    println!("created at block {high}");
    println!("found in {runs} RPC calls");
    runs
}

pub async fn interpolation_search_creation_block(
    web3: &Web3<Http>,
    address: Address,
    estimation: u64,
    bias: Option<f64>,
    max: Option<u64>,
) -> i32 {
    let mut low = 1;

    let mut high = get_latest_block(web3).await;
    if let Some(v) = max {
        if v < high {
            high = v;
        }
    };

    let mut inner_bias = 5.0;
    if let Some(v) = bias {
        inner_bias = v
    };

    let mut runs = 0;

    let original_count = (high - low) as f64;
    let estimation = estimation as f64;
    while low < high {
        // fraction of current possibilities
        let possibility_fraction = (high - low) as f64 / original_count;

        // raise the power of the fraction to bias more towards the guess or less
        let raised_possibility_fraction = 1.0 - possibility_fraction.powf(inner_bias);

        // tradition binary guess - ie middle
        let binary_value = (high + low) / 2;

        // linear interpolation between binary search and estimation
        let weighted_guess = (1.0 - raised_possibility_fraction) * binary_value as f64
            + raised_possibility_fraction * estimation;

        let weighted_index = weighted_guess as u64;

        let code = contract_contains_code(web3, address, Some(weighted_index)).await;
        // println!("h {} - m {} - l {} - {}", high, binary_value, low, , code);
        match code {
            // contract is created, block is either `middle` or smaller than `middle`
            true => high = weighted_index,
            // contract doesn't exist, block can't be `middle` and must be larger
            false => low = weighted_index + 1,
        }
        runs += 1;
    }
    println!("created at block {high}");
    println!("found in {runs} RPC calls");
    runs
}

async fn contract_contains_code(
    web3: &Web3<Http>,
    address: Address,
    block_number: Option<u64>,
) -> bool {
    let block_number = block_number.map(|bn| bn.into());
    let code = web3.eth().code(address, block_number).await.unwrap(); // TODO errors

    code.ne(&NO_CODE)
}

async fn get_latest_block(web3: &Web3<Http>) -> u64 {
    web3.eth().block_number().await.unwrap().as_u64() // TODO no unwrap
}

pub async fn run_command(web3: &Web3<Http>, address: Address) {
    // Check its a contract by not specifying a block number. If its a contract it will return true
    let is_contract = contract_contains_code(web3, address, None).await;
    println!("is contract {is_contract}");

    if is_contract {
        binary_search_creation_block(web3, address).await;
    }
}

fn build_web3() -> Web3<Http> {
    let node = "https://eth-mainnet.g.alchemy.com/v2/nP_NafDRVjtS1WucTiZ5XEjvlP5T1Y9O";
    let http = Http::new(node).expect("transport failure");
    Web3::new(http)
}
#[tokio::test]
async fn test_binary_search() {
    let web3 = build_web3();
    let address = Address::from_str("0x837b40be9ce60c79b63d1356a5f9fcad721421ec").unwrap();
    binary_search_creation_block(&web3, address).await;
}

#[tokio::test]
async fn test_interpolation_search() {
    let web3 = build_web3();
    let address = Address::from_str("0x837b40be9ce60c79b63d1356a5f9fcad721421ec").unwrap();
    interpolation_search_creation_block(&web3, address, 13308978, Some(5.0), Some(13308978 * 10))
        .await;
}
