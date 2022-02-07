# Rust Attack

![image](https://user-images.githubusercontent.com/78645074/152182659-b52176eb-07e2-49e9-af39-7304d097f077.png)


This repository is inspired by different Rust repositories in order to define my own Offensive Windows Rust cheatsheet and weapons.
The entire project has been developed and compiled on a Linux machine.

## Index

- [RustAttack](#rustattack)
  * [Examples](#examples)
    * [DLLs](#dlls)
      - [MsgBox](#msgbox)
      - [Code Execution](#code-execution)
      - [Shellcode - CreateRemoteThread method](#shellcode---createremotethread-method-no-av-detection-defender---07022022)
      - [Shellcode - CreateThread method](#shellcode---createthread-method-no-av-detection-defender---07022022)
    * [Executables ToDo](#executables)
  * [Other Rust Projects](#other-rust-projects)

## Examples

### DLLs
You can find 3 different projects inside _01_DLLs/_
```
$ ls 01_DLLs
code_execution  msgbox  shellcode_CreateThread
```
#### MsgBox
Launch a simple Windows prompt.

```
$ cd /01_DLLs/msgbox
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib
```
This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/debug/_
```
$ ls
build  deps  examples  incremental  libmsgbox_dll.dll.a  msgbox_dll.d  msgbox_dll.dll
```
![image](https://user-images.githubusercontent.com/78645074/152153443-d6f23873-1c49-4189-929c-acd127edac77.png)


For execute it in Windows:

```
C:\>rundll32 msgbox_dll.dll,exec
```
![image](https://user-images.githubusercontent.com/78645074/152153601-f6f3656f-ae6b-4868-b9e0-14409fbef73f.png)


#### Code Execution
Execute commands from a new cmd process.

```
$ cd 01_DLLs/code_execution
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/debug/_
```
$ ls
build  code_execution_dll.d  code_execution_dll.dll  deps  examples  incremental  libcode_execution_dll.dll.a
```
![image](https://user-images.githubusercontent.com/78645074/152176734-f562faaa-6082-46bd-b391-1ebfd8c5c830.png)

For execute it in Windows:

```
C:\>rundll32 code_execution_dll.dll,exec
```

![image](https://user-images.githubusercontent.com/78645074/152177263-05b67e8d-03db-4037-909a-9ef93ca21cba.png)

##### Customization
To change the purpose and/or command execution of the DLL, open _lib.rs_ modify the following line and recompile:

```
let output = Command::new("cmd").args(&["/C", "calc.exe"]).output().expect("failed to execute process");
```
#### Shellcode - CreateRemoteThread method [NO AV DETECTION (DEFENDER) - 07/02/2022]
Launch a shellcode inside a especific process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 01_DLLs/shellcode_CreateRemoteThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/debug/_
```
$ ls
build  deps  examples  incremental  libshellcodeshellcode_create_remote_thread_dll.dll.a  shellcode_create_remote_thread_dll.d  shellcode_create_remote_thread_dll.dll
```

For execute it in Windows:

```
C:\>rundll32 shellcode_create_remote_thread_dll.dll,exec <process_name>
```
If the process name is not specified, the shellcode is injected into the explorer.exe process by default.

![image](https://user-images.githubusercontent.com/78645074/152763348-89d80856-375d-480e-9eb2-f267ea5751ed.png)


##### Customization
To change the purpose and/or shellcode of the DLL, open _lib.rs_ modify the following line and recompile:

```
let shellcode:[u8;276] = [0xfc, 0x48, ... <blablabla>];
```

In addition, if you want to change default process for inject the shellcode, you can edit the following line in _lib.rs_ and recompile:
```
let mut processname = "explorer.exe";
```

#### Shellcode - CreateThread method [NO AV DETECTION (DEFENDER) - 07/02/2022]
Launch a shellcode inside the current process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 01_DLLs/shellcode_CreateThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/debug/_
```
$ ls
build  deps  examples  incremental  libshellcode_createThread_dll.dll.a  shellcode_createThread_dll.d  shellcode_createThread_dll.dll
```
![image](https://user-images.githubusercontent.com/78645074/152154583-b85ad5db-d9eb-4d3d-b7dc-2214064c4db6.png)

For execute it in Windows:

```
C:\>rundll32 shellcode_createThread_dll.dll,exec
```
![image](https://user-images.githubusercontent.com/78645074/152155070-9699ac8f-cb8d-4014-8be1-8c9e8155c596.png)

##### Customization
To change the purpose and/or shellcode of the DLL, open _lib.rs_ modify the following line and recompile:

```
let shellcode:[u8;276] = [0xfc, 0x48, ... <blablabla>];
```

**NOTE**: you must specify shellcode length.

### Executables

## Other Rust Projects

Interesting links for other offensive Rust project:
- https://github.com/trickster0/OffensiveRust
- https://github.com/kmanc/remote_code_oxidation
- https://github.com/skerkour/black-hat-rust
