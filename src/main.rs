#![allow(dead_code)]

use phonenumber::PhoneNumber;

fn main() {
    todo!()
}

struct PlaceholderType;

struct Customer {
    name: Name,
    primary_phone: PhoneNumber,
    alternate_phones: Vec<PhoneNumber>,
    email: EmailAddress,
    contact_method: ContactMethod,
    // TODO: This will be replaced by a refferal info type later
    referral: Option<Box<Customer>>,
}

struct Name {
    first: String,
    last: String,
}

struct EmailAddress {
    user: String,
    domain: Domain,
}

struct Domain {
    name: String,
    tld: String,
}

enum ContactMethod {
    Call,
    Text,
    Email,
    None,
}
