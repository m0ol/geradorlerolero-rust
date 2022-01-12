use std::io;

mod words_find{
    pub fn find_words(words: &Vec<String>, word: &str) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        for w in words{
            if w.contains(word){
                result.push(w.to_string());
            }
        }
        result
    }
}

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
    //if words.contains(&input.trim()) {
    //if words.iter().any(|&i| i == input.trim()) {
    //println!("{}", words[0]);
     for i in words.iter() {
        if words[0].find(input) == input.trim() {
            println!("{}", i);
        }
    //let answer = words[0].find("xar");
    println!("{:?}", answer);
    // } else {
    //   println!("NO");
    //}
    /*if words.contains( input){
        println!("yes");}
    for input in words {
        println!("{}", input);
    }*/
    //println!("{:?}", words);
}
