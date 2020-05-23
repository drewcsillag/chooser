use chooser;

#[test]
fn basic_works() {
    let mut v = vec![];
    let choices = vec![0, 1];
    chooser::run_choices(|c: &mut chooser::Chooser| {
        v.push(format!(
            "{0} {1} {2}",
            c.choose(&choices),
            c.choose(&choices),
            c.choose(&choices)
        ))
    });
    assert_eq!(
        vec!["0 0 0", "0 0 1", "0 1 0", "0 1 1", "1 0 0", "1 0 1", "1 1 0", "1 1 1"],
        v
    );
}
