use std::collections::HashMap;

use git::Git;

mod git;

#[tokio::main]
async fn main() {
    let endpoint = "https://api.github.com/graphql";
    let mut headers = HashMap::new();
    headers.insert(
        "authorization",
        "Bearer REMOVED_TOKEN",
    );
    headers.insert("user-agent", "PostmanRuntime/7.29.2");
    let client =
        gql_client::Client::new_with_headers(endpoint, headers);

    Git::get_latest_commit_by_repo(
        client,
        "git-stats-bot",
        "maemreyo",
        5,
    )
    .await;
}
