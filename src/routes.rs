use actix_web::{HttpResponse, get, Responder, post, web::{Json, self}};

use crate::{models::{Contact, DynamicData}, utils::create_json_response};

#[get("/contact")]

// List contacts
pub async fn list_contacts() -> impl Responder {	
	let db_connection = super::create_database().unwrap_or_else(|_| panic!("Could not connect to db"));
	match Contact::list(&db_connection) {
		Ok(m) => {
			let json_response = create_json_response(true, String::new(), Some(DynamicData::List(m)));
			HttpResponse::Ok().json(json_response)
		},
		Err(e) => {
			let json_response = create_json_response(false, e, None);
			HttpResponse::InternalServerError().json(json_response)
		},
	}
}

// Create new contact
#[post("/contact/new")]
pub async fn create_contact(contact: Json<Contact>) -> impl Responder {
	if contact.name.is_none(){
		return HttpResponse::BadRequest().body("Please enter a valid name")
	}
	
	if contact.number.is_none() {
		return HttpResponse::BadRequest().body("Please enter a valid number")
	}	
	let db_connection = super::create_database().unwrap_or_else(|_| panic!("Could not connect to db"));

	match contact.create(&db_connection) {
		Err(e) => {
			let json_response = create_json_response(false, e, None);
			HttpResponse::InternalServerError().json(json_response)
		},
		Ok(m) => {
			let json_response = create_json_response(true, m, None);
			HttpResponse::Ok().json(json_response)
		}
	}
}


// Get specific contact
#[get("/contact/{id}")]
pub async fn get_contact(id: web::Path<u32>) -> impl Responder {
	let db_connection = super::create_database().unwrap_or_else(|_| panic!("Could not connect to db"));

	match Contact::get(&db_connection, id.into_inner()) {
		Err(e) => {
			let json_response = create_json_response(false, e, None);
			HttpResponse::InternalServerError().json(json_response)
		},
		Ok(m) => {
			let json_response = create_json_response(true, "".to_string(), Some(DynamicData::Item(m)));
			HttpResponse::Ok().json(json_response)
		}
	}
}


// Edit contact
#[post("/contact/{id}/edit")]
pub async fn edit_contact(id: web::Path<u32>, contact: Json<Contact>) -> impl Responder {
	let db_connection = super::create_database().unwrap_or_else(|_| panic!("Could not connect to db"));
	let id = id.into_inner();
	let old_info = Contact::get(&db_connection, id).unwrap();


	let new_info = Contact {
		id: Some(id as i32),
		name: if contact.name.is_none() {old_info.name} else { contact.name.clone()},
		number: if contact.number.is_none() {old_info.number} else { contact.number.clone()},
		img_src: match (contact.img_src.clone(), old_info.img_src) {
			(None, None) => Some("".to_string()),
			(None, Some(old)) => Some(old),
			(Some(new), _) => Some(new)			
		},
		email: if contact.email.is_none() {old_info.email} else {contact.email.clone()}
	};

	match new_info.edit(&db_connection, id) {
		Err(e) => {
			let json_response = create_json_response(false, e, None);
			HttpResponse::InternalServerError().json(json_response)
		},
		Ok(m) => {
			let json_response = create_json_response(true, m, None);
			HttpResponse::Ok().json(json_response)
		}
	}
}

// Delete contact
#[get("/contact/{id}/delete")]
pub async fn delete_contact(id: web::Path<u32>) -> impl Responder {
	let db_connection = super::create_database().unwrap_or_else(|_| panic!("Could not connect to db"));

	match Contact::delete(&db_connection, id.into_inner()) {
		Err(e) => {
			let json_response = create_json_response(false, e, None);
			HttpResponse::InternalServerError().json(json_response)
		},
		Ok(m) => {
			let json_response = create_json_response(true, m, None);
			HttpResponse::Ok().json(json_response)
		}
	}
}