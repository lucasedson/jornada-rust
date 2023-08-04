# Exercicios | Lifetimes

1. Exercício: Contador de Palavras
Crie uma função chamada contar_palavras que aceite uma referência a uma string e retorne o número de palavras na string. Use lifetimes para garantir que a referência seja válida durante a execução da função.

2. Exercício: Estrutura de Nota
Implemente uma struct chamada Nota<'a> que represente uma nota musical com um nome (referência a string) e uma duração (número inteiro). Crie um método exibir que imprima a nota.

3. Exercício: Banco de Dados de Produtos
Crie uma estrutura Produto que represente um produto com um nome (referência a string) e um preço (número de ponto flutuante). Implemente um método exibir para a estrutura que imprima as informações do produto.

4. Exercício: Sistema de Mensagens
Implemente um sistema simples de mensagens usando structs. Crie uma struct Mensagem<'a> que contenha um remetente (referência a string) e um conteúdo (referência a string). Crie um método exibir para a struct que imprima a mensagem.

5. Exercício: Fila Circular
Crie uma struct chamada FilaCircular<'a, T> que represente uma fila circular de elementos genéricos. A fila deve ser implementada como um vetor circular. Implemente métodos para enfileirar e desenfileirar elementos, bem como um método para exibir os elementos da fila.

6. Exercício: Estrutura de Tarefa
Crie uma struct chamada Tarefa<'a> que represente uma tarefa com um título (referência a string) e uma descrição (referência a string). Implemente um método exibir para a struct que imprima os detalhes da tarefa.

7. Exercício: Estrutura de Conta Bancária
Implemente uma estrutura ContaBancaria<'a> que represente uma conta bancária com um titular (referência a string) e um saldo (número de ponto flutuante). Crie métodos para depositar e sacar dinheiro da conta, garantindo que o saldo não se torne negativo.

8. Exercício: Conversor de Temperatura
Crie uma função chamada converter_temp que aceite uma referência a um valor numérico e uma unidade de temperatura (referência a string, "C" para Celsius ou "F" para Fahrenheit) e retorne o valor convertido para a outra unidade. Use lifetimes para garantir que a referência seja válida durante a execução da função.

9. Exercício: Playlist de Músicas
Crie uma estrutura Musica<'a> que represente uma música com um título (referência a string) e um artista (referência a string). Implemente um método exibir para a struct que imprima os detalhes da música.

10. Exercício: Calculadora Genérica
Implemente uma calculadora genérica que aceite dois valores (genéricos) e um operador (referência a string, "+", "-", "*" ou "/") como entrada. A calculadora deve realizar a operação especificada nos valores e retornar o resultado. Use lifetimes para garantir a validade das referências.

Lifetimes são essenciais para garantir que as referências sejam válidas durante toda a execução de funções, métodos e structs em Rust. Esses exercícios ajudarão você a praticar e aprofundar seu entendimento sobre como usar lifetimes de maneira eficaz e segura.