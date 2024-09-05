use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Clone, Debug)]
pub enum Change {
    Absolute(usize),
    Delta(isize),
    Range { lo: isize, hi: isize, offset: isize },
}

impl Change {
    pub fn to_absolutes(&self, bound: usize) -> Result<Vec<usize>> {
        match self.clone() {
            Change::Absolute(i) if i < bound => Ok(vec![i]),
            Change::Absolute(i) if i >= bound => bail!("out of range: {i} >= {bound}"),
            Change::Absolute(_) => unreachable!(),
            Change::Delta(i) => Ok(vec![((bound as isize) + i) as usize]),
            Change::Range { lo, hi, offset } => {
                let mut next: isize;
                let end: isize;
                if offset > 0 {
                    // Normal loop from lo up to hi.
                    next = lo;
                    end = hi;
                } else {
                    // Inverse loop from hi down to lo.
                    next = hi;
                    end = lo;
                }
                let mut absolutes = vec![];
                while next < end {
                    absolutes.push(next as usize);
                    next += offset;
                }
                Ok(absolutes)
            }
        }
    }
}

impl FromStr for Change {
    type Err = Error;

    /// Parses the given string into a Change.
    fn from_str(s: &str) -> Result<Self> {
        if s.contains("..") {
            // This is a range of changes of the forms:
            // lo..hi
            // hi..lo
            // lo..+i..hi
            // hi..-i..lo

            let mut nums = vec![];
            for s in s.split("..").take(3) {
                let n = s.parse()?;
                nums.push(n);
            }

            match *nums.as_slice() {
                [lo, hi] if lo <= hi => Ok(Change::Range { lo, hi, offset: 1 }),
                [hi, lo] if hi > lo => Ok(Change::Range { lo, hi, offset: -1 }),
                [lo, offset, hi] if lo <= hi && offset > 0 => Ok(Change::Range { lo, hi, offset }),
                [lo, offset, hi] if lo <= hi && offset <= 0 => bail!("this range never terminates"),
                [hi, offset, lo] if hi > lo && offset < 0 => Ok(Change::Range { lo, hi, offset }),
                [hi, offset, lo] if hi > lo && offset >= 0 => bail!("this range never terminates"),
                _ => bail!("unsupported range"), // Unreachable?
            }
        } else if s.contains("+") || s.contains("-") {
            // This is a relative change of the forms:
            // +n
            // -n
            return Ok(Change::Delta(s.parse().with_context(|| "invalid number")?));
        } else {
            // A plain ol number indicates an absolute number.
            return Ok(Change::Absolute(
                s.parse().with_context(|| "invalid number")?,
            ));
        }
    }
}
