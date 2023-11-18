trait SafeValue<T> {
	fn safe_value(&self) -> T;
}

impl SafeValue<String> for serde_json::Value {
	fn safe_value(&self) -> String {
		match self.as_str() {
			Some(s) => s.to_string(),
			None => "".to_string(),
		}
	}
}

impl SafeValue<i32> for serde_json::Value {
	fn safe_value(&self) -> i32 {
		match self.as_i64() {
			Some(s) => s as i32,
			None => 0,
		}
	}
}

impl SafeValue<bool> for serde_json::Value {
	fn safe_value(&self) -> bool {
		match self.as_bool() {
			Some(s) => s,
			None => false,
		}
	}
}

impl SafeValue<String> for String {
	fn safe_value(&self) -> String {
		self.clone()
	}
}

/// Option からの安全な unwrap() を提供します。
impl SafeValue<String> for Option<String> {
	fn safe_value(&self) -> String {
		match self {
			Some(s) => s.clone(),
			None => "".to_string(),
		}
	}
}

impl SafeValue<i32> for Option<i32> {
	fn safe_value(&self) -> i32 {
		match self {
			Some(s) => s.clone(),
			None => 0,
		}
	}
}

impl SafeValue<bool> for Option<bool> {
	fn safe_value(&self) -> bool {
		match self {
			Some(s) => s.clone(),
			None => false,
		}
	}
}

#[derive(Debug, serde_derive::Deserialize)]
struct GoogleChromeExtensionManifestData {
	description: Option<String>,
	manifest_version: Option<i32>,
	name: String,
	version: String,
	version_name: Option<String>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct ServiceWorkerRegistrationInfoData {
	#[allow(unused)]
	version: String,
}

#[derive(Debug, serde_derive::Deserialize)]
struct GoogleChromeExtensionData {
	active_bit: Option<bool>,
	first_install_time: Option<String>,
	from_webstore: Option<bool>,
	last_update_time: Option<String>,
	manifest: Option<GoogleChromeExtensionManifestData>,
	state: Option<i32>,
	was_installed_by_default: Option<bool>,
	was_installed_by_oem: Option<bool>,
	withholding_permissions: Option<bool>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct ChomeUserExtension {
	#[allow(unused)]
	manifest: serde_json::Value,
}

#[derive(Debug, serde_derive::Deserialize)]
struct Extensions {
	settings: std::collections::BTreeMap<String, GoogleChromeExtensionData>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct ChomeUserProfile {
	name: String,
}

#[derive(Debug, serde_derive::Deserialize)]
struct ChomeUserPreferences {
	profile: ChomeUserProfile,
	extensions: Extensions,
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

/// プロファイル名を列挙する
fn parse_local_state_file(text: &str) -> Result<LocalState, serde_json::Error> {
	let local_state: LocalState = serde_json::from_str(text)?;
	return Ok(local_state);
}

fn read_chrome_user_profile(name: &str) -> Result<ChomeUserPreferences, Box<dyn std::error::Error>> {
	let preferences_path = get_profile_dir_path(&name);
	let text = read_text_file(&preferences_path)?;
	let v: ChomeUserPreferences = serde_json::from_str(&text)?;
	return Ok(v);
}

fn get_profile_dir_path(name: &str) -> String {
	let path = std::env::var("LOCALAPPDATA").unwrap_or_default();

	let preferences_path = std::path::Path::new(&path).join("Google").join("Chrome").join("User Data").join(name).join("Preferences");
	let preferences_path = preferences_path.to_str().unwrap();

	return preferences_path.to_string();
}

/// Local State ファイルのパスを検出します。
fn detect_local_state() -> String {
	let root = std::env::var("LOCALAPPDATA").unwrap_or_default();
	let path = std::path::Path::new(&root).join("Google").join("Chrome").join("User Data").join("Local State");
	let path = path.to_str().unwrap();
	return path.to_string();
}

/// Google Chrome のプロファイル名を列挙します。
fn enum_chrome_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>> {
	// Local State ファイルを読み込み
	let path = detect_local_state();
	let text = read_text_file(&path)?;

	// Local State のパース
	let local_state = parse_local_state_file(&text)?;
	
	// プロファイル名を列挙
	let profiles = local_state.profile.info_cache;
	let result: Vec<String> = profiles.keys().map(|s| s.to_string()).collect();
	
	return Ok(result);
}

/// Chrome プロファイルを列挙します。
fn enum_profiles() -> Result<std::collections::BTreeMap<String, ChomeUserPreferences>, Box<dyn std::error::Error>> {
	// プロファイルのキーを列挙
	let profiles = enum_chrome_profiles()?;

	// プロファイルを収集
	let mut result = std::collections::BTreeMap::new();
	for name in profiles {
		let profile = read_chrome_user_profile(&name)?;
		result.insert(name, profile);
	}

	return Ok(result);
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	// Chrome のプロファイルを列挙します。
	let profiles = enum_profiles();
	if profiles.is_err() {
		let error = profiles.err().unwrap();
		eprintln!("{}", error);
		return;
	}

	// プロファイルごとにダンプします。
	let profiles = profiles.unwrap();
	for (key, profile) in &profiles {
		let profile_name = &profile.profile.name;

		println!("😐プロファイル: [{}, {}]", key, profile_name);

		let extensions = &profile.extensions.settings;
		for (key, extension) in extensions {
			let manifest = &extension.manifest;
			if manifest.is_none() {
				continue;
			}
			let manifest = manifest.as_ref().unwrap();
	
			println!("    ▶Extension: [{}]", key);
			println!("        active_bit: {}", extension.active_bit.safe_value());
			println!("        first_install_time: {}", extension.first_install_time.safe_value());
			println!("        from_webstore: {}", extension.from_webstore.safe_value());
			println!("        last_update_time: {}", extension.last_update_time.safe_value());
			println!("        manifest:");
			println!("            description: {}", manifest.description.safe_value());
			println!("            manifest_version: {}", manifest.manifest_version.safe_value());
			println!("            name: {}", manifest.name.safe_value());
			println!("            version: {}", manifest.version.safe_value());
			println!("            version_name: {}", manifest.version_name.safe_value());
			println!("        state: {}", extension.state.safe_value());
			println!("        was_installed_by_default: {}", extension.was_installed_by_default.safe_value());
			println!("        was_installed_by_oem: {}", extension.was_installed_by_oem.safe_value());
			println!("        withholding_permissions: {}", extension.withholding_permissions.safe_value());
			println!();
		}
	}
}
