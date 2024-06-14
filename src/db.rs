use bson::Document;
use mongodb::{ bson::doc, options:: ClientOptions , Client };

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;

    let client = Client::with_options(client_options)?;

    // Example find operation
    let result: Option<Document> = client
        .database("test")
        .collection("orders")
        .find_one(doc! {"city": { "$lte": "Gadarpur" }}, None)
        .await?;

    if let Some(doc) = result {
        if let Some(id) = doc.get("_id") {
            println!("{:?}",id);
        }
    }
    
    
    Ok(())
}