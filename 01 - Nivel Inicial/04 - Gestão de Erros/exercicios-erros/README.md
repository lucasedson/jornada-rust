# Exercícios | Gestão de Erros

1. Exercício: Divisão Segura
Escreva uma função que aceite dois números de ponto flutuante como entrada e retorne o resultado da divisão deles. No entanto, verifique se o divisor não é zero usando um Result. Se for zero, retorne um Err com uma mensagem de erro apropriada.

2. Exercício: Encontrar Elemento
Implemente uma função que receba um vetor de inteiros e um número inteiro como parâmetros. A função deve procurar o número no vetor e retornar um Result com o índice onde o número foi encontrado (caso encontrado) ou um Err caso não tenha sido encontrado.

3. Exercício: Conversão para Inteiro
Crie uma função que tente converter uma String em um número inteiro usando o método parse(). A função deve retornar um Result<i32, ParseIntError>. Teste a função com diferentes valores, incluindo um caso em que a conversão falhe.

4. Exercício: Busca de Arquivo
Implemente uma função que tente abrir um arquivo com base em um nome de arquivo fornecido como parâmetro. Se o arquivo for aberto com sucesso, retorne um Result<File, io::Error>. Se não for possível abrir o arquivo, retorne um Err com um erro apropriado.

5. Exercício: Validador de E-mail
Crie uma função que valide um endereço de e-mail. A função deve receber uma String contendo um endereço de e-mail e retornar um Result<(), String>. Se o e-mail for válido, retorne um Ok(()), caso contrário, retorne um Err com uma mensagem de erro.

6. Exercício: Opção de Nome
Implemente uma função que receba um nome como entrada e retorne um Option<String>. Se o nome for "Alice", retorne Some("Olá, Alice!"), caso contrário, retorne None.

7. Exercício: Raiz Quadrada
Crie uma função que calcule a raiz quadrada de um número positivo. Use um Option para retornar Some(raiz) se o número for positivo e a raiz puder ser calculada, ou None se o número for negativo.

8. Exercício: Idade Permitida
Escreva um programa que peça ao usuário para inserir sua idade. Converta a entrada para um número inteiro usando parse() e retorne um Result<u8, String>. Se a idade estiver dentro do intervalo permitido (por exemplo, entre 18 e 65 anos), retorne a idade como Ok, caso contrário, retorne um Err com uma mensagem de erro apropriada.

9. Exercício: Busca de Palavra
Crie uma função que aceite uma string e uma palavra como parâmetros. A função deve procurar a palavra na string e retornar um Option<usize> com o índice onde a palavra foi encontrada (caso encontrado) ou None caso não tenha sido encontrada.

10. Exercício: Verificação de Número Par
Implemente uma função que verifique se um número inteiro é par. A função deve receber um número como parâmetro e retornar um Option<bool>. Se o número for par, retorne Some(true), caso contrário, retorne Some(false).

Lembre-se de que o uso de Result e Option promove uma abordagem segura e explícita para lidar com erros e valores opcionais em Rust. Pratique esses exercícios para aprimorar suas habilidades na manipulação desses tipos e aprofundar sua compreensão de como tratá-los de maneira eficaz.