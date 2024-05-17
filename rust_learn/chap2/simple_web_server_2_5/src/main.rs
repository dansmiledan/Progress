use actix_web::{web, App, HttpResponse, Responder, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
		.route("/gcd", web::post().to(post_gcd))
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


async fn post_gcd(form: web::Form<GcdParamter>) -> HttpResponse {
	if form.n == 0 || form.m == 0 {
		return HttpResponse::BadRequest().content_type("text/html").body("computing gcd with zero is boring");
	}
	let response = format!(
		"The greatest common divisor of the number {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m)
	);
	HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m = m % n;
	}
	return n;
}

#[derive(Deserialize)]
struct GcdParamter {
	n: u64,
	m: u64
}