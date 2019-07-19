extern crate lettre;
extern crate lettre_email;

use lettre::{SmtpClient, Transport};
use lettre_email::{mime::TEXT_PLAIN, Email};
use std::path::Path;

pub fn send_email() {
    let email = Email::builder()
        // Addresses can be specified by the tuple (email, alias)
        .to(("e_t_rag_as@gmail.com", "Elias Tragas"))
        .to(("ka_th_ge@gmail.com", "Kathy Ge"))
        // ... or by an address only
        .from("cat_predatora!!hlu_hmu_hb@gmail.com")
        .subject("Hi, Hello world")
        .text("Hello world.")
        .attachment_from_file(Path::new("Cargo.toml"), None, &TEXT_PLAIN)
        .unwrap()
        .build()
        .unwrap();

    // Open a local connection on port 25
    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}
