mod two_d {
    pub mod triangle;
}

struct Cube {
    FrontA: triangle::Triangle,
    FrontB: triangle::Triangle,
    BackA: triangle::Triangle,
    BackB: triangle::Triangle,
    TopA: triangle::Triangle,
    TopB: triangle::Triangle,
    BottomA: triangle::Triangle,
    BottomB: triangle::Triangle,
    LeftA: triangle::Triangle,
    LeftB: triangle::Triangle,
    RightA: triangle::Triangle,
    RightB: triangle::Triangle,
}
