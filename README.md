# recorrelaberintos
En este repositorio se encuentran varias implementaciones del algoritmo *breadth first search* para resolver laberintos. Antes
de ejecutar este algoritmo el laberinto es transformado en un grafo con la función `read_graph` del módulo `main`.
De hecho, dediqué más tiempo a implementar la lectura del grafo que a implementar el algoritmo BFS.
Al algoritmo BFS le he hecho estas variaciones:

- `bfs`: esta es la primera que se me ocurrió. Directa y al grano, es de manual.
- `pbfs`: mi primer intento de hacer el algoritmo paralelo. La abandoné pronto al ver que no funcionaba muy bien.
De hecho creo que en este momento no resuelve el laberinto correctamente.
- `bfs2`: después de mi fracaso con la versión paralela intenté hacer otra de otra forma, y para comprobar que resolvía
correctamente el laberinto la hice primero de forma iterativa en vez de paralela.
- `pbfs2`: esta versión paralela sí que funciona correctamente. La diferencia que tiene con `bfs` es que en vez de en
cada iteración seguir una sola rama del laberinto, esta versión sigue varias ramas (número determinado por la global *N*) en
paralelo.

De las distintas versiones solo recomiendo `bfs` y `pbfs2`, las demás o no funcionan o son solo pruebas. `bfs` es
notablemente más rápida que `pbfs2`, aunque pueda sorprender. Creo que BFS es un algoritmo difícil de paralelizar, sobre
todo si encontrar el siguiente nodo es sencillo (como ocurre en este caso).

Para hacer pruebas he utilizado los laberintos utilizados en
[este vídeo de Computerphile](https://www.youtube.com/watch?v=rop0W4QDOUI), que se encuentran en `computerphile-mazesolver/examples`.
El ordenador en el que he realizado las pruebas tiene 8GB de RAM, 8GB de memoria swap y un procesador i5-4460 de 4 núcleos de
3.20GHz (no son muchos detalles pero creo que suficientes). El laberinto más grande, `perfect15k`, se ejecuta en 70 segundos
con la versión `bfs` y en 110 con `pbfs2`. No le he visto consumir más de 5GB en ningún momento con ninguna de las versiones.
Me alegra poder resolver este laberinto porque el ponente del vídeo de Computerphile avisa de que se necesita un ordenador
con más de 16GB de RAM para poder ejecutarlo, y bueno, si contamos la swap sí que tengo 16GB de memoria, pero el programa solo
necesita 5GB.

## Instalación y utilización
Vas a necesitar [rust](https://www.rust-lang.org/) instalado. Una vez lo tengas, simplemente ejecuta
`cargo run --release -- <algoritmo> <laberinto>` en el directorio donde descargaste el código. La bandera `--release`
es para aplicar todas las optimizaciones, pero no es estrictamente necesaria. Actualmente el código lo único que hace
es imprimir el número de nodos (o esquinas) que tiene el laberinto y el número de nodos que tienes que recorrer para llegar
a la salida, eres libre de modificar el código para que haga algo interesante, como generar otra imagen con el camino hacia
la salida (si lo haces no dudes en hacer una pull request o por lo menos enseñármelo). Me gustaría hacer que cronometrase
el tiempo de ejecución, pero de momento utilizo el comando `time` de *bash* para hacer mis pruebas de rendimiento. Si lo
quieres utilizar, simplemente ejecuta `time ./target/release/recorrelaberintos` (o `time ./target/debug/recorrelaberintos`
si compilaste sin la bandera `--release`) con los argumentos que gustes.

## Conclusiones
He aprendido bastante realizando este proyecto y sobre todo ha sido bastante gratificante (es de las primeras cosas que
estoy haciendo que salen en mi libro de inteligencia artificial). Tengo pensado seguir añadiendo algunas cosillas
y pulir el uso de las estructuras de datos que hay así que este repositorio de momento no se quedará muerto. Nada más,
muchas gracias por leer hasta el final :).
