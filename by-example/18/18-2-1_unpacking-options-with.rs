struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Get the area code of this person's work phone number, if one exists
    fn work_phone_area_code(&self) -> Option<u8> {
        // Without the `?` operator, this would require many nested `match` statements
        // This will require more code - try writing it yourself and see which one is easier
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 412333333,
            })
        })
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}