pub trait SafeValue<T> {
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
