pub mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();     //m-s-f is private!
    outermost::inside::inner_function();     //mod inside is private!
    outermost::inside::secret_function();    // "
}