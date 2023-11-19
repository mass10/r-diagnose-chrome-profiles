/// Option からの安全な unwrap() を提供します。
pub trait SafeValue<T> {
	fn safe_value(&self) -> T;
}

impl SafeValue<String> for String {
	fn safe_value(&self) -> String {
		self.clone()
	}
}

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
