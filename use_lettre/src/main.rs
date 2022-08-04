use lettre::{smtp::authentication::Credentials};
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder,Mailbox};

fn main() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("x@163.com".to_string()))
        .to(Mailbox::new("y@qq.com".to_string()))
        .subject("test lettre")
        .body("This is a email from lettre")
        .build()
        .unwrap();
    
        let creds = Credentials::new("".to_string(),"".to_string());
        
        let mut mailer = SmtpClient::new_simple("smtp.163.com")
            .unwrap()
            .credentials(creds)
            .transport();
        
        let result = mailer.send(email.into());
        if result.is_ok() {
            println!("Email sent");
        } else {
            println!("Could not send email:{:?}",result);
        }

        assert!(result.is_ok());
    
}
