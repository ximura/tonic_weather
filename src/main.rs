use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_weather {
    tonic::include_proto!("weather");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("weather_descriptor");
}
use grpc_weather::weather_server::{Weather, WeatherServer};
use grpc_weather::{GetWeatherRequest, Unit, WeatherResponse};

#[derive(Debug, Default)]
pub struct WeatherService {}

#[tonic::async_trait]
impl Weather for WeatherService {
    async fn get(
        &self,
        request: Request<GetWeatherRequest>,
    ) -> Result<Response<WeatherResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = grpc_weather::WeatherResponse {
            temperature: 10.0,
            unit: Unit::Celsius as i32,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = WeatherService::default();
    let server = WeatherServer::new(service);
    let weather = tonic_web::enable(server);

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<WeatherServer<WeatherService>>()
        .await;

    let reflect_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_weather::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Running on port {}...", port);
    Server::builder()
        .accept_http1(true)
        .add_service(reflect_service)
        .add_service(health_service)
        .add_service(weather)
        .serve(addr)
        .await?;

    Ok(())
}
