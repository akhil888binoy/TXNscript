

use crate::{chain_config::chain_config::create_provider, error::error::AppError, tokens::tokens::TOKENS};
use std::{str::FromStr, time::Duration};  
use alloy::{primitives::{Address, U256, utils::parse_units}, sol};
use tokio;
use tokio::time::sleep;

sol!(
    #[sol(rpc)]
    ERC20,
    "src/utils/abi/ERC20.json"
);

const WITHDRAW_INTERVAL: Duration = Duration::from_secs(1);
pub const MAX_RETRIES: u32 = 1;
pub const RETRY_BACKOFF: Duration = Duration::from_secs(1);

pub async fn run_script(
    worker_id: u64,
) ->  Result<(), AppError> {

    println!("DEPOSITOR :{} RUNNING", worker_id);

    loop {
                for (chain_name , tokens) in TOKENS.iter(){


                    let provider = create_provider(&chain_name, worker_id).await.map_err(|e| {
                        eprintln!("Cannot create provider on  {:?}: {:?}", chain_name, e);
                        AppError::InternalError(format!("Provider error: {e}"))
                    })?;
                    let amount = parse_units("10", 18).unwrap().get_absolute();

                    for (token_name , token_address) in tokens {
                                let erc20 = ERC20::new(*token_address, &provider.0);
                                let tx_hash = erc20.transfer(Address::from_str("0x0841dA92A8986d3AEa0Da821e280F1b1a1842c7a").unwrap(), amount).send().await.map_err(|e|{
                                                eprintln!("Error: Cannot sent {:?}: {:?}", Address::from_str("0x0841dA92A8986d3AEa0Da821e280F1b1a1842c7a") , e);
                                                AppError::InternalError(format!("Provider error: {e}"))
                                        })?.watch().await.map_err(|e|{
                                                eprintln!("Error : Cannot get receipt  {:?}: {:?}", Address::from_str("0x0841dA92A8986d3AEa0Da821e280F1b1a1842c7a") , e);
                                                AppError::InternalError(format!("Provider error: {e}"))
                                })?;
                                sleep(Duration::from_secs(30)).await; 
                                println!("TxnHash:{} Token:{} ", tx_hash, token_name);
                        }

        };
        sleep(WITHDRAW_INTERVAL).await;
        tokio::task::yield_now().await;
    }
}


