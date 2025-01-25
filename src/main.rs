use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, Metadata},
    path::{Path, PathBuf},
    str::FromStr,
    thread::sleep,
};

use clap::Parser;
use itertools::Itertools;
use log;
use reqwest;
use secrecy::SecretString;

mod spec;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();
    let args = spec::args::Args::parse();

    let out_file = std::fs::File::create(args.output_zip_path).unwrap();
    let mut tarrer = tar::Builder::new(out_file);

    log::info!("Fetching access token");
    let mut client = reqwest::Client::new();
    let token = get_token(
        &mut client,
        reqwest::Url::from_str("https://osu.ppy.sh/oauth/token").expect("legit url"),
        args.client_id,
        &args.secret,
    )
    .await;

    if args.rate_limit_per_minute > 60 {
        log::warn!("Your current rate limit is above the maximum recommended in the docs");
        log::warn!("From the docs:");
        log::warn!(
            r#"Current rate limit is set at an insanely high 1200 requests per minute, with burst capability of up to 200 beyond that. If you require more, you probably fall into the above category of abuse. If you are doing more than 60 requests a minute, you should probably give peppy a yell."#
        );
        log::warn!("The program will continue in 1 minute.");
        sleep(std::time::Duration::from_secs(60));
    }

    log::info!("Obtaining possible osu! song folders.");
    let song_dirs = paths_in_dir_meta_filter(&args.songs_folder, |meta| meta.is_dir());
    log::info!("Possible song dirs found: {}", song_dirs.len());

    log::info!("Fetching list of files in song folders.");
    let files_in_song_dirs: Vec<PathBuf> = song_dirs
        .into_iter()
        .map(|song_dir| paths_in_dir_meta_filter(&song_dir, |meta| meta.is_file()))
        .flatten()
        .collect();
    log::info!("Files found: {}", files_in_song_dirs.len());

    log::info!("Filtering to .osu files.");
    let dot_osu_file_paths: Vec<PathBuf> = files_in_song_dirs
        .into_iter()
        .filter(|p| {
            if let Some(ext) = p.extension() {
                ext == "osu"
            } else {
                false
            }
        })
        .collect();
    log::info!(".osu files found: {}", dot_osu_file_paths.len());

    let mut ids_to_containing_folders_to_check: HashMap<u64, PathBuf> = HashMap::new();
    let mut song_folders_with_unsubmitted = HashSet::new();

    for dot_osu_file_path in &dot_osu_file_paths {
        let contents = read_to_string(&dot_osu_file_path);
        if let Err(e) = &contents {
            log::error!("Failed reading file {dot_osu_file_path:?}: {e:?}");
            continue;
        }
        let contents = contents.expect("Checked");
        let id: Option<u64> = contents.lines().find_map(|l| {
            if l.starts_with("BeatmapID:") {
                if let Some((_, id)) = l.split_once(':') {
                    let id = id.parse::<u64>().ok();
                    if let Some(id) = id {
                        if id == 0 {
                            log::info!("Found unsubmitted: {dot_osu_file_path:#?}");
                            song_folders_with_unsubmitted
                                .insert(dot_osu_file_path.parent().expect("checked").to_owned());
                            None
                        } else {
                            Some(id)
                        }
                    } else {
                        id
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });
        if let Some(id) = id {
            ids_to_containing_folders_to_check
                .insert(id, dot_osu_file_path.parent().expect("checked").to_owned());
        }
    }

    log::info!(
        "Beginning API requests for {} .osu files.",
        ids_to_containing_folders_to_check.len()
    );

    let mut counter = 0;
    let mut last_req_time = std::time::Instant::now();
    let req_wait_time = std::time::Duration::from_millis(60000 / args.rate_limit_per_minute);
    for chunk in &ids_to_containing_folders_to_check.iter().chunks(50) {
        let chunk = chunk.collect_vec();
        let next_req_time = last_req_time + req_wait_time;
        let now = std::time::Instant::now();
        if next_req_time > now {
            sleep(next_req_time - now);
        }
        last_req_time = std::time::Instant::now();

        log::info!(
            "[{0}/{1}] Checking id {0} to {2}.",
            counter * 50 + 1,
            ids_to_containing_folders_to_check.len(),
            chunk.len() + counter * 50,
        );
        let ids: Vec<u64> = chunk.iter().map(|(id, _)| **id).collect();
        let map = check_song_ids_are_unsubmitted(
            &ids,
            &mut &mut client,
            args.lookup_beatmap_api_url.clone(),
            &token,
        )
        .await;
        for (id, path) in chunk.into_iter() {
            if *map.get(id).unwrap_or(&true) {
                log::info!("Unsubmitted found ({id}): {path:#?}");
                song_folders_with_unsubmitted.insert(path.to_owned());
            }
        }
        counter += 1;
    }

    log::info!("All song folders with unsubmitted .osu: {song_folders_with_unsubmitted:#?}");
    log::info!("Zipping/Tarring song folders");
    for song_folder in song_folders_with_unsubmitted {
        tarrer
            .append_dir_all(song_folder.file_name().unwrap(), &song_folder)
            .unwrap();
    }
    tarrer.finish().unwrap();
}

async fn check_song_ids_are_unsubmitted(
    ids: &[u64],
    client: &mut reqwest::Client,
    url: reqwest::Url,
    token: &str,
) -> HashMap<u64, bool> {
    let query: Vec<(String, String)> = ids
        .into_iter()
        .map(|id| ("ids[]".to_owned(), id.to_string()))
        .collect();

    let res = client
        .get(url)
        .query(&query)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let mut out: HashMap<u64, bool> = HashMap::new();
    if let Ok(res) = res {
        if let Ok(text) = res.text().await {
            if let Ok(beatmaps_response) =
                serde_json::from_str::<spec::web::GetBeatmapsResponse>(&text)
            {
                for beatmap in beatmaps_response.beatmaps {
                    log::info!(
                        "Submitted found: {} - {}[{}]",
                        beatmap.beatmapset.artist,
                        beatmap.beatmapset.title,
                        beatmap.version
                    );
                    out.insert(beatmap.id, false);
                }
            } else {
                log::error!("Unexpected response format, aborting: {text}");
            }
        } else {
            log::error!("Network error getting body");
        }
    } else {
        log::error!("Network error sending request");
    }
    out
}

async fn get_token(
    client: &mut reqwest::Client,
    auth_url: reqwest::Url,
    client_id: u64,
    secret: &SecretString,
) -> String {
    let body =
        serde_json::to_string(&spec::web::TokenRequest::new(client_id, &secret)).expect("encodes");
    let res = client
        .post(auth_url)
        .body(body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .expect("auth request should succeed");
    let text = res.text().await.expect("auth response should succeed");
    let res: spec::web::TokenResponse =
        serde_json::from_str(&text).expect("should get creds, check secret/clientID");
    res.access_token
}

fn paths_in_dir_meta_filter<F>(dir: &Path, meta_check: F) -> Vec<PathBuf>
where
    F: Fn(&Metadata) -> bool,
{
    let mut res_paths = Vec::new();
    if let Ok(paths) = std::fs::read_dir(dir) {
        for p in paths {
            // CBA with if let indent spam.
            if p.is_err() {
                continue;
            }
            let p = p.expect("checked");

            let meta = p.metadata();
            if meta.is_err() {
                continue;
            }
            let meta = meta.expect("checked");

            if meta_check(&meta) {
                res_paths.push(p.path());
            }
        }
        res_paths
    } else {
        panic!("Failed to read songs dir: {dir:?}");
    }
}

async fn check_song_id_is_unsubmitted(
    id: u64,
    client: &mut reqwest::Client,
    url: reqwest::Url,
    token: &str,
) -> bool {
    let res = client
        .get(url)
        .query(&[("id", id.to_string())])
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;
    if let Ok(res) = res {
        if let Ok(text) = res.text().await {
            if let Ok(beatmap_response) =
                serde_json::from_str::<spec::web::GetBeatmapResponse>(&text)
            {
                match beatmap_response {
                    spec::web::GetBeatmapResponse::Found(beatmap) => {
                        log::info!(
                            "Submitted found: {} - {}[{}]",
                            beatmap.beatmapset.artist,
                            beatmap.beatmapset.title,
                            beatmap.version
                        );
                        false
                    }
                    spec::web::GetBeatmapResponse::Error(e) => {
                        if e.error.is_none() {
                            log::info!("Unsubmitted found!");
                            true
                        } else {
                            log::error!("Unexpected error response, marking unsubmitted: {text}");
                            false
                        }
                    }
                }
            } else {
                log::error!("Unexpected response format, marking unsubmitted: {text}");
                true
            }
        } else {
            false
        }
    } else {
        false
    }
}

// for (i, (id, containing_folder)) in ids_to_containing_folders_to_check.iter().enumerate() {
//     sleep(std::time::Duration::from_millis(
//         60000 / args.rate_limit_per_minute,
//     ));
//     log::info!(
//         "[{}/{}] Checking id {id}.",
//         i + 1,
//         ids_to_containing_folders_to_check.len()
//     );
//     if check_song_id_is_unsubmitted(
//         *id,
//         &mut client,
//         args.lookup_beatmap_api_url.clone(),
//         &token,
//     )
//     .await
//     {
//         song_folders_with_unsubmitted.insert(containing_folder.clone());
//     }
// }
