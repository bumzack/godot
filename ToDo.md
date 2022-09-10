# Things ToDo

## reference impl
- make the uv mapping functions exchangable (currently they are hardcoded in the traits, but one shoould be able to use the ImageTexture functionality with a plane or a sphere or a ...)
- this was omitted, due the generics madness that happened -> find a better/easier way
- image texture mapping not working as expected

## future ToDos

- make a Vec with configured backends in Backend. add a ```get_backends()``` which returns a  list
of available backends. add a ```get_backend(enumType)  -> Result<Backend, Error>``` to get a backend

 
- fix math crate: depending on cuda feature, use std::powf etc or use intrinsinics
- fix rand import or make own simple generator  - its uncritical