use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use mysql::*;
use mysql::prelude::*;

pub struct APIServer { }
/**
 * Class for Creating an API server for fetching participation rate.
 */
impl APIServer {
    pub fn new() -> Result<Self> {
        Ok(APIServer {  })
    }

    /**
     * Fetch participation rate of the network.
     */
    async fn pp_rate(db_pool: web::Data<Pool>) -> impl Responder {
        let pool = db_pool.as_ref();
        let mut conn = pool.get_conn().unwrap();
        let mut missed_attestations :u64 = 0;
        match conn.query_first("SELECT COUNT(*) from attestations where attested=0") {
            Ok(result) => {
                match result {
                    Some(num) => {
                        missed_attestations = num;
                    },
                    None => {
                    }
                    
                }
            },
            Err(err) => {
                println!("Unable to perform query {}", err.to_string());
                return HttpResponse::InternalServerError().json("Unable to perform query");
            }
        }
        let mut total_size :u64 = 0;
        match conn.query_first("SELECT COUNT(*) from attestations") {
            Ok(result) => {
                match result {
                    Some(num) => {
                        total_size = num;
                    },
                    None => {
                    }
                    
                }
            },
            Err(err) => {
                println!("Unable to perform query {}", err.to_string());
                return HttpResponse::InternalServerError().json("Unable to perform query");
            }
        }
        if total_size == 0 {
            let response_str = "No data present";
            return  HttpResponse::InternalServerError().json(response_str);
        }
        let mut pp_rate : f64 = missed_attestations as f64;
        pp_rate/= total_size as f64;
        pp_rate = (1 as f64) - pp_rate;
        println!("No. of missed attesttion {}, total participation {}", missed_attestations, total_size);
        return HttpResponse::Ok().json(pp_rate);
    }

    /**
     * Fetch participation rate of a particular validator based on public Key
     */
    async fn pp_rate_pubkey(web::Path(pubkey) : web::Path<String>,db_pool: web::Data<Pool>) -> impl Responder {
        let pool = db_pool.as_ref();
        let mut conn = pool.get_conn().unwrap();
        let mut missed_attestations :u64 = 0;
        match conn.query_first(format!("SELECT COUNT(*) from attestations where attested=0 and pubkey=\"{}\"", pubkey)) {
            Ok(result) => {
                match result {
                    Some(num) => {
                        missed_attestations = num;
                    },
                    None => {}
                    
                }
            },
            Err(err) => {
                println!("Unable to perform query {}", err.to_string());
                return HttpResponse::InternalServerError().json("Unable to perform query");
            }
        }
        let mut total_size :u64 = 0;
        match conn.query_first(format!("SELECT COUNT(*) from attestations where pubkey=\"{}\"", pubkey)) {
            Ok(result) => {
                match result {
                    Some(num) => {
                        total_size = num;
                    },
                    None => {}
                }
            },
            Err(err) => {
                println!("Unable to perform query {}", err.to_string());
                return HttpResponse::InternalServerError().json("Unable to perform query");
            }
        }
        if total_size == 0 {
            let response_str = "Given validator's public key not found";
            return  HttpResponse::NotFound().json(response_str);
        }
        let mut pp_rate : f64 = missed_attestations as f64;
        pp_rate/= total_size as f64;
        pp_rate = (1 as f64) - pp_rate;
        println!("No. of missed attesttion {}, total participation {}, publickkey : {}", missed_attestations, total_size, pubkey);
        return HttpResponse::Ok().json(pp_rate);
    }

    #[actix_web::main]
    pub async fn start_server(&self) -> std::io::Result<()> {
        let url = std::env::var("MYSQL_URL").unwrap();
        println!("started API server");
        let connection = Pool::new(url.as_str()).unwrap();
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
