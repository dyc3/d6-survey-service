use chrono::{DateTime, Utc};

use crate::db::models::{Survey, SurveyResponse};

pub trait Cacheable {
	fn modified_time(&self) -> Option<DateTime<Utc>> { None }
	fn etag(&self) -> Option<&String> { None }

	fn is_modified_since(&self, since: impl Into<DateTime<Utc>>) -> bool {
		match self.modified_time() {
			Some(modified_time) => modified_time > since.into(),
			None => false,
		}
	}

	fn is_etag_match<'a>(&'a self, etag: impl Into<&'a str>) -> bool {
		match self.etag() {
			Some(obj_etag) => obj_etag == etag.into(),
			None => false,
		}
	}

	fn last_modified_header(&self) -> Option<String> {
		match self.modified_time() {
			Some(modified_time) => Some(modified_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()),
			None => None,
		}
	}

	fn etag_header(&self) -> Option<String> {
		match self.etag() {
			Some(etag) => Some(etag.to_owned()),
			None => None,
		}
	}
}

impl Cacheable for Survey {
	fn modified_time(&self) -> Option<DateTime<Utc>> {
		// by pure luck, the updated_at field is actually in UTC
		Some(DateTime::from_utc(self.updated_at, Utc))
	}
}

impl Cacheable for SurveyResponse {
	fn modified_time(&self) -> Option<DateTime<Utc>> {
		Some(self.updated_at)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::prelude::*;

	struct TestObj {
		etag: String,
		modified_time: DateTime<Utc>,
	}

	impl Cacheable for TestObj {
		fn etag(&self) -> Option<&String> {
			Some(&self.etag)
		}

		fn modified_time(&self) -> Option<DateTime<Utc>> {
			Some(self.modified_time)
		}
	}

	#[test]
	fn test_modified_time() {
		let obj = TestObj {
			etag: "test".to_string(),
			modified_time: Utc::now(),
		};
		assert!(obj.is_modified_since(Utc::now() - chrono::Duration::seconds(1)));
		assert!(!obj.is_modified_since(Utc::now() + chrono::Duration::seconds(1)));
	}

	#[test]
	fn test_etag_match() {
		let obj = TestObj {
			etag: "test".to_string(),
			modified_time: Utc::now(),
		};
		assert!(obj.is_etag_match("test"));
		assert!(!obj.is_etag_match("test2"));
	}

	#[test]
	fn test_build_headers() {
		let obj = TestObj {
			etag: "test".to_string(),
			modified_time: Utc.with_ymd_and_hms(2015, 10, 21, 7, 28, 0).unwrap(),
		};
		assert_eq!(obj.etag_header().unwrap(), "test");
		assert_eq!(
			obj.last_modified_header().unwrap(),
			"Wed, 21 Oct 2015 07:28:00 GMT"
		);
	}
}