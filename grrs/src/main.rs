use std::io;

fn main() {
    let mut words = vec![
        "Hermético",
        "Definir",
        "Esmagamento",
        "Anexar",
        "Luta",
        "Esportista",
        "Azul",
        "Domingo",
        "Contagioso",
        "Fotos",
        "Cachos",
        "gostar",
        "festa",
        "dados",
        "questão",
        "especial",
        "solta",
        "receber",
        "peça",
        "mãe",
        "papel",
        "início",
        "enquanto",
        "cama",
        "três",
        "possivelmente",
        "acreditam",
        "mundo",
        "personagem",
        "descansar",
        "voto",
        "financeiro",
        "acima",
        "comércio",
        "Beira",
        "bilhão",
        "três",
        "o suficiente",
        "ar",
        "passar",
        "Contudo",
        "seu",
        "americano",
        "desejo",
        "objetivo",
        "ir",
        "bom",
        "ele",
        "Função",
        "conjunto",
        "acertar",
        "viver",
        "voto",
        "evita",
        "sentar",
        "Como",
        "direção",
        "fonte",
        "acredita",
    ];
    words.sort();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    for i in 0..words.len() {
        if words[i].find(&input[..1]) != None {
            if words[i].find(&input[1..2]) != None {
                if words[i].find(&input[2..3]) != None {
                    println!("{:?} encontrada na posição {} ", words[i], i);
                }
            }
        }
    }
}
