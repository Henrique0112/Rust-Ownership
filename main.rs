use std::io;
#[derive(PartialEq)]
#[derive(Debug)]
enum Permissoes {
  Ler,
  Escrever,
  Executar,
}

fn main() {
  // Criando os arquivos (Nome do arquivo, Conteúdo, Permissões)
  let mut arquivo1: (String, String, Vec<Permissoes>) = (String::from("ola.txt"),
                                                          String::from("ola, mundo!"),
                                                          vec![Permissoes::Ler],);
  let mut arquivo2: (String, String, Vec<Permissoes>) = (String::from("main.rs"),
                                                          String::from("fn main() { println!('Hello, world'); }"),
                                                          vec![Permissoes::Escrever],);

  // Armazenando os arquivos dentro de um Vetor
  let armazena_arquivo: Vec<&(String, String, Vec<Permissoes>)> = vec![&arquivo1,&arquivo2];

  // Escolher a opção!
  for i in 0..armazena_arquivo.len(){
    println!("{:?}", armazena_arquivo[i]);
  }
  println!();

  println!("1.Ler\n2.Escrever\n3.Executar\n4.Sair");
  println!("Escolha uma das opções acima:");

  let mut escolha = String::new();
  io::stdin().read_line(&mut escolha).expect("Falha ao ler entrada das opções!");
  escolha = escolha.trim().to_string();

  if escolha == "1" || escolha == "2" || escolha == "3" {

    println!("Digite o nome do arquivo:");

      let mut escolhe_arquivo = String::new();
      io::stdin().read_line(&mut escolhe_arquivo).expect("Falha ao ler entrada da escolha!");
      escolhe_arquivo = escolhe_arquivo.trim().to_string();

      match escolhe_arquivo.as_str() {
        "ola.txt" => match escolha.as_str() {
          "1" => {
            if arquivo1.2.contains(&Permissoes::Ler){
              println!("{}", arquivo1.1);
            } 
            else {
              println!("Você não tem permissão para ler esse arquivo!");
            }
          }
          "2" => {
            if arquivo1.2.contains(&Permissoes::Escrever) {
              println!("Digite o novo conteúdo para o arquivo:");
              let mut novo_conteudo = String::new();
              io::stdin().read_line(&mut novo_conteudo).expect("Falha ao ler entrada do conteúdo!");
              arquivo1.1 = novo_conteudo.trim().to_string();
            } 
            else {
              println!("Você não tem permissão para escrever nesse arquivo!");
            }
          }
          "3" => {
            if arquivo1.2.contains(&Permissoes::Executar) {
              println!("Executando o arquivo ola.txt...");
              println!("{}", arquivo1.1	);
            } 
            else {
              println!("Você não tem permissão para executar esse arquivo!");
            }
          }
          _ => println!("Opção inválida!")
          }
        "main.rs" => match escolha.as_str() {
          "1" => {
            if arquivo2.2.contains(&Permissoes::Ler) {
              println!("{}", arquivo2.1);
            } 
            else {
              println!("Você não tem permissão para ler esse arquivo!");
            }
          }
          "2" => {
            if arquivo2.2.contains(&Permissoes::Escrever) {
              println!("Digite o novo conteúdo para o arquivo:");
              let mut novo_conteudo = String::new();
              io::stdin().read_line(&mut novo_conteudo).expect("Falha ao ler entrada do conteúdo!");
              arquivo2.1 = novo_conteudo.trim().to_string();
            } 
            else {
              println!("Você não tem permissão para escrever nesse arquivo!");
            }
          }
          "3" => {
            if arquivo2.2.contains(&Permissoes::Executar) {
              println!("Executando o arquivo main.rs...");
            } 
            else {
              println!("Você não tem permissão para executar esse arquivo!");
            }
          }
          _ => println!("Opção inválida!")
          }
        _ => println!("Esse arquivo não existe!")
      }
  }
  else {
    println!("Você saiu!");
  }
}