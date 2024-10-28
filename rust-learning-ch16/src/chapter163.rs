// Concorrência de Estado Compartilhado

// Além de passar mensagens, outra abordagem para concorrência é o compartilhamento de memória entre
// várias threads. Em Rust, canais são usados para transferência única de valores, semelhante à
// propriedade única; já a concorrência de memória compartilhada permite que várias threads acessem
// a mesma memória, o que exige controle cuidadoso. Para isso, Rust oferece Mutex, que limita o
// acesso a dados por uma thread de cada vez, garantindo exclusão mútua.

// Usando Mutex para Acesso Controlado de Dados

// Um Mutex permite que uma única thread acesse dados ao mesmo tempo. Para usar um Mutex, uma thread
// deve travá-lo antes de acessar o dado e destravá-lo após o uso. Exemplo de código:

use std::sync::{Arc, Mutex};

pub fn call_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Neste exemplo, usamos m.lock() para adquirir o bloqueio e atualizar o valor interno do Mutex. Ao
// final do escopo, o bloqueio é liberado automaticamente.


// Compartilhando um Mutex Entre Múltiplas Threads:
// Para permitir que várias threads incrementem um contador protegido por Mutex, criamos 10 threads,
// cada uma incrementando o valor do contador. No entanto, o código inicial causa um erro de
// compilação, pois o Mutex é movido para cada thread. Código com erro:

// fn wrong_counter() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Result: {}", *counter.lock().unwrap());
// }

// Propriedade Múltipla com Arc

// Para resolver o problema, envolvemos o Mutex com Arc, um contador de referência atômico seguro
// para concorrência:

use std::thread;

pub fn atom_ref() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Esse código resulta em Result: 10, mostrando que o valor foi compartilhado e atualizado de forma
// segura.


// Similaridades Entre RefCell/Rc e Mutex/Arc

// Assim como RefCell permite mutabilidade interna, Mutex permite modificar conteúdos dentro de um
// Arc. Contudo, o uso de Mutex pode resultar em deadlocks, então é necessário cuidado para evitar
// problemas de travamento quando múltiplos recursos precisam ser bloqueados por várias threads.

//--------------------------------------------------------------------------------------------------

// Concorrência Extensível com os Traits Sync e Send
//
// Rust não possui muitos recursos de concorrência embutidos na linguagem em si; a maioria dos
// recursos discutidos até aqui faz parte da biblioteca padrão. Contudo, os conceitos de
// concorrência dos traits Sync e Send estão embutidos na linguagem.
//
// Transferência de Propriedade Entre Threads com Send

// O trait Send permite transferir a propriedade de valores entre threads. A maioria dos tipos em
// Rust implementa Send, exceto por alguns, como Rc<T>. Este não é Send pois, ao tentar transferir
// um clone para outra thread, ambas poderiam atualizar o contador de referência simultaneamente,
// causando problemas de segurança. Em contrapartida, Arc<T> é Send, permitindo o uso seguro entre
// threads.
//
// Acesso por Múltiplas Threads com Sync

// O trait Sync indica que um tipo pode ser acessado com segurança por várias threads. Um tipo T é
// Sync se uma referência imutável &T pode ser enviada para outra thread de forma segura. Rc<T> e
// RefCell<T>, devido ao controle interno de empréstimos em tempo de execução, não são Sync e,
// portanto, não são seguros para acesso entre múltiplas threads. Já Mutex<T> é Sync, possibilitando
// o compartilhamento de dados entre threads de forma controlada.

// Implementação Manual de Send e Sync é Insegura
// Como os tipos compostos por tipos Send e Sync são automaticamente marcados como tais, geralmente
// não é necessário implementar esses traits manualmente. Implementar manualmente esses traits
// envolve código unsafe, e deve-se ter cuidado para manter as garantias de segurança.
//
// Resumo

// Rust utiliza crates para várias soluções de concorrência, mais rápidas na evolução do que a
// biblioteca padrão. Com canais para passagem de mensagens e ponteiros inteligentes como Mutex<T> e
// Arc<T>, Rust evita condições de corrida e referências inválidas. O sistema de tipos e o
// verificador de empréstimo asseguram que o código é seguro e rodará corretamente em múltiplas
// threads após a compilação, tornando a programação concorrente mais acessível e segura.