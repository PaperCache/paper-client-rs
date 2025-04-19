use crate::policy::PaperPolicy;

#[derive(Debug)]
pub struct Stats {
	max_size: u64,
	used_size: u64,
	num_objects: u64,

	total_gets: u64,
	total_sets: u64,
	total_dels: u64,

	miss_ratio: f64,

	policies: Vec<PaperPolicy>,
	policy: PaperPolicy,
	is_auto_policy: bool,

	uptime: u64,
}

impl Stats {
	/// Creates a new instance of the cache's stats.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	/// ```
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		max_size: u64,
		used_size: u64,
		num_objects: u64,

		total_gets: u64,
		total_sets: u64,
		total_dels: u64,

		miss_ratio: f64,

		policies: Vec<PaperPolicy>,
		policy: PaperPolicy,
		is_auto_policy: bool,

		uptime: u64,
	) -> Self {
		Stats {
			max_size,
			used_size,
			num_objects,

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

	/// Returns the cache's maximum size in bytes.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(1000, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_max_size(), 1000);
	/// ```
	pub fn get_max_size(&self) -> u64 {
		self.max_size
	}

	/// Returns the cache's used size in bytes.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(1000, 500, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_used_size(), 500);
	/// ```
	pub fn get_used_size(&self) -> u64 {
		self.used_size
	}

	/// Returns the number of objects in the cache.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(1000, 500, 10, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_num_objects(), 10);
	/// ```
	pub fn get_num_objects(&self) -> u64 {
		self.num_objects
	}

	/// Returns the cache's total number of gets.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 10, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_total_gets(), 10);
	/// ```
	pub fn get_total_gets(&self) -> u64 {
		self.total_gets
	}

	/// Returns the cache's total number of sets.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 10, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_total_sets(), 10);
	/// ```
	pub fn get_total_sets(&self) -> u64 {
		self.total_sets
	}

	/// Returns the cache's total number of dels.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 10, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_total_dels(), 10);
	/// ```
	pub fn get_total_dels(&self) -> u64 {
		self.total_dels
	}

	/// Returns the cache's miss ratio.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 1.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_miss_ratio(), 1.0);
	/// ```
	pub fn get_miss_ratio(&self) -> f64 {
		self.miss_ratio
	}

	/// Returns the cache's configured eviction policies.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_policies(), &[PaperPolicy::Lru]);
	/// ```
	pub fn get_policies(&self) -> &[PaperPolicy] {
		&self.policies
	}

	/// Returns the cache's eviction policy.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 0);
	///
	/// assert_eq!(stats.get_policy(), &PaperPolicy::Lru);
	/// ```
	pub fn get_policy(&self) -> &PaperPolicy {
		&self.policy
	}

	/// Returns `true` if the cache is configured to the [`PaperPolicy::Auto`]
	/// eviction policy.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, true, 0);
	///
	/// assert!(stats.is_auto_policy());
	/// ```
	pub fn is_auto_policy(&self) -> bool {
		self.is_auto_policy
	}

	/// Returns the cache's uptime.
	///
	/// # Examples
	/// ```
	/// use paper_client::{Stats, PaperPolicy};
	///
	/// let stats = Stats::new(0, 0, 0, 0, 0, 0, 0.0, vec![PaperPolicy::Lru], PaperPolicy::Lru, false, 1);
	///
	/// assert_eq!(stats.get_uptime(), 1);
	/// ```
	pub fn get_uptime(&self) -> u64 {
		self.uptime
	}
}
