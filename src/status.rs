/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::policy::PaperPolicy;

#[derive(Debug)]
pub struct Status {
	pid: u32,

	max_size: u64,
	used_size: u64,
	num_objects: u64,

	rss: u64,
	hwm: u64,

	total_gets: u64,
	total_sets: u64,
	total_dels: u64,

	miss_ratio: f64,

	policies: Vec<PaperPolicy>,
	policy: PaperPolicy,
	is_auto_policy: bool,

	uptime: u64,
}

impl Status {
	/// Creates a new instance of the cache's status.
	#[allow(clippy::too_many_arguments)]
	#[must_use]
	pub fn new(
		pid: u32,

		max_size: u64,
		used_size: u64,
		num_objects: u64,

		rss: u64,
		hwm: u64,

		total_gets: u64,
		total_sets: u64,
		total_dels: u64,

		miss_ratio: f64,

		policies: Vec<PaperPolicy>,
		policy: PaperPolicy,
		is_auto_policy: bool,

		uptime: u64,
	) -> Self {
		Status {
			pid,

			max_size,
			used_size,
			num_objects,

			rss,
			hwm,

			total_gets,
			total_sets,
			total_dels,

			miss_ratio,

			policies,
			policy,
			is_auto_policy,

			uptime,
		}
	}

	/// Returns the cache's PID.
	#[must_use]
	pub fn pid(&self) -> u32 {
		self.pid
	}

	/// Returns the cache's maximum size in bytes.
	#[must_use]
	pub fn max_size(&self) -> u64 {
		self.max_size
	}

	/// Returns the cache's used size in bytes.
	#[must_use]
	pub fn used_size(&self) -> u64 {
		self.used_size
	}

	/// Returns the number of objects in the cache.
	#[must_use]
	pub fn num_objects(&self) -> u64 {
		self.num_objects
	}

	/// Returns the cache's resident set size.
	#[must_use]
	pub fn rss(&self) -> u64 {
		self.rss
	}

	/// Returns the cache's resident set size high water mark.
	#[must_use]
	pub fn hwm(&self) -> u64 {
		self.hwm
	}

	/// Returns the cache's total number of gets.
	#[must_use]
	pub fn total_gets(&self) -> u64 {
		self.total_gets
	}

	/// Returns the cache's total number of sets.
	#[must_use]
	pub fn total_sets(&self) -> u64 {
		self.total_sets
	}

	/// Returns the cache's total number of dels.
	#[must_use]
	pub fn total_dels(&self) -> u64 {
		self.total_dels
	}

	/// Returns the cache's miss ratio.
	#[must_use]
	pub fn miss_ratio(&self) -> f64 {
		self.miss_ratio
	}

	/// Returns the cache's configured eviction policies.
	#[must_use]
	pub fn policies(&self) -> &[PaperPolicy] {
		&self.policies
	}

	/// Returns the cache's eviction policy.
	#[must_use]
	pub fn policy(&self) -> &PaperPolicy {
		&self.policy
	}

	/// Returns `true` if the cache is configured to the [`PaperPolicy::Auto`] eviction policy.
	#[must_use]
	pub fn is_auto_policy(&self) -> bool {
		self.is_auto_policy
	}

	/// Returns the cache's uptime.
	#[must_use]
	pub fn uptime(&self) -> u64 {
		self.uptime
	}
}
