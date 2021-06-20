use dynamodb;
use dynamodb::model::AttributeValue;
use std::collections::HashMap;

struct User {
    id: usize,
    name: String,
    subscribed: bool,
    followers: Vec<String>,
    tweets: Vec<Tweet>,
}

struct Tweet {
    title: String,
    like: usize,
    liked_users: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), dynamodb::Error> {
    let client = dynamodb::Client::from_env();
    let user = User {
        id: 0,
        name: "Sergey".to_string(),
        subscribed: true,
        followers: vec!["Satou".to_string(), "Cathy".to_string()],
        tweets: vec![Tweet {
            title: "Rust最高！".to_string(),
            like: 2,
            liked_users: vec!["Sizuka".to_string(), "Mononohu".to_string()],
        }],
    };
    put_item_manually(&client, user).await?;
    Ok(())
}

async fn put_item_manually(client: &dynamodb::Client, user: User) -> Result<(), dynamodb::Error> {
    let res = client
        .put_item()
        .table_name("example_dynamo")
        .item("id", AttributeValue::N(user.id.to_string()))
        .item("name", AttributeValue::S(user.name.clone()))
        .item("subscribed", AttributeValue::Bool(user.subscribed))
        .item(
            "followers",
            AttributeValue::L(
                user.followers
                    .iter()
                    .map(|x| AttributeValue::S(x.to_string()))
                    .collect(),
            ),
        )
        .item(
            "tweets",
            AttributeValue::L(
                user.tweets
                    .iter()
                    .map(|t| {
                        let mut map = HashMap::new();
                        map.insert("title".to_string(), AttributeValue::S(t.title.clone()));
                        map.insert("like".to_string(), AttributeValue::N(t.like.to_string()));
                        map.insert(
                            "liked_users".to_string(),
                            AttributeValue::L(
                                t.liked_users
                                    .iter()
                                    .map(|x| AttributeValue::S(x.to_string()))
                                    .collect(),
                            ),
                        );
                        AttributeValue::M(map)
                    })
                    .collect(),
            ),
        )
        .send()
        .await?;
    Ok(())
}
