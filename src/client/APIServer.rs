use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use mime::APPLICATION_JSON;
use serde::Serialize;
use mysql::*;
use mysql::prelude::*;
// type SqlConn = Arc<Mutex<HashMap<usize, PooledConn>>>;
// type Result<T> = std::result::Result<T, Rejection>;
pub struct APIServer {
}
impl APIServer {
    pub fn new() -> Result<Self> {
        Ok(APIServer {  })
    }
    async fn pp_rate(db_pool: web::Data<Pool>) -> impl Responder {
        // async fn pp_rate() -> impl Responder {
        let pool = db_pool.as_ref();
        let mut conn = pool.get_conn().unwrap();
        let missed_attestations :u64 = conn.query_first("SELECT COUNT(*) from attestations where attested=0").unwrap().unwrap();
        let total_size :u64 = conn.query_first("SELECT COUNT(*) from attestations").unwrap().unwrap();
        let mut pp_rate : f64 = missed_attestations as f64;
        pp_rate/=(total_size as f64);
        pp_rate = (1 as f64) - pp_rate;
        println!("{} {}", missed_attestations, total_size);
        return HttpResponse::Ok().json(pp_rate);
        // println!("got req");
        // return HttpResponse::Ok().json(format!("12421"));
    }

    async fn pp_rate_pubkey(web::Path(pubkey) : web::Path<String>,db_pool: web::Data<Pool>) -> impl Responder {
        // async fn pp_rate_pubkey(web::Path(pubkey) : web::Path<String>) -> impl Responder {
            // println!("got req {}", pubkey);
        let pool = db_pool.as_ref();
        let mut conn = pool.get_conn().unwrap();
        println!("{}", pubkey);
        let missed_attestations :u64 = conn.query_first(format!("SELECT COUNT(*) from attestations where attested=0 and pubkey=\"{}\"", pubkey)).unwrap().unwrap();
        let total_size :u64 = conn.query_first(format!("SELECT COUNT(*) from attestations where pubkey=\"{}\"", pubkey)).unwrap().unwrap();
        let mut pp_rate : f64 = missed_attestations as f64;
        pp_rate/=(total_size as f64);
        pp_rate = (1 as f64) - pp_rate;
        println!("{} {}", missed_attestations, total_size);
        return HttpResponse::Ok().json(pp_rate);
        // return HttpResponse::Ok().json(format!("12421"));
    }

    #[actix_web::main]
    pub async fn start_server(&self) -> std::io::Result<()> {
        let url = std::env::var("MYSQL_URL").unwrap();
        println!("{}", url);
        println!("started srver");
        let connection = Pool::new(url.as_str()).unwrap();
        println!("started srver");
        HttpServer::new(move ||
            App::new()
            .data(connection.clone())
            .route("/get/{pubkey}",web::get().to( APIServer::pp_rate_pubkey))
            .route("/get",web::get().to( APIServer::pp_rate))
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
    }
}
