pub const USER1: Info = Info {
    id: Id::User1,
    boot_mepc: 0x80100000,
    pmp_idx: 0,
};

pub const USER2: Info = Info {
    id: Id::User2,
    boot_mepc: 0x80200000,
    pmp_idx: 1,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Id {
    User1,
    User2,
}
#[derive(PartialEq, Clone, Copy)]
pub struct Info {
    pub id: Id,
    pub boot_mepc: usize,
    pub pmp_idx: usize,
}
