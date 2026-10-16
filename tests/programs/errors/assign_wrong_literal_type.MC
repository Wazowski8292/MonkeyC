fn main() {
    int x = "hello";
}
## ignore: the analyzer currently lets this compile (the plain-variable type check never fires)
## expect-error: Type mismatch. Expected: int, Found: <string literal>
