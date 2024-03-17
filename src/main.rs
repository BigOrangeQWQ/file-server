mod util;
mod db;

use std::path::Path;
use std::sync::OnceLock;
use db::TokenData;
use salvo::fs::NamedFile;
use salvo::prelude::*;

use util::RespData;
use util::random_chars;

static DATADB: OnceLock<TokenData> = OnceLock::new();


#[inline]
pub fn get_db() -> &'static TokenData {
    DATADB.get().unwrap()
}




#[handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let files = req.files("files").await;
    if let Some(files) = files {
        let mut msgs = Vec::with_capacity(files.len());
        for file in files {
            let token = random_chars(6);
            let dest = format!("data/{}", token);

            get_db().put(&token, &file.name().unwrap());
            // affix::insert(token.clone(), file.path().clone());
            if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(
                    RespData::error_str(format!("failed to save file: {}", e))
                ));
            } else {
                msgs.push(token);
            }
        }
        res.status_code(StatusCode::OK);
        res.render(Json(
            RespData::ok(msgs)
        ));
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(
            RespData::error_str("no file uploaded"),
        ));
    }
}

#[handler]
async fn index(req: &mut Request, res: &mut Response) {
    NamedFile::open("data/index.html").await.unwrap().
        send(req.headers(), res).await;
    // println!("index");
}

#[handler]
async fn get_file(req: &mut Request, res: &mut Response) {
    let id = req.param::<&str>("*+id").unwrap();
    let name = get_db().get(id);

    let path = format!("data/{}", id);
    if Path::new(&path).exists() {
        NamedFile::builder(path).attached_name(name).
            send(req.headers(), res).await;
    } else {
        res.status_code(StatusCode::NOT_FOUND);
        res.render(Json(
            RespData::error_str("file not found"),
        ));
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let _ = DATADB.set(TokenData::new());

    let router = Router::new()
        .get(index)
        .post(upload)
        .push(
            Router::with_path("<*+id>").get(get_file)
        );

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
