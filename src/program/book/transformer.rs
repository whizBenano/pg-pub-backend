use tokio::process::Command;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, post, Result};
use futures_util::StreamExt;
use tempfile::tempdir;

#[post("/convert")]
async fn convert(mut payload: Multipart) -> Result<HttpResponse> {
    let dir = tempdir()?;

    let docx_path = dir.path().join("input.docx");
    let epub_path = dir.path().join("output.epub");
    
    let mut docx_file = std::fs::File::create(&docx_path)?;

    while let Some(item) = payload.next().await {
        let mut field = item?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            use std::io::Write;
            docx_file.write_all(&data)?;
        }
    }

    let output = Command::new("pandoc")
        .arg(&docx_path)
        .arg("-o")
        .arg(&epub_path)
        .output()
        .await?;

    if !output.status.success() {
        return Ok(HttpResponse::InternalServerError()
            .body(String::from_utf8_lossy(&output.stderr).to_string()));
    }

    let epub = std::fs::read(&epub_path)?;

    Ok(HttpResponse::Ok()
        .content_type("application/epub+zip")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"book.epub\"",
        ))
        .body(epub)
    )
}