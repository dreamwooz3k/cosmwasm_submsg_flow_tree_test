use cosmwasm_std::Deps;
use serde::{Deserialize, Serialize};

const PRINTER_ADDR: &str = "supergodprinter";

#[derive(Serialize, Deserialize)]
struct PrintRequest {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct PrintResponse {
    ack: bool,
}

pub fn print(deps: Deps, msg: &str) {
    let msg = PrintRequest {
        msg: msg.to_string(),
    };
    let _: PrintResponse = deps.querier.query_wasm_smart(PRINTER_ADDR, &msg).unwrap();
}
