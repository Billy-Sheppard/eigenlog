use super::*;

#[cfg(feature = "sled")]
mod sled_impl;

#[cfg(feature = "nebari")]
mod nebari_impl;

#[cfg(feature = "rusqlite")]
mod rusqlite_impl;

pub trait Storage: Clone + Send {
    fn submit(&self, host: &Host, app: &App, level: Level, log_batch: LogBatch) -> Result<()>;

    fn query(&self, params: QueryParams) -> Result<Vec<QueryResponse>>;

    fn detail(&self, host: &Host, app: &App, level: Level) -> Result<LogTreeDetail>;

    fn info(&self) -> Result<Vec<result::Result<LogTreeInfo, ParseLogTreeInfoError>>>;

    fn flush(&self, host: &Host, app: &App) -> Result<()>;
}

fn filter_with_option<T: AsRef<str>>(input: &T, filter: &Option<T>) -> bool {
    filter
        .as_ref()
        .map(|f| input.as_ref().contains(f.as_ref()))
        .unwrap_or(true)
}

fn ulid_floor(input: ulid::Ulid) -> u128 {
    let mut base = u128::from(input).to_be_bytes();

    for i in base.iter_mut().skip(6) {
        *i = u8::MIN;
    }

    u128::from_be_bytes(base)
}

fn ulid_ceiling(input: ulid::Ulid) -> u128 {
    let mut base = u128::from(input).to_be_bytes();

    for i in base.iter_mut().skip(6) {
        *i = u8::MAX;
    }

    u128::from_be_bytes(base)
}

fn slice_be_to_u128(slice: &[u8]) -> crate::Result<u128> {
    let mut bytes = [0; 16];

    if slice.len() != 16 {
        return Err(crate::Error::InvalidLengthBytesForUlid(slice.len()));
    }

    for (i, b) in slice.iter().enumerate() {
        bytes[i] = *b;
    }

    Ok(u128::from_be_bytes(bytes))
}
