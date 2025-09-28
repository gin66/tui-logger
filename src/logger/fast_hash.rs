/// A very simple and fast string hash function, based on the Java String.hashCode() algorithm.
pub fn fast_str_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xdeadbeef;
    for b in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u64);
    }
    hash
}
