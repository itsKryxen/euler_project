use core::fmt;

const BASE: u64 = 1_000_000_000_000_000_000;
#[derive(Clone, Debug)]
pub struct BigInt {
    digits: Vec<u64>,
}
impl BigInt {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        let mut digits = Vec::new();
        let bytes = s.as_bytes();
        let mut i = bytes.len();

        while i > 0 {
            let start = i.saturating_sub(18);
            let chunk = &s[start..i];

            let value = match chunk.parse::<u64>() {
                Ok(v) => v,
                Err(_) => return None,
            };

            digits.push(value);
            i = start;
        }
        let mut res = Self { digits };
        res.normalize();
        Some(res)
    }
    fn normalize(&mut self) {
        while self.digits.len() > 1 && *self.digits.last().unwrap() == 0 {
            self.digits.pop();
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        let n = self.digits.len().max(other.digits.len());
        let mut res = Vec::with_capacity(n + 1);
        let mut carry: u64 = 0;
        for i in 0..n {
            let a = *self.digits.get(i).unwrap_or(&0) as u128;
            let b = *other.digits.get(i).unwrap_or(&0) as u128;
            let sum = a + b + carry as u128;
            res.push((sum % BASE as u128) as u64);
            carry = (sum / BASE as u128) as u64;
        }
        if carry > 0 {
            res.push(carry);
        }
        let mut result = Self { digits: res };
        result.normalize();
        result
    }
}
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.digits.iter().rev();
        if let Some(&first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for &d in iter {
            write!(f, "{:018}", d)?;
        }
        Ok(())
    }
}
