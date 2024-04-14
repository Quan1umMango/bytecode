# Virtual Machine in Rust
Virtual Machine made in Rust.

# Example
```
; This is a comment

label main: ; Heart of our program. This where code execution begins.

    mov rax, 10  ; Move into register rax the value of 10. Theres 4 user available registers 
                 ; and one program only register.
    mov rbx, 20 
    add rax, rbx ; Add register rax and rbx and store the value into rax 
                 ; Other math operations like sub, mul, div and mod also exist
    push rax     ; Push onto the stack
    pop rcx      ; Pop into rcx 

    cmp rcx rax  ; Compare. This sets flags based on the result
    je Equal     ; Jump if equal to label Equal
    
    halt         ; Halts the program (optional). If this is missing then the program 
                 ; executes the next label

label Equal:
    display rcx  ; Display the numeric value of rcx
    ret          ; Return to the called label (in this case main). Code execution happens normally after the jne
                 ; If not return instruction exists then the program executes the instructions 
                 ; below this label if any otherwise halts.
```

Count to 10:

```
label main:
    
    mov rax, 0  ; Inital value
    mov rbx, 10 ; Final value 

    jmp loop
    

label loop:
    display rax
    cmp rax rbx
    add rax, 1 
    jne loop
    jl loop     ; Jump if less 
    ret

```
Look into ``examples`` for more examples.
Note: make sure to run in the directory of the file you want to run. In other words, run the program in the ``examples`` folder to test them out. This issue will be fixed in the future. 
## Usage
```
bytecode <file-name>.basm
```

