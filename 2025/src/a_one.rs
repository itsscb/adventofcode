#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_safe() {
        let mut safe = Safe::default();
        assert_eq!(safe.current, 50);

        safe.rotate(&Rotation::L(68));
        assert_eq!(safe.current, 82);
    }
}

#[derive(Debug, Clone)]
pub struct Safe {
    current: u32,
    counter: u32,
}

impl Default for Safe {
    fn default() -> Self {
        Self {
            current: 50,
            counter: Default::default(),
        }
    }
}

impl Safe {
    const fn right(&mut self, count: u32) {
        self.current = (self.current + count) % 100;
    }

    const fn left(&mut self, count: u32) {
        let num = count % 100;

        self.current = if self.current >= num {
            self.current - num
        } else {
            100 - (num - self.current)
        };
    }

    pub const fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::L(r) => self.left(*r),

            Rotation::R(r) => self.right(*r),
        }
        if self.current == 0 {
            self.counter += 1;
        }
    }

    #[must_use]
    pub const fn get_counter(&self) -> u32 {
        self.counter
    }
}

#[derive(Debug, Clone)]
pub enum Rotation {
    L(u32),
    R(u32),
}

#[allow(clippy::fallible_impl_from, clippy::unwrap_used)]
impl From<&[u8]> for Rotation {
    fn from(value: &[u8]) -> Self {
        let num: &[u8] = &value[1..];
        // SAFETY: Promised by input
        let num: u32 = unsafe { std::str::from_utf8_unchecked(num) }
            .parse::<u32>()
            .unwrap();
        match value[0] {
            b'L' => Self::L(num),
            b'R' => Self::R(num),
            _ => unreachable!("should never happen"),
        }
    }
}

pub fn solve_one(input: &[u8]) -> u32 {
    let mut safe = Safe::default();
    input
        .split(|c| *c == b'\r' || *c == b'\n')
        .filter(|line| !line.is_empty())
        .map(Rotation::from)
        .for_each(|rotation| safe.rotate(&rotation));
    safe.counter
}
