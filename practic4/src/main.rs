const SIZE: usize = 5; // Размер ромба

fn main() {
    let mut result = String::new();

    // Верхняя часть ромба
    for i in 0..SIZE {
        for _ in 0..(SIZE - i - 1) {
            result.push(' '); // Добавляем пробелы перед *
        }
        for _ in 0..(2 * i + 1) {
            result.push('*'); // Добавляем звездочки
        }
        result.push('\n'); // Переход на новую строку
    }

    // Нижняя часть ромба
    for i in (0..SIZE - 1).rev() {
        for _ in 0..(SIZE - i - 1) {
            result.push(' '); // Добавляем пробелы перед *
        }
        for _ in 0..(2 * i + 1) {
            result.push('*'); // Добавляем звездочки
        }
        result.push('\n'); // Переход на новую строку
    }

    print!("{}", result); // Используем print! один раз
}
