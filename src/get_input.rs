use std::error::Error;
use std::fs::File;
use csv::{Reader, ReaderBuilder};
use chrono::{Utc, Datelike, TimeZone, NaiveDate};

use crate::structs::{Month, Workout, Set};
use crate::enums::WorkoutType;
use crate::get_input::WorkoutType::{Legs, Other, Pull, Push};

fn get_reader() -> Result<Reader<File>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("oliver45649 2023-03-28 164733.csv")?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    Ok(reader)
}

pub(crate) fn get_input() -> Result<Month, Box<dyn Error>> {
    // Get the date for the first day of the previous Month
    let prev_month = Utc.ymd(Utc::now().year(), Utc::now().month() - 1, 1);

    let mut reader = get_reader()?;
    let mut previous_month_data = Month {
        workouts: vec![]
    };
    let mut current_workout : Workout= Workout{
        sets: vec![],
        date: NaiveDate::parse_from_str("2001-01-01", "%Y-%m-%d")?,
        workout_type: Other,
    };
    for result in reader.records() {
        let record = result?;
        let date_lifted = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
        if date_lifted.year() == prev_month.year() && date_lifted.month() == prev_month.month() {
            current_workout.date = date_lifted;
            break;
        }
    }

    // Loop through each row in the CSV file
    for result in reader.records() {
        let record = result?;
        // Parse the date lifted from the CSV record
        let date_lifted = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;

        // Check if the date is in the previous Month
        if date_lifted.year() == prev_month.year() && date_lifted.month() == prev_month.month() {
            if date_lifted.day() != current_workout.date.day() {
                previous_month_data.workouts.insert(0, current_workout.clone());
                current_workout = Workout{
                    sets: vec![],
                    date: date_lifted,
                    workout_type: Other,
                }
            }

            // Read the exercise, weight, reps, etc. from the record
            let record_as_set: Set = Set {
                exercise: record[1].to_string().clone(),
                date_lifted,
                weight_kg: record[2].parse::<f32>()?,
                weight_lb: record[3].parse::<f32>()?,
                reps: record[4].parse::<u32>()?,
                body_weight_kg: record[5].parse::<f32>()?,
                body_weight_lb: record[6].parse::<f32>()?,
                percentile: record[7].parse::<f32>()?,
                is_warm_up: &record[8] == "1",
            };

            if record_as_set.exercise == "Bench Press" {
                current_workout.workout_type = Push
            } else if record_as_set.exercise == "Pull Ups" {
                current_workout.workout_type = Pull
            } else if record_as_set.exercise == "Squat" {
                current_workout.workout_type = Legs
            }
            current_workout.sets.insert(0,record_as_set);
        }
    }
    previous_month_data.workouts.insert(0,current_workout);
    Ok(previous_month_data)
}
