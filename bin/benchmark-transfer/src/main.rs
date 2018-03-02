#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate tokio_core;

#[macro_use]
extern crate client_utils;
extern crate ekiden_core_common;
extern crate ekiden_rpc_client;

extern crate token_api;

use clap::{App, Arg};
use futures::future::Future;
use rand::{thread_rng, Rng};

use ekiden_rpc_client::create_client_rpc;
use token_api::with_api;

with_api! {
    create_client_rpc!(token, token_api, api);
}

/// Create a new random token address.
fn create_address() -> String {
    thread_rng().gen_ascii_chars().take(32).collect()
}

const OTHER_ACCOUNT_COUNT: usize = 200;
lazy_static! {
    static ref OTHER_ACCOUNTS: Vec<String> = {
        // Generate some random account names.
        (0..OTHER_ACCOUNT_COUNT).map(|_| create_address()).collect::<Vec<String>>()
    };
}

/// Initializes the token scenario.
fn init<Backend>(client: &mut token::Client<Backend>, _runs: usize, _threads: usize)
where
    Backend: ekiden_rpc_client::backend::ContractClientBackend,
{
    // Create new token contract.
    let mut request = token::CreateRequest::new();
    request.set_sender("bank".to_string());
    request.set_token_name("Ekiden Token".to_string());
    request.set_token_symbol("EKI".to_string());
    request.set_initial_supply(8);

    client.create(request).wait().unwrap();

    // Populate the other accounts.
    for other_account in OTHER_ACCOUNTS.iter() {
        client
            .transfer({
                let mut request = token::TransferRequest::new();
                request.set_sender("bank".to_string());
                request.set_destination(other_account.clone());
                request.set_value(1);
                request
            })
            .wait()
            .unwrap();
    }
}

/// Runs the token scenario.
fn scenario<Backend>(client: &mut token::Client<Backend>)
where
    Backend: ekiden_rpc_client::backend::ContractClientBackend,
{
    // Transfer some funds.
    client
        .transfer({
            let mut request = token::TransferRequest::new();
            request.set_sender("bank".to_string());
            request.set_destination("dest".to_string());
            request.set_value(1);
            request
        })
        .wait()
        .unwrap();
}

/// Finalize the token scenario.
fn finalize<Backend>(client: &mut token::Client<Backend>, runs: usize, threads: usize)
where
    Backend: ekiden_rpc_client::backend::ContractClientBackend,
{
    // Check final balance.
    let response = client
        .get_balance({
            let mut request = token::GetBalanceRequest::new();
            request.set_account("dest".to_owned());
            request
        })
        .wait()
        .unwrap();
    assert_eq!(response.get_balance(), (threads * runs) as u64);
}

fn main() {
    let results = benchmark_client!(token, init, scenario, finalize);
    results.show();
}
