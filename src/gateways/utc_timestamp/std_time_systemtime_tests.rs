use attestify_kernel::gateways::utc_timestamp::UTCTimestampGateway;
use crate::gateways::utc_timestamp::std_time_systemtime::SystemTimeUTCTimeStampGW;

use attestify_test_framework::is_ok;

/// # Test: `utc_timestamp_now_success`
///
/// ## Purpose
/// - Verify `SystemTimeUTCTimeStampGW::now()` returns a valid `UTCTimestamp`.
/// - Check that the numeric getters for the timestamp (`as_nano()`, `as_milli()`, `as_sec()`)
///   are consistent with each other according to the expected unit relationships.
///
/// ## Background (simple, for newcomers)
/// - `SystemTime` represents a point in time; the timestamp values here are integer counts
///   in different units:
///   - `as_nano()` -> total elapsed nanoseconds (integer)
///   - `as_milli()` -> total elapsed milliseconds (integer, truncated/floored)
///   - `as_sec()`  -> total elapsed seconds (integer, truncated/floored)
/// - Truncation/floor behavior: converting from a finer unit to a coarser integer unit drops
///   the fractional remainder. For example, converting 1_500_000 nanoseconds to milliseconds
///   gives 1 millisecond (the fractional 0.5 ms is discarded).
///
/// ## What the test does (step-by-step)
/// 1. Create the system-time gateway and call `now()`. Use `is_ok!` to assert the call succeeded
///    and to extract the `UTCTimestamp`.
/// 2. Read three integer representations: `nanos`, `millis`, and `secs`. Cast them to `u128` to
///    avoid overflow in arithmetic checks.
/// 3. Define constants for unit conversion:
///    - `NANOS_PER_MILLI = 1_000_000`
///    - `NANOS_PER_SEC = 1_000_000_000`
///    - `MILLIS_PER_SEC = 1000`
/// 4. Check relationships reflecting truncation:
///    - `nanos` must be at least `millis * NANOS_PER_MILLI` and less than `(millis + 1) * NANOS_PER_MILLI`.
///      This ensures `millis` is the floor of `nanos / NANOS_PER_MILLI`.
///    - `millis` must be at least `secs * MILLIS_PER_SEC` and less than `(secs + 1) * MILLIS_PER_SEC`.
///      This ensures `secs` is the floor of `millis / MILLIS_PER_SEC`.
///    - Cross-check `nanos` against `secs` to ensure the same bucket logic holds across nanos and seconds.
///
/// ## Why these checks are useful
/// - They validate that the different accessors are coherent and implement the expected integer
///   truncation semantics, rather than returning inconsistent or scaled values.
///
/// ## Notes
/// - Using integer arithmetic and inequalities is robust against small timing differences.
/// - Casting to `u128` gives headroom for arithmetic without overflow.
/// - Unable to verify the two failure modes given there is no control over `SystemTime::now()`.
///
#[test]
fn utc_timestamp_now_success() {

	let gateway = SystemTimeUTCTimeStampGW::new();
	let utc_timestamp = gateway.now();
	let timestamp = is_ok!(utc_timestamp);

	let nanos: u128 = timestamp.as_nano() as u128;
	let millis: u128 = timestamp.as_milli() as u128;
	let secs: u128 = timestamp.as_sec() as u128;

	const NANOS_PER_MILLI: u128 = 1_000_000;
	const NANOS_PER_SEC: u128 = 1_000_000_000;
	const MILLIS_PER_SEC: u128 = 1000;

	// nanos should be within the millisecond bucket (floor behavior)
	assert!(nanos >= millis * NANOS_PER_MILLI,
			"nanos {} < millis * NANOS_PER_MILLI {}", nanos, millis * NANOS_PER_MILLI);
	assert!(nanos < (millis + 1) * NANOS_PER_MILLI,
			"nanos {} >= (millis+1) * NANOS_PER_MILLI {}", nanos, (millis + 1) * NANOS_PER_MILLI);

	// millis should be within the second bucket (floor behavior)
	assert!(millis >= secs * MILLIS_PER_SEC,
			"millis {} < secs * MILLIS_PER_SEC {}", millis, secs * MILLIS_PER_SEC);
	assert!(millis < (secs + 1) * MILLIS_PER_SEC,
			"millis {} >= (secs+1) * MILLIS_PER_SEC {}", millis, (secs + 1) * MILLIS_PER_SEC);

	// cross-check nanos vs secs
	assert!(secs <= nanos / NANOS_PER_SEC,
			"secs {} > nanos / NANOS_PER_SEC {}", secs, nanos / NANOS_PER_SEC);
	assert!(nanos < (secs + 1) * NANOS_PER_SEC,
			"nanos {} >= (secs+1) * NANOS_PER_SEC {}", nanos, (secs + 1) * NANOS_PER_SEC);

}