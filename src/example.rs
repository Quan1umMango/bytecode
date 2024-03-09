 
    let mut vm = VM::new();
    

    vm.start_label("someLabel");
           vm.add_instruction(Instruction::Mov(REGA,0));
           vm.add_command(Command::Return); // Optional if label is not empty
    vm.end_label("someLabel");

    vm.start_label("main"); // Entrypoint of our program, this where code execution begins.
        vm.add_instruction(Instruction::Mov(REGA,1)); // Instruction to move into register A, the
                                                     // value of 1.
        // Instructions are an abstraction over Commands. Commands are the lowest level
        // representation of our code. Instruction makes it more convienient for us to write our
        // program 
        vm.add_instruction(Instruction::Mov(REGB,10));
        vm.add_instruction(Instruction::Add(REGA,REGB)); // add register A and B and store the
                                                         // value in register A
        vm.add_instruction(Instruction::Display(REGA)); // Display the contents of register A in
                                                        // integer form
        vm.add_instruction(Instruction::Jump("someLabel".to_string()));  // jumps to the label `someLabel`
        // Currently, labels must be defined before it is called. So defining the label below the
        // main label would not work 
        // The program jumps to the label, executes the instructions inside the label, then returns
        // to the calling label  (in this case, `main` label) and executes the next line (the line
        // below)
        vm.add_instruction(Instruction::Push(10)); // Pushes the value of 10 onto the stack
        vm.add_instruction(Instruction::PushRegister(REGA)); // Pushes the value of register onto
                                                             // the stack
        vm.add_instruction(Instruction::Pop(REGB)); // Pops the most recently added value onto the
                                                // given register 
     
    vm.add_instruction(Instruction::Halt); // Halts the program. Optional in `main` label
    vm.end_label("main"); // All labels must have an end.
    vm.register_start(); // Register the `main` label as out entry point. 
    
   
    // Run the code
    vm.eval();


    println!("______END OF PROGRAM______\n Bytecode:");
    print_bytecode(vm.get_raw_byte_code());


    // Write to a file in  binary format if needed
    
   let _ = vm.write_to_file("yeah.bc");
   

    // Load from binary format 
    let mut new_vm = VM::new();
    let _ = new_vm.read_from_file("yeah.bc");
    new_vm.eval();
