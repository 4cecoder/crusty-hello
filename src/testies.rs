#[test]
pub fn test_yo_gangsta() -> std::io::Result<()> {
    let state = setup_dat_thang()?; // expected to succeed
    do_dat_thang(&state)?;          // expected to succeed
    Ok(())
}