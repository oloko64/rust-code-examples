use std::{cmp::Reverse, collections::HashMap, iter::Sum, ops::Add};

pub struct Profile {
    pub user_id: String,
    pub created_at: i64,
    pub login_type: LoginType,
}

impl AsRef<Profile> for Profile {
    fn as_ref(&self) -> &Profile {
        self
    }
}

pub enum LoginType {
    Email,
    Apple,
    Google,
    Facebook,
}

pub struct SignUpStats {
    methods_per_day: Vec<StatDateCount<MethodCounts>>,
    methods_totals: MethodCounts,
}

impl SignUpStats {
    pub fn new<T>(profiles: &[T]) -> Result<SignUpStats, ()>
    where
        for<'a> T: AsRef<Profile>,
    {
        let mut method_counts: HashMap<i64, MethodCounts> = HashMap::new();

        for profile in profiles {
            method_counts
                .entry(profile.as_ref().created_at)
                .and_modify(|c| c.increment(&profile.as_ref().login_type))
                .or_insert(MethodCounts::new(&profile.as_ref().login_type));
        }

        let methods_totals = method_counts.values().sum();

        let methods_per_day = StatDateCount::to_sorted_vec(method_counts);

        Ok(SignUpStats {
            methods_per_day,
            methods_totals,
        })
    }
}

pub struct StatDateCount<T: Counter> {
    date: i64,
    counts: T,
}

impl<T: Counter> StatDateCount<T> {
    fn to_sorted_vec(counts: HashMap<i64, T>) -> Vec<Self> {
        let mut date_counts: Vec<_> = counts
            .into_iter()
            .map(|(date, counts)| StatDateCount { date, counts })
            .collect();

        date_counts.sort_by_key(|c| Reverse(c.date));

        date_counts
    }
}

#[derive(Default, Clone)]
struct MethodCounts {
    apple: u32,
    email: u32,
    facebook: u32,
    google: u32,
    total: u32,
}

pub trait Counter: for<'a> Add<&'a Self, Output = Self> + for<'a> Sum<&'a Self> + Default {
    type Value;

    fn new(value: &Self::Value) -> Self;
    fn increment(&mut self, value: &Self::Value);
}

impl Counter for MethodCounts {
    type Value = LoginType;

    fn new(login_type: &Self::Value) -> Self {
        let mut count = Self::default();

        Self::increment(&mut count, login_type);

        count
    }

    fn increment(&mut self, login_type: &Self::Value) {
        match login_type {
            LoginType::Apple => self.apple += 1,
            LoginType::Email => self.email += 1,
            LoginType::Facebook => self.facebook += 1,
            LoginType::Google => self.google += 1,
        }

        self.total += 1;
    }
}

impl<'a> Add<&'a MethodCounts> for MethodCounts {
    type Output = MethodCounts;

    fn add(self, other: &'a Self) -> MethodCounts {
        MethodCounts {
            apple: self.apple + other.apple,
            email: self.email + other.email,
            facebook: self.facebook + other.facebook,
            google: self.google + other.google,
            total: self.total + other.total,
        }
    }
}

impl<'a> Sum<&'a MethodCounts> for MethodCounts {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Default::default(), |acc, x| acc + x)
    }
}

fn run() {
    let profile1 = Profile {
        user_id: "1".to_string(),
        created_at: 1,
        login_type: LoginType::Email,
    };

    let profile2 = Profile {
        user_id: "2".to_string(),
        created_at: 1,
        login_type: LoginType::Email,
    };

    // You can use a vector of references to the profiles
    let profiles = vec![&profile1, &profile2];
    let stats = SignUpStats::new(&profiles).unwrap();

    // Or you can use a vector of owned profiles
    let profiles = vec![profile1, profile2];
    let stats = SignUpStats::new(&profiles).unwrap();
}
