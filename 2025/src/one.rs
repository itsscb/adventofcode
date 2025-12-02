#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_safe() {
        let mut safe = Safe::default();
        assert_eq!(safe.current, 50);

        safe.rotate(Rotation::L(68));
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
    fn right(&mut self, count: u32) {
        self.current = (self.current + count) % 100;
    }

    fn left(&mut self, count: u32) {
        let num = count % 100;

        self.current = if self.current >= num {
            self.current - num
        } else {
            100 - (num - self.current)
        };
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::L(r) => self.left(r),

            Rotation::R(r) => self.right(r),
        }
        if self.current == 0 {
            self.counter += 1;
        }
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }
}

#[derive(Debug, Clone)]
pub enum Rotation {
    L(u32),
    R(u32),
}

impl From<&[u8]> for Rotation {
    fn from(value: &[u8]) -> Self {
        let num: &[u8] = &value[1..];
        // SAFETY: Promised by input
        let num: u32 = std::str::from_utf8(num).unwrap().parse::<u32>().unwrap();
        let res = match value[0] {
            b'L' => Rotation::L(num),
            b'R' => Rotation::R(num),
            _ => unreachable!("should never happen"),
        };
        res
    }
}

pub fn solve_one(input: &[u8]) -> u32 {
    let mut safe = Safe::default();
    input
        .split(|c| *c == b'\r' || *c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Rotation::from(line))
        .for_each(|rotation| safe.rotate(rotation));
    safe.counter
}
