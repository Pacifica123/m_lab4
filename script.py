import sys
import ast
import matplotlib.pyplot as plt
import os

# Функция для построения графика по данным выборки
def plot_distribution(p_range, h, min_temp, ax, label, offset=0):
    # Создаем диапазон значений по оси x для температур с шагом h (от min_temp до max_temp)
    x = [min_temp + i * h + offset for i in range(len(p_range))]

    # Отрисовываем гистограмму
    ax.bar(x, p_range, width=0.8 * h, edgecolor='black', alpha=0.5, label=label)

# Принимаем выборки из аргументов командной строки
p_range_1 = ast.literal_eval(sys.argv[1])
h_1 = float(sys.argv[2])
min_temp_1 = float(sys.argv[3])
max_temp_1 = float(sys.argv[4])

p_range_2 = ast.literal_eval(sys.argv[5])
h_2 = float(sys.argv[6])
min_temp_2 = float(sys.argv[7])
max_temp_2 = float(sys.argv[8])

# Создаем фигуру и оси
fig, ax = plt.subplots(figsize=(8, 6))

# Строим графики для обеих выборок
plot_distribution(p_range_1, h_1, min_temp_1, ax, label='Эмпирическая')
plot_distribution(p_range_2, h_2, min_temp_2, ax, label='Сгенерированная', offset=0.2)

# Настраиваем оси
ax.set_xlabel('Значение')
ax.set_ylabel('Вероятность')
ax.set_title('Гистограмма распределения')

# Добавляем легенду
ax.legend()

# Задаем имя файла
file_name = 'distributions.png'

# Проверяем наличие файла с заданным именем
i = 1
while os.path.exists(file_name):
    # Если файл существует, добавляем номер к имени файла
    file_name = f'distributions_{i}.png'
    i += 1

# Сохраняем график в файл
plt.savefig(file_name)

print(f"График сохранен в файл: {file_name}")
