extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

fn main() {
	let builder = aster::AstBuilder::new();

	let number = builder.item().fn_("number")
		.arg().id("a").ty().i32()
		.arg().id("b").ty().i32()
		.return_().i32()
		.block()
			.expr().add().id("a").id("b");

    println!("{}", syntax::print::pprust::item_to_string(&number));
}
