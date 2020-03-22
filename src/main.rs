use actix_multipart::Multipart;
use actix_web::body::Body;
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use image;
use image::imageops::FilterType;
use image::ImageOutputFormat;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

async fn compress_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // TODO:
    // To handle file checking
    // return response with new image and what not
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let headers = field.headers();
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        if let Some(header) = headers.iter().next() {
            println!("Got headers {}!", header.0.as_str());
            println!("Got value {:?}!", header.1.to_str());
        }
        println!("Got filename {}!", filename);
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let img = image::load_from_memory(&data).unwrap();
            let resized_img = img.resize(1000, 1000, FilterType::Nearest);
            let mut vec = Vec::new();
            resized_img
                .write_to(&mut vec, ImageOutputFormat::Png)
                .unwrap();
            let response = HttpResponse::with_body(
                StatusCode::from_u16(200).unwrap(),
                Body::from_slice(&vec[..]),
            );
            return Ok(response);
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(compress_file))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
