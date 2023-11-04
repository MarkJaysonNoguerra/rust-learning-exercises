mod Normal_use {
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        return fmt::Result::Ok(());
    }

    fn function2() -> io::Result<()> {
        return io::Result::Ok(());
    }
}

mod Aliased_use {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        return Result::Ok(());
    }

    fn function2() -> IoResult<()> {
        return IoResult::Ok(());
    }
}
