use std::cmp::Ordering;

#[derive(Debug)]
struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)

        // if self.salary == other.salary {
        //     Some(Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Some(Ordering::Less)
        // } else if self.salary > other.salary {
        //     Some(Ordering::Greater)
        // } else {
        //     None
        // }
    }
}

fn main() {
    let long_commute = Job {
        salary: 100000,
        commute_time: 2,
    };

    let short_commute = Job {
        salary: 75000,
        commute_time: 1,
    };

    println!("{}", long_commute > short_commute);
    println!("{}", long_commute < short_commute);
    println!("{}", long_commute <= short_commute);
    println!("{}", long_commute >= short_commute);
}
