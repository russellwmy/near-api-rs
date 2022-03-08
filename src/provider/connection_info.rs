use hashbrown::HashMap;

#[derive(Clone)]
pub struct ConnectionInfo {
    pub url: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub allow_insecure: bool,
    pub timeout: Option<u32>,
    pub headers: HashMap<String, String>,
}

impl ConnectionInfo {
    pub fn new(url: String) -> Self {
        let mut headers = HashMap::new();

        headers.insert("Content-Type", "application/json");

        Self {
            url,
            user: None,
            password: None,
            allow_insecure: false,
            timeout: Some(60),
            headers: HashMap::new(),
        }
    }
}
