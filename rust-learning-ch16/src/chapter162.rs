// Usando a Passagem de Mensagens para Transferir Dados entre Threads
// Uma abordagem cada vez mais popular para garantir a segurança em concorrência é a passagem de
// mensagens, onde threads ou atores se comunicam enviando mensagens que contêm dados. Aqui está o
// conceito, resumido em um slogan da documentação do Go: "Não se comunique compartilhando memória;
// em vez disso, compartilhe memória comunicando-se."

// Para realizar a concorrência por envio de mensagens, a biblioteca padrão do Rust fornece uma
// implementação de canais. Um canal é um conceito geral de programação pelo qual dados são enviados
// de uma thread para outra.

// Você pode imaginar um canal na programação como um canal direcional de água, como um riacho ou um
// rio. Se você colocar algo como um pato de borracha em um rio, ele viajará corrente abaixo até o
// final do curso d'água.

// Um canal possui duas partes: um transmissor e um receptor. A parte transmissora é o local a
// montante onde você coloca os patos de borracha no rio, e a parte receptora é onde o pato acaba
// corrente abaixo. Uma parte do seu código chama métodos no transmissor com os dados que você
// deseja enviar, e outra parte verifica o lado receptor para as mensagens que chegam. Diz-se que um
// canal está fechado se a parte transmissora ou a receptora for descartada.

// Aqui, vamos desenvolver um programa que possui uma thread para gerar valores e enviá-los por um
// canal, e outra thread que receberá os valores e os imprimirá. Vamos enviar valores simples entre
// threads usando um canal para ilustrar essa funcionalidade. Uma vez que você estiver familiarizado
// com a técnica, poderá usar canais para qualquer thread que precise se comunicar entre si, como um
// sistema de chat ou um sistema onde várias threads realizam partes de um cálculo e enviam as partes
// para uma thread que agrega os resultados.

// Primeiro, no Exemplo 16-6, vamos criar um canal, mas ainda não faremos nada com ele. Observe que
// isso não compilará ainda porque o Rust não sabe que tipo de valores queremos enviar pelo canal.

use std::sync::mpsc;

// pub fn create_channel() {
//     let (tx, rx) = mpsc::channel();
// }

// Criamos um novo canal usando a função mpsc::channel; mpsc significa múltiplos produtores, um
// único consumidor. Em resumo, a maneira como a biblioteca padrão do Rust implementa canais
// significa que um canal pode ter várias extremidades de envio que produzem valores, mas apenas uma
// extremidade de recebimento que consome esses valores. Imagine vários riachos fluindo juntos para
// formar um grande rio: tudo enviado por qualquer um dos riachos acabará em um rio no final.
// Começaremos com um único produtor, mas adicionaremos vários produtores quando este exemplo
// estiver funcionando.
//
// A função mpsc::channel retorna uma tupla, cujo primeiro elemento é a extremidade de envio — o
// transmissor — e o segundo elemento é a extremidade de recebimento — o receptor. As abreviações tx
// e rx são tradicionalmente usadas em muitos campos para transmissor e receptor, respectivamente,
// então nomeamos nossas variáveis dessa forma para indicar cada extremidade. Estamos usando uma
// declaração let com um padrão que desestrutura as tuplas; discutiremos o uso de padrões em
// declarações let e desestruturação no Capítulo 18. Por enquanto, saiba que usar uma declaração
// let dessa maneira é uma abordagem conveniente para extrair as partes da tupla retornada por
// mpsc::channel.
//
// Vamos mover a extremidade de transmissão para uma thread gerada e fazer com que ela envie uma
// string, para que a thread gerada se comunique com a thread principal, como mostrado no Exemplo
// 16-7. Isso é como colocar um pato de borracha no rio a montante ou enviar uma mensagem de chat
// de uma thread para outra.

use std::thread;

pub fn move_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

// Novamente, estamos usando thread::spawn para criar uma nova thread e, em seguida, usamos move
// para mover tx para o fechamento, para que a thread gerada possua tx. A thread gerada precisa
// possuir o transmissor para poder enviar mensagens pelo canal. O transmissor possui um
// método send que recebe o valor que queremos enviar. O método send retorna um tipo Result<T, E>,
// então, se o receptor já tiver sido descartado e não houver onde enviar um valor, a operação de
// envio retornará um erro. Neste exemplo, estamos chamando unwrap para gerar um pânico em caso de
// erro. Mas, em uma aplicação real, trataríamos isso adequadamente: volte ao Capítulo 9 para
// revisar estratégias de tratamento de erros adequado.

// No Exemplo 16-8, receberemos o valor do receptor na thread principal. Isso é como recuperar o
// pato de borracha da água no final do rio ou receber uma mensagem de chat.

pub fn recover_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}