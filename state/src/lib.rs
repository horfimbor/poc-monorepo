use horfimbor_eventsource::horfimbor_eventsource_derive::StateNamed;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use mono_shared::command::MonoCommand;
use mono_shared::error::MonoError;
use mono_shared::event::{Delayed, MonoEvent};
use mono_shared::{MONO_STATE_NAME, START_VALUE};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed)]
#[state(MONO_STATE_NAME)]
pub struct MonoState {
    value: usize,
    last_id: usize,
    delayed: Vec<Delayed>,
}

impl MonoState {
    #[must_use]
    pub fn get_value(&self) -> usize {
        self.value
    }

    #[cfg(debug_assertions)]
    pub fn delayed(&self) -> &Vec<Delayed> {
        &self.delayed
    }

    #[cfg(debug_assertions)]
    pub fn time_pass(&mut self, nb: u64) {
        for delay in &mut self.delayed {
            delay.timestamp -= nb;
        }
    }
    #[cfg(debug_assertions)]
    pub fn get_id(&self, nb: usize) -> usize {
        self.delayed[nb].id
    }
}

impl Default for MonoState {
    fn default() -> Self {
        Self {
            value: START_VALUE,
            last_id: 0,
            delayed: vec![],
        }
    }
}

impl Dto for MonoState {
    type Event = MonoEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            MonoEvent::Added(i) => self.value += i,
            MonoEvent::Removed(i) => self.value -= i,
            MonoEvent::Delayed(d) => {
                self.last_id = d.id;
                self.delayed.push(d.clone());
            }
            MonoEvent::DelayDone(id) => {
                self.delayed = self
                    .delayed
                    .clone()
                    .into_iter()
                    .filter(|d| d.id != *id)
                    .collect();
            }
        }
    }
}

impl State for MonoState {
    type Command = MonoCommand;
    type Error = MonoError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            MonoCommand::Add(i) => {
                if self.value + i > 3000 {
                    Err(Self::Error::CannotAdd(i))
                } else {
                    Ok(vec![Self::Event::Added(i)])
                }
            }
            MonoCommand::Reset => {
                if self.value == 0 {
                    Err(Self::Error::AlreadyEmpty)
                } else {
                    Ok(vec![Self::Event::Removed(self.value)])
                }
            }
            MonoCommand::Delayed(d) => {
                if d.delay < 1 || d.delay > 10 {
                    return Err(Self::Error::DelayOutOfBound(d.delay));
                }

                let now = SystemTime::now();
                let duration = Duration::new(d.delay as u64, 0);
                let end = now + duration;
                let end = end
                    .duration_since(UNIX_EPOCH)
                    .map_err(|_| Self::Error::CannotCalculateTime)?;

                Ok(vec![Self::Event::Delayed(Delayed {
                    id: self.last_id + 1,
                    timestamp: end.as_secs(),
                    to_add: d.to_add,
                })])
            }
            MonoCommand::Finalize(id) => {
                let now = SystemTime::now();
                let epoch = now
                    .duration_since(UNIX_EPOCH)
                    .map_err(|_| Self::Error::CannotCalculateTime)?
                    .as_secs();

                for i in 0..self.delayed.len() {
                    if self.delayed[i].id == id && epoch >= self.delayed[i].timestamp {
                        return Ok(vec![
                            Self::Event::DelayDone(id),
                            Self::Event::Added(self.delayed[i].to_add),
                        ]);
                    }
                }

                Err(Self::Error::DelayNotFound)
            }
        }
    }
}
