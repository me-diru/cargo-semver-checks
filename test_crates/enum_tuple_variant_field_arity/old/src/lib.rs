// Should not affect since the enum is not public
enum NotPublicEnum {
    TupleVariant(i64, u8),
}

// basic test case of public enum tuple variant with new field added
pub enum EnumTupleFieldAdded {
    TupleVariantWithPublicFieldAdded(i64, u8),
}

// basic test case of public enum tuple variant with new field added
pub enum EnumTupleFieldDeleted {
    TupleVariantWithPublicFieldDeleted(i64, u8),
}

