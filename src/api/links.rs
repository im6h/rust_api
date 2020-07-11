use actix_web::{HttpRequest, HttpResponse};

use crate::models::link::LinkList;
pub fn index(_req: HttpRequest) -> HttpResponse {
  return  HttpResponse::Ok().json(LinkList::list());
}

use crate::models::link::NewLink;
use actix_web::web;
pub fn create(new_link: web::Json<NewLink>) -> Result<HttpResponse,HttpResponse> {
  return new_link
    .create()
    .map(|link| HttpResponse::Ok().json(link))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()));
}

use crate::models::link::Link;
pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
  return Link::find(&id)
    .map(|link| HttpResponse::Ok().json(link))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()));
}

pub fn delete(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
  return Link::delete(&id)   
    .map(|_| HttpResponse::Ok().json("Delete successed"))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()));
}

pub fn update(id: web::Path<i32>, new_link: web::Json<NewLink>) -> Result<HttpResponse, HttpResponse> {
  return Link::update(&id, &new_link)
    .map(|_| HttpResponse::Ok().json("Update successed"))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()));
}