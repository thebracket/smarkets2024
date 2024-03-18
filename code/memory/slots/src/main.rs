use slotmap::SlotMap;

fn main() {
    let mut sm = SlotMap::new();
    let foo = sm.insert("foo"); // Key generated on insert.
    let bar = sm.insert("bar");
    assert_eq!(sm[foo], "foo");
    assert_eq!(sm[bar], "bar");
}
