use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// Enum for recording the final state of a frame
#[derive(Debug, PartialEq)]
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

        self.rolls.push(roll);
        if self.rolls.len() == 0 {
            if roll != 10 {
                if roll > self.remaining {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.remaining -= roll;
            } else {
                self.fill = true;
            }
        } else {
            if roll > self.remaining {
                return Err(Error::NotEnoughPinsLeft);
            }

            if roll == self.remaining {
                self.fill = true;
                self.remaining = 10;
            } else {
                self.remaining -= roll;
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

            let finalframe_throws = (self.final_frame.rolls[0], self.final_frame.rolls[1]);
            let finalframe = FinishedFrame::Normal(finalframe_throws);

            let mut last_two: VecDeque<&FinishedFrame> = VecDeque::new();
            last_two.push_back(&finalframe);

            for frame in self.previous_frames.iter().rev() {
                total += frame.total();

                match frame {
                    FinishedFrame::Strike => {
                        if let Some(&prev) = last_two.back() {
                            if prev == &FinishedFrame::Strike {
                                if let Some(twobefore) = last_two.front() {
                                    total += twobefore.first();
                                }
                            }

                            total += prev.total();
                        }
                    }
                    FinishedFrame::Spare(_) => {
                        if let Some(&prev) = last_two.back() {
                            total += prev.first();
                        }
                    }
                    _ => {}
                }

                if last_two.len() == 2 {
                    last_two.pop_front();
                }
                last_two.push_back(&frame);
            }

            return Some(total);
        }

        None
    }
}
