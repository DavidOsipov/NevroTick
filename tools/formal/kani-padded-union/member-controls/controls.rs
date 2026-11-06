//! Narrow post-build MEMBER controls; not general Kani or product qualification.
//! Every union starts with a byte-array field covering its complete storage.

#[repr(C)]
union Padded {
    storage: [u8; 12],
    array: [u8; 10],
    scalar: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Pair {
    head: u32,
    tail: u32,
}

#[repr(C)]
union Nested {
    storage: [u8; 12],
    pair: Pair,
}

#[repr(C)]
union EqualSize {
    storage: [u8; 4],
    scalar: u32,
}

/// Independent bytes cover all 96 input bits; no array generator or copy loop.
fn bytes12() -> [u8; 12] {
    [kani::any(), kani::any(), kani::any(), kani::any(),
     kani::any(), kani::any(), kani::any(), kani::any(),
     kani::any(), kani::any(), kani::any(), kani::any()]
}

/// Independent little-endian oracle for the fixed x86_64 Linux target.
fn word(a: u8, b: u8, c: u8, d: u8) -> u32 {
    u32::from(a) | (u32::from(b) << 8) | (u32::from(c) << 16) | (u32::from(d) << 24)
}

#[kani::proof]
fn padded_array_read() {
    let bytes = bytes12();
    let index = usize::from(kani::any::<u8>() % 10);
    let value = Padded { storage: bytes };
    // SAFETY: storage initializes all 12 union bytes. Every u8 representation
    // is valid; index is in 0..10, and the array field occupies the first 10 bytes.
    let actual = unsafe { value.array[index] };
    assert!(actual == bytes[index], "padded array selected byte retention");
    kani::cover!(index == 0 && actual == 0, "padded array zero first byte");
    kani::cover!(index == 9 && actual == u8::MAX, "padded array maximum last byte");
}

#[kani::proof]
fn padded_scalar_read() {
    let bytes = bytes12();
    let value = Padded { storage: bytes };
    let expected = word(bytes[0], bytes[1], bytes[2], bytes[3]);
    // SAFETY: the complete union is initialized, aligned to u32, and every
    // four-byte u32 representation is valid. The selected target is little endian.
    let actual = unsafe { value.scalar };
    assert!(actual == expected, "padded scalar exact retention");
    kani::cover!(actual == 0, "padded scalar zero");
    kani::cover!(actual == u32::MAX, "padded scalar maximum");
}

#[kani::proof]
fn initialized_union_write_read() {
    let first = kani::any::<u32>();
    let second = kani::any::<u32>();
    let mut value = Padded { storage: [0; 12] };
    value.scalar = first;
    // SAFETY: scalar was initialized by the preceding assignment. No reference
    // to uninitialized storage is formed, and every u32 bit pattern is valid.
    let first_read = unsafe { value.scalar };
    assert!(first_read == first, "first initialized union write retained");
    value.scalar = second;
    // SAFETY: scalar was initialized by the second assignment.
    let second_read = unsafe { value.scalar };
    assert!(second_read == second, "second initialized union write replaces first");
    kani::cover!(first == 0 && second == u32::MAX && second_read == second,
        "initialized writes zero to maximum");
    kani::cover!(first == u32::MAX && second == 0 && second_read == second,
        "initialized writes maximum to zero");
}

#[kani::proof]
fn raw_address_write_read() {
    let bytes = bytes12();
    let replacement = kani::any::<u32>();
    let mut value = Padded { storage: bytes };
    let base = core::ptr::addr_of_mut!(value);
    // SAFETY: value is live and fully initialized. This macro makes a raw
    // pointer, not a reference; the scalar field has offset zero and u32 alignment.
    let pointer = unsafe { core::ptr::addr_of_mut!((*base).scalar) };
    assert!(pointer.cast::<u8>() == base.cast::<u8>(), "padded field address equals union base");
    // SAFETY: pointer designates the initialized scalar bytes of live value.
    let before = unsafe { pointer.read() };
    assert!(before == word(bytes[0], bytes[1], bytes[2], bytes[3]),
        "raw pointer reads initialized scalar");
    // SAFETY: pointer remains aligned and within live value, and there are no
    // live references or competing accesses. Every replacement bit pattern is valid.
    unsafe { pointer.write(replacement) };
    // SAFETY: the preceding write initializes the scalar field in this object.
    let through_union = unsafe { (*base).scalar };
    // SAFETY: the same pointer still designates the initialized scalar field.
    let through_pointer = unsafe { pointer.read() };
    assert!(through_union == replacement, "raw write is visible through union field");
    assert!(through_pointer == replacement, "raw write is visible through pointer");
    kani::cover!(replacement == 0 && through_union == 0, "raw pointer zero write");
    kani::cover!(replacement == u32::MAX && through_pointer == u32::MAX,
        "raw pointer maximum write");
}

#[kani::proof]
fn nested_payload_field() {
    let bytes = bytes12();
    let replacement = kani::any::<u32>();
    let mut value = Nested { storage: bytes };
    let expected_head = word(bytes[0], bytes[1], bytes[2], bytes[3]);
    let expected_tail = word(bytes[4], bytes[5], bytes[6], bytes[7]);
    // SAFETY: storage initializes all union bytes. Pair has two consecutive
    // u32 fields, no internal padding, and every representation is valid.
    let head = unsafe { value.pair.head };
    // SAFETY: the same initialized Pair representation covers tail at offset four.
    let tail = unsafe { value.pair.tail };
    assert!(head == expected_head, "nested payload head retention");
    assert!(tail == expected_tail, "nested payload tail retention");
    // SAFETY: this writes the initialized tail field; the union object stays live.
    unsafe { value.pair.tail = replacement };
    // SAFETY: both scalar fields remain initialized; no references were created.
    let head_after = unsafe { value.pair.head };
    // SAFETY: tail was initialized by the preceding write.
    let tail_after = unsafe { value.pair.tail };
    assert!(head_after == expected_head, "nested write preserves neighboring head");
    assert!(tail_after == replacement, "nested payload write retention");
    kani::cover!(replacement == 0 && tail_after == 0, "nested payload zero write");
    kani::cover!(replacement == u32::MAX && tail_after == u32::MAX,
        "nested payload maximum write");
}

#[kani::proof]
fn equal_size_neighbor() {
    let bytes: [u8; 4] = [kani::any(), kani::any(), kani::any(), kani::any()];
    let value = EqualSize { storage: bytes };
    // SAFETY: all four union bytes are initialized and all u32 patterns are valid.
    let actual = unsafe { value.scalar };
    assert!(actual == word(bytes[0], bytes[1], bytes[2], bytes[3]),
        "equal-size scalar exact retention");
    kani::cover!(actual == 0, "equal-size zero");
    kani::cover!(actual == u32::MAX, "equal-size maximum");
}

#[kani::proof]
fn false_retention_control() {
    let bytes = bytes12();
    let value = Padded { storage: bytes };
    let expected = word(bytes[0], bytes[1], bytes[2], bytes[3]);
    // SAFETY: complete initialized union storage, valid u32 representation.
    let actual = unsafe { value.scalar };
    assert!(actual == expected, "negative control retains healthy scalar predicate");
    assert!(actual != expected, "deliberately false padded retention");
}
