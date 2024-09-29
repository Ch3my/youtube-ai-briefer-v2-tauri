## Embedding Exe

https://tauri.app/v1/guides/building/sidecar/

El binario debe tener un sufijo que depende de SO. En caso de windows es `-x86_64-pc-windows-msvc`. El sufijo correspondiente a tu plataforma se puede obtener utilizando

`rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}`

El exe debe estar en src-tauri/binaries con su respectivo sufijo

## Compilar

Ejecutar `npm run tauri build` si todo esta bien quedan dos instaladores dentro de src-tauri/tarjet

## Configuraicon del Sidecar

Se tuvo que copiar el exe y la carpeta _internal a la raiz de src-tauri porque al incluir internal dentro de la carpeta binaries luego cuando tauri compila queda ai_brain.exe en root y binaries/_internal encontces el exe no cuentra sus recursos