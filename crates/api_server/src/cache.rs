use chrono::{DateTime, Utc};
use rocket::http::Header;

use crate::db::models::{Survey, SurveyResponse};

pub trait ModifiedTime {
	fn modified_time(&self) -> DateTime<Utc>;

	fn is_modified_since(&self, since: impl Into<DateTime<Utc>>) -> bool {
		self.modified_time() > since.into()
	}

	fn last_modified_header(&self) -> Header<'_> {
		Header::new("Last-Modified", self.modified_time().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
	}
}

impl ModifiedTime for Survey {
	fn modified_time(&self) -> DateTime<Utc> {
		// by pure luck, the updated_at field is actually in UTC
		DateTime::from_utc(self.updated_at, Utc)
	}
}

impl ModifiedTime for SurveyResponse {
	fn modified_time(&self) -> DateTime<Utc> {
		self.updated_at
	}
}

pub trait ETagged {
	fn etag(&self) -> &String;

	fn is_etag_match<'a>(&'a self, etag: impl Into<&'a str>) -> bool {
		self.etag() == etag.into()
	}

	fn etag_header(&self) -> Header<'_> {
		Header::new("ETag", self.etag())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::prelude::*;

	struct TestModifiedTime {
		modified_time: DateTime<Utc>,
	}

	impl ModifiedTime for TestModifiedTime {
		fn modified_time(&self) -> DateTime<Utc> {
			self.modified_time
		}
	}

	#[test]
	fn test_modified_time() {
		let obj = TestModifiedTime {
			modified_time: Utc::now(),
		};
		assert!(obj.is_modified_since(Utc::now() - chrono::Duration::seconds(1)));
		assert!(!obj.is_modified_since(Utc::now() + chrono::Duration::seconds(1)));
	}

	struct TestETagged {
		etag: String,
	}

	impl ETagged for TestETagged {
		fn etag(&self) -> &String {
			&self.etag
		}
	}

	#[test]
	fn test_etag_match() {
		let obj = TestETagged {
			etag: "test".to_string(),
		};
		assert!(obj.is_etag_match("test"));
		assert!(!obj.is_etag_match("test2"));
	}

	struct TestBoth {
		etag: String,
		modified_time: DateTime<Utc>,
	}

	impl ModifiedTime for TestBoth {
		fn modified_time(&self) -> DateTime<Utc> {
			self.modified_time
		}
	}

	impl ETagged for TestBoth {
		fn etag(&self) -> &String {
			&self.etag
		}
	}

	#[test]
	fn test_build_headers() {
		let obj = TestBoth {
			etag: "test".to_string(),
			modified_time: Utc.with_ymd_and_hms(2015, 10, 21, 7, 28, 0).unwrap(),
		};
		assert_eq!(obj.etag_header().value(), "test");
		assert_eq!(
			obj.last_modified_header().value(),
			"Wed, 21 Oct 2015 07:28:00 GMT"
		);
	}
}