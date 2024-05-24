import sys
import ast
import matplotlib.pyplot as plt
import os

p_range = ast.literal_eval(sys.argv[1])
h = float(sys.argv[2])
min_temp = float(sys.argv[3])
max_temp = float(sys.argv[4])

# Создаем диапазон значений по оси x для температур с шагом h (от min_temp до max_temp)
# x = [i for i in range(int(min_temp), int(max_temp), int(h))]

# Создаем диапазон значений по оси x для температур с шагом h (от min_temp до max_temp)
x = [min_temp + i * h for i in range(len(p_range))]

# Создаем фигуру и оси
fig, ax = plt.subplots(figsize=(8, 6))

# Отрисовываем гистограмму
ax.bar(x, p_range, width=0.8 * h, edgecolor='black')

# Настраиваем оси
ax.set_xlabel('Значение')
ax.set_ylabel('Вероятность')
ax.set_title('Гистограмма распределения')

# Задаем имя файла
file_name = 'distribution.png'

# Проверяем наличие файла с заданным именем
i = 1
while os.path.exists(file_name):
    # Если файл существует, добавляем номер к имени файла
    file_name = f'distribution_{i}.png'
    i += 1

# Сохраняем график в файл
plt.savefig(file_name)

print(f"График сохранен в файл: {file_name}")
