use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tokio::{
    sync::Mutex,
    net::TcpStream,
};
use tokio_tungstenite::{accept_async, WebSocketStream, tungstenite::Message};
use std::{collections::HashMap};
use crate::message::{
    request::{Request, self},
    response::{Response, self},
};
use std::sync::Arc;
use sqlx::{Sqlite, SqlitePool, Transaction};

type User = String;
type AnyError = Box<dyn std::error::Error + Send + Sync>;

struct Responder {
    sinks: Mutex<HashMap<User, Vec<SplitSink<WebSocketStream<TcpStream>, Message>>>>
}

impl Responder {
    async fn add_user(&self, user: User, sink: SplitSink<WebSocketStream<TcpStream>, Message>) {
        self.sinks.lock().await.entry(user).or_insert(Vec::new()).push(sink);
    }

    async fn respond(&self, to: Vec<User>, response: &Response) {
        //let string = serde_json::to_string(&Response::RegisterResult(RegisterResult::InvalidName)).unwrap();
    }
}

pub struct Transport {
    responder: Arc<Responder>,
    conn: Arc<SqlitePool>,
}

impl Transport {
    pub async fn new() -> Self {
        Transport {
            responder: Arc::new(Responder {
                sinks: Mutex::new(HashMap::new())
            }),
            conn: Arc::new(SqlitePool::connect("sqlite:database.db").await.unwrap())
        }
    }

    pub async fn connect(&self, stream: TcpStream) {
        log::info!("New connection!");

        let responder = self.responder.clone();
        let conn = self.conn.clone();
        
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut sink, mut stream) = ws_stream.split();
            let mut user = None;

            while let Some(msg) = stream.next().await {
                let msg = msg.unwrap();
                if let Ok(text) = msg.to_text() {
                    let request: Request = serde_json::from_str(text).unwrap();
                    let mut transaction = conn.begin().await.unwrap();

                    let handler = async {
                        match request {
                            Request::Register(request::Register {
                                username,
                                password
                            }) => {
                                let available = sqlx::query!("
                                    SELECT *
                                    FROM users
                                    WHERE username = $1",
                                    username)
                                    .fetch_all(&mut transaction)
                                    .await?
                                    .is_empty();
    
                                if available {
                                    sqlx::query!("
                                        INSERT INTO users (username, password)
                                        VALUES ($1, $2)",
                                        username, password)
                                        .execute(&mut transaction)
                                        .await?;
    
                                    user = Some(username);
                                    sink.send(Response::Register(response::Register::Success).as_message()).await?;
                                } else {
                                    sink.send(Response::Register(response::Register::UsernameTaken).as_message()).await?;
                                }
                                
                            },
                            Request::Login(request::Login {
                                username,
                                password
                            }) => {
                                let users = sqlx::query!("
                                    SELECT password
                                    FROM users
                                    WHERE username = $1",
                                    username)
                                    .fetch_all(&mut transaction)
                                    .await?;

                                assert!(users.len() <= 1);

                                if let Some(row) = users.first() {
                                    if password == row.password {
                                        user = Some(username);
                                        sink.send(Response::Login(response::Login::Success).as_message()).await?;
                                    } else {
                                        sink.send(Response::Login(response::Login::IncorrectPassword).as_message()).await?;
                                    }
                                } else {
                                    sink.send(Response::Login(response::Login::UserNotFound).as_message()).await?;
                                }
                                
                            },
                            request if user.is_some() => {
                                // Handle authenticated requests.
                                game(user.clone().unwrap(), responder.clone(), request, &mut transaction).await?;
                            },
                            _ => {
                                // Ignore unauthenticated requests.
                                log::warn!("Unauthenticated request received.")
                            }
                        }
                        Ok::<(), AnyError>(())
                    };

                    match handler.await {
                        Ok(_) => {
                            transaction.commit().await.unwrap();
                        },
                        Err(err) => {
                            // If an error occured, roll back database.
                            transaction.rollback().await.unwrap();
                            log::error!("An error occured: {}", err);
                        }
                    }

                    /*let response = handle(request).await;*/
                    //transport.recv(transport);
                    //log::debug!("Request received: {:?}", request);
                    //let string = serde_json::to_string(&).unwrap();
                    //ws_stream.send(Message::Text(string)).await?;
                    //transport.respond(vec![String::new()], &Response::RegisterResult(RegisterResult::InvalidName)).await;
                }
            }
        });
    }
}

async fn game<'c>(user: User, responder: Arc<Responder>, request: Request, transaction: &mut Transaction<'c, Sqlite>) -> Result<(), AnyError> {
    //responder.respond(vec![user], &Response::Init).await;
    panic!();
}