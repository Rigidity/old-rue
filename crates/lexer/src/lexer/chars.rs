use unicode_xid::UnicodeXID;

pub fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_xid_start()
}

pub fn is_ident_continue(c: char) -> bool {
    c.is_xid_continue()
}
