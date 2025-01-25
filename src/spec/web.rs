use secrecy::ExposeSecret;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct TokenRequest<'a> {
    client_id: u64,
    client_secret: &'a str,
    grant_type: GrantType,
    scope: Scope,
}

impl<'a> TokenRequest<'a> {
    pub fn new(client_id: u64, client_secret: &'a SecretString) -> Self {
        Self {
            client_id,
            client_secret: client_secret.expose_secret(),
            grant_type: GrantType::ClientCredentials,
            scope: Scope::Public,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GrantType {
    ClientCredentials,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Scope {
    Public,
}

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct GetBeatmapsResponse {
    pub beatmaps: Vec<Beatmap>,
}

#[derive(Deserialize)]
pub struct Beatmap {
    // pub beatmapset_id: i32,
    // pub difficulty_rating: f64,
    pub id: u64,
    // pub mode: String,
    // pub status: String,
    // pub total_length: i32,
    // pub user_id: i32,
    pub version: String,
    // pub accuracy: i32,
    // pub ar: i32,
    // pub bpm: f64,
    // pub convert: bool,
    // pub count_circles: i32,
    // pub count_sliders: i32,
    // pub count_spinners: i32,
    // pub cs: i32,
    // pub deleted_at: Option<String>,
    // pub drain: i32,
    // pub hit_length: i32,
    // pub is_scoreable: bool,
    // pub last_updated: String,
    // pub mode_int: i32,
    // pub passcount: i32,
    // pub playcount: i32,
    // pub ranked: i32,
    // pub url: String,
    // pub checksum: String,
    pub beatmapset: Beatmapset,
    // pub failtimes: FailTimes,
    // pub max_combo: i32,
}

#[derive(Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    // pub artist_unicode: String,
    // pub covers: Covers,
    // pub creator: String,
    // pub favourite_count: i32,
    // pub hype: Option<i32>,
    // pub id: i32,
    // pub nsfw: bool,
    // pub offset: i32,
    // pub play_count: i32,
    // pub preview_url: String,
    // pub source: String,
    // pub spotlight: bool,
    // pub status: String,
    pub title: String,
    // pub title_unicode: String,
    // pub track_id: Option<i32>,
    // pub user_id: i32,
    // pub video: bool,
    // pub bpm: f64,
    // pub can_be_hyped: bool,
    // pub deleted_at: Option<String>,
    // pub discussion_enabled: bool,
    // pub discussion_locked: bool,
    // pub is_scoreable: bool,
    // pub last_updated: String,
    // pub legacy_thread_url: String,
    // pub nominations_summary: NominationsSummary,
    // pub ranked: i32,
    // pub ranked_date: Option<String>,
    // pub storyboard: bool,
    // pub submitted_date: String,
    // pub tags: String,
    // pub availability: Availability,
    // pub ratings: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub struct Covers {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    pub slimcover: String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover_2x: String,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct NominationsSummary {
    pub current: i32,
    pub eligible_main_rulesets: Vec<String>,
    pub required_meta: RequiredMeta,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct RequiredMeta {
    pub main_ruleset: i32,
    pub non_main_ruleset: i32,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct Availability {
    pub download_disabled: bool,
    pub more_information: Option<String>,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct FailTimes {
    pub fail: Vec<i32>,
    pub exit: Vec<i32>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_beatmaps_response_found() {
        let response_str = r#"
{"beatmaps":[{"beatmapset_id":54671,"difficulty_rating":6.35,"id":166123,"mode":"osu","status":"graveyard","total_length":67,"user_id":1669560,"version":"Insane","accuracy":8,"ar":9,"bpm":226.5,"convert":false,"count_circles":614,"count_sliders":7,"count_spinners":0,"cs":3,"deleted_at":null,"drain":7,"hit_length":65,"is_scoreable":false,"last_updated":"2014-03-10T16:31:10Z","mode_int":0,"passcount":46,"playcount":1306,"ranked":-2,"url":"https:\/\/osu.ppy.sh\/beatmaps\/166123","checksum":"5c14d5259276198e007135f4fee0fb7b","beatmapset":{"artist":"DJSharpnel","artist_unicode":"DJSharpnel","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/cover.jpg?1458225342","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/cover@2x.jpg?1458225342","card":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/card.jpg?1458225342","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/card@2x.jpg?1458225342","list":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/list.jpg?1458225342","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/list@2x.jpg?1458225342","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/slimcover.jpg?1458225342","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/54671\/covers\/slimcover@2x.jpg?1458225342"},"creator":"nrii","favourite_count":5,"hype":null,"id":54671,"nsfw":false,"offset":0,"play_count":1306,"preview_url":"\/\/b.ppy.sh\/preview\/54671.mp3","source":"","spotlight":false,"status":"graveyard","title":"Kanpai2000","title_unicode":"Kanpai2000","track_id":null,"user_id":1669560,"video":false,"bpm":226.5,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":false,"last_updated":"2012-07-11T13:35:10Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/90483","nominations_summary":{"current":0,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":-2,"ranked_date":null,"storyboard":false,"submitted_date":"2012-07-11T12:25:06Z","tags":"","availability":{"download_disabled":false,"more_information":null},"ratings":[0,0,0,0,0,0,0,0,0,0,0]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9,2,0,9,1,77,112,149,140,87,63,28,19,20,18,10,11,0,20,9,18,0,0,0,2,0,0,0,1,27,0,0,0,9,1,0,0,1,0,1,18,0,0,0,0,0,0,0,9,0,0,9,0,0,0,0,0,0,0,0,9,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0],"exit":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,27,0,0,0,9,0,18,0,1,19,19,12,20,29,1,19,0,10,10,3,27,2,0,18,0,10,0,0,1,10,0,0,0,0,0,10,9,18,9,0,0,0,1,9,0,0,0,9,0,9,1,9,1,0,0,0,0,0,0,0,0,1,1,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0]},"max_combo":654},{"beatmapset_id":292599,"difficulty_rating":7.35,"id":658488,"mode":"osu","status":"ranked","total_length":86,"user_id":610988,"version":"Die","accuracy":9.8,"ar":9.5,"bpm":250,"convert":false,"count_circles":375,"count_sliders":168,"count_spinners":6,"cs":4,"deleted_at":null,"drain":5.8,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:26Z","mode_int":0,"passcount":815,"playcount":10459,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/658488","checksum":"b789dcb67ff457eff31f03d0742a93c3","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,12,115,183,1225,329,143,249,540,84,524,547,137,175,65,88,62,412,157,503,118,146,339,544,530,77,401,304,130,290,483,513,19,24,67,30,55,97,117,149,105,360,146,53,57,39,174,227,170,81,21,0,0,0,0,0,0,0,0,0,0,0,0,0,1,38,9,0,0,0,0,0,0,9,0,0,0,0,0,1,0,9,32,98,63,27,10,64,21,131,136,19,10,108],"exit":[0,0,0,0,0,0,0,0,90,258,1181,879,153,147,186,311,54,426,123,33,101,39,28,19,227,100,213,31,14,47,199,109,0,25,9,84,86,200,118,65,11,10,9,1,10,27,10,58,37,0,31,0,31,53,39,149,54,20,27,1,0,1,0,12,19,27,0,0,2,9,19,30,0,10,9,0,0,1,0,0,0,1,0,0,0,0,9,18,10,27,46,9,9,9,3,11,36,9,1,298]},"max_combo":791},{"beatmapset_id":292599,"difficulty_rating":1.74,"id":658569,"mode":"osu","status":"ranked","total_length":86,"user_id":610988,"version":"Easy","accuracy":2,"ar":3,"bpm":250,"convert":false,"count_circles":29,"count_sliders":51,"count_spinners":5,"cs":3,"deleted_at":null,"drain":2,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:26Z","mode_int":0,"passcount":1308,"playcount":2818,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/658569","checksum":"f630a9a0d7fd797c278fb58d3d1f87dd","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,0,0,0,0,0,20,9,10,9,18,0,0,27,0,0,0,0,0,0,18,0,0,9,9,0,0,0,0,0,0,0,0,0,0,0,27,0,0,9,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,9,0,0,0,0,0,0,0,0,0,18,0,0,0,9,0,0,0,0,9,18,1,0,0,0,0,0,0,0,0,0],"exit":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,19,72,36,46,45,46,9,2,10,19,18,18,29,9,9,27,9,18,18,63,18,0,9,9,18,9,18,9,0,0,0,10,0,0,1,0,0,9,0,0,0,0,9,9,18,18,9,0,0,19,0,0,0,0,9,0,0,0,0,18,0,0,1,0,0,0,9,9,0,0,10,0,0,9,0,0,9,0,0,0,9,9,0,0,0,81]},"max_combo":215},{"beatmapset_id":292599,"difficulty_rating":2.56,"id":658725,"mode":"osu","status":"ranked","total_length":86,"user_id":610988,"version":"Normal","accuracy":4.5,"ar":6.5,"bpm":250,"convert":false,"count_circles":57,"count_sliders":129,"count_spinners":5,"cs":3,"deleted_at":null,"drain":3,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:27Z","mode_int":0,"passcount":1309,"playcount":3925,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/658725","checksum":"35f2c98aedde729d98b5f86c703e15d4","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,0,9,18,72,0,92,27,0,18,11,54,1,1,1,10,19,9,27,10,18,22,0,9,2,9,0,0,18,0,0,9,0,11,0,9,20,10,9,9,0,0,19,9,0,0,0,0,0,9,0,10,9,0,9,0,0,9,0,10,10,18,18,0,0,1,0,1,9,9,11,46,20,20,74,21,9,9,5,48,12,0,1,9,9,0,0,0,1,0,9,10,9],"exit":[0,0,0,0,0,0,0,0,0,0,76,39,9,36,46,64,11,11,54,45,18,18,36,9,37,9,18,0,27,19,29,18,56,81,18,0,0,27,28,55,46,63,29,30,0,0,10,20,10,18,2,1,0,9,9,18,9,27,1,18,1,9,12,0,0,18,1,0,9,9,0,0,0,18,45,10,10,9,0,29,20,18,9,28,0,1,19,27,1,9,0,0,9,0,0,0,0,0,10,27]},"max_combo":381},{"beatmapset_id":292599,"difficulty_rating":3.38,"id":658877,"mode":"osu","status":"ranked","total_length":86,"user_id":610988,"version":"Hard","accuracy":6,"ar":7.5,"bpm":250,"convert":false,"count_circles":106,"count_sliders":123,"count_spinners":3,"cs":4,"deleted_at":null,"drain":4,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:27Z","mode_int":0,"passcount":1315,"playcount":4186,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/658877","checksum":"67993ea2a718e9ae3127c853a88ef056","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,0,0,0,0,10,40,74,60,38,59,2,9,10,18,9,0,0,1,0,30,36,141,41,1,27,39,44,75,39,47,0,9,9,9,2,2,10,24,29,1,1,27,9,0,0,0,0,0,1,0,1,0,0,10,18,10,2,0,0,0,0,0,0,0,10,9,9,0,0,10,0,0,9,9,0,0,9,2,0,0,0,0,0,0,0,9,0,0,9,9,0,0],"exit":[0,0,0,0,0,0,0,0,36,29,21,63,125,73,120,118,57,49,55,33,36,15,39,9,2,29,6,2,99,28,122,53,2,21,58,64,47,22,21,29,9,0,10,30,30,51,64,36,20,29,2,55,37,20,1,9,0,10,0,9,9,1,10,10,27,28,9,0,1,0,0,1,18,29,9,3,0,0,9,10,0,18,9,1,1,0,0,18,0,9,9,9,0,0,0,0,10,0,0,5]},"max_combo":523},{"beatmapset_id":292599,"difficulty_rating":5.41,"id":659404,"mode":"osu","status":"ranked","total_length":86,"user_id":610988,"version":"Another","accuracy":8.5,"ar":9,"bpm":250,"convert":false,"count_circles":170,"count_sliders":209,"count_spinners":4,"cs":4,"deleted_at":null,"drain":6,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:28Z","mode_int":0,"passcount":884,"playcount":8983,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/659404","checksum":"6eaa329ec6c32ebc3907b3f5b52f7b3a","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,367,186,1178,280,1769,223,260,503,280,542,77,167,28,79,10,51,54,361,83,11,12,21,36,20,67,1,0,9,9,0,9,20,21,84,9,0,0,0,1,1,0,20,0,1,1,1,0,0,1,9,1,47,39,83,10,14,27,0,0,10,1,19,32,119,129,9,0,2,3,19,0,0,18,0,18,9,18,0,0,27,0,0,28,28,10,10,0,1,0,9,10,9],"exit":[0,0,0,0,18,0,0,0,133,221,624,911,280,218,48,102,194,127,64,85,32,57,31,1,30,12,156,27,20,9,11,27,20,19,9,18,2,84,18,13,30,2,29,18,19,2,3,10,19,19,10,0,22,9,1,28,11,1,2,20,31,54,28,10,48,19,22,0,11,0,10,68,48,45,2,0,11,9,19,0,9,0,0,9,0,0,9,0,11,1,2,1,10,0,9,0,0,0,11,96]},"max_combo":734},{"beatmapset_id":292599,"difficulty_rating":7.22,"id":664513,"mode":"osu","status":"ranked","total_length":86,"user_id":19048,"version":"Mismagius'Extreme","accuracy":9,"ar":9,"bpm":250,"convert":false,"count_circles":322,"count_sliders":180,"count_spinners":3,"cs":3.5,"deleted_at":null,"drain":6,"hit_length":84,"is_scoreable":true,"last_updated":"2024-06-04T13:50:28Z","mode_int":0,"passcount":356,"playcount":3817,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/664513","checksum":"04ec6f25951dbc641358d024ae7d649f","beatmapset":{"artist":"5StepSoundTeam","artist_unicode":"5StepSoundTeam","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover.jpg?1717509043","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/cover@2x.jpg?1717509043","card":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card.jpg?1717509043","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/card@2x.jpg?1717509043","list":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list.jpg?1717509043","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/list@2x.jpg?1717509043","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover.jpg?1717509043","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/292599\/covers\/slimcover@2x.jpg?1717509043"},"creator":"PoMuTa","favourite_count":60,"hype":null,"id":292599,"nsfw":false,"offset":0,"play_count":46574,"preview_url":"\/\/b.ppy.sh\/preview\/292599.mp3","source":"5StepAdventure","spotlight":false,"status":"ranked","title":"NeverSayDie","title_unicode":"NeverSayDie","track_id":null,"user_id":610988,"video":false,"bpm":250,"can_be_hyped":false,"deleted_at":null,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2024-06-04T13:50:25Z","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/309501","nominations_summary":{"current":2,"eligible_main_rulesets":["osu"],"required_meta":{"main_ruleset":2,"non_main_ruleset":1}},"ranked":1,"ranked_date":"2024-06-13T07:06:00Z","storyboard":false,"submitted_date":"2015-03-10T18:44:00Z","tags":"lapfoxtraxrenardqueenstondaveremmlervulpvibeoscpomutamismagiusbluedragoninstrumentalelectronicemmaessexvideogame","availability":{"download_disabled":false,"more_information":null},"ratings":[0,4,0,0,0,1,0,3,4,8,55]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,38,173,97,46,11,673,318,46,15,60,45,405,70,66,24,78,79,19,0,11,0,12,181,70,128,19,41,9,153,375,14,41,57,109,36,0,0,23,11,6,25,170,246,136,60,120,72,55,45,19,79,276,37,33,0,97,0,1,0,36,35,10,59,260,215,18,0,0,48,11,0,0,0,0,0,0,0,19,0,0,10,13,36,0,9,19,54,0,0,50,99,0],"exit":[0,0,0,0,0,0,0,0,41,54,217,67,68,275,460,126,21,56,1,21,117,38,29,24,30,19,1,19,1,10,108,102,24,0,13,20,20,107,55,20,12,11,9,19,0,0,0,10,15,76,51,8,84,46,45,41,0,10,27,30,29,18,1,28,37,9,0,0,19,0,0,30,27,11,9,0,1,0,9,0,0,9,9,9,0,0,0,9,9,18,0,0,9,18,1,0,0,10,30,64]},"max_combo":777}]}
        "#;

        let _: GetBeatmapsResponse = serde_json::from_str(&response_str).unwrap();
    }
}
