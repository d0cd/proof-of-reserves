use axum::{response::Html, extract::{Form, State}};
use std::sync::Arc;
use tokio::sync::RwLock;
use html_escape::encode_text;
use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    address: Option<String>,
    action: String, // "add" or "remove" or "run_task"
}

pub async fn get_form(State(state): State<Arc<RwLock<AppState>>>) -> Html<String> {
    let st = state.read().await;
    let addresses = &st.addresses;
    let address_list = if addresses.is_empty() {
        "<li>No addresses tracked yet.</li>".to_string()
    } else {
        addresses
            .iter()
            .map(|addr| {
                let safe_addr = encode_text(addr);
                format!(
                    r#"<li>{safe_addr}
                        <form action="/form" method="post" style="display:inline;">
                            <input type="hidden" name="address" value="{addr}">
                            <button type="submit" name="action" value="remove">Remove</button>
                        </form>
                       </li>"#
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    Html(format!(
        r#"
        <html>
            <head>
                <title>Address Tracker</title>
            </head>
            <body>
                <h3>Tracked Addresses</h3>
                <ul>
                    {}
                </ul>
                <form action="/form" method="post" style="margin-top:20px;">
                    <input type="text" name="address" style="width:400px;" placeholder="Enter address">
                    <button type="submit" name="action" value="add">Add</button>
                    <button type="submit" name="action" value="run_task">Run Task Now</button>
                </form>
            </body>
        </html>
        "#,
        address_list
    ))
}

pub async fn handle_form(
    State(state): State<Arc<RwLock<AppState>>>,
    Form(input): Form<FormData>,
) -> Html<String> {
    let mut st = state.write().await;
    match input.action.as_str() {
        "add" => {
            if let Some(addr) = input.address {
                let trimmed = addr.trim();
                if !trimmed.is_empty() && !st.addresses.contains(&trimmed.to_string()) {
                    st.addresses.push(trimmed.to_string());
                }
            }
        }
        "remove" => {
            if let Some(addr) = input.address {
                st.addresses.retain(|a| a != &addr);
            }
        }
        "run_task" => {
            // Trigger the background task immediately
            let _ = st.task_tx.try_send(crate::background::BackgroundTaskMsg::RunNow);
        }
        _ => (),
    }

    drop(st); // release the write lock
    get_form(State(state)).await
}
