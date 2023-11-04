# huggingface_tokenizer_flutter
- huggingface_tokenizer : flutter binding

- See URL : https://bguru.tistory.com/64
<br>  


## INSTALL
1. visual studio 설치(vscode X)
2. rust 설치 : https://www.rust-lang.org/tools/install
3. Android Studio 설치
4. Android Studio > Preferences > Appearance & Behaviour > Android SDK > SDK Tools : 아래 4개 설치
```
* Android SDK Tools
* NDK
* CMake
* LLDB
```

5. Rust 설정
```shell
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```