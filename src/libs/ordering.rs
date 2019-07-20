#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

#[derive(Debug)]
pub enum Shape {
    Sphere { center: i32, radius: i32 },
}

#[derive(Debug)]
pub struct DifferentialEquation {}
#[derive(Debug)]
pub struct EarlyModernisPoem {}

#[derive(Debug)]
pub enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernisPoem,
    },
}

use std::collections::HashMap;
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl TimeUnit {
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

impl RoughTime {
    pub fn test() {
        let past = RoughTime::InThePast(TimeUnit::Years, 3);
        println!("{:?}", past);
    }
}

impl Shape {
    pub fn test() {
        let unit_sphere = Shape::Sphere {
            center: 1,
            radius: 1,
        };
        println!("{:?}", unit_sphere);
    }
}

impl RelationshipStatus {
    pub fn test() {
        let r = RelationshipStatus::Single;
        println!("{:?}", r);
    }
}

pub fn test() {
    let rt = RoughTime::InTheFuture(TimeUnit::Years, 100);
    let mat = match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    };
    println!("{}", mat);
}
