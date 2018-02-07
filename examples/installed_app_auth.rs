//! This example shows authorizing as an installed app to retrieve info about the user authorized.
//!
//! This example requires registering the app as an installed app at [Reddit](https://www.reddit.com/prefs/apps)

extern crate hyper;
extern crate orca;

use orca::{App, InstalledAppError, OAuthApp, ResponseGenFn};

use hyper::Response;

fn input(query: &str) -> String {
	use std::io::Write;
	let stdin = std::io::stdin();
	print!("{}", query);
	std::io::stdout().flush().unwrap();
	let mut input = String::new();
	stdin.read_line(&mut input).unwrap();
	input.trim().to_string()
}

fn main() {
	let mut reddit = App::new("orca_installed_app_example", "1.0", "/u/IntrepidPig").unwrap();

	println!("Please enter the requested information");
	let id = input("App id: ");
	let redirect = input("Redirect URI: ");
	// If you don't want to deal with custom response generation you can just set this to None to have simple defaults
	let response_gen: Option<std::sync::Arc<ResponseGenFn>> = Some(std::sync::Arc::new(|result| match result {
		Ok(_code) => {
			println!("Authorized successfully");
			Ok(Response::new().with_body("Congratulations! You authorized successfully"))
		}
		Err(_e) => {
			println!("Authorization error");
			Ok(Response::new().with_body("Sorry! There was an error with the authorization."))
		}
	}));

	let auth = OAuthApp::InstalledApp {
		id,
		redirect,
		response_gen,
	};
	reddit.authorize(&auth).unwrap();

	let user = reddit.get_self().unwrap();
	println!("Got data: {}", user);
}
