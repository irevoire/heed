use std::error::Error;
use std::{fs, str};
use std::cmp::Ordering;
use std::path::Path;

use heed::types::*;
use heed::{EnvOpenOptions, CustomKeyCmp};

enum StringAsIntCmp {}

// This function takes two strings which represent positive numbers,
// parses them into i32s and compare the parsed value.
// Therefore "-1000" < "-100" must be true even without '0' padding.
impl<'a> CustomKeyCmp<'a> for StringAsIntCmp {
    type Key = &'a str;

    fn compare(a: Self::Key, b: Self::Key) -> Ordering {
        let a: i32 = a.parse().unwrap_or(0);
        let b: i32 = b.parse().unwrap_or(0);
        a.cmp(&b)
    }
}

// In this test we are checking that we can use
// a custom key comparison function at database creation.
fn main() -> Result<(), Box<dyn Error>> {
    let env_path = Path::new("target").join("custom-key-cmp.mdb");

    let _ = fs::remove_dir_all(&env_path);

    fs::create_dir_all(&env_path)?;
    let env = EnvOpenOptions::new()
        .map_size(10 * 1024 * 1024) // 10MB
        .max_dbs(3)
        .open(env_path)?;

    // Here we try to create and then open a database we the same syntax
    // to check that the function signatures are valid.
    env.create_database_with_custom_key_cmp::<Str, Unit, StringAsIntCmp, _>(None)?;
    let db = env.open_database_with_custom_key_cmp::<Str, Unit, StringAsIntCmp, _>(None)?.unwrap();

    let mut wtxn = env.write_txn()?;

    // We fill our database with entries.
    db.put(&mut wtxn, "-100000", &())?;
    db.put(&mut wtxn, "-10000", &())?;
    db.put(&mut wtxn, "-1000", &())?;
    db.put(&mut wtxn, "-100", &())?;
    db.put(&mut wtxn, "100", &())?;

    // We check that the key are in the right order ("-100" < "-1000" < "-10000"...)
    let mut iter = db.iter(&wtxn)?;
    assert_eq!(iter.next().transpose()?, Some(("-100000", ())));
    assert_eq!(iter.next().transpose()?, Some(("-10000", ())));
    assert_eq!(iter.next().transpose()?, Some(("-1000", ())));
    assert_eq!(iter.next().transpose()?, Some(("-100", ())));
    assert_eq!(iter.next().transpose()?, Some(("100", ())));
    drop(iter);

    Ok(())
}
