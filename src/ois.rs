use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Highlight {
	pub id: String,
	#[serde(rename="name")]
	pub team: String,
	pub description: String
}

#[derive(Deserialize)]
pub struct EditionOverview {
	#[serde(rename="name")]
	pub str_id: String,
	pub title: String,
	#[serde(rename="year")]
	pub year_str: String, // "2022/23", "2016/17"
	#[serde(rename="teams")]
	pub total_teams: u32,
	#[serde(rename="regions")]
	pub total_regions: u32,
	#[serde(rename="points")]
	pub total_points: u32,
	#[serde(rename="fullscore")]
	pub score_ceiling: u32,
	#[serde(rename="positive")]
	pub _positive: u32,
	#[serde(rename="highest")]
	pub high_score: u32,
	#[serde(rename="average")]
	pub average_score: f64,
	#[serde(rename="medpos")]
	pub _medpos: u32,
	#[serde(rename="tasks")]
	pub total_tasks: u32,
	#[serde(rename="instnum")]
	pub _instnum: u32,
	#[serde(rename="lastEd")]
	pub last_edition: u32, // this is the LAST edition, not the previous one
	#[serde(rename="id")]
	pub num_id: u32
}

#[derive(Deserialize)]
pub struct CompetitionOverview {
	pub highlights: Vec<Highlight>,
	#[serde(rename="allreg")]
	pub _all_reg: u32,
	#[serde(rename="avgreg")]
	pub _avg_reg: f64,
	pub editions: Vec<EditionOverview>,
	#[serde(rename="regions")]
	pub _regions: u32,
	#[serde(rename="instnum")]
	pub _instnum: u32,
	#[serde(rename="teams")]
	pub _teams: u32,
	#[serde(rename="points")]
	pub _points: u32,
	#[serde(rename="tasks")]
	pub _tasks: u32
}

pub async fn get_competition_overview() -> Result<CompetitionOverview, String> {
	let url = "https://squadre.olinfo.it/json/edition.json";
	
	let resp = match reqwest::get(url).await {
		Ok(v) => v,
		Err(e) => return Err(e.to_string())
	};

	let raw = match resp.text().await {
		Ok(v) => v,
		Err(e) => return Err(e.to_string())
	};

	let dump = match serde_json::from_str(&raw) {
		Ok(v) => v,
		Err(e) => return Err(e.to_string())
	};

	return Ok(dump);
}
