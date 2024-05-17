use actix_web::{web, App, HttpResponse, Responder, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    println!("serve on http://localhost:3000");
    let _ = server
        .bind("127.0.0.1:3000")
        .expect("unable bind to 3000")
        .run().await;
}


async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
		r#"
		<title>GCD Calculator</title>
		<form action="/gcd" method="post">
		<input type="text" name="n"/>
		<input type="text" name="m"/>
		<button type="submit">Compute GCD</button>
		</form>
		"#,
	)
}
