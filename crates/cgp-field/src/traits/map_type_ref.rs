pub trait MapTypeRef {
    type Map<'a, T: 'a>: 'a;
}
