fn try_me() {
    outer::public();
    // outer::private();
    // outer::inner::public();
    // outer::inner::private();
}

mod outer {
    pub fn public() {}
    fn private() {}

    mod inner {
        pub fn public() {}
        fn private() {}
    }
}
