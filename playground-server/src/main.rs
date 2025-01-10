use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use inference::{compile_to_wat, wat_to_wasm};
use serde::{Deserialize, Serialize};

use wat_fmt::format;

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Deserialize, Serialize)]
struct Response {
    wat: String,
    wasm: Vec<u8>,
    errors: Vec<String>,
}

fn parse_inf_file(input: &str) -> Response {
    let (wat, errors) = match compile_to_wat(input) {
        Ok(wat) => (wat, vec![]),
        Err(e) => (String::new(), vec![e.to_string()]),
    };
    if !wat.is_empty() {
        let (wasm, errors) = match wat_to_wasm(&wat) {
            Ok(wasm) => (wasm, errors),
            Err(e) => (vec![], vec![e.to_string()]),
        };
        let wat = format(&wat);
        Response { wat, wasm, errors }
    } else {
        Response {
            wat: String::new(),
            wasm: vec![],
            errors,
        }
    }
}

#[post("/compile")]
async fn compile_code(payload: web::Json<CompileRequest>) -> impl Responder {
    let code = &payload.code;
    let compiled_result = parse_inf_file(code);
    HttpResponse::Ok().json(compiled_result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec!["Content-Type"])
                    .supports_credentials(),
            )
            .service(compile_code)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
