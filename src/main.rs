mod ahorcado;

fn main() {
    let path = "./ahorcado.txt".to_string();
    ahorcado::jugar(&path);
}