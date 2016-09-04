use std::hash::{Hash, Hasher};
use gm2::*;
use super::*;

pub type ClimberId = UniqGameId;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TimedLocation {
    pub loc: Vec2i,
    pub inner_loc: Vec2i,
    pub at: Tick,
}

impl TimedLocation {
    pub fn exact_location(&self, z:f64) -> Vec3 {
        Vec3::new(
            self.loc.x as f64 + self.inner_loc.x as f64 / INNER_BLOCK_RESOLUTION as f64, 
            self.loc.y as f64 + self.inner_loc.y as f64 / INNER_BLOCK_RESOLUTION as f64, 
            z
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Climber {
    pub id: UniqGameId,
    pub morale: i8, // regenerates with time or tent, degrades on tragedy, negative is paniced
    pub prev: TimedLocation,
    pub next: TimedLocation,
}

impl Climber {
    pub fn exact_location_at(&self, tick:Tick, z:f64) -> Vec3 {
        let clamped = clamp(tick.at, self.prev.at.at, self.next.at.at);
        let action_duration = self.next.at.at - self.prev.at.at;
        let progress = (clamped - self.prev.at.at) as f64 / action_duration as f64;
     
        lerp(self.prev.exact_location(z), self.next.exact_location(z), progress)
    }
}

impl Hash for Climber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn tl_at(loc:Vec2i, inner_loc: Vec2i, at:Tick) -> TimedLocation {
    TimedLocation {
        loc: loc,
        inner_loc: inner_loc,
        at: at,
    }
}

pub fn climber_spawning_at(id:UniqGameId, at:Vec2i, now:Tick) -> Climber {
    Climber {
        id: id,
        morale: 8,
        prev: tl_at(at - vec2i(0, 1), vec2i(8, 8), now),
        next: tl_at(at,               vec2i(8, 4), now.plus(240)), // 4 seconds
    }
}

