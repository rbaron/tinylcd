#[repr(u8)]
#[derive(Copy, Clone)]
#[rustfmt::skip]
pub enum Seg { A = 0, B, C, D, E, F, G }

impl Seg {
    const fn com(self) -> usize {
        match self {
            Seg::A => 3,
            Seg::B => 2,
            Seg::C => 1,
            Seg::D => 0,
            Seg::E => 1,
            Seg::F => 3,
            Seg::G => 2,
        }
    }

    const fn pin_offset(self) -> usize {
        match self {
            Seg::A => 0,
            Seg::B => 0,
            Seg::C => 0,
            Seg::D => 1,
            Seg::E => 1,
            Seg::F => 1,
            Seg::G => 1,
        }
    }
}

// Compact, 1-byte representation of a character pattern.
pub struct Patt(u8);

impl Patt {
    pub const fn from_segments(segments: &[Seg]) -> Self {
        // Iterators cannot be used in const fn yet.
        // Self(segments.iter().fold(0u8, |p, seg| p | (1 << (*seg as u8))))
        let mut patt = 0u8;
        let mut i = 0;
        while i < segments.len() {
            let seg = segments[i];
            patt |= 1 << (seg as u8);
            i += 1;
        }
        Self(patt)
    }

    const fn has_segment(&self, seg: Seg) -> bool {
        (self.0 & (1 << (seg as u8))) != 0
    }
}

const PATT_0: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F]);
const PATT_1: Patt = Patt::from_segments(&[Seg::B, Seg::C]);
const PATT_2: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::D, Seg::E, Seg::G]);
const PATT_3: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C, Seg::D, Seg::G]);
const PATT_4: Patt = Patt::from_segments(&[Seg::B, Seg::C, Seg::F, Seg::G]);
const PATT_5: Patt = Patt::from_segments(&[Seg::A, Seg::C, Seg::D, Seg::F, Seg::G]);
const PATT_6: Patt = Patt::from_segments(&[Seg::A, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G]);
const PATT_7: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C]);
const PATT_8: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G]);
const PATT_9: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C, Seg::D, Seg::F, Seg::G]);
const PATT_A: Patt = Patt::from_segments(&[Seg::A, Seg::B, Seg::C, Seg::E, Seg::F, Seg::G]);
const PATT_B: Patt = Patt::from_segments(&[Seg::C, Seg::D, Seg::E, Seg::F, Seg::G]);
const PATT_C: Patt = Patt::from_segments(&[Seg::A, Seg::D, Seg::E, Seg::F]);
const PATT_D: Patt = Patt::from_segments(&[Seg::B, Seg::C, Seg::D, Seg::E, Seg::G]);
const PATT_E: Patt = Patt::from_segments(&[Seg::A, Seg::D, Seg::E, Seg::F, Seg::G]);
const PATT_F: Patt = Patt::from_segments(&[Seg::A, Seg::E, Seg::F, Seg::G]);
const PATT_N: Patt = Patt::from_segments(&[Seg::C, Seg::E, Seg::G]);
const PATT_R: Patt = Patt::from_segments(&[Seg::E, Seg::G]);
const PATT_T: Patt = Patt::from_segments(&[Seg::D, Seg::E, Seg::F, Seg::G]);
const PATT_DASH: Patt = Patt::from_segments(&[Seg::G]);

const fn get_char_pattern(c: char) -> Patt {
    match c {
        '0' => PATT_0,
        '1' => PATT_1,
        '2' => PATT_2,
        '3' => PATT_3,
        '4' => PATT_4,
        '5' => PATT_5,
        '6' => PATT_6,
        '7' => PATT_7,
        '8' => PATT_8,
        '9' => PATT_9,
        'A' | 'a' => PATT_A,
        'B' | 'b' => PATT_B,
        'C' | 'c' => PATT_C,
        'D' | 'd' => PATT_D,
        'E' | 'e' => PATT_E,
        'F' | 'f' => PATT_F,
        'R' | 'r' => PATT_R,
        'O' | 'o' => PATT_0,
        'N' | 'n' => PATT_N,
        'T' | 't' => PATT_T,
        '-' => PATT_DASH,
        _ => Patt(0),
    }
}

pub struct Buff {
    pub data: [u8; 4],
}

impl Buff {
    pub fn new() -> Self {
        Self { data: [0; 4] }
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    pub fn set_pattern(&mut self, char_pos: u8, patt: &Patt) {
        if char_pos > 2 {
            return;
        }
        for seg in [Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G] {
            let seg_pin = ((2 - char_pos) as usize * 2) + seg.pin_offset();
            if patt.has_segment(seg) {
                self.data[seg.com()] |= 1 << seg_pin;
            } else {
                self.data[seg.com()] &= !(1 << seg_pin);
            }
        }
    }

    pub fn set_char(&mut self, char_pos: u8, char: char) {
        self.set_pattern(char_pos, &get_char_pattern(char));
    }
}
