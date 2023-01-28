use std::sync::Arc;
use actix_web::{get, web, Responder};

use book_usecase::BookUsecase;

#[get("/hello")]
async fn hello(usecase: web::Data<Arc<BookUsecase>>) -> impl Responder {
    usecase.hello()
}
