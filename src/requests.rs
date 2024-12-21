#[allow(dead_code)]
pub async fn line_changes(sha: &str,repo: &str) {
    println!("{} {}",sha,repo);
    let resp = reqwest::get(format!("https://api.github.com/repos/{}/commits/{}",repo,sha)).await;
    match resp {
        Ok(value)  => println!("{:#?}", value),
        Err(value) => println!("{:#?}", value),
    }
    
}