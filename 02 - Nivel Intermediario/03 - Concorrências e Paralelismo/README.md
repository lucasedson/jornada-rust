
# Concorrência e Paralelismo

Concorrência refere-se à execução simultânea de tarefas, enquanto paralelismo envolve a execução efetiva de tarefas simultaneamente em múltiplos núcleos de CPU ou unidades de processamento. Rust é uma linguagem que enfatiza a segurança na concorrência e no paralelismo, graças ao seu sistema de propriedade e ao modelo de ownership.

## Threads em Rust:
Em Rust, você pode criar threads para executar tarefas concorrentes usando o módulo std::thread. Cada thread representa uma linha separada de execução e pode ser usado para realizar operações simultâneas.

Exemplo de criação de threads em Rust:

```rust
Copy code
use std::thread;

fn main() {
    let thread1 = thread::spawn(|| {
        // Código da primeira thread
    });

    let thread2 = thread::spawn(|| {
        // Código da segunda thread
    });

    // Aguardar as threads finalizarem
    thread1.join().unwrap();
    thread2.join().unwrap();
}
```


Sincronização com Mutex e Arc:
Quando várias threads acessam e modificam dados compartilhados, é necessário garantir que essa manipulação seja feita de maneira segura para evitar data races e resultados inconsistentes. O tipo Mutex (mutexes) é usado em Rust para garantir exclusão mútua, permitindo que apenas uma thread acesse um recurso compartilhado por vez.

Exemplo de uso de Mutex em Rust:

```rust
Copy code
use std::sync::Mutex;

fn main() {
    let contador = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut contador = contador.lock().unwrap();
            *contador += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Valor final do contador: {}", *contador.lock().unwrap());
}
```

## Comunicação entre Threads com Canais (Channels):
Canais são uma forma de comunicação segura entre threads em Rust. O módulo std::sync::mpsc (multiple-producer, single-consumer) oferece canais para enviar e receber mensagens entre threads.

Exemplo de uso de canais em Rust:

```rust
Copy code
use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmissor, receptor) = mpsc::channel();

    thread::spawn(move || {
        let mensagem = String::from("Olá do thread transmissor!");
        transmissor.send(mensagem).unwrap();
    });

    let mensagem_recebida = receptor.recv().unwrap();
    println!("Mensagem recebida: {}", mensagem_recebida);
}
```
Em resumo, Rust oferece suporte seguro e eficaz para concorrência e paralelismo por meio de threads, sincronização com Mutex e Arc, e canais para comunicação entre threads. Esses recursos permitem aproveitar o poder do hardware multicore enquanto mantém a integridade dos dados e a segurança.