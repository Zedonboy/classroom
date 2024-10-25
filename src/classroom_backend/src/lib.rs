use std::cell::RefCell;

use candid::{CandidType, Deserialize, Nat};
use ic_cdk::{api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs, TransformContext, TransformFunc}, id, init, query, update};
use serde_json::{json, Value};


#[derive(CandidType, Deserialize)]
struct FinancialGoal {
    purpose: String,
    timeframe: String,
    total_amount: f64,
    total_income_monthly: f64,
    total_expenses_monthly: f64,
}

// Replace with your Alpha Vantage API key
const API_KEY: &str = "QXJFM3I8BRTR1UAI";
const BASE_URL: &str = "https://www.alphavantage.co/query";

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ExchangeRate {
    from_currency: String,
    to_currency: String,
    rate: f64,
    last_updated: String,
}

thread_local! {
    static LATEST_RATE: RefCell<Option<ExchangeRate>> = RefCell::new(None);
}


#[update]
async fn submit_financial_goal(goal: FinancialGoal) -> Result<(), String> {
    let payload = json!({
        "purpose": goal.purpose,
        "timeframe": goal.timeframe,
        "total_amount": goal.total_amount,
        "total_income_monthly": goal.total_income_monthly,
        "total_expenses_monthly": goal.total_expenses_monthly
    });

    let request = CanisterHttpRequestArgument {
        url: "https://icp-api-budget.fly.dev/budget-plan".to_string(),
        method: HttpMethod::POST,
        body: Some(serde_json::to_vec(&payload).map_err(|e| e.to_string())?),
        max_response_bytes: Some(2048), // Adjust based on expected response size
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        headers: vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }],
    };

    match http_request(request, 10_000_000).await {
        Ok((response,)) => {
            let response_body = String::from_utf8(response.body)
                .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;

            ic_cdk::println!("response: {}", response_body);
            
            // let api_response: ApiResponse = serde_json::from_str(&response_body)
            //     .map_err(|e| format!("Failed to parse response: {}", e))?;
            
            // Ok(api_response)

            Ok(())
        }
        Err((code, msg)) => {
            Err(format!("HTTP request failed with code and message: {}", msg))
        }
    }
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let res = String::from_utf8(raw.response.body)
        .expect("Invalid UTF-8 sequence");
    
    HttpResponse {
        status: raw.response.status,
        body: res.as_bytes().to_vec(),
        headers: vec![],
    }
}

// Health check endpoint
#[query]
fn health_check() -> String {
    "Canister is healthy".to_string()
}

#[update]
async fn get_exchange_rate(from_currency: String, to_currency: String) -> Result<ExchangeRate, String> {
    let url = format!(
        "{}?function=CURRENCY_EXCHANGE_RATE&from_currency={}&to_currency={}&apikey={}",
        BASE_URL, from_currency, to_currency, API_KEY
    );

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(2048),
        transform: None,
        headers: vec![],
    };

    match http_request(request, 30_000_000_000).await {
        Ok((response,)) => {
            if response.status != Nat::from(200u16) {
                return Err(format!("HTTP request failed: {}", response.status));
            }

            let body = String::from_utf8(response.body)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;

            let data: Value = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            if let Some(rate_data) = data.get("Realtime Currency Exchange Rate") {
                let exchange_rate = ExchangeRate {
                    from_currency: rate_data["1. From_Currency Code"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    to_currency: rate_data["3. To_Currency Code"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    rate: rate_data["5. Exchange Rate"]
                        .as_str()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    last_updated: rate_data["6. Last Refreshed"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                };

                LATEST_RATE.with(|rate| {
                    *rate.borrow_mut() = Some(exchange_rate.clone());
                });

                Ok(exchange_rate)
            } else {
                Err("No exchange rate data found in response".to_string())
            }
        }
        Err((code, msg)) => Err(format!("HTTP request failed: {}", msg)),
    }
}

#[query]
fn get_latest_rate() -> Option<ExchangeRate> {
    LATEST_RATE.with(|rate| rate.borrow().clone())
}

// Initialize the canister
#[init]
fn init() {
    ic_cdk::println!("Finance canister initialized");
}

#[query]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}