pub mod day;
pub mod solution;

pub trait Parts {
    fn part_1(&self, input: &str) -> PartSolution;
    fn part_2(&self, input: &str) -> PartSolution;
}

#[derive(Debug)]
pub enum PartSolution {
    I32(i32),
    U32(u32),
    U64(u64),
    USize(usize),
    String(String),
    Vec(Vec<String>),
    #[allow(dead_code)]
    Manual,
    #[allow(dead_code)]
    None,
}
impl PartSolution {
    #[must_use]
    pub fn has_solution(&self) -> bool {
        !matches!(self, PartSolution::None)
    }
}

impl PartialEq<PartSolution> for PartSolution {
    fn eq(&self, other: &PartSolution) -> bool {
        match self {
            PartSolution::I32(i) => i == other,
            PartSolution::U32(i) => i == other,
            PartSolution::U64(i) => i == other,
            PartSolution::USize(i) => i == other,
            PartSolution::String(i) => i == other,
            PartSolution::Vec(i) => i == other,
            PartSolution::None => matches!(other, &PartSolution::None),
            PartSolution::Manual => matches!(other, &PartSolution::Manual),
        }
    }
}

impl From<i32> for PartSolution {
    fn from(v: i32) -> Self {
        PartSolution::I32(v)
    }
}

impl From<u32> for PartSolution {
    fn from(v: u32) -> Self {
        PartSolution::U32(v)
    }
}

impl From<u64> for PartSolution {
    fn from(v: u64) -> Self {
        PartSolution::U64(v)
    }
}

impl From<usize> for PartSolution {
    fn from(v: usize) -> Self {
        PartSolution::USize(v)
    }
}

impl From<Vec<String>> for PartSolution {
    fn from(v: Vec<String>) -> Self {
        PartSolution::Vec(v)
    }
}

impl From<String> for PartSolution {
    fn from(v: String) -> Self {
        PartSolution::String(v)
    }
}

impl From<Option<PartSolution>> for PartSolution {
    fn from(value: Option<PartSolution>) -> Self {
        match value {
            Some(v) => v,
            None => PartSolution::None,
        }
    }
}

impl std::fmt::Display for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            PartSolution::I32(x) => x.to_string(),
            PartSolution::U32(x) => x.to_string(),
            PartSolution::U64(x) => x.to_string(),
            PartSolution::USize(x) => x.to_string(),
            PartSolution::String(x) => x.to_string(),
            PartSolution::Vec(x) => format!("\n{}", x.join("\n")),
            PartSolution::Manual => "Manual".to_owned(),
            PartSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}

impl std::cmp::PartialEq<PartSolution> for i32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => self == x,
            PartSolution::U32(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            PartSolution::U64(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: u32| &o == self),
            PartSolution::U32(x) => self == x,
            PartSolution::U64(x) => (x) == &u64::from(*self),
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: u32| &o == self),
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u64 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: u64| &o == self),
            PartSolution::U32(x) => &u64::from(*x) == self,
            PartSolution::U64(x) => x == self,
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: u64| &o == self),
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for usize {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::U32(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::U64(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::USize(x) => x == self,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for String {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::String(s) => s == self,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for Vec<String> {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::Vec(v) => {
                if v.len() != self.len() {
                    return false;
                }

                for (l, r) in self.iter().zip(v) {
                    if l != r {
                        return false;
                    }
                }

                true
            },
            _ => false,
        }
    }
}
