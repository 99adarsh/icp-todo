use candid::CandidType;

#[derive(CandidType)]
pub enum Error {
    TodoNotFound,
}
