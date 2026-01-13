//! Demo Rust Lettre async tokio1 rustls tls
//!
//! This is a demonstration of the Rust Lettre crate to
//! send email asynchronously using tokio1 rustls tls.
//!
//! - <https://docs.rs/crate/lettre/>
//!
//! Get the code:
//!
//! ```sh
//! git clone https://github.com/joelparkerhenderson/demo-rust-lettre-async-tokio1-rustls-tls.git
//! ```
//!
//! Set environment variables any way you want:
//!
//! ```sh
//! FROM="alice@example.com"
//! TO="bob@example.com"
//! MAILER_HOST="smtp.example.com"
//! MAILER_PORT=465
//! MAILER_USERNAME="mailer@example.com"
//! MAILER_PASSWORD="654f4b94c733f619"
//! ```
//!
//! Run:
//!
//! ```sh
//! cargo run
//! ```
//!
//! Output:
//!
//! ```stdout
//! Email sent successfully!
//! ```
//!
//! ## Purpose
//!
//! This project helps us with debugging SMTP issues, and the project
//! intentionally prints a bunch of debugging information.
//!
//! ## Comparison
//!
//! We have mulitple projects so you can choose among configurations:
//!
//! ### sync
//!
//! Try this project for learning because it's easiest:
//!
//! <https://github.com/joelparkerhenderson/demo-rust-lettre-sync>
//!
//! Cargo.toml dependency:
//!
//! ```toml
//! lettre = "0.11"
//! ```
//!
//! ### async tokio1 rustls tls
//!
//! Try this project for cross-plaform production use.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-lettre-async-tokio1-rustls-tls>
//!
//! Cargo.toml dependency:
//!
//! ```toml
//! lettre = { version = "0.11", default-features = false, features = [
//!     "builder",
//!     "hostname",
//!     "smtp-transport",
//!     "tokio1",
//!     "tokio1-rustls",
//!     "rustls-tls",
//! ]}
//! ```
//!
//! ### async tokio1 native tls
//!
//! Try this project for plaform-specific production use.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-lettre-async-tokio1-native-tls>
//!
//! Cargo.toml dependency:
//!
//! ```toml
//! lettre = { version = "0.11", default-features = false, features = [
//!     "builder",
//!     "hostname",
//!     "smtp-transport",
//!     "tokio1",
//!     "tokio1-native-tls",
//! ]}
//! ```
//!
//! ## Features
//!
//! Be sure to read the documentation "Features" link because Lettre
//! behaves differently depending on feature for TLS and sync/async.
//!
//! Show which features are actually compiled in:
//!
//! ```sh
//! cargo tree -i lettre -e features
//! ```
//!
//! The output should show:
//!
//! - If lettre is using rustls vs native-tls
//! - If lettre is using sync vs async
//!
//! Example default output:
//!
//! ```stdout
//! lettre v0.11.19
//! ├── lettre feature "builder"
//! │   └── lettre feature "default"
//! │       └── demo-rust-lettre-async-tokio1-rustls-tls v0.1.0 (/Users/jph/git/joelparkerhenderson/demo/demo-rust-lettre-async-tokio1-rustls-tls)
//! │           └── demo-rust-lettre-async-tokio1-rustls-tls feature "default" (command-line)
//! ├── lettre feature "default" (*)
//! ├── lettre feature "hostname"
//! │   └── lettre feature "default" (*)
//! ├── lettre feature "native-tls"
//! │   ├── lettre feature "default" (*)
//! │   └── lettre feature "rustls-tls"
//! │       └── demo-rust-lettre-async-tokio1-rustls-tls v0.1.0 (/Users/jph/git/joelparkerhenderson/demo/demo-rust-lettre-async-tokio1-rustls-tls) (*)
//! ├── lettre feature "pool"
//! │   └── lettre feature "default" (*)
//! ├── lettre feature "smtp-transport"
//! │   └── lettre feature "default" (*)
//! ├── lettre feature "tokio1"
//! │   └── demo-rust-lettre-async-tokio1-rustls-tls v0.1.0 (/Users/jph/git/joelparkerhenderson/demo/demo-rust-lettre-async-tokio1-rustls-tls) (*)
//! │   └── lettre feature "rustls-tls" (*)
//! └── lettre feature "rustls-tls" (*)
//! ```

use lettre::{
    Message,
    AsyncSmtpTransport,
    AsyncTransport,
    Tokio1Executor,
    transport::smtp::authentication::Credentials,
};

#[tokio::main]
async fn main() {
    let message = message();
    let credentials = credentials();
    let async_smtp_transport = async_smtp_transport(credentials);
    match async_smtp_transport.send(message).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Error: {:?}", e),
    }
}

/// Create a credentials struct.
fn credentials() -> Credentials {
    let mailer_username = std::env::var("MAILER_USERNAME").expect("MAILER_USERNAME");
    let mailer_password = std::env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD");
    dbg!(&mailer_username);
    dbg!(&mailer_password);
    Credentials::new(mailer_username, mailer_password)
}

/// Create an async_smtp_transport.
fn async_smtp_transport(credentials: Credentials) -> AsyncSmtpTransport<Tokio1Executor> {
    let mailer_host = std::env::var("MAILER_HOST").expect("MAILER_HOST");
    let mailer_port = std::env::var("MAILER_PORT").expect("MAILER_PORT").parse::<u16>().expect("MAILER_PORT parse::<u16>");
    dbg!(&mailer_host);
    dbg!(&mailer_port);
    AsyncSmtpTransport::<Tokio1Executor>::relay(&mailer_host)
    .unwrap()
    .port(mailer_port)
    .credentials(credentials)
    .build()
}

/// Create a timestamp string.
fn timestamp() -> String {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("UNIX_EPOCH").as_secs().to_string()
}

/// Create a message.
fn message() -> Message {
    let from = std::env::var("FROM").expect("FROM");
    let to = std::env::var("TO").expect("TO");
    let subject = format!("Test {}", timestamp());
    let body = format!("Test from {} to {} ", from, to);
    dbg!(&from);
    dbg!(&to);
    dbg!(&subject);
    dbg!(&body);
    lettre::message::Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap()
}
