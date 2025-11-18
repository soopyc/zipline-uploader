use crate::api::ZiplineApi;

pub struct Client {
	token: String
}

impl ZiplineApi for Client {
	fn new(token: &str) -> Self {
		Self {
			token: token.into()
		}
	}

	fn upload() {
		todo!()
	}

	fn upload_partial() {
		todo!()
	}

	fn get_settings() {
		todo!()
	}
}
