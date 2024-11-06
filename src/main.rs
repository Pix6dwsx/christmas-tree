fn draw_tree(triangles: usize) {
    // Высота каждого треугольника равна его номеру (1, 2, 3, ...)
    (1..=triangles).for_each(|triangle| {
        // Рисуем текущий треугольник
        (0..triangle).for_each(|row| {
            let stars = 2 * row + 1; // Количество звездочек в строке
            let spaces = triangles + triangle - row - 1; // Количество пробелов слева

            // Вывод строки с пробелами и звездочками
            print!("{:width$}", "", width = spaces);
            println!("{}", "*".repeat(stars));
        });
    });

    // Рисуем ствол
    let trunk_width = 1;
    let trunk_height = triangles;
    let trunk_spaces = triangles + trunk_height - 1;

    (0..trunk_height).for_each(|_| {
        print!("{:width$}", "", width = trunk_spaces);
        println!("{}", "*".repeat(trunk_width));
    });
}

fn main() {
    let triangles = 5; // Задайте количество треугольников
    draw_tree(triangles);
}
