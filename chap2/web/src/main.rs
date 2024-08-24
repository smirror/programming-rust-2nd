use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m ==0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
    format!("The greateset common divisor of the numbers {} and {} is <b>{}</b>\n",
    form.m, form.n, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}