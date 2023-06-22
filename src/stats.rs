use crate::policy::Policy;

#[derive(Debug)]
pub struct Stats {
	max_size: u64,
	used_size: u64,
	total_gets: u64,
	miss_ratio: f64,
	policy: Policy,
}

impl Stats {
	/// Creates a new instance of the cache's stats.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(0, 0, 0, 0.0, Policy::Lru);
	/// ```
	pub fn new(
		max_size: u64,
		used_size: u64,
		total_gets: u64,
		miss_ratio: f64,
		policy: Policy,
	) -> Self {
		Stats {
			max_size,
			used_size,
			total_gets,
			miss_ratio,
			policy,
		}
	}

	/// Returns the cache's maximum size in bytes.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(1000, 0, 0, 0.0, Policy::Lru);
	/// assert_eq(stats.get_max_size(), 1000);
	/// ```
	pub fn get_max_size(&self) -> &u64 {
		&self.max_size
	}

	/// Returns the cache's used size in bytes.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(1000, 500, 0, 0.0, Policy::Lru);
	/// assert_eq(stats.get_used_size(), 500);
	/// ```
	pub fn get_used_size(&self) -> &u64 {
		&self.used_size
	}

	/// Returns the cache's total number of gets.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(0, 0, 10, 0.0, Policy::Lru);
	/// assert_eq(stats.get_total_gets(), 10);
	/// ```
	pub fn get_total_gets(&self) -> &u64 {
		&self.total_gets
	}

	/// Returns the cache's miss ratio.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(0, 0, 0, 1.0, Policy::Lru);
	/// assert_eq(stats.get_miss_ratio(), 1.0);
	/// ```
	pub fn get_miss_ratio(&self) -> &f64 {
		&self.miss_ratio
	}

	/// Returns the cache's eviction policy.
	///
	/// # Examples
	/// ```
	/// let stats = Stats::new(0, 0, 0, 0.0, Policy::Lru);
	/// assert_eq(stats.get_policy(), &Policy::Lru);
	/// ```
	pub fn get_policy(&self) -> &Policy {
		&self.policy
	}
}
