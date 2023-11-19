#[derive(Debug, serde_derive::Deserialize)]
pub struct GoogleChromeExtensionManifestData {
	#[allow(unused)]
	pub description: Option<String>,
	#[allow(unused)]
	pub manifest_version: Option<i32>,
	#[allow(unused)]
	pub name: String,
	#[allow(unused)]
	pub version: String,
	#[allow(unused)]
	pub version_name: Option<String>,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct ServiceWorkerRegistrationInfoData {
	#[allow(unused)]
	pub version: String,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct GoogleChromeExtensionData {
	#[allow(unused)]
	pub active_bit: Option<bool>,
	#[allow(unused)]
	pub first_install_time: Option<String>,
	#[allow(unused)]
	pub from_webstore: Option<bool>,
	#[allow(unused)]
	pub last_update_time: Option<String>,
	#[allow(unused)]
	pub manifest: Option<GoogleChromeExtensionManifestData>,
	#[allow(unused)]
	pub state: Option<i32>,
	#[allow(unused)]
	pub was_installed_by_default: Option<bool>,
	#[allow(unused)]
	pub was_installed_by_oem: Option<bool>,
	#[allow(unused)]
	pub withholding_permissions: Option<bool>,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct ChomeUserExtension {
	#[allow(unused)]
	pub manifest: serde_json::Value,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct Extensions {
	#[allow(unused)]
	pub settings: std::collections::BTreeMap<String, GoogleChromeExtensionData>,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct ChomeUserProfile {
	#[allow(unused)]
	pub name: String,
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct ChomeUserPreferences {
	#[allow(unused)]
	pub profile: ChomeUserProfile,
	#[allow(unused)]
	pub extensions: Extensions,
}

fn read_text_file(path: &str) -> Result<String, std::io::Error> {
	use std::io::Read;
	let mut file = std::fs::File::open(path)?;
	let mut text = String::new();
	file.read_to_string(&mut text)?;
	Ok(text)
}

#[derive(Debug, serde_derive::Deserialize)]
struct ProfileEntry {
	#[allow(unused)]
	name: String,
	#[allow(unused)]
	shortcut_name: String,
	#[allow(unused)]
	user_name: String,
}

#[derive(Debug, serde_derive::Deserialize)]
struct Profile {
	info_cache: std::collections::BTreeMap<String, ProfileEntry>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct LocalState {
	profile: Profile,
}

/// Chrome プロファイルのルートディレクトリを検出します。
fn detect_chrome_profiles_home_dir() -> Result<String, Box<dyn std::error::Error>> {
	let path = std::env::var("LOCALAPPDATA").unwrap_or_default();
	let path = std::path::Path::new(&path).join("Google").join("Chrome").join("User Data");
	let path = path.to_str().unwrap();

	return Ok(path.to_string());
}

/// Chrome プロファイルのルートディレクトリを取得します。
fn detect_chrome_profile_file_path(name: &str) -> Result<String, Box<dyn std::error::Error>> {
	let root = detect_chrome_profiles_home_dir()?;
	let path = std::path::Path::new(&root).join(name).join("Preferences");
	let path = path.to_str().unwrap();

	return Ok(path.to_string());
}

/// Chrome プロファイルを読み込みます。
fn read_chrome_profile_file_of(name: &str) -> Result<ChomeUserPreferences, Box<dyn std::error::Error>> {
	// Preferences ファイルを読み込み
	let path = detect_chrome_profile_file_path(&name)?;

	let text = read_text_file(&path)?;

	// Preferences のパース
	let preferences: ChomeUserPreferences = serde_json::from_str(&text)?;

	return Ok(preferences);
}

/// Local State ファイルのパスを検出します。
fn detect_local_state_file_path() -> Result<String, Box<dyn std::error::Error>> {
	let root = detect_chrome_profiles_home_dir()?;
	let path = std::path::Path::new(&root).join("Local State");
	let path = path.to_str().unwrap();

	return Ok(path.to_string());
}

/// Google Chrome のプロファイル名を列挙します。
fn enum_profile_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
	// Local State ファイルを読み込み
	let path = detect_local_state_file_path()?;

	let text = read_text_file(&path)?;

	// Local State のパース
	let local_state: LocalState = serde_json::from_str(&text)?;

	// プロファイル名を列挙
	let profiles = local_state.profile.info_cache;
	let result: Vec<String> = profiles.keys().map(|s| s.to_string()).collect();

	return Ok(result);
}

/// Chrome プロファイルを列挙します。
pub fn configure() -> Result<std::collections::BTreeMap<String, ChomeUserPreferences>, Box<dyn std::error::Error>> {
	// プロファイルのキーを列挙
	let profiles = enum_profile_names()?;

	// プロファイルを収集
	let mut result = std::collections::BTreeMap::new();
	for name in profiles {
		let profile = read_chrome_profile_file_of(&name)?;
		result.insert(name, profile);
	}

	return Ok(result);
}
