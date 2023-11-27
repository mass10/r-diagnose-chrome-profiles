mod chromeprofiles;
mod util;

/// Rust アプリケーションのエントリーポイント
fn diagnose_chrome_profiles(filter: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	use crate::util::MatchesFilter;
	use crate::util::SafeValue;

	// Chrome のプロファイルを列挙します。
	let profiles = chromeprofiles::configure()?;

	// プロファイルごとにダンプします。
	for (key, profile) in &profiles {
		let profile_name = &profile.profile.name;

		// フィルタが指定されていた場合は、そのフィルタに合致しないプロファイルはスキップします。
		if filter.len() == 0 {
			// NOP
		} else if profile_name.matches_one_of(filter) {
			// NOP
		} else if key.matches_one_of(filter) {
			// NOP
		} else {
			continue;
		}

		println!("😐プロファイル: [{}, {}]", key, profile_name);

		let extensions = &profile.extensions.settings;
		for (key, extension) in extensions {
			let manifest = &extension.manifest;
			if manifest.is_none() {
				continue;
			}
			let manifest = manifest.as_ref().unwrap();

			let description = serde_json::to_string(&manifest.description.safe_value())?;
			let name = serde_json::to_string(&manifest.name.safe_value())?;
			let version = serde_json::to_string(&manifest.version.safe_value())?;
			let version_name = serde_json::to_string(&manifest.version_name.safe_value())?;

			println!("    ▶Extension: [{}]", key);
			println!("        active_bit: {}", extension.active_bit.safe_value());
			println!("        first_install_time: {}", extension.first_install_time.safe_value());
			println!("        from_webstore: {}", extension.from_webstore.safe_value());
			println!("        last_update_time: {}", extension.last_update_time.safe_value());
			println!("        manifest:");
			println!("            description: {}", description);
			println!("            manifest_version: {}", manifest.manifest_version.safe_value());
			println!("            name: {}", name);
			println!("            version: {}", version);
			println!("            version_name: {}", version_name);
			println!("        state: {}", extension.state.safe_value());
			println!("        was_installed_by_default: {}", extension.was_installed_by_default.safe_value());
			println!("        was_installed_by_oem: {}", extension.was_installed_by_oem.safe_value());
			println!("        withholding_permissions: {}", extension.withholding_permissions.safe_value());
			println!();
		}
	}

	return Ok(());
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();

	let result = diagnose_chrome_profiles(&args);
	if result.is_err() {
		let error = result.err().unwrap();
		eprintln!("{}", error);
		return;
	}
}
