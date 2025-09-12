use chrono::{DateTime, TimeZone, Utc};
use neo4rs::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

#[derive(Debug)]
struct Tweet {
    user: String,
    created_at: chrono::DateTime<Utc>,
    retweet_count: u32,
    retweeted: bool,
    favorite_count: u32,
    text: String,
    retweet_from_user: String,
}

// the use of clone here seems unfavorable. Is there a better way?
async fn insert_tweets(graph: Arc<Graph>, tweets: Vec<Tweet>) -> anyhow::Result<()> {
    const BATCH_SIZE: usize = 500;

    for batch in tweets.chunks(BATCH_SIZE) {
        let mut tx = graph.start_txn().await?;
        for tweet in batch {
            tx.run(
                query(
                    "MERGE (u:User {user_id: $user_id})\
                    MERGE (r:User {user_id: $retweet_from_user})\
                    CREATE (t:Tweet {\
                    text: $text,
                    created_at: $created_at,
                    retweet_count: $retweet_count,
                    retweeted: $retweeted,
                    favorite_count: $favorite_count,
                    retweet_from_user: $retweet_from_user
                    })
                    MERGE (u)-[:TWEETED]->(t)
                    MERGE (u)-[:RETWEETED]->(r)",
                )
                .param("user_id", tweet.user.clone())
                .param("text", tweet.text.clone())
                .param("created_at", tweet.created_at.timestamp())
                .param("retweet_count", tweet.retweet_count)
                .param("retweeted", tweet.retweeted)
                .param("favorite_count", tweet.favorite_count)
                .param("retweet_from_user", tweet.retweet_from_user.clone()),
            )
            .await?;
        }
        tx.commit().await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "neo4j+s://d7de04cc.databases.neo4j.io";
    let user = "neo4j";
    let pass = std::env::var("NEO4J_PASSWORD").unwrap();
    let graph = Arc::new(Graph::new(uri, user, pass).await?);

    // load tweets from file and insert them into the graph. This is a one time thing.
    /*let tweets =
        load_tweets("../data.csv");

    let graph = Arc::new(Graph::new(uri, user, pass).await?);

    let result = insert_tweets(graph.clone(), tweets).await;
    println!("{:?}", result);*/

    let retweet_rings_cyper = "MATCH (u1:User)-[:RETWEETED]->(u2:User) WITH u1,\
                                     collect(u2.user_id) as retweeted_accounts, count(*) as n where n > 50 \
                                     RETURN u1.user_id as user, n ORDER BY n DESC";
    let mut result = graph.execute(query(retweet_rings_cyper)).await?;
    while let Ok(Some(row)) = result.next().await {
        let user_id: String = row.get("user").unwrap();
        let n: i32 = row.get("n").unwrap();
        println!("{}: {}", user_id, n);
    }

    let tight_group_cypher = "MATCH (u:User)-[:RETWEETED]->(v:User) \
                                    WITH collect(DISTINCT u) + collect(DISTINCT v) AS users \
                                    MATCH path = (a:User)-[:RETWEETED*3..5]-(b:User) return path limit 20";
    let mut result = graph.execute(query(tight_group_cypher)).await?;
    while let Ok(Some(row)) = result.next().await {
        let path: Path = row.get("path").unwrap();
        let mut user_ids = Vec::new();
        for node in path.nodes() {
            if node.labels().contains(&"User") {
                if let Ok(user_id) = node.get::<String>("user_id") {
                    user_ids.push(user_id);
                }
            }
        }
        println!("{:?}", user_ids);
    }

    let rapid_fire = "MATCH (u:User)-[:TWEETED]->(t:Tweet) \
                            WITH u, datetime({epochSeconds: t.created_at}) AS ts \
                             ORDER BY u.user_id, ts WITH u, collect(ts) AS times \
                             UNWIND range(0, size(times)-2) as i \
                             WITH u, duration.inSeconds(times[i], times[i+1]) AS between, times[i] AS time1, times[i+1] AS time2 \
                             WHERE between.seconds < 30 \
                             RETURN u.user_id as user, count(*) AS rapid_posts \
                             ORDER BY rapid_posts DESC LIMIT 10";
    let mut rapid_fire_result = graph.execute(query(rapid_fire)).await?;
    while let Ok(Some(row)) = rapid_fire_result.next().await {
        let user_id: String = row.get("user").unwrap();
        let rapid_posts: i32 = row.get("rapid_posts").unwrap();
        println!("{}: {}", user_id, rapid_posts);
    }

    Ok(())
}

// a function which reads a CSV file and returns a vector of tweets
fn load_tweets(filename: &str) -> Vec<Tweet> {
    let mut tweets = Vec::new();
    let csv = File::open(filename).unwrap();
    let reader = BufReader::new(csv);
    // for each line in the file
    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(",").collect::<Vec<&str>>();
        if parts.len() < 7 {
            // skip this line
            continue;
        }
        let tweet = Tweet {
            user: parts[1].to_string(),
            created_at: parse_timestamp(parts[2]),
            retweet_count: safely_parse_number_sometimes_empty(parts[4]),
            retweeted: safely_parse_bool_sometimes_empty(parts[5]),
            favorite_count: safely_parse_number_sometimes_empty(parts[6]),
            text: parts[7].to_string(),
            retweet_from_user: extrapolate_rt(parts[7].to_string()),
        };
        tweets.push(tweet);
    }

    tweets
        .into_iter()
        .filter(|tweet| tweet.created_at.timestamp() != 0)
        .collect()
}

fn safely_parse_bool_sometimes_empty(retweeted_field: &str) -> bool {
    if retweeted_field.is_empty() {
        false
    } else {
        retweeted_field.parse().unwrap_or(false)
    }
}

fn safely_parse_number_sometimes_empty(count_field: &str) -> u32 {
    if count_field.is_empty() {
        0
    } else {
        count_field.parse().unwrap_or(0)
    }
}

fn parse_timestamp(timestamp_string: &str) -> DateTime<Utc> {
    let millis: i64 = timestamp_string.parse().unwrap_or(0);
    let secs = millis / 1000;
    let nanos = ((millis % 1000) * 1_000_000) as u32;
    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(0, 0).single().unwrap())
}

fn extrapolate_rt(text: String) -> String {
    if text.contains("RT @") {
        text.split("RT @").collect::<Vec<&str>>()[1]
            .split(":")
            .collect::<Vec<&str>>()[0]
            .to_string()
    } else {
        "".to_string()
    }
}
