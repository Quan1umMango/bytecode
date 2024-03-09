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
    
    halt         ; Halts the program (optional)
endlabel

label Equal:
    display rcx  ; Display the numeric value of rcx
    ret          ; Return to the called label (in this case main). Code execution happens normally after the jne
                 ; If not return instruction exists then the program execution after the endlabel of this label.
endlabel
```

Count to 10:

```
label main:
    
    mov rax, 0  ; Inital value
    mov rbx, 10 ; Final value 

    jmp loop
    
endlabel

label loop:
    display rax
    cmp rax rbx
    add rax, 1 
    jne loop
    jl loop     ; Jump is less 
    ret
endlabel

```

## Usage
In `main.rs` plug in your file name and run. This is a makeshift solution, but in the future there will a cli interface to dynamically run the program.
