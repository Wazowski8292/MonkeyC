fn helper() -> int {
    return 1;
}
## ignore: the semantic analyzer does not check that main exists (maybe another stage does)
## expect-error: main
