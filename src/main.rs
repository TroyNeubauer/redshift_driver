use anyhow::{Context, Result};
use chrono::{naive::NaiveTime, prelude::*, Utc};
use clap::Parser;
use enterpolation::{
    linear::{Linear, LinearError},
    Curve,
    Generator,
};
use serde::{de, Deserialize, Deserializer};
use std::{process, vec};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "schedule.toml")]
    schedule_file: String,
}

fn naive_time_from_str<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let time = NaiveTime::parse_from_str(&s, "%H:%M")
        .map_err(de::Error::custom)?;

    let seconds = time.second() as f32
        + time.minute() as f32 * 60.0
        + time.hour() as f32 * 60.0 * 60.0;

    Ok(seconds)
}

#[derive(Debug, Deserialize)]
struct Keyframe {
    #[serde(deserialize_with = "naive_time_from_str")]
    time: f32, // Seconds since midnight
    percent: f32,
}

#[derive(Debug, Deserialize)]
struct Toml {
    frames: Vec<Keyframe>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let schedule = std::fs::read_to_string(&args.schedule_file)
        .with_context(|| format!("failed to read schedule file {}", args.schedule_file))?;

    let toml: Toml = toml::from_str(&schedule)?;
    println!("{:#?}", toml);

    let mut times = vec![];
    let mut values = vec![];
    for frame in toml.frames.into_iter() {
        times.push(frame.time);
        values.push(frame.percent);
    }
    let lin = Linear::builder().elements(values).knots(times).build()?;
    let mut a = lin.sample(std::iter::once(200.0f32));
    println!("{:?}", a.next());

    Ok(())
}
