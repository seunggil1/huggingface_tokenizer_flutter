fn main() {
    // OpenSSL 라이브러리 경로를 환경 변수에서 가져옵니다.
    let lib_dir = std::env::var("OPENSSL_LIB_DIR").unwrap_or_else(|_| String::from("/path/to/openssl/lib"));

    println!("OPENSSL_LIB_DIR is set to: {}", lib_dir);
    
    // 라이브러리 경로를 링커에게 알립니다.
    println!("cargo:rustc-link-search=native={}", lib_dir);

    // 링커에게 필요한 라이브러리를 링크하도록 지시합니다.
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
}
