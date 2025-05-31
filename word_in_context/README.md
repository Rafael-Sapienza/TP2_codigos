## Avaliação 01: Exercises in Programming Style
- Alunos:
  - Arthur Menezes Botelho: 231003362
  - Maria Eduarda Araujo Carvalho: 232023969
  - Gustavo Borges Santos 232002557
- Estilo: Kick Forward
- Linguagem: Rust
- Link da vídeo aula: -
- Dependências do projeto: linguagem Rust e ferramenta Cargo

### Para fazer o build e executar o projeto:
O build pode ser realizado com "cargo build". Porém, já são disponibilizados dois scripts de terminal, "linux.sh" e "windows.bat", que já fazem o build e executam logo depois.

A execução do projeto pode ser feita pelos scripts ou com "cargo run" **a partir do diretório principal, word_in_context"**. O resultado irá aparecer tanto na tela quanto no arquivo "output.txt".

Argumentos de execução:
1. Path de input com o texto (default: input de exemplo do Teams)
2. Path de stop words (default: stop words do exemplo do Teams)

Exemplos:
```
sh linux.sh path_input
```
```
windows.bat path_input path_stop_words
```
```
sh linux.sh
```
```
cargo build
cargo run (ver o output pelo arquivo)
```


### Para executar os testes do projeto:
Use o comando `cargo test` para executar automaticamente os testes unitários e de integração.
