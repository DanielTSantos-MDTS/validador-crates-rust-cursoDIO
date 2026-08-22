Como Instalar (Cargo.toml)

''' Crate
[dependencies]
validador_crates_rust_cursoDIO = "0.1.0"

# Busca do crates.io

'''

Como utilizar

''' rust
use std::io;
use validador_crates_rust_cursoDIO::validar_cpf as vd;

fn main() {
println!("Digite um cpf:");

    let mut cpf = String::new();

    io::stdin()
        .read_line(&mut cpf)
        .expect("Falha ao ler a linha");

    let texto_limpo = cpf.trim();

    let validado = vd(texto_limpo);

    if validado {
        println!("O CPF {}, é válido", texto_limpo);
    } else {
        println!("O CPF {}, é inválido", texto_limpo);
    }

}
