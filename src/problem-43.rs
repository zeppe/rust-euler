struct Level {
    n: i64, // number with l digits
    l: i64, // number of digits so far
    u: [u32; 10], // digits used (0: used; 1: not used)
    r: Vec<Option<Level>>, // valid numbers leading with n
}

impl Level {
    // this fills the possible solutions checking iteratively whether they are feasible
    // basically, some dynamic programming
    pub fn fill(&mut self) {
        let mul = match self.l {
            0..3 => None,
            3 => Some(2),
            4 => Some(3),
            5 => Some(5),
            6 => Some(7),
            7 => Some(11),
            8 => Some(13),
            9 => Some(17),
            _ => panic!("not possible"),
        };
        self.r = (0..10)
            .map(|x| {
                if self.u[x as usize] == 0 {
                    return None;
                } // digit already used
                if self.l == 0 && x == 0 {
                    return None;
                } // remove leading zero
                let mut lvl = Level {
                    n: self.n * 10 + x,
                    l: self.l + 1,
                    u: self.u,
                    r: Vec::new(),
                };
                lvl.u[x as usize] = 0; // mark the digit as used
                if let Some(mul) = mul {
                    if (lvl.n % 1000) % mul != 0 {
                        return None;
                    }
                }
                if lvl.l < 10 {
                    lvl.fill();
                }
                Some(lvl)
            })
            .collect()
    }

    pub fn extract(&self) -> Vec<i64> {
        if self.l == 10 {
            return vec![self.n as i64];
        }
        return self
            .r
            .iter()
            .map(|l| l.as_ref().map(|l| l.extract()).unwrap_or(vec![]))
            .flatten()
            .collect();
    }
}

fn main() {
    let mut l = Level {
        n: 0,
        l: 0,
        u: [1; 10],
        r: Vec::new(),
    };
    l.fill();
    let results = l.extract();
    results.iter().for_each(|r| println!("{0}", r));
    println!("result: {0}", results.iter().sum::<i64>());
}
