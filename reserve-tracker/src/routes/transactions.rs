use std::str::FromStr;
use axum::{
    response::Html,
    extract::{Form, Query, State},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Deserialize;
use crate::state::AppState;
use html_escape::encode_text;
use snarkvm::prelude::{Output, Argument, Transition, Address, PrivateKey};
use crate::CurrentNetwork;
use crate::utilities::{get_block_height, get_block_timestamp, get_confirmed_transaction, get_mapping_value};

#[derive(Deserialize)]
pub struct TransactionsFormData {
    action: String,
    txid: Option<String>,
}

#[derive(Deserialize)]
pub struct TransactionsQuery {
    show: Option<String>,
}

/// GET /transactions
pub async fn get_transactions_page(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(query): Query<TransactionsQuery>,
) -> Html<String> {
    let st = state.read().await;
    let transactions = &st.transactions;

    let mut transaction_list_items = Vec::new();

    for txid in transactions {
        let safe_txid = encode_text(txid);
        let mut details_html = String::new();
        if Some(txid) == query.show.as_ref() {
            let height = get_block_height::<CurrentNetwork>(txid, &st.endpoint, "testnet").unwrap();
            let block_timestamp = get_block_timestamp::<CurrentNetwork>(&height, &st.endpoint, "testnet").unwrap();
            let transaction = get_confirmed_transaction::<CurrentNetwork>(txid, &st.endpoint, "testnet").unwrap().clone();
            // Pull out the data hash from the first argument of the future in the output of the first transition.
            let transition: Transition<CurrentNetwork> = transaction.execution().unwrap().transitions().next().unwrap().clone();
            let data_hash = match transition.outputs().get(0) {
                Some(Output::Future(_, Some(future))) => {
                    match future.arguments().get(0).unwrap().clone() {
                        Argument::Plaintext(plaintext) => plaintext.to_string(),
                        _ => "Could not find data hash.".to_string()
                    }
                }
                _ => "Could not find data hash.".to_string()
            };
            // Construct a query for the total balance at that point in time.
            let address = Address::<CurrentNetwork>::try_from(PrivateKey::from_str(&st.private_key).unwrap()).unwrap();
            let raw_string = format!("{{user:{address},hash:{data_hash},height:{height}u32}}");
            let total_balance = get_mapping_value::<CurrentNetwork>("proof_of_reserves_v0_1_0.aleo", "data", raw_string.as_str(), &st.endpoint, "testnet").unwrap();

            details_html = format!(r#"
            <div style="margin-top:10px; border:1px solid #ccc; padding:10px;">
                <p><b>Transaction Height:</b> {}</p>
                <p><b>Timestamp:</b> {}</p>
                <p><b>Total Balance:</b> {}</p>
            </div>
            "#, height, block_timestamp, total_balance);
        }

        transaction_list_items.push(format!(
            r#"<li>
                <form action="/transactions" method="get" style="display:inline;">
                    <input type="hidden" name="show" value="{safe_txid}">
                    <button type="submit" style="border:none;background:none;color:blue;text-decoration:underline;cursor:pointer;">{safe_txid}</button>
                </form>
                <form action="/transactions" method="post" style="display:inline;margin-left:10px;">
                    <input type="hidden" name="txid" value="{safe_txid}">
                    <button type="submit" name="action" value="remove">Remove</button>
                </form>
                {details_html}
            </li>"#
        ));
    }

    let tx_list = if transaction_list_items.is_empty() {
        "<li>No transactions tracked.</li>".to_string()
    } else {
        transaction_list_items.join("\n")
    };

    Html(format!(
        r#"
        <html>
            <head><title>Verification History</title></head>
            <body>
                <h3>Verification History (Transactions)</h3>
                <ul>
                    {}
                </ul>
                <p><a href="/">Back to Addresses</a></p>
            </body>
        </html>
        "#,
        tx_list
    ))
}

/// POST /transactions
pub async fn handle_transactions_form(
    State(state): State<Arc<RwLock<AppState>>>,
    Form(form): Form<TransactionsFormData>,
) -> Html<String> {
    let mut st = state.write().await;
    match form.action.as_str() {
        "remove" => {
            if let Some(txid) = form.txid {
                st.transactions.retain(|t| t != &txid);
                // Save transactions if needed immediately:
                // crate::storage::save_transactions(&st.transactions, &st.transactions_file).await.ok();
            }
        }
        _ => (),
    }
    drop(st);
    get_transactions_page(State(state), Query(TransactionsQuery { show: None })).await
}
