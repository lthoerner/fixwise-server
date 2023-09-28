#![allow(dead_code)]

use phonenumber::PhoneNumber;
use uuid::Uuid;

struct PlaceholderType;

pub struct CustomerID {
    id: Uuid,
}

pub struct Customer {
    id: CustomerID,
    name: Name,
    primary_phone: PhoneNumber,
    alternate_phones: Vec<PhoneNumber>,
    email: EmailAddress,
    contact_method: ContactMethod,
    // TODO: This will be replaced by a referral info type later
    referral: Option<CustomerID>,
}

pub struct Name {
    first: String,
    last: String,
}

pub struct EmailAddress {
    user: String,
    domain: Domain,
}

pub struct Domain {
    name: String,
    tld: String,
}

pub enum ContactMethod {
    Call,
    Text,
    Email,
    None,
}
