# Exercícios | Manipulação de Arquivos e I/O

1. Exercício: Leitura e Impressão Assíncrona
Leia o conteúdo de um arquivo de texto de forma assíncrona usando tokio::fs::read_to_string e imprima-o no console.

2. Exercício: Escrita Assíncrona
Crie um programa que solicite ao usuário que insira uma mensagem e, em seguida, escreva essa mensagem em um arquivo de saída de forma assíncrona usando tokio::fs::write.

3. Exercício: Copiar Arquivos Assincronamente
Leia o conteúdo de um arquivo e copie-o para um novo arquivo de forma assíncrona usando as funções de leitura e escrita assíncronas do tokio::fs.

4. Exercício: Download de Arquivo Assíncrono
Implemente um programa que faça o download de um arquivo da internet de forma assíncrona e o salve localmente usando as operações assíncronas de I/O do tokio.

5. Exercício: Processamento de Dados Assíncrono
Leia uma lista de números de um arquivo de entrada, calcule a soma dos números de forma assíncrona e grave o resultado em um arquivo de saída.

6. Exercício: Leitura Assíncrona de Diretório
Liste todos os arquivos de um diretório de forma assíncrona usando tokio::fs::read_dir.

7. Exercício: Leitura e Filtragem Assíncrona
Leia o conteúdo de vários arquivos de um diretório, filtre os arquivos que contenham uma determinada palavra e imprima os nomes dos arquivos filtrados.

8. Exercício: I/O de Rede Assíncrono
Crie um servidor TCP simples usando tokio::net::TcpListener que aceita conexões assincronamente e responde com uma mensagem.

9. Exercício: Leitura Assíncrona de Entrada Padrão
Leia entradas do usuário de forma assíncrona usando tokio::io::AsyncBufRead.

10. Exercício: Processamento Paralelo de Arquivos
Leia o conteúdo de vários arquivos de um diretório de forma assíncrona, processe-os paralelamente usando tokio::spawn, e imprima os resultados.

Lembre-se de usar o atributo #[tokio::main] para marcar a função main como assíncrona e habilitar o contexto de execução assíncrona do tokio. Esses exercícios ajudarão você a praticar a manipulação de arquivos e I/O usando Async/Await e tokio, permitindo que você desenvolva habilidades na criação de programas eficientes e responsivos que lidam com operações de E/S.