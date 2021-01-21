# Shared Secrets
Programa para cifrar y decifrar archivos que usa el esquema de Shamir para compartir las llaves usadas.

# Integrtantes
- Jonás García Chavelas
- Daniel Linares Gil
---
## Compilar
Para compilar es necesario tener el compilador de Rust y el programa Cargo los cuales se pueden instalar ejecutando
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Además es necesario instalar las siguientes herramientas para compilar C, por ejemplo en Debian
```
sudo apt install diffutils gcc m4 make
```
Una vez instalado, se puede compilar el programa ejecutando
```
cargo build --release
```
En las arquitecturas x86/x64 se puede aprovechar el soporte para AES compilando con las siguientes banderas
```
RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3" cargo build --release
```
## Ejecutar
Una vez compilado, el programa se puede ejecutar con el comando
```
./target/release/shared_secrets c <Archivo a cifrar> <Nombre Archivo Resultante> <Total de fragmentos de la llave> <Fragmentos mínimos necesario para decifrar>
```
para cifrar archivos, y para decifar
```
./target/release/shared_secrets d <Archivo a decifrar> <Archivo con los fragmentos de la llave>
```
## Correr Pruebas
Para correr las pruebas unitarias ejecutar
```
cargo test
```
## Abrir documentación
Para abrir la documentación ejecutar
```
cargo doc --open --no-deps
```

# Falta
- ~~Refactor~~
- Mejorar Documentación
- Hacer el README (Recordar añadir instrucciones para compilar las dependencias para compilar C)
- Probarlo mas

# Mejoras
- ~~Integration Test (Opcional)~~
- ~~Guardar las llaves en hexadecimal( Mejora, Optimizacion)~~
- Optimizar run_encrypt ILP (Mejora, Optimizacion)
- Intentar sustituir aes-gcm por aes (Mejora, Optimizacion, Peligroso)

