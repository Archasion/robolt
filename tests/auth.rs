use robolt::Robolt;

#[test]
fn is_authenticated() {
	let client = Robolt::new();
	assert!(!client.is_authenticated());
}
