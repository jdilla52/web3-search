use once_cell::sync::Lazy;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, Bytes};
use web3::Web3;

static NO_CODE: Lazy<web3::types::Bytes> = Lazy::new(Bytes::default);

pub async fn binary_search_creation_block(
    web3: &Web3<Http>,
    address: Address,
    min: Option<u64>,
    max: Option<u64>,
) -> i32 {
    let mut low = 1;
    if let Some(v) = min {
        low = v
    }

    let mut high = get_latest_block(web3).await;
    if let Some(v) = max {
        if v < high {
            high = v;
        }
    };
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
    guess: u64,
    bias: f64,
    min: u64,
    max: u64,
) -> i32 {
    let mut low = min;
    let mut high = max;

    // check we're not out of range
    let latest = get_latest_block(web3).await;
    if latest < high {
        high = latest;
    }

    let guess = guess as f64;
    let range = (high - low) as f64;

    let mut runs = 0;
    while low < high {
        // fraction of current possibilities
        let possibility_fraction = (high - low) as f64 / range;
        // raise the power of the fraction to bias more towards the guess or less
        let raised_possibility_fraction = possibility_fraction.powf(bias);

        // tradition binary guess - ie middle
        let binary_value = (high + low) / 2;

        // linear interpolation between binary search and estimation
        // this should move from the guess to binary search
        let weighted_guess = (1.0 - raised_possibility_fraction) * binary_value as f64
            + (raised_possibility_fraction) * guess;

        let weighted_index = weighted_guess as u64;

        let code = contract_contains_code(web3, address, Some(weighted_index)).await;
        println!(
            "h {} - bi {} - wi {} - l {} - pf {} - rpf {}",
            high,
            binary_value,
            weighted_index,
            low,
            possibility_fraction,
            raised_possibility_fraction
        );
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
        binary_search_creation_block(web3, address, None, None).await;
    }
}

fn build_web3() -> Web3<Http> {
    let node = "https://eth-mainnet.g.alchemy.com/v2/NrrMfEnkJ9SUIkuDSUoITLM-niV_0tUb";
    let http = Http::new(node).expect("transport failure");
    Web3::new(http)
}
#[tokio::test]
async fn test_binary_search() {
    let web3 = build_web3();
    let address = Address::from_str("0x837b40be9ce60c79b63d1356a5f9fcad721421ec").unwrap();
    binary_search_creation_block(&web3, address, Some(12908978), Some(13908978)).await;
}

#[tokio::test]
async fn test_interpolation_search() {
    let web3 = build_web3();
    let address = Address::from_str("0x837b40be9ce60c79b63d1356a5f9fcad721421ec").unwrap();
    interpolation_search_creation_block(&web3, address, 13308978, 0.1, 12908978, 13908978).await;
}
