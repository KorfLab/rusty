
#[derive(Debug)]
struct FastaEntry {
	def: String,
	seq: String,
}


fn main() {
    let def = String::from("foo");
    let seq = String::from("ACGT");
    let e = FastaEntry { def, seq };
    println!("{:?}", e);
    println!("def=>{} seq=>{}\n", e.def, e.seq);
}
