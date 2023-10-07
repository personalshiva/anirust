use std::collections::HashMap;

pub mod decrypt;

pub async fn handle_source(
    client: &reqwest::Client,
    link: &String,
) -> Option<HashMap<u32, String>> {
    let keys: [&str; 2] = ["vipanicdn", "anifastcdn"];
    let qualities: HashMap<u32, String> = if keys.iter().any(|&key| link.contains(key)) {
        match handle_vipanicdn_anifastcdn(client, link).await {
            Ok(Some(qualities)) => qualities,
            _ => return None, // skip this iteration if we encounter an error or None
        }
    } else if link.contains("repackager.wixmp.com") {
        match handle_wixmp(link) {
            Some(qualities) => qualities,
            _ => return None, // skip this iteration on error
        }
    } else {
        vec![(1, link.to_string())].into_iter().collect()
    };
    Some(qualities)
}

async fn handle_vipanicdn_anifastcdn(
    client: &reqwest::Client,
    link: &str,
) -> Result<Option<HashMap<u32, String>>, reqwest::Error> {
    if link.contains("original.m3u") {
        return Ok(None);
    }

    let relative_link = {
        let mut parts: Vec<&str> = link.split('/').collect();
        parts.pop(); // remove the last element
        parts.join("/")
    };

    let response = client.get(link).send().await?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let text = response.text().await?;
    let episode_qualities: HashMap<u32, String> = text
        .lines()
        .filter(|line| !line.trim().starts_with('#'))
        .filter_map(|episode| {
            let quality: Option<u32> = episode
                .split(".m3u8")
                .next()
                .and_then(|s| s.split('.').last())
                .and_then(|s| s.parse().ok());

            quality.map(|q| (q, format!("{}/{}", relative_link, episode)))
        })
        .collect();

    Ok(Some(episode_qualities))
}

fn handle_wixmp(link: &str) -> Option<HashMap<u32, String>> {
    let link = link
        .replace("repackager.wixmp.com/", "")
        .split(".urlset")
        .collect::<Vec<&str>>()[0]
        .to_string();
    let segments: Vec<&str> = link.split(',').collect();

    let mut qualities = HashMap::new();

    for segment in &segments[1..segments.len() - 1] {
        let url = format!("{}{}{}", segments[0], segment, segments[segments.len() - 1]);
        let quality = segment.replace('p', "");

        match quality.parse::<u32>() {
            Ok(quality) => {
                qualities.insert(quality, url);
            }
            Err(_) => {
                eprintln!("Failed to parse quality: {}", quality);
                return None;
            }
        }
    }

    Some(qualities)
}
