pub mod cfg;
pub mod svg_draw;

/// Unit test
#[cfg(test)]
mod tests {
    use libswe_sys::swerust::handler_swe02;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn version() {
        assert_eq!(handler_swe02::version(), "2.08");
    }
}
