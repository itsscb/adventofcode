#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_safe() {
        let mut safe = Safe::default();
        assert_eq!(safe.current, 50);
        safe.rotate(&Rotation::R(1000));
        assert_eq!(safe.current, 50);
        assert_eq!(safe.get_counter(), 10);
        let mut safe = Safe::default();
        assert_eq!(safe.current, 50);

        safe.rotate(&Rotation::L(68));
        assert_eq!(safe.current, 82);
        assert_eq!(safe.get_counter(), 1);
        safe.rotate(&Rotation::L(30));
        assert_eq!(safe.current, 52);
        safe.rotate(&Rotation::R(48));
        assert_eq!(safe.current, 0);
        assert_eq!(safe.get_counter(), 2);
        safe.rotate(&Rotation::L(5));
        assert_eq!(safe.current, 95);
        assert_eq!(safe.get_counter(), 2);
        safe.rotate(&Rotation::R(60));
        assert_eq!(safe.current, 55);
        assert_eq!(safe.get_counter(), 3);
        safe.rotate(&Rotation::L(55));
        assert_eq!(safe.current, 0);
        assert_eq!(safe.get_counter(), 4);

        safe.rotate(&Rotation::L(1));
        assert_eq!(safe.current, 99);
        assert_eq!(safe.get_counter(), 4);

        safe.rotate(&Rotation::L(99));
        assert_eq!(safe.current, 0);
        assert_eq!(safe.get_counter(), 5);
        safe.rotate(&Rotation::R(14));
        assert_eq!(safe.current, 14);
        safe.rotate(&Rotation::L(82));
        assert_eq!(safe.current, 32);
        assert_eq!(safe.get_counter(), 6);
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
        let num = count % 100;
        self.counter += count / 100;
        if self.current + num > 100 {
            self.counter += 1;
        }
        self.current = (self.current + count) % 100;
        if self.current == 0 {
            self.counter += 1;
        }
    }

    const fn left(&mut self, count: u32) {
        let num = count % 100;
        if num <= self.current {
            self.current -= num;
        } else {
            if self.current != 0 {
                self.counter += 1;
            }
            self.current = 100 - (num - self.current);
        }
        if self.current == 0 {
            self.counter += 1;
        }

        self.counter += count / 100;
    }

    pub const fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::L(r) => self.left(*r),

            Rotation::R(r) => self.right(*r),
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

pub fn solve_two(input: &[u8]) -> u32 {
    let mut safe = Safe::default();
    input
        .split(|c| *c == b'\r' || *c == b'\n')
        .filter(|line| !line.is_empty())
        .map(Rotation::from)
        .for_each(|rotation| safe.rotate(&rotation));
    safe.counter
}
