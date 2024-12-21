#[allow(unused_imports)]
use log::{info, warn};


pub async fn line_changes(sha: &str, repo: &str) {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.github.com/repos/{}/commits/{}", repo, sha))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2224.3 Safari/537.36")
        .send()
        .await;
    let mut change:u64 = 0;
    let mut additions:u64 = 0;
    let mut deletions:u64 = 0;
    let mut filecount= 0;
    if let Ok(response) = resp {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            if let Some(files) = json.get("files").and_then(|f| f.as_array()) {
                filecount = files.len();
                for file in files {
                    if let Some(changes) = file.get("changes"){
                        change += changes.as_u64().unwrap();
                    }
                    if let Some(changes) = file.get("additions"){
                        additions += changes.as_u64().unwrap();
                    }
                    if let Some(changes) = file.get("deletions"){
                        deletions += changes.as_u64().unwrap();
                    }
                }
            }
        }
    }
    info!("Number of lines changed {} | Commit {} | Additions: {} | Deletions: {} | Files changed: {}",change,sha,additions,deletions,filecount);
}