
pub mod request {
    use serde::{Serialize, Deserialize};
    use tokio_tungstenite::tungstenite::Message;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Request {
        Register(Register),
        Login(Login),
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Register {
        pub username: String,
        pub password: String,
    }
    
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Login {
        pub username: String,
        pub password: String,
    }
}

pub mod response {
    use serde::{Serialize, Deserialize};
    use tokio_tungstenite::tungstenite::Message;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Response {
        Register(Register),
        Login(Login),
    }

    impl Response {
        pub fn as_message(&self) -> Message {
            Message::Text(serde_json::to_string(&self).unwrap())
        }
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Register {
        Success,
        InvalidName,
        UsernameTaken,
    }


    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Login {
        Success,
        UserNotFound,
        IncorrectPassword,
    }
}

