

com::interfaces! {
    #[uuid("00000000-0000-0000-A731-00A0C9082637")]
    pub unsafe interface IScript : $IDispatch {
        pub fn eval(&self) -> com::sys::HRESULT;
    }
}

com::class! {
    pub class Script : IScript($IDispatch)
    {}

    impl IScript for Script {
        pub fn eval(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }
}

