#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// Enum for recording the final state of a frame
#[derive(Debug)]
enum FinishedFrame {
    Strike,
    Spare(u16),
    Normal((u16, u16)),
}

impl FinishedFrame {
    pub fn first(&self) -> u16 {
        match *self {
            FinishedFrame::Strike => 10,
            FinishedFrame::Spare(r1) => r1,
            FinishedFrame::Normal((r1, _)) => r1,
        }
    }

    pub fn total(&self) -> u16 {
        match *self {
            FinishedFrame::Strike => 10,
            FinishedFrame::Spare(_) => 10,
            FinishedFrame::Normal((r1, r2)) => r1 + r2,
        }
    }
}

// default kind of frame for frames 1-9, can only be rolled up to twice
#[derive(Debug)]
struct NormalFrame {
    downed: u16,
    rolled: bool,
}

impl NormalFrame {
    pub fn new() -> Self {
        NormalFrame {
            downed: 0,
            rolled: false,
        }
    }
    fn roll(&mut self, roll: u16) -> Result<Option<FinishedFrame>, Error> {
        if roll > (10 - self.downed) {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.rolled {
            let total = self.downed + roll;
            if total == 10 {
                return Ok(Some(FinishedFrame::Spare(self.downed)));
            } else {
                return Ok(Some(FinishedFrame::Normal((self.downed, roll))));
            }
        } else {
            if roll == 10 {
                return Ok(Some(FinishedFrame::Strike));
            } else {
                self.rolled = true;
                self.downed = roll;
            }
        }

        Ok(None)
    }
}

#[derive(Debug)]
struct LastFrame {
    rolls: Vec<u16>,
    fill: bool,
    remaining: u16,
}

impl LastFrame {
    pub fn new() -> Self {
        LastFrame {
            rolls: vec![],
            fill: false,
            remaining: 10,
        }
    }

    pub fn is_finished(&self) -> bool {
        (self.rolls.len() == 2 && !self.fill) || (self.rolls.len() == 3 && self.fill)
    }

    pub fn total(&self) -> u16 {
        self.rolls.iter().sum()
    }

    pub fn roll(&mut self, roll: u16) -> Result<(), Error> {
        if self.is_finished() {
            return Err(Error::GameComplete);
        }

        if self.rolls.len() == 1 {
            if roll > self.remaining {
                return Err(Error::NotEnoughPinsLeft);
            }

            self.rolls.push(roll);

            if roll == self.remaining {
                self.fill = true;
                self.remaining = 10;
            }
        } else {
            self.rolls.push(roll);
            if roll != 10 {
                self.remaining -= roll;
            } else {
                self.fill = true;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    previous_frames: Vec<FinishedFrame>,
    current_frame: NormalFrame,
    final_frame: LastFrame,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            previous_frames: vec![],
            current_frame: NormalFrame::new(),
            final_frame: LastFrame::new(),
        }
    }

    pub fn roll(&mut self, roll: u16) -> Result<(), Error> {
        if roll > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.previous_frames.len() < 9 {
            let rollres = self.current_frame.roll(roll)?;

            if let Some(finished) = rollres {
                self.previous_frames.push(finished);
                self.current_frame = NormalFrame::new();
            }
        } else if !self.final_frame.is_finished() {
            self.final_frame.roll(roll)?;
        } else {
            return Err(Error::GameComplete);
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // println!("{:?}", self);
        if self.previous_frames.len() == 9 && self.final_frame.is_finished() {
            let mut total = self.final_frame.total();

            let mut prev_frame: Option<&FinishedFrame> = None;

            for frame in &self.previous_frames {
                total += frame.total();

                total += match prev_frame {
                    Some(&FinishedFrame::Strike) => frame.total(),
                    Some(&FinishedFrame::Spare(_)) => frame.first(),
                    _ => 0,
                };

                prev_frame = Some(frame);
            }

            return Some(total);
        }

        None
    }
}
