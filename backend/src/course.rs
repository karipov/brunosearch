use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Course {
    pub department_full: String,
    pub department_short: String,
    pub code: String,
    pub title: String,
    pub professor: String,
    pub time: String,
    pub description: String,
    pub writ: bool,
    pub soph: bool,
    pub fys: bool,
    pub rpp: bool,
    pub embedding: Option<Vec<f32>>,
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = format!(
            "Department: {} - {}\n\
            Course code: {}\n\
            Title: {}\n\
            Professor: {}\n\
            Time: {}\n\
            Description: {}",
            self.department_full,
            self.department_short,
            self.code,
            self.title,
            self.professor,
            self.time,
            self.description
        );

        if self.writ {
            output.push_str("\nThis course satsifies the WRIT / writing requirement.");
        }
        if self.soph {
            output.push_str("\nThis course is a SOPH / sophomore seminar.");
        }
        if self.fys {
            output.push_str("\nThis course is a FYS / first-year seminar.");
        }
        if self.rpp {
            output.push_str("\nThis course is under the RPP / Race Power Privilege category.");
        }

        write!(f, "{}", output)
    }
}
