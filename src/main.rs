mod chromeprofiles;
mod util;

/// Rust アプリケーションのエントリーポイント
fn main() {
	use crate::util::SafeValue;

	// Chrome のプロファイルを列挙します。
	let profiles = chromeprofiles::configure();
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
