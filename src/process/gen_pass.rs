use rand::seq::SliceRandom;

const UPPER_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CHARS: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER_CHARS: &[u8] = b"123456789";
const SYMBOL_CHARS: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER_CHARS);
        password.push(*UPPER_CHARS.choose(&mut rng).expect("upper char won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER_CHARS);
        password.push(*LOWER_CHARS.choose(&mut rng).expect("lower char won't be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER_CHARS);
        password.push(*NUMBER_CHARS.choose(&mut rng).expect("number char won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL_CHARS);
        password.push(*SYMBOL_CHARS.choose(&mut rng).expect("symbol char won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("password: {}", password);

    let estimate = zxcvbn::zxcvbn(&password, &[]);
    eprintln!("{}", estimate.score());

    Ok(())
}
