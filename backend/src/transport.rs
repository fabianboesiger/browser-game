use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::{Mutex, mpsc::{self, UnboundedSender}}};
use tokio_tungstenite::accept_async;
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
    sinks: Mutex<HashMap<User, Vec<UnboundedSender<Response>>>>
}

impl Responder {
    async fn add_user(&self, user: User, tx: UnboundedSender<Response>) {
        self.sinks.lock().await.entry(user).or_insert(Vec::new()).push(tx);
    }

    async fn update_user(&self, user: User, new_user: User) {
        //self.sinks.lock().await.entry(user).or_insert(Vec::new()).push(tx);
    }

    async fn respond(&self, to: &[User], response: Response) {
        //let string = serde_json::to_string(&Response::RegisterResult(RegisterResult::InvalidName)).unwrap();
        for user in to {
            if let Some(txs) = self.sinks.lock().await.get(user) {
                for tx in txs {
                    tx.send(response.clone()).unwrap();
                }
            }
        }
    }
}

pub struct Transport {
    responder: Arc<Responder>,
    conn: SqlitePool,
}

impl Transport {
    pub async fn new() -> Self {
        Transport {
            responder: Arc::new(Responder {
                sinks: Mutex::new(HashMap::new())
            }),
            conn: SqlitePool::connect("sqlite:database.db").await.unwrap()
        }
    }

    pub async fn connect(&self, stream: TcpStream) {
        log::info!("New connection!");

        let responder = self.responder.clone();
        let conn = self.conn.clone();
        
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut sink, mut stream) = ws_stream.split();
            let (tx, mut rx) = mpsc::unbounded_channel::<Response>(); 
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    sink.send(msg.as_message()).await.unwrap();
                }
            });
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
                                    responder.add_user(user.clone().unwrap(), tx.clone()).await;
                                    tx.send(Response::Register(response::Register::Success))?;
                                } else {
                                    tx.send(Response::Register(response::Register::UsernameTaken))?;
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
                                        responder.add_user(user.clone().unwrap(), tx.clone()).await;
                                        tx.send(Response::Login(response::Login::Success))?;
                                    } else {
                                        tx.send(Response::Login(response::Login::IncorrectPassword))?;
                                    }
                                } else {
                                    tx.send(Response::Login(response::Login::UserNotFound))?;
                                }
                                
                            },
                            request if user.is_some() => {
                                // Handle authenticated requests.
                                game(user.clone().unwrap(), request, responder.clone(), &mut transaction).await?;
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

async fn game<'c>(user: User, request: Request, responder: Arc<Responder>, transaction: &mut Transaction<'c, Sqlite>) -> Result<(), AnyError> {
    //responder.respond(vec![user], &Response::Init).await;
    match request {
        Request::Register(_) | Request::Login(_) => unreachable!(),
    }
}