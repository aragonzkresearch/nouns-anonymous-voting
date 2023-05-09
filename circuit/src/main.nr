use dep::std::hash::poseidon;

fn main(x: Field, out: pub Field) {
  let res = x * x;
  assert(res == out);
}
