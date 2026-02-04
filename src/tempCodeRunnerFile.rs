 let mut pick = String::new();
    println!("1. Packet sniffer \n2. Net hacking");
    println!("Choose mode: ");
    io::stdin().read_line(&mut pick).expect("Failed to read line");