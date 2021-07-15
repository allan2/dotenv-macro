use dotenv_codegen::dotenv;

fn main() {
    println!("{}", dotenv!("MSG"));
}
