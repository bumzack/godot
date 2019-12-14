# Things ToDo
- make a Vec with configured backends in Backend. add a ```get_backends()``` which returns a  list
of available backends. add a ```get_backend(enumType)  -> Result<Backend, Error>``` to get a backend

 
- fix math crate: depending on cuda feature, use std::powf etc or use intrinsinics
- fix rand import or make own simple generator  - its uncritical

- math crate: in atan or atan2 i use fabsf instead of fabs - is this legal ?!