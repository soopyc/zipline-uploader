mod v4;

pub trait ZiplineApi {
	/// initialize a new api client.
	fn new(token: &str) -> Self;

	/// upload a file in whole.
	fn upload();
	/// upload a file in chunks, as specified by the server.
	fn upload_partial();
	/// get server settings. we won't need all of them so this will just return a subset.
	fn get_settings();
}
