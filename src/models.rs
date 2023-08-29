use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
	pub status: bool,
	pub message: String,
	pub data: Option<DynamicData>
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum DynamicData {
	List(Vec<Contact>),
	Item(Contact),
	Message(String)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Contact {
	pub id: Option<i32>,
	pub name: Option<String>,
	pub number: Option<String>,
	pub img_src: Option<String>,
	pub email: Option<String>
}

impl Contact {
	pub fn create(&self, db_connection: &Connection) -> Result<String, String> {
		let name = self.name.to_owned();
		let number = self.number.to_owned();
		let img_src = self.img_src.to_owned();
		let email = self.email.to_owned();

		if name.is_none() || number.is_none() {
			return Err("Invalid parameter. Please enter a valid name and number".to_string());
		}

		db_connection.execute(
			"
				INSERT INTO contacts (name, number, img_src, email) VALUES (?1, ?2, ?3, ?4)
			",
			(name, number, img_src, email)
		).map_err(|e| e.to_string())?;

		Ok("Contact Saved!".to_string())
	}

	
	pub fn list(db_connection: &Connection) -> Result<Vec<Contact>, String> {
		let mut db_result = db_connection.prepare(
			"
				SELECT id, name, number, img_src, email FROM contacts
			"
		).map_err(|e| e.to_string())?;

		let contacts = db_result.query_map([], |row| {
			Ok(Contact {
				id: row.get(0)?,
				name: row.get(1)?,
				number: row.get(2)?,
				img_src: row.get(3)?,
				email: row.get(4)?
			})
		}).map_err(|e| e.to_string())?;

		let mut result = Vec::new();
		for i in contacts {
			result.push(i.unwrap());
		}

		Ok(result)
	}

	
	pub fn get(db_connection: &Connection, id: u32) -> Result<Contact, String> {
		let mut db_result = db_connection.prepare(
			"
				SELECT
					id,
					name,
					number,
					img_src,
					email
				FROM contacts
				WHERE id =:id
			"
		).map_err(|e| e.to_string())?;

		let contacts = db_result.query_map(&[(":id", id.to_string().as_str())], |row| {
			Ok(Contact {
				id: row.get(0)?,
				name: row.get(1)?,
				number: row.get(2)?,
				img_src: row.get(3)?,
				email: row.get(4)?
			})
		}).map_err(|e| e.to_string())?;

		let mut result = Vec::new();
		for i in contacts {
			result.push(i.unwrap());
		}

		if result.is_empty() {
			return Err("No user with given ID found".to_string());
		}

		Ok(result[0].clone())
	}

	
	pub fn delete(db_connection: &Connection, id: u32) -> Result<String, String> {
		let mut prepared = db_connection.prepare(
			"
				DELETE
				FROM contacts
				WHERE id = ?1
			"
		).map_err(|e| e.to_string())?;

		prepared.execute([id]).map_err(|e| e.to_string())?;
		Ok("Contact deleted!".to_string())
	}

	
	pub fn edit(&self, db_connection: &Connection, id: u32) -> Result<String, String> {
		let mut prepared = db_connection.prepare(
			"
				UPDATE contacts
				SET name = ?1,
				number = ?2,
				img_src = ?3,
				email = ?4
				WHERE id = ?5
			"
		).map_err(|e| e.to_string())?;

		prepared.execute([self.name.to_owned(), self.number.to_owned(), self.img_src.to_owned(), self.email.to_owned(), Some(id.to_string())]).map_err(|e| e.to_string())?;
		Ok("Contact updated!".to_string())

		// let name = self.name.to_owned();
		// let number = self.number.to_owned();
		// let img_src = self.img_src.to_owned();
		// let email = self.email.to_owned();

		// if name.is_none() || number.is_none() {
		// 	return Err("Invalid parameter. Please enter a valid name and number".to_string());
		// }

		// db_connection.execute(
		// 	"
		// 		INSERT INTO contacts (name, number, img_src, email) VALUES (?1, ?2, ?3, ?4)
		// 	",
		// 	(name, number, img_src, email)
		// ).map_err(|e| e.to_string())?;

		// Ok("Contact Saved!".to_string())
	}
}