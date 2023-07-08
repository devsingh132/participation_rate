pub struct ValidatorData {
    pub pubkey : String,
    pub is_attested : bool
}

impl ValidatorData {
    pub fn new(public_key : String, attested : bool) -> Self {
        ValidatorData { pubkey: public_key, is_attested: attested }
    }
}