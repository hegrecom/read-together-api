use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data, Response, Rocket, Orbit};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::guards::Logger;

pub struct LoggerFairing;

#[rocket::async_trait]
impl Fairing for LoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logger for request and response",
            kind: Kind::Liftoff | Kind::Request | Kind::Response,
        }
    }

    async fn on_liftoff(&self, _: &Rocket<Orbit>) {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set default tracing subscriber");
    }

    async fn on_request(&self, request: &mut Request<'_>, data: &mut Data<'_>) {
        let logger = request.local_cache(|| Logger::new());
        logger.log_request(request, data).await.expect("Failed to log request");
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {

    }
}
