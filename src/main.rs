use orbtk::prelude::*;
use actix_web::{HttpServer, App, middleware, HttpRequest, web, get};
use std::thread::spawn;

fn client_run() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - Calculator example")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .child(ItemsWidget::create()
                    .child(TextBox::create().text("张").build(ctx))
                    .child(TextBox::create().text("b").build(ctx))
                    .build(ctx))
                .build(ctx)
        })
        .run();
}

fn service_run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
}

#[get("/resource1/{name}/index.html")]
fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", name)
}

fn main() -> std::io::Result<()> {
    spawn(|| {
        client_run();
    });

    service_run()
}
