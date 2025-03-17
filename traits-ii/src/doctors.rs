use std::clone::Clone;

#[derive(Clone, Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         Self {
//             doctor: self.doctor.clone(),
//             start_time: self.start_time.clone(),
//             end_time: self.end_time.clone(),
//         }
//     }
// }

fn main() {
    let morning_appt = Appointment::new("Dr. Andrews", "07:30", "09:30");
    let mut replacement_appt = morning_appt.clone();
    replacement_appt.doctor = "Dr. Smith".to_string();
    println!("{}", morning_appt.doctor);
    println!("{}", replacement_appt.doctor);
}
