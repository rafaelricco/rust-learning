// O Rust tem como objetivo principal facilitar a programação concorrente
// de maneira segura e eficiente. A programação concorrente (onde partes
// do programa são executadas de forma independente) e paralela (onde partes
// do programa são executadas ao mesmo tempo) têm se tornado cada vez mais
// importantes com o uso crescente de computadores com múltiplos processadores.
// Tradicionalmente, esses contextos são difíceis e propensos a erros, mas o
// Rust pretende mudar isso.

// Inicialmente, o time de Rust pensava que garantir segurança de memória e prevenir
// problemas de concorrência eram desafios distintos. Contudo, perceberam que o sistema
// de posse (ownership) e o sistema de tipos de Rust são ferramentas eficazes para gerenciar
// ambos. Graças a essas verificações, muitos erros de concorrência em Rust se tornam
// erros de compilação, não de execução, permitindo que o código incorreto não seja
// compilado e explique o problema, facilitando a correção durante o desenvolvimento.
// Esse aspecto é chamado de "concorrência destemida" (fearless concurrency), onde o
// código é livre de bugs sutis e fácil de refatorar sem introduzir novos problemas.

// Ao contrário de outras linguagens que oferecem soluções limitadas para concorrência,
// Rust fornece uma variedade de ferramentas para modelar problemas da forma mais
// adequada à situação e às necessidades do programador.

//--------------------------------------------------------------------------------------------
// Nos sistemas operacionais atuais, o código de um programa é executado em um processo,
// que o sistema gerencia junto com outros processos. Dentro de um programa, também é
// possível executar partes independentes simultaneamente usando threads. Por exemplo,
// um servidor web pode ter várias threads para responder a múltiplas solicitações ao
// mesmo tempo.

// Dividir a computação do programa em múltiplas threads pode melhorar o desempenho,
// mas adiciona complexidade. Como as threads podem rodar ao mesmo tempo, não há uma
// garantia de ordem de execução entre elas, o que pode levar a problemas como:

// Condições de corrida: threads acessam dados ou recursos em ordens inconsistentes;
// Deadlocks: duas threads esperam uma pela outra, impedindo ambas de continuar;
// Bugs de execução difícil de reproduzir: ocorrem apenas em situações específicas.
// O Rust tenta mitigar esses problemas, mas a programação com múltiplas threads exige
// uma estrutura de código cuidadosa.

// Linguagens implementam threads de formas diferentes, e muitos sistemas operacionais
// fornecem uma API para a criação de threads. A biblioteca padrão de Rust usa um modelo
// de thread 1:1, em que cada thread do programa corresponde a uma thread do sistema
// operacional.

// Criando uma Nova Thread com spawn

// Para criar uma nova thread, usa-se a função thread::spawn, passando uma closure com
// o código que queremos executar. No exemplo abaixo, uma thread principal imprime
// mensagens enquanto uma nova thread imprime outras mensagens:

use std::thread;
use std::time::Duration;

pub fn spawn_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

// Quando a thread principal termina, todas as threads criadas também são encerradas,
// independentemente de terem concluído. A execução das threads é intercalada de maneira
// não determinística, dependendo do agendamento do sistema operacional.


// Esperando a Conclusão de Todas as Threads com join:

// Para garantir que a thread criada finalize antes da thread principal, podemos armazenar
// o retorno de thread::spawn em uma variável JoinHandle e chamar join. Isso bloqueia a
// thread principal até que a nova thread termine.

pub fn waiting_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Usando Closures com move em Threads:

// Para usar dados da thread principal em uma thread criada, a closure precisa capturar os valores
// necessários. A palavra-chave move força a closure a assumir a posse dos valores do ambiente.

// No exemplo a seguir, move permite que a closure acesse o vetor v da thread principal:

pub fn handle_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Aqui está um vetor: {:?}", v);
    });

    handle.join().unwrap();
}

// Usar move garante que os dados transferidos para a thread criada não sejam usados novamente na
// thread principal, evitando problemas de posse.