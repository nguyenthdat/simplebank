use rand::Rng;

pub fn random_int(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let s: String = (0..len)
        .map(|_| {
            let c: char = rng.gen_range(b'a'..b'z') as char;
            c
        })
        .collect();
    s
}

pub fn random_owner() -> String {
    random_string(6)
}

pub fn random_money() -> i64 {
    random_int(0, 1000)
}

pub fn random_currency() -> String {
    let currencies = vec!["USD", "EUR", "JPY", "CNY", "KRW"];
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..currencies.len());
    currencies[idx].to_string()
}
