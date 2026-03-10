1) Этап 1 - комплияция

cd plugins/blur
cargo build

И

cd plugins/mirror
cargo build

библиотеки скомпилируются по пути plugins\blur\target\debug и plugins\mirror\target\debug

2) Этап 2 - компиляция cli утилиты

cargo build

утилита скомпилируется по пути .\target\debug\

3) Этап 3 - запуск с параметрами 

Всего в приложении 4 параметра 
--input  //путь до изображения. В проекте есть тестовое изображение по пути ./image/example.png (кадр из фильма Маяк 2019)
-P //какой плагин подгрузить. Либо mirror либо blug
--params //параметры для плагина. Формат параметров можно найти в plugins\blur\params и plugins\mirror\params
--plugin-path //путь до скомпилирванных либ ( см пункт 1 )
--output //путь куда положить готовое изображенеи

пример запуска
.\target\debug\image-processor.exe 
--input rs-course-fourth-project\image\example.png 
--output rs-course-fourth-project\image\example-new.png 
-P mirror 
--params rs-course-fourth-project\plugins\mirror\params\params.txt 
--plugin-path rs-course-fourth-project\plugins\mirror\target\debug



