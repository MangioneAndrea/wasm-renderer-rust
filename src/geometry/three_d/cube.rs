mod two_d {
    pub mod triangle;
}

struct Cube<T> {
    FrontA: triangle::Triangle<T>,
    FrontB: triangle::Triangle<T>,
    BackA: triangle::Triangle<T>,
    BackB: triangle::Triangle<T>,
    TopA: triangle::Triangle<T>,
    TopB: triangle::Triangle<T>,
    BottomA: triangle::Triangle<T>,
    BottomB: triangle::Triangle<T>,
    LeftA: triangle::Triangle<T>,
    LeftB: triangle::Triangle<T>,
    RightA: triangle::Triangle<T>,
    RightB: triangle::Triangle<T>,
}
