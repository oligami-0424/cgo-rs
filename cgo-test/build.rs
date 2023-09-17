fn main() {
    cgo_oligami::Build::new()
        .trimpath(true)
        .ldflags("-s -w")
        .change_dir("./tests/example")
        .package("main.go")
        .build("integrationtest");
}
