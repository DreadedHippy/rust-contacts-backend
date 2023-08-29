use crate::models::{DynamicData, JsonResponse};

pub fn create_json_response(status: bool, message: String, data: Option<DynamicData>) -> JsonResponse {
	JsonResponse { status, message, data}
}