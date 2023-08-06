# Desenvolvimento Web com Actix

O Actix é um framework de desenvolvimento web em Rust que permite criar aplicações web rápidas, seguras e escaláveis. Ele é projetado para aproveitar as vantagens de Rust, como segurança, alto desempenho e sistema de tipos robusto, enquanto oferece uma abstração poderosa para o desenvolvimento de aplicações web modernas. O Actix é baseado no modelo de atores, o que o torna adequado para construir sistemas concorrentes e paralelos.

Aqui estão alguns aspectos e recursos importantes do desenvolvimento de aplicações web utilizando o framework Actix:

1. Arquitetura de Ator:
O Actix utiliza uma arquitetura de ator, na qual os atores são unidades independentes de processamento que se comunicam entre si por meio de mensagens. Isso permite a construção de sistemas altamente concorrentes e eficientes, onde as partes da aplicação podem ser divididas em atores isolados.

2. Roteamento e Manipuladores:
O roteamento em Actix é feito por meio de manipuladores (handlers), que são funções que lidam com as solicitações HTTP. Isso permite mapear URLs para funções específicas que realizam a lógica da aplicação.

3. Suporte a Async/Await:
O Actix tira proveito das capacidades assíncronas do Rust e oferece suporte completo a Async/Await para lidar com operações de I/O não bloqueantes, o que é essencial para o desempenho de aplicações web.

4. Middleware:
O Actix permite a criação de middlewares, que são componentes que interceptam as solicitações e respostas HTTP antes de serem processadas pelos manipuladores. Isso é útil para adicionar funcionalidades comuns, como autenticação, controle de acesso, logging, entre outros.

5. WebSockets:
O Actix suporta comunicação em tempo real por meio de WebSockets, permitindo a construção de aplicações interativas e em tempo real.

6. Bancos de Dados:
O framework suporta integração com bancos de dados populares, como PostgreSQL, MySQL e SQLite, permitindo a persistência de dados em suas aplicações.

7. Autenticação e Autorização:
O Actix oferece suporte para implementar autenticação e autorização em suas aplicações, com várias opções de estratégias e bibliotecas de autenticação.

8. Templates:
Embora o Actix não possua um sistema de templates incorporado, ele permite a integração com engines de templates populares, como Handlebars ou tera.

9. Desempenho e Escalabilidade:
Devido à sua arquitetura baseada em atores e ao uso de características assíncronas do Rust, o Actix é conhecido por oferecer alto desempenho e escalabilidade para aplicações web.

10. Comunidade Ativa:
A comunidade do Actix é ativa e em crescimento, o que significa que há suporte contínuo, atualizações e recursos disponíveis para os desenvolvedores.


## Projeto Minimo:

### Passo 1: Configuração do Ambiente

Certifique-se de que você tem o ambiente de desenvolvimento Rust configurado em sua máquina. Se você ainda não tem o Rust instalado, pode seguir as instruções em https://www.rust-lang.org/learn/get-started para instalá-lo.

### Passo 2: Criar um Novo Projeto

Abra o terminal e navegue até o diretório onde deseja criar seu projeto Actix. Em seguida, execute o seguinte comando para criar um novo projeto usando o gerenciador de pacotes Cargo:

```bash
cargo new nome-do-projeto
```
Substitua "nome-do-projeto" pelo nome desejado para o seu projeto.

### Passo 3: Adicionar Dependências

Dentro do diretório do projeto, abra o arquivo Cargo.toml e adicione as seguintes dependências para o framework Actix e outras bibliotecas úteis:

```toml
[dependencies]
actix-web = "3"
tokio = { version = "1", features = ["full"]}
```

O Actix usa a versão 3 da biblioteca actix-web e também precisamos da biblioteca tokio para suporte assíncrono.

### Passo 4: Crie um Manipulador (Handler)

Dentro do diretório src do seu projeto, crie um arquivo chamado main.rs. Vamos começar com um exemplo simples de um manipulador (handler) que responde a uma solicitação HTTP:

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Olá, mundo!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
Este é um exemplo mínimo que cria um servidor Actix com um único manipulador para a rota "/".

Passo 5: Executar o Projeto

Para executar o projeto, no terminal, navegue até o diretório do projeto e execute o seguinte comando:

```bash
cargo run
```
Isso compilará e executará o projeto. Você poderá acessar a aplicação em seu navegador ou por meio de uma ferramenta como curl no endereço http://127.0.0.1:8080/.


### CRUD Básico
Crie um arquivo main.rs dentro do diretório src do seu projeto e adicione o seguinte código:

```rust
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};

// Estrutura de dados para representar uma tarefa
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

// Estado compartilhado com estado mutável (Mutex)
struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[get("/tasks")]
async fn get_tasks(data: web::Data<Arc<AppState>>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

#[post("/tasks")]
async fn create_task(data: web::Data<Arc<AppState>>, new_task: web::Json<Task>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    tasks.push(new_task.into_inner());
    HttpResponse::Created()
}

#[put("/tasks/{id}")]
async fn update_task(
    data: web::Data<Arc<AppState>>,
    path: web::Path<(i32,)>,
    updated_task: web::Json<Task>,
) -> impl Responder {
    let (id,) = path.into_inner();
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        *task = updated_task.into_inner();
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(
    data: web::Data<Arc<AppState>>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let (id,) = path.into_inner();
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        HttpResponse::NoContent()
    } else {
        HttpResponse::NotFound()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        tasks: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .service(get_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

```

Aqui estão os principais pontos do código:

- Definimos a estrutura Task para representar uma tarefa com campos id, title e completed.

- Criamos um estado compartilhado AppState que contém um Mutex<Vec<Task>> para armazenar as tarefas. Usamos Arc para permitir o compartilhamento seguro entre as threads.

- Definimos rotas para diferentes operações CRUD usando os atributos get, post, put e delete.

- Os manipuladores (handlers) das rotas usam o estado compartilhado para acessar e manipular a lista de tarefas.

O método main inicia o servidor Actix e vincula as rotas definidas ao servidor.

Agora você pode executar o projeto com o comando cargo run e testar as operações CRUD usando uma ferramenta como curl ou um cliente HTTP. Este exemplo demonstra como criar um simples CRUD usando o Actix em Rust, permitindo que você crie, leia, atualize e exclua tarefas por meio de endpoints HTTP.