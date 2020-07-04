use crate::api_client::api_client::*;

pub struct SnsClient<'a> {
    user_name: &'a str
}

pub struct SnsClientFactory<'a> {
    user_name: &'a str
}

impl<'a> SnsClientFactory<'a> {
    pub fn new (user_name: &'a str) -> Box<dyn ApiClientFactory + 'a> {
        Box::new(SnsClientFactory { user_name })
    }
}

impl<'a> ApiClientFactory<'a> for SnsClientFactory<'a> {
    fn build(&self) -> Box<dyn ApiClient + 'a> {
        Box::new(SnsClient { user_name: self.user_name.clone()})
    }
}

impl ApiClient for SnsClient<'_> {
    fn post(&self, message: &str) -> String {
        format!("post message \"{}\" as user \"{}\"", message, &self.user_name)
    }
}