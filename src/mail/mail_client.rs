use crate::api_client::api_client::*;

pub struct MailClient<'a> {
    mail_address: &'a str
}

pub struct MailClientFactory<'a> {
    mail_address: &'a str
}

impl<'a> MailClientFactory<'a> {
    pub fn new (user_name: &'a str) -> Box<dyn ApiClientFactory + 'a> {
        Box::new(MailClientFactory{mail_address: user_name})
    }
}

impl<'a> ApiClientFactory<'a> for MailClientFactory<'a> {
    fn build(&self) -> Box<dyn ApiClient + 'a> {
        Box::new(MailClient{mail_address: self.mail_address.clone()})
    }
}

impl ApiClient for MailClient<'_> {
    fn post(&self, message: &str) -> String {
        format!("[{}]send massage \"{}\"", &self.mail_address, message)
    }
}