use chrono::{DateTime, Utc};

trait ETagged {
	fn etag(&self) -> String;
}

trait ModifiedTime {
	fn modified_time(&self) -> DateTime<Utc>;
}