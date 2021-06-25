use lmdb_sys as ffi;

pub use ffi::mdb_filehandle_t;
pub use ffi::MDB_cursor;
pub use ffi::MDB_dbi;
pub use ffi::MDB_env;
pub use ffi::MDB_txn;

pub use ffi::MDB_APPEND;
pub use ffi::MDB_CP_COMPACT;
pub use ffi::MDB_CREATE;
pub use ffi::MDB_CURRENT;
pub use ffi::MDB_RDONLY;

pub use ffi::mdb_env_close;
pub use ffi::mdb_env_copyfd2 as mdb_env_copy2fd;
pub use ffi::mdb_env_create;
pub use ffi::mdb_env_open;
pub use ffi::mdb_env_set_mapsize;
pub use ffi::mdb_env_set_maxdbs;
pub use ffi::mdb_env_set_maxreaders;
pub use ffi::mdb_env_sync;

pub use ffi::mdb_drop;

pub unsafe fn mdb_dbi_open(
    txn: *mut ffi::MDB_txn,
    name: *const ::libc::c_char,
    flags: ::libc::c_uint,
    dbi: *mut ffi::MDB_dbi,
) -> ::libc::c_int {
    let result = ffi::mdb_dbi_open(txn, name, flags, dbi);
    eprintln!("mdb_dbi_open({:?} {:?} {:?} {:?}) -> {:?}", *txn, std::ffi::CStr::from_ptr(name), flags, *dbi, result);
    result
}

use bstr::ByteSlice;

pub unsafe fn mdb_get(
    txn: *mut ffi::MDB_txn,
    dbi: ffi::MDB_dbi,
    key: *mut ffi::MDB_val,
    data: *mut ffi::MDB_val,
) -> ::libc::c_int {
    let result = ffi::mdb_get(txn, dbi, key, data);
    let data = if result == 0 {
        Some(from_val_ref(data).as_bstr())
    } else {
        None
    };
    let key = from_val_ref(key);
    if key == b"toi" {
        eprintln!("wow");
    }
    eprintln!("mdb_get({:?} {:?} {:?} {:?}) -> {:?}", *txn, dbi, key.as_bstr(), data, result);
    result
}

pub unsafe fn mdb_put(
    txn: *mut ffi::MDB_txn,
    dbi: ffi::MDB_dbi,
    key: *mut ffi::MDB_val,
    data: *mut ffi::MDB_val,
    flags: ::libc::c_uint,
) -> ::libc::c_int {
    let result = ffi::mdb_put(txn, dbi, key, data, flags);
    let data = if result == 0 {
        Some(from_val_ref(data).as_bstr())
    } else {
        None
    };
    let key = from_val_ref(key);
    if key == b"toi" {
        eprintln!("wow");
    }
    eprintln!("mdb_put({:?} {:?} {:?} {:?} {:?}) -> {:?}", *txn, dbi, key.as_bstr(), data, flags, result);
    result
}

pub unsafe fn mdb_del(
    txn: *mut ffi::MDB_txn,
    dbi: ffi::MDB_dbi,
    key: *mut ffi::MDB_val,
    data: *mut ffi::MDB_val,
) -> ::libc::c_int {
    let result = ffi::mdb_del(txn, dbi, key, data);
    eprintln!("mdb_del({:?} {:?} {:?}) -> {:?}", *txn, dbi, from_val_ref(key).as_bstr(), result);
    result
}

pub use ffi::mdb_txn_abort;

pub unsafe fn mdb_txn_begin(
    env: *mut ffi::MDB_env,
    parent: *mut ffi::MDB_txn,
    flags: ::libc::c_uint,
    txn: *mut *mut ffi::MDB_txn,
) -> ::libc::c_int {
    let result = ffi::mdb_txn_begin(env, parent, flags, txn);
    eprintln!("mdb_txn_begin({:?} {:?} {:?} {:?}) -> {:?}", env, parent, flags, txn, result);
    result
}

pub unsafe fn mdb_txn_commit(txn: *mut ffi::MDB_txn) -> ::libc::c_int {
    let result = ffi::mdb_txn_commit(txn);
    eprintln!("mdb_txn_commit({:?}) -> {:?}", txn, result);
    result
}

pub unsafe fn mdb_cursor_put(
    cursor: *mut ffi::MDB_cursor,
    key: *mut ffi::MDB_val,
    data: *mut ffi::MDB_val,
    flags: ::libc::c_uint,
) -> ::libc::c_int {

    if from_val_ref(key) == b"and" {
        eprintln!("wow");
    }
    let result = ffi::mdb_cursor_put(cursor, key, data, flags);
    let key = from_val_ref(key);
    if key == b"toi" {
        eprintln!("wow");
    }
    eprintln!("mdb_cursor_put({:?} {:?} {:?} {:?}) -> {:?}", cursor, key.as_bstr(), from_val_ref(data).as_bstr(), flags, result);
    result
}
pub unsafe fn mdb_cursor_del(cursor: *mut ffi::MDB_cursor, flags: ::libc::c_uint) -> ::libc::c_int {
    let result = ffi::mdb_cursor_del(cursor, flags);
    eprintln!("mdb_cursor_del({:?} {:?}) -> {:?}", cursor, flags, result);
    result
}

pub unsafe fn mdb_cursor_get(
    cursor: *mut ffi::MDB_cursor,
    key: *mut ffi::MDB_val,
    data: *mut ffi::MDB_val,
    op: ffi::MDB_cursor_op,
) -> ::libc::c_int {
    let result = ffi::mdb_cursor_get(cursor, key, data, op);
    let data = if result == 0 {
        Some(from_val_ref(data).as_bstr())
    } else {
        None
    };
    let key = if result == 0 {
        let key = from_val_ref(key);
        if key == b"toi" {
            eprintln!("wow");
        }
        Some(key.as_bstr())
    } else {
        None
    };
    eprintln!("mdb_cursor_get({:?} {:?} {:?} {:?}) -> {:?}", cursor, key, data, op, result);
    result
}

pub use ffi::mdb_cursor_close;
pub use ffi::mdb_cursor_open;

pub mod cursor_op {
    use super::ffi::{self, MDB_cursor_op};

    pub const MDB_FIRST: MDB_cursor_op = ffi::MDB_FIRST;
    pub const MDB_LAST: MDB_cursor_op = ffi::MDB_LAST;
    pub const MDB_SET_RANGE: MDB_cursor_op = ffi::MDB_SET_RANGE;
    pub const MDB_PREV: MDB_cursor_op = ffi::MDB_PREV;
    pub const MDB_NEXT: MDB_cursor_op = ffi::MDB_NEXT;
    pub const MDB_GET_CURRENT: MDB_cursor_op = ffi::MDB_GET_CURRENT;
}

pub unsafe fn into_val(value: &[u8]) -> ffi::MDB_val {
    ffi::MDB_val {
        mv_data: value.as_ptr() as *mut libc::c_void,
        mv_size: value.len(),
    }
}

pub unsafe fn from_val<'a>(value: ffi::MDB_val) -> &'a [u8] {
    std::slice::from_raw_parts(value.mv_data as *const u8, value.mv_size)
}

pub unsafe fn from_val_ref<'a>(value: *mut ffi::MDB_val) -> &'a [u8] {
    let mdb = ffi::MDB_val {
        mv_data: (*value).mv_data,
        mv_size: (*value).mv_size,
    };
    from_val(mdb)
}
