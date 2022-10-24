use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

#[derive(Clone)]
pub struct Rotor {
    len: usize,
    pos: usize,
    straight: Vec<usize>,
    backward: Vec<usize>,
}

impl Rotor {
    pub fn from_seed(len: usize, seed: u64) -> Rotor {
        let mut straight: Vec<usize> = (0..len).collect();
        let mut backward: Vec<usize> = (0..len).map(|_| 0).collect();
        let mut rng = StdRng::seed_from_u64(seed);

        straight.as_mut_slice().shuffle(&mut rng);

        for (i, j) in straight.iter().enumerate() {
            backward[*j] = i;
        }

        Rotor {
            len,
            pos: 0,
            straight,
            backward,
        }
    }

    pub fn rotate(&mut self) -> bool {
        self.pos = (self.pos + 1) % self.len;
        self.pos == 0
    }

    fn _rotated(&self, x: usize) -> usize {
        (self.pos + x) % self.len
    }

    fn _unrotated(&self, x: usize) -> usize {
        (x + self.len - self.pos) % self.len
    }

    pub fn get_straight(&self, x: usize) -> usize {
        self._unrotated(self.straight[self._rotated(x)])
    }

    pub fn get_backward(&self, x: usize) -> usize {
        self._unrotated(self.backward[self._rotated(x)])
    }
}

pub struct Reflector {
    reflects: Vec<usize>,
}

impl Reflector {
    pub fn from_seed(len: usize, seed: u64) -> Reflector {
        let mut rng = StdRng::seed_from_u64(seed);

        let mut reflects: Vec<usize> = (0..len).map(|_| len).collect();

        let stable = if (len & 1) == 0 {
            len
        } else {
            rng.gen_range(0..len)
        };

        for i in 0..len {
            if reflects[i] != len {
                continue;
            }

            if i == stable {
                reflects[i] = stable;
                continue;
            }

            loop {
                let j = rng.gen_range(i..len);
                if j == stable || reflects[j] != len {
                    continue;
                }

                reflects[i] = j;
                reflects[j] = i;
                break;
            }
        }

        Reflector { reflects }
    }

    pub fn get_reflect(&self, x: usize) -> usize {
        self.reflects[x]
    }
}

pub struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl Enigma {
    pub fn from_seed(n_rotors: usize, seed: u64) -> Enigma {
        let mut rng = StdRng::seed_from_u64(seed);
        let alphabet_size = 256;

        let rotors = (0..n_rotors)
            .map(|_| Rotor::from_seed(alphabet_size, rng.gen()))
            .collect();
        let reflector = Reflector::from_seed(alphabet_size, rng.gen());

        Enigma { rotors, reflector }
    }

    fn _process_one(&mut self, x: usize) -> usize {
        let mut x = x;

        for rot in self.rotors.iter() {
            x = rot.get_straight(x);
        }

        x = self.reflector.get_reflect(x);

        for rot in self.rotors.iter().rev() {
            x = rot.get_backward(x);
        }

        for rot in self.rotors.iter_mut() {
            if !rot.rotate() {
                break;
            }
        }

        x
    }

    pub fn run(&mut self, input: &[u8]) -> Vec<u8> {
        input
            .iter()
            .map(|x| {
                self._process_one((*x).try_into().unwrap())
                    .try_into()
                    .unwrap()
            })
            .collect()
    }
}
