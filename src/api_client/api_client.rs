pub trait ApiClient {
    fn post(&self, message: &str) -> String;
}

pub trait ApiClientFactory<'a> {
    fn build(&self) -> Box<dyn ApiClient + 'a>;
}