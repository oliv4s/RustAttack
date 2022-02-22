# Rust Attack

![image](https://user-images.githubusercontent.com/78645074/152182659-b52176eb-07e2-49e9-af39-7304d097f077.png)


This repository is inspired by different Rust repositories in order to define my own Offensive Windows Rust cheatsheet and weapons.
The entire project has been developed and compiled on a Linux machine.

## Index

- [RustAttack](#rust-attack)
  * [Examples](#examples)
    * [1 - DLLs](#1---dlls)
      - [1.1 - MsgBox](#11---msgbox)
      - [1.2 - Code Execution](#12---code-execution)
      - [1.3 - Shellcode - CreateRemoteThread method](#13---shellcode---createremotethread-method)
      - [1.4 - Shellcode - CreateThread method](#14---shellcode---createthread-method)
    * [2 - Executables](#2---executables)
      - [2.1 - MsgBox](#21---msgbox)
      - [2.2 - Code Execution](#22---code-execution)
      - [2.3 - Shellcode - CreateRemoteThread method](#23---shellcode---createremotethread-method)
      - [2.4 - Shellcode - CreateThread method](#24---shellcode---createthread-method)
  * [To-Do](#to-do)
  * [Other Rust Projects](#other-rust-projects)

## Examples

### 1 - DLLs
You can find 3 different projects inside _01_DLLs/_
```
$ ls 01_DLLs
code_execution  msgbox  shellcode_CreateRemoteThread  shellcode_CreateThread
```
#### 1.1 - MsgBox
Launch a simple Windows prompt.

```
$ cd /01_DLLs/msgbox
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib --release
```
This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/release/_
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


#### 1.2 - Code Execution
Execute commands from a new cmd process.

```
$ cd 01_DLLs/code_execution
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib --release
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/release/_
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
#### 1.3 - Shellcode - CreateRemoteThread method
**[NO AV DETECTION (DEFENDER, ESET, FSECURE, MCAFEE, CORTEX) - 15/02/2022]**

Launch a shellcode inside a especific process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 01_DLLs/shellcode_CreateRemoteThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib --release
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/release/_
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

**AV DETECTIONS** (with Covenant Grunt shellcode)

![image](https://user-images.githubusercontent.com/78645074/155105179-d96ab89d-7f9f-4413-a87c-31318ce470ef.png)

##### Customization
To change the purpose and/or shellcode of the DLL, open _lib.rs_ modify the following line and recompile:

```
let shellcode:[u8;276] = [0xfc, 0x48, ... <blablabla>];
```

In addition, if you want to change default process for inject the shellcode, you can edit the following line in _lib.rs_ and recompile:
```
let mut processname = "explorer.exe";
```

#### 1.4 - Shellcode - CreateThread method
**[NO AV DETECTION (DEFENDER, ESET, FSECURE, MCAFEE, CORTEX) - 15/02/2022]**

Launch a shellcode inside the current process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 01_DLLs/shellcode_CreateThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --lib --release
```

This generate a **.dll** file with the export function _exec_. It is located at: _target/x86_64-pc-windows-gnu/release/_
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

### 2 - Executables

#### 2.1 - MsgBox
Launch a simple Windows prompt.

```
$ cd /02_EXEs/msgbox
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --release
```
This generate a **.exe** file. It is located at: _target/x86_64-pc-windows-gnu/release/_
```
$ ls
build  deps  examples  incremental  msgbox_exe.d  msgbox_exe.exe
```

For execute it in Windows:

```
C:\>msgbox_exe.exe
```

#### 2.2 - Code Execution
Execute commands from a new cmd process.

```
$ cd 02_EXEs/code_execution
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --release
```

This generate a **.exe** file. It is located at: _target/x86_64-pc-windows-gnu/release/_
```
$ ls
build  code_execution_exe.d  code_execution_exe.exe  deps  examples  incremental
```

For execute it in Windows:

```
C:\>code_execution_exe.exe
```

##### Customization
To change the purpose and/or command execution of the EXE, open _main.rs_ modify the following line and recompile:

```
let output = Command::new("cmd").args(&["/C", "calc.exe"]).output().expect("failed to execute process");
```
#### 2.3 - Shellcode - CreateRemoteThread method
**[NO AV DETECTION (DEFENDER, ESET, FSECURE, MCAFEE, CORTEX) - 15/02/2022]**

Launch a shellcode inside a especific process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 02_EXEs/shellcode_CreateRemoteThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --release
```

This generate a **.exe** file. It is located at: _target/x86_64-pc-windows-gnu/release/_
```
$ ls
build  deps  examples  incremental  shellcode_create_remote_thread_exe.d  shellcode_create_remote_thread_exe.exe
```

For execute it in Windows:

```
C:\>shellcode_create_remote_thread_exe.exe <process_name>
```
If the process name is not specified, the shellcode is injected into the explorer.exe process by default.

**AV DETECTIONS** (with Covenant Grunt shellcode)

![image](https://user-images.githubusercontent.com/78645074/155107242-c887e458-c016-4ab9-a54b-2e9c6a7a6bae.png)


##### Customization
To change the purpose and/or shellcode of the DLL, open _main.rs_ modify the following line and recompile:

```
let shellcode:[u8;276] = [0xfc, 0x48, ... <blablabla>];
```

In addition, if you want to change default process for inject the shellcode, you can edit the following line in _main.rs_ and recompile:
```
let mut processname = "explorer.exe";
```

#### 2.4 - Shellcode - CreateThread method
**[NO AV DETECTION (DEFENDER, ESET, FSECURE, MCAFEE, CORTEX) - 15/02/2022]**

Launch a shellcode inside the current process. **This example execute a shellcode generated by msfvenom which launch calc.exe.**

```
$ cd 02_EXEs/shellcode_CreateThread
$ ls
Cargo.toml  src
$ cargo build --target x86_64-pc-windows-gnu --release
```

This generate a **.exe** file. It is located at: _target/x86_64-pc-windows-gnu/release/_
```
$ ls
build  deps  examples  incremental  shellcode_createThread_exe.d  shellcode_createThread_exe.exe
```

For execute it in Windows:

```
C:\>shellcode_createThread_exe.exe
```

##### Customization
To change the purpose and/or shellcode of the DLL, open _main.rs_ modify the following line and recompile:

```
let shellcode:[u8;276] = [0xfc, 0x48, ... <blablabla>];
```

**NOTE**: you must specify shellcode length.

## To-DO
- AMSI Bypass
- ETW Bypass
- ETW Patch
- OpSec
- Hell'sGate Rust Implementation

## Other Rust Projects

Interesting links for other offensive Rust project:
- https://github.com/trickster0/OffensiveRust
- https://github.com/kmanc/remote_code_oxidation
- https://github.com/skerkour/black-hat-rust
