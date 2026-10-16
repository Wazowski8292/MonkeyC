fn main() {
    int x = 1;
    int x = 2;
}
## ignore: the semantic analyzer has no redeclaration check yet
## expect-error: already
