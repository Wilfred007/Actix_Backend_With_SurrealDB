use actix_web:: {
    http:: {
        header:: ContentType, StatusCode,
    },  HttpResponse, ResponseError
};


use derive_more::Display;
#[derive(Debug, Display)]
pub enum PizzaError {
    NoPizzaFound = 0,
    PizzaCreationError = 1,
    NoSuchPizzaFound =2,
}


impl ResponseError for PizzaError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())

    }

    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzaFound => StatusCode::NOT_FOUND, //404 page not found
            PizzaError::PizzaCreationError => StatusCode::INTERNAL_SERVER_ERROR, //500 internal server error
            PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND //404 page not found
        }
    }
}