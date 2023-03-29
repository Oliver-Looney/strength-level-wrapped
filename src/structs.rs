use chrono::NaiveDate;
use crate::enums::WorkoutType;

#[derive(Debug)]
pub(crate) struct Month {
    pub(crate) workouts: Vec<Workout>
}

#[derive(Clone, Debug)]
pub(crate) struct Workout {
    pub(crate) sets: Vec<Set>,
    pub(crate) date: NaiveDate,
    pub(crate) workout_type: WorkoutType
}

#[derive(Clone, Debug)]
pub(crate) struct Set {
    pub(crate) exercise: String,
    pub(crate) date_lifted: NaiveDate,
    pub(crate) weight_kg: f32,
    pub(crate) weight_lb: f32,
    pub(crate) reps: u32,
    pub(crate) body_weight_kg: f32,
    pub(crate) body_weight_lb: f32,
    pub(crate) percentile: f32,
    pub(crate) is_warm_up: bool
}
