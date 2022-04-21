#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Debug)]
enum Roll {
    Strike,
    Spare(u16),
    Open(u16),
    Bonus(u16),
}

impl Roll {
    pub fn value(&self) -> u16 {
        match self {
            Roll::Strike => 10,
            Roll::Spare(i) => *i,
            Roll::Open(i) => *i,
            Roll::Bonus(i) => *i,
        }
    }
}

pub struct BowlingGame {
    frames: Vec<Roll>,
    frame: u16,
    rolls_this_frame: u16,
    remaining_pins: u16,
    bonus: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
            frame: 1,
            rolls_this_frame: 0,
            remaining_pins: 10,
            bonus: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame == 11 {
            return Err(Error::GameComplete);
        }

        if pins > self.remaining_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls_this_frame += 1;

        if pins == 10 {
            // special case, strike on very last roll
            if self.bonus {
                self.frames.push(Roll::Bonus(10));
                self.next_frame();
            } else if self.frame == 10 && self.rolls_this_frame == 1 {
                self.frames.push(Roll::Strike);
                self.rolls_this_frame = 0;
                self.remaining_pins = 10;
                self.bonus = true;
            } else {
                // if it's a strike
                self.frames.push(Roll::Strike);
                self.next_frame();
            }
            return Ok(());
        } else if pins == self.remaining_pins {
            // special case, spare on very last roll
            if self.bonus {
                self.frames.push(Roll::Bonus(pins));
                self.next_frame();
            } else if self.frame == 10 && self.rolls_this_frame == 2 {
                self.frames.push(Roll::Spare(pins));
                self.rolls_this_frame -= 1;
                self.remaining_pins = 10;
                self.bonus = true;
            } else {
                // if its a spare
                self.frames.push(Roll::Spare(pins));
                self.next_frame();
            }
            return Ok(());
        } else {
            if self.bonus {
                self.frames.push(Roll::Bonus(pins));
            } else {
                self.frames.push(Roll::Open(pins));
            }
        }
        self.remaining_pins -= pins;
        if self.rolls_this_frame == 2 {
            self.next_frame();
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:?}", self.frames);
        println!("{:?}", self.frame);
        if self.frame != 11 {
            return None;
        }

        let mut frames = self.frames.clone();
        frames.push(Roll::Open(0));
        frames.push(Roll::Open(0));

        let score = frames
            .windows(3)
            .map(|window| match window[0] {
                Roll::Strike => &window[..],
                Roll::Spare(_) => &window[..=1],
                _ => &window[0..1],
                // Roll::Open(_) => &window[0..1],
                // Roll::Bonus(i) => &[Roll::Open(i)]
            })
            .filter(|window| {
                !window.iter().all(|roll| match roll {
                    Roll::Bonus(_) => true,
                    _ => false,
                })
            })
            .map(|window| {
                println!("{:?}", window);
                window.iter().map(|roll| roll.value()).sum::<u16>()
            })
            .sum();

        Some(score)
    }

    fn next_frame(&mut self) {
        self.frame += 1;
        self.rolls_this_frame = 0;
        self.remaining_pins = 10;
    }
}
