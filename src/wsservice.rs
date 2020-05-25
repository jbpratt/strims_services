use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    log::info!("{:?}", r);
    let res = ws::start(WSService::new(), &r, stream);
    log::info!("{:?}", res);
    res
}

struct WSService {
    hb: Instant,
}

impl WSService {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                log::info!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    /// Set the current stream defined for a client
    ///
    // Clear the current stream defined for the client. Dispatch the command to
    // the appropriate handler based on the json payload shape.
    //
    // If two strings are given treat them as channel and service names.
    // ex: ["setStream", "angelthump", "dariusirl"]
    //
    // If one string is given treat it as an overrustle user id.
    // ex: ["setStream", "dariusirl"]
    //
    // If a null literal is given ack without setting a stream
    // ex: ["setStream", null]
    fn set_stream(&self, input: Vec<String>) {
        let mut stream_id: u64 = 0;
        if input.len() == 3 {
            let channel = input[1].clone();
            let service = input[2].clone();
            Self::set_stream_to_channel(channel.as_str(), service.as_str(), &stream_id);
        } else if input.len() == 2 {
        }
    }

    fn set_stream_to_channel(channel: &str, service: &str, stream_id: &u64) {}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSService {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::info!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let res: Vec<String> = serde_json::from_str(text.as_str()).unwrap_or(Vec::new());
                if res.len() > 0 {
                    let action = res.first().unwrap();
                    if action.is_empty() {
                        return;
                    }

                    match action.as_str() {
                        "setAfk" => log::info!("setAfk: {:?}", res),
                        "setStream" => log::info!("setStream: {:?}", res),
                        "getStream" => log::info!("getStream: {:?}", res),
                        _ => log::info!("Unknown: {} {:?}", action, res),
                    }
                }
                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Actor for WSService {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}
