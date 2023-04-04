#[macro_use] extern crate rocket;
use rocket::fs::TempFile;
use uuid::Uuid;
use std::path::Path;

#[post("/upload", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    let id = Uuid::new_v4();
    let form = format!("./files/{}", id.to_string());
    let path = Path::new(&form);
    file.persist_to(path).await?;
    Ok(())
}       


#[get("/download/<identifier>")]
fn download(identifier: &str) -> &'static str {
    "You did a heckin download!"
}

#[delete("/delete/<identifier>")]
fn delete(identifier: &str) -> &'static str {
    "OMG ITS GONE!"
}

#[put("/replace/<identifier>")]
fn replace(identifier: &str) -> &'static str {
    "A NEW FILE"
}

#[get("/list")]
fn list() -> &'static str {
    "A new list"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload, download, delete])
}
