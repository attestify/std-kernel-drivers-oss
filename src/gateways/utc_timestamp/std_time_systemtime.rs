use std::time::{SystemTime, UNIX_EPOCH};

use attestify_kernel::values::datetime::utc_timestamp::UTCTimestamp;
use attestify_kernel::gateways::utc_timestamp::UTCTimestampGateway;
use attestify_kernel::error::Error;
use attestify_kernel::error::Kind;

#[derive(Clone, Default)]
pub struct SystemTimeUTCTimeStampGW;

impl SystemTimeUTCTimeStampGW {
	pub fn new() -> Self {
		Self
	}
}

impl UTCTimestampGateway for SystemTimeUTCTimeStampGW {
	fn now(&self) -> Result<UTCTimestamp, Error> {
		let duration = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|e| {
				Error::for_system(Kind::GatewayError,
								  format!("Failed to retrieve the SystemTime because of: {}", e))
		})?;

		UTCTimestamp::builder()
			.use_ns(duration.as_nanos())
			.build()
			.map_err(|e| { Error::for_system(Kind::ProcessingFailure,
											 format!("Failed build the UTCTimestamp from the SystemTime: {}", e))
			})
	}
}