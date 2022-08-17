use actix::prelude::*;
use actix::{Actor, Handler, StreamHandler};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws::{self, WsResponseBuilder};
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use serde::Serialize;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;
use tokio::task;

#[get("/")]
async fn get() -> impl Responder {
  println!("GET /");
  HttpResponse::Ok().body("test")
}

/// Define http actor
struct MyWs;

impl Actor for MyWs {
  type Context = ws::WebsocketContext<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Payload<T> {
  pub payload: T,
}

impl<T> Handler<Payload<T>> for MyWs
where
  T: Serialize + Debug,
{
  type Result = ();

  fn handle(&mut self, msg: Payload<T>, ctx: &mut Self::Context) {
    println!("handle {:?}", msg.payload);
    ctx.text(serde_json::to_string(&msg.payload).expect("Cannot serialize"));
  }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => ctx.text(text),
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => ctx.close(reason),
      _ => {}
    }
  }
}

async fn index(req: HttpRequest, stream: web::Payload) -> HttpResponse {
  let (addr, resp) = WsResponseBuilder::new(MyWs {}, &req, stream)
    .start_with_addr()
    .unwrap();

  let recipient = addr.recipient();
  task::spawn(async move {
    loop {
      let result = recipient.send(Payload {
        payload: get_gpu_temperature(),
      });
      let result = result.await;

      if result.is_err() || !recipient.connected() {
        break;
      }

      sleep(Duration::from_secs(1));
    }
  });
  println!("{:?}", resp);
  resp
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  println!("Started");
  HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
    .bind("0.0.0.0:8120")?
    .run()
    .await
}

fn get_gpu_temperature() -> String {
  let nvml = Nvml::init().unwrap();
  let device = nvml.device_by_index(0).unwrap();
  let temperature = device.temperature(TemperatureSensor::Gpu).unwrap();

  temperature.to_string().replace("[^0-9.]", "")
}
