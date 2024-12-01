use std::ops::Deref;

use actix_web::{self, get, patch, post, web, App, HttpResponse, HttpServer, Responder, Result};
use outro_08::{TicketDraft, TicketId, TicketStore};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

struct AppState {
    ticket_store: Mutex<TicketStore>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut ticket_store = web::Data::new(AppState {
        ticket_store: Mutex::new(TicketStore::new()),
    });
    println!("🚀 Server started successfully");
    HttpServer::new(move || {
        App::new()
            .app_data(ticket_store.clone())
            .service(hello)
            .service(create_ticket)
            .service(get_ticket_by_id)
            .service(edit_ticket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[derive(Serialize)]
struct TicketCreatedResponse {
    id: Option<TicketId>,
    message: String,
}

#[post("/ticket/create")]
async fn create_ticket(
    data: web::Data<AppState>,
    json: web::Json<TicketDraft>,
) -> Result<impl Responder> {
    let mut ticket_store = data.ticket_store.lock().await;
    let id = ticket_store.add_ticket(json.0).await;
    let resp = TicketCreatedResponse {
        id: Some(id),
        message: "Ticket was created!".to_string(),
    };
    Ok(web::Json(resp))
}

#[derive(Debug, Serialize, Deserialize)]
struct TicketGetByIdResponse {
    id: TicketId,
    title: String,
    description: String,
    // TODO:
    // status: String,
}

#[get("/ticket/{ticket_id}")]
async fn get_ticket_by_id(
    data: web::Data<AppState>,
    path: web::Path<TicketId>,
) -> Result<impl Responder> {
    let ticket_store = data.ticket_store.lock().await;
    let resp = ticket_store.get(*path).await.unwrap();
    Ok(web::Json(TicketGetByIdResponse {
        id: resp.id,
        description: resp.description.clone(),
        title: resp.title.clone(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateTicketSchema {
    title: Option<String>,
    description: Option<String>,
}

#[patch("/ticket/{ticket_id}")]
async fn edit_ticket(
    data: web::Data<AppState>,
    path: web::Path<TicketId>,
    body: web::Json<Option<UpdateTicketSchema>>,
) -> impl Responder {
    let mut ticket_store = data.ticket_store.lock().await;
    let mut ticket = ticket_store.get_mut(*path).await.unwrap();

    if let Some(body) = body.0 {
        if let Some(title) = body.title {
            todo!()
        }
        return HttpResponse::Ok().body("Succesfull update ticket!");
    }

    HttpResponse::Ok().body("Nothing to update!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{patch, test, App};
    use serde_json::json;
    use web::Bytes;

    #[actix_web::test]
    async fn test_create_ticket() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    ticket_store: Mutex::new(TicketStore::new()),
                }))
                .service(create_ticket),
        )
        .await;
        let req_body = TicketDraft {
            title: String::from("test"),
            description: String::from("test"),
        };
        let req = test::TestRequest::post()
            .uri("/ticket/create")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let expected_json = serde_json::json!({"id": 0, "message": "Ticket was created!"});

        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }

    #[actix_web::test]
    async fn test_retrieve_ticket() {
        let mut store = TicketStore::new();
        store
            .add_ticket(TicketDraft {
                title: String::from("test"),
                description: String::from("test"),
            })
            .await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    ticket_store: Mutex::new(store),
                }))
                .service(get_ticket_by_id),
        )
        .await;
        let req = test::TestRequest::get().uri("/ticket/0").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: TicketGetByIdResponse = test::read_body_json(resp).await;

        // FIXME:
        // assert_eq!(body.id, TicketId(0));
        assert_eq!(body.title, "test".to_string());
        assert_eq!(body.description, "test".to_string());
    }

    #[actix_web::test]
    async fn test_patch_ticket() {
        let mut store = TicketStore::new();
        store
            .add_ticket(TicketDraft {
                title: String::from("test"),
                description: String::from("test"),
            })
            .await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    ticket_store: Mutex::new(store),
                }))
                .service(get_ticket_by_id)
                .service(edit_ticket),
        )
        .await;
        let req = test::TestRequest::patch()
            .set_json(json!({}))
            .uri("/ticket/0")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let req_body = UpdateTicketSchema {
            title: Some("THE".to_string()),
            description: Some("most powerful programming language.".to_string()),
        };
        let req = test::TestRequest::patch()
            .set_json(&req_body)
            .uri("/ticket/0")
            .to_request();

        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, Bytes::from_static(b"Succesfull update ticket!"));

        let req = test::TestRequest::get().uri("/ticket/0").to_request();
        let resp: TicketGetByIdResponse =
            test::try_call_and_read_body_json(&app, req).await.unwrap();

        // assert_eq!(resp.title, "THE".to_string());
        // assert_eq!(
        //     resp.description,
        //     "most powerful programming language.".to_string()
        // )
    }
}
