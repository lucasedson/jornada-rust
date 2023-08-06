# Exercícios | Desenvolvimento Web +  `actix`


1. Exercício: Rotas e Manipuladores
Crie uma aplicação que tenha rotas para exibir informações básicas sobre um usuário, como nome, idade e localização. Use manipuladores para lidar com as solicitações e retorne as informações formatadas em JSON.

2. Exercício: Middlewares
Adicione um middleware à sua aplicação que registre informações sobre cada solicitação recebida, como o método HTTP, a rota e o horário. Isso pode ser útil para fins de monitoramento e depuração.

3. Exercício: Validação de Dados
Crie uma rota para adicionar novas tarefas à lista de tarefas do exemplo anterior. Adicione validações para garantir que os campos obrigatórios, como o título da tarefa, sejam fornecidos.

4. Exercício: Manipulação de Erros
Implemente uma manipulação de erros personalizada para lidar com solicitações inválidas ou rotas inexistentes. Retorne respostas HTTP apropriadas, como "400 Bad Request" ou "404 Not Found".

5. Exercício: Autenticação e Autorização
Adicione autenticação básica à sua aplicação usando um middleware. Crie rotas protegidas que requerem autenticação para acessar e implemente uma verificação de autorização para determinar se um usuário tem permissão para realizar certas operações.

6. Exercício: WebSockets
Crie uma funcionalidade de chat simples usando WebSockets. Os usuários devem poder se conectar ao servidor WebSocket, enviar mensagens e receber mensagens de outros usuários em tempo real.

7. Exercício: Bancos de Dados
Integre sua aplicação com um banco de dados, como SQLite ou PostgreSQL. Crie rotas para listar, criar, atualizar e excluir registros no banco de dados.

8. Exercício: Testes de Unidade e Integração
Escreva testes de unidade e integração para as diferentes partes da sua aplicação, como os manipuladores, validações e lógica de banco de dados. Use ferramentas como actix-test para simular solicitações HTTP nos testes.

9. Exercício: Documentação
Adicione documentação aos seus manipuladores e rotas usando anotações (atributos) e o sistema de documentação do Rust (cargo doc). Isso ajudará outros desenvolvedores (e você mesmo) a entenderem como usar sua aplicação.

10. Exercício: Atualizações em Tempo Real
Crie uma funcionalidade que permita aos usuários receber atualizações em tempo real quando uma nova tarefa for criada. Use WebSockets ou tecnologias de push para enviar notificações aos clientes.

Lembre-se de que o desenvolvimento é uma jornada contínua e a prática é essencial para aprimorar suas habilidades. À medida que você avança nesses exercícios, você ganhará mais confiança no uso do Actix e na criação de aplicações web mais sofisticadas. Além disso, a documentação oficial do Actix (https://actix.rs/docs/) é uma excelente fonte de informações para aprender mais sobre seus recursos e casos de uso avançados.