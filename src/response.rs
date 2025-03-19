pub async fn print_response(response: reqwest::Response) -> anyhow::Result<()> {
    let status = response.status().to_string();
    println!("Status: {}", status);

    println!("-----------------------------------");

    let headers = response.headers();
    println!("Headers: {:?}", headers);

    println!("-----------------------------------");

    let body = response.text().await?;
    println!("Body: {}", body);

    println!("----------------------------------------------------------------------|");
    Ok(())
}
