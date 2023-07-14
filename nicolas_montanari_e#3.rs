#![allow(unused)]
#[macro_use]


extern crate serde;
extern crate serde_json;

use std::io::BufReader;
use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::Deserializer;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::vec::Vec;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Fecha{
    dia:i32,
    mes:i32,
    anio:i32,
}
impl Fecha{
     fn new(unDia:i32, unMes:i32, unAnio:i32) ->Fecha{
          Fecha{
            dia:unDia,
            mes:unMes,
            anio:unAnio,
          }
     }

     fn es_bisiesto(&self) ->bool{
        let mut bisiesto:bool = false;
        if self.anio % 4 == 0 {
        bisiesto = true;
        }

        return bisiesto
     }

     fn es_fecha_valida(&self) ->bool{
        let mut valido:bool = false;
        if self.anio > 0 && self.anio < 5000{             
                  if self.mes <= 12 && self.mes > 0 {
                       if self.mes == 2 {     //SI CAE FEBRERO
                          if self.es_bisiesto(){      //SI ES BISIESTO
                         if self.dia > 0 && self.dia < 30{
                            valido = true;
                         }
                        }else{     // si no es bisiesto
                            if self.dia > 0 && self.dia < 29{
                                valido = true;
                             }
                        }
                       } else if self.mes < 8{  //si cae en un mes antes de Agosto
                             if self.mes % 2 == 0{  //si es par
                                if self.dia > 0 && self.dia <= 30{
                                    valido = true;
                                 } 
                             }else{  //si es impar
                                if self.dia > 0 && self.dia <= 31 {
                                    valido = true;
                                 }
                             }
                       } else { //si cae de Agosto en adelante
                        if self.mes % 2 == 0{  //si es par
                            if self.dia > 0 && self.dia <= 31{
                                valido = true;
                             } 
                         }else{  //si es impar
                            if self.dia > 0 && self.dia <= 30 {
                                valido = true;
                             }
                         }
                       }
                  }
              }
          return valido

        }

        fn sumar_dias(&mut self, mut suma_dias: i32){
               while suma_dias != 0 {
                  self.dia = self.dia + 1;
                  suma_dias = suma_dias - 1;
                  if self.mes == 2{  //SI EL MES ES FEBRERO
                    if self.es_bisiesto() {  //si es año bisiesto
                        if self.dia == 30 { 
                              self.dia = 1;
                              self.mes = 3;
                        }
                    }else{  //si no es bisiesto
                       if self.dia == 29 {
                          self.dia = 1;
                          self.mes = 3
                       }
                    }
                  }else {  //SI NO ES FEBRERO
                      if self.mes <= 7 {  //SI ESTA ENTRE ENERO Y JULIO
                        if self.mes % 2 == 0 { //SI ES PAR Y NO ES FEBRERO SOLO TIENE 30 DIAS
                          if self.dia == 31{
                            self.dia = 1;
                            self.mes = self.mes + 1;
                          }
                        }else{  //SI ES IMPAR ENTRE ENERO Y JULIO TIENE 31 DIAS
                            if self.dia == 32{
                                self.dia = 1;
                                self.mes = self.mes + 1;
                            }
                        }
                      }else{ //SI ESTA ENTRE AGOSTO Y DICIEMBRE
                         if self.mes % 2 == 0 { //SI ES PAR TIENE 31 DIAS
                             if self.dia == 32{
                                self.dia = 1;
                                self.mes = self.mes + 1;
                             } 
                             }else{   //SI ES IMPAR TIENE 30 DIAS
                                if self.dia == 31 {
                                    self.dia = 1;
                                    self.mes = self.mes + 1;
                                }
                             }
                         }
                      }
                      if self.mes == 13 {
                        self.mes = 1;
                        self.anio = self.anio + 1;
                       }
                  }
               }

               fn restar_dias(&mut self,  mut resta_dias:i32){
                while resta_dias != 0 {
                   self.dia = self.dia -1;
                   resta_dias = resta_dias - 1;
                   if self.mes == 3{  //SI EL MES ES MARZO
                     if self.es_bisiesto() {  //si es año bisiesto
                         if self.dia == 0 { 
                               self.dia = 29;
                               self.mes = 2;
                         }
                     }else{  //si no es bisiesto
                        if self.dia == 0 {
                           self.dia = 28;
                           self.mes = 2;
                        }
                     }
                   }else {  //SI NO ES FEBRERO
                       if self.mes <= 7 {  //SI ESTA ENTRE ENERO Y JULIO
                         if self.mes % 2 == 0 { //SI ES PAR Y NO ES FEBRERO SOLO TIENE 30 DIAS
                           if self.dia == 0{
                             self.dia = 30;
                             self.mes = self.mes - 1;
                           }
                         }else{  //SI ES IMPAR ENTRE ENERO Y JULIO TIENE 31 DIAS
                             if self.dia == 0{
                                 self.dia = 31;
                                 self.mes = self.mes - 1;
                             }
                         }
                       }else{ //SI ESTA ENTRE AGOSTO Y DICIEMBRE
                          if self.mes % 2 == 0 { //SI ES PAR TIENE 31 DIAS POR LO TANTO EL ANTERIOR 30
                              if self.dia == 0{
                                 self.dia = 30;
                                 self.mes = self.mes - 1;
                              } 
                              }else{   //SI ES IMPAR TIENE 30 DIAS POR LO TANTO EL ANTERIOR 31
                                 if self.dia == 0 {
                                     self.dia = 31;
                                     self.mes = self.mes -1 ;
                                 }
                              }
                          }
                       }
                       if self.mes == 0 {
                         self.mes = 12;
                         self.anio = self.anio - 1;
                        }
                   }
                }


          fn es_mayor(&self, otra_fecha:&Fecha) ->bool{
                let mut mayor:bool = false;
                if self.anio > otra_fecha.anio{
                  mayor = true;
                }else if self.anio == otra_fecha.anio{
                  if self.mes > otra_fecha.mes{
                    mayor = true;
                  }else if self.mes == otra_fecha.mes{
                    if self.dia > otra_fecha.dia{
                      mayor = true;
                    }
                  }
                }
                return mayor
          }
            fn clonar(&self) ->Fecha{
                let clon = Fecha::new(self.dia, self.mes, self.anio);
                return clon
            }

            }

            #[test]
            fn test_fecha(){
                let mut fecha1 = Fecha::new(22,4,6);
                let fecha2 = Fecha::new(2,3,34);
                let fecha3 = Fecha::new(23,5,2224);
                assert_eq!(fecha3.es_bisiesto(), true);
                assert_eq!(fecha3.es_fecha_valida(), true);
                let fecha4 = Fecha::new(21,8,2220);
                assert_eq!(fecha4.es_bisiesto(), true); 
                assert_eq!(fecha2.es_mayor(&fecha1), true);
                fecha1.sumar_dias(23);
                fecha1.restar_dias(34);
                assert_eq!(fecha2.es_bisiesto(), false);
                assert_eq!(fecha2.es_fecha_valida(), true);
                let fecha5 = Fecha::new(37,5,18);
                assert_eq!(fecha5.es_fecha_valida(), false);
                let mut fecha6 = Fecha::new(29,2,2224);
                assert_eq!(fecha6.es_fecha_valida(), true);
                assert_eq!(fecha6.es_bisiesto(), true);
                fecha6.restar_dias(543);
                fecha6.sumar_dias(345);
                assert_eq!(fecha6.es_fecha_valida(), true);
                let fecha7 = Fecha::new(24,2,2223);
                assert_eq!(fecha7.es_bisiesto(), false);
                assert_eq!(fecha7.es_fecha_valida(), true);
                let fecha8 = Fecha::new(21,9, 2225);
                assert_eq!(fecha8.es_bisiesto(), false);
                assert_eq!(fecha8.es_fecha_valida(), true);
            }


#[derive(Serialize, Deserialize,Clone, Debug, PartialEq)]
pub struct Cliente{
    nombre:String,
    telefono:i32,
    direccion:String,
}
impl Cliente{
    fn new(unNombre:String, unTelefono:i32, unaDireccion:String)-> Cliente{
         Cliente{
            nombre:unNombre,
            telefono:unTelefono,
            direccion:unaDireccion,
         }
    }
    
}
#[test]
    fn test_eq(){
        let cliente1 = Cliente::new("Marcos".to_string(), 1234, "Calle 118".to_string());
        let cliente2 = Cliente::new("Juan".to_string(),4321, "Otro lugar".to_string());
        let cliente3 = Cliente::new("Marcos".to_string(), 1234, "Calle 118".to_string());
        assert_eq!(cliente1, cliente3);
        assert_ne!(cliente1, cliente2);
    }
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Genero{
    NOVELA,
    INFANTIL,
    TECNICO,
    OTRO,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]

pub struct Libro{
    codigo_identificador:i32,
    titulo:String,
    autor:String,
    cantidad_paginas:i32,
    genero:Genero,
}

impl Libro{
    fn new(un_codigo:i32, unTitulo:String, unAutor:String, unaCantidad:i32, unGenero:Genero) -> Libro{
        Libro{
            codigo_identificador:un_codigo,
            titulo:unTitulo,
            autor:unAutor,
            cantidad_paginas:unaCantidad,
            genero:unGenero,
        }
    }
    fn Eq(&self, otro:&Libro) ->bool{
        let mut equivale:bool = false;
        if self.codigo_identificador == self.codigo_identificador{
            equivale = true;
        }
        return equivale
    }
}
#[test]
fn test_libros(){
    let libro1 = Libro::new(234,"untitulo".to_string(), "unautor".to_string(), 236, Genero::NOVELA);
    let libro2 = libro1.clone();
    assert_eq!(libro1, libro2);
    assert_eq!(libro1.Eq(&libro2), true);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Prestamo{
    codigo_libro: i32,
    cliente:Cliente,
    fecha_vencimiento:Fecha,
    fecha_devolucion:Fecha,
    estado:Estado,
}
impl Prestamo{
    fn new(unLibro:i32, unCliente:Cliente, unaFecha:Fecha, otraFecha:Fecha, unEstado:Estado) -> Prestamo{
        Prestamo{
            codigo_libro:unLibro,
            cliente:unCliente,
            fecha_vencimiento:unaFecha,
            fecha_devolucion:otraFecha,
            estado:unEstado,
        }
    }
    fn Eq(&self, otro:&Prestamo) -> bool{
        let mut equivale:bool = false;
        if self.codigo_libro == otro.codigo_libro && self.cliente.eq(&otro.cliente) {
                equivale = true;
        }
        return equivale
    }
    fn clone(&self) -> Prestamo{
        let clon = Prestamo::new(self.codigo_libro, self.cliente.clone(), self.fecha_vencimiento.clonar(), self.fecha_devolucion.clonar(), self.estado.Clonar());
        return clon
    }
}

//TEST PRESTAMOS
#[test]
fn test_prestamos(){
    let libro1 = Libro::new(234,"La Biblia".to_string(), "Gabriel Marquez".to_string(), 324, Genero::NOVELA);
    let cliente1 = Cliente::new("Carlos Uno".to_string(), 4731812,"carlos_uno@hotmail.com".to_string());
    let prestamo1 = Prestamo::new(libro1.codigo_identificador, cliente1, Fecha::new(23,6,24), Fecha::new(29,7,24), Estado::ENPRESTAMO);
    let mut prestamo2 = prestamo1.clone();
    assert_eq!(prestamo1, prestamo2);
    prestamo2.estado = Estado::DEVUELTO;
    let un_estado = prestamo2.estado.Clonar();
    assert_eq!(prestamo1.Eq(&prestamo2),true);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Registro{
    libro:Libro,
    cantidad_copias:i32,
}
 impl Registro{
    fn new(unLibro:Libro, unaCantidad:i32) -> Registro{
        Registro{
            libro:unLibro, 
            cantidad_copias:unaCantidad,
        }
    } 
    fn Eq(&self, otro:&Registro) -> bool{
        let mut equivale:bool = false;
        if self.libro.Eq(&otro.libro) {
            equivale = true;
        }
        return equivale
    }
}

//TEST REGISTROS
#[test]

fn test_registro(){
    let registro1 = Registro::new(Libro::new(233,"La Biblia".to_string(), "Gabriel ".to_string(), 324, Genero::NOVELA), 1);
    let registro2 = Registro::new(Libro::new(233,"La Biblia".to_string(), "Gabriel ".to_string(), 324, Genero::NOVELA), 1);
    assert_eq!(registro1, registro2);
    assert_eq!(registro1.Eq(&registro2), true);
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Estado{
    DEVUELTO,
    ENPRESTAMO,
}
impl Estado{

 
 fn Clonar(&self) ->Estado{
    match self{
        Estado::DEVUELTO => return Estado::DEVUELTO,
        Estado::ENPRESTAMO => return Estado::ENPRESTAMO,
    }
 }
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]

 struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: Vec<Registro>,
    prestamos: VecDeque<Prestamo>,
}

 impl Biblioteca {

    fn new(unNombre:String, unaDireccion:String) ->Biblioteca{
        File::create("src/archivo_registros.json");
        File::create("src/archivo_prestamos.json");

         
       let registros: Vec<Registro> = Vec::new();


    let registros_json = serde_json::to_string(&registros).unwrap();

    
        let mut archivo = File::create("src/archivo_registros.json").unwrap();
        archivo.write_all(registros_json.as_bytes()).unwrap();


        let inicio_prestamos: VecDeque<Prestamo> = VecDeque::new();


       let prestamos_json = serde_json::to_string(&inicio_prestamos).unwrap();

    
        let mut archivo2 = File::create("src/archivo_prestamos.json").unwrap();
        archivo2.write_all(prestamos_json.as_bytes()).unwrap();

         Biblioteca{
            nombre:unNombre,
            direccion:unaDireccion,
            libros:Vec::new(),
            prestamos:VecDeque::new(),
        }
       
    }
    /*fn obtener_cantidad_copias(&mut self, otro_libro:&Libro) -> i32{      //ESTE ES EL METODO ORIGINAL DE LA PRACTICA 3
        let mut i:i32 = 0;
        let mut cant:i32 = 0;
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(&otro_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(&otro_libro){
            cant = self.libros[i as usize].cantidad_copias;
        }
        return cant

    }*/
    fn obtener_cantidad_copias(&mut self, otro_libro: &Libro) -> i32 {
        let mut cant: i32 = 0;

        let path = "src/archivo_registros.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

        let vec_registros: Vec<Registro> = serde_json::from_str(&contenido.as_str()).unwrap();
        for registro in vec_registros {
            if registro.libro.Eq(&otro_libro) {
                cant = registro.cantidad_copias;
                break;
            }
        }

        cant
    }

    
    /*fn decrementar_cantidad_copias(&mut self, otro_libro:&Libro) -> bool{    //METODO ORIGINAL DE LA PRACTICA 3

        let mut logrado:bool = false;
        let mut i:i32 = 0;

        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(&otro_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(&otro_libro) {
             logrado = true;
             self.libros[i as usize].cantidad_copias =  self.libros[i as usize].cantidad_copias - 1;
        }
        return logrado
    }*/
    fn decrementar_cantidad_copias(&mut self, otro_libro: &Libro) -> bool {
        let mut logrado: bool = false;
        let mut i: i32 = 0;
    
        let path = "src/archivo_registros.json";
        let mut file = File::open(path).expect("No se pudo abrir el archivo");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("No se pudo leer el archivo");
    
        
        self.libros = serde_json::from_str(&contents.as_str()).unwrap();
        if self.libros.len() > 0{
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(otro_libro) {
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(otro_libro) && self.libros[i as usize].cantidad_copias > 0 {
            logrado = true;
            self.libros[i as usize].cantidad_copias = self.libros[i as usize].cantidad_copias - 1;
            let actualizacion = serde_json::to_string(&self.libros).unwrap();

            let mut archivo_actualizado = File::create("src/archivo_registros.json").expect("No se pudo crear el archivo");
            archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");
        }
       }
        return logrado
        
        
    }

    fn incrementar_cantidad_copias(&mut self, otro_libro:&Libro) -> bool{
        let mut logrado: bool = false;
        let mut i: i32 = 0;
    
        let path = "src/archivo_registros.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

        self.libros = serde_json::from_str(&contenido.as_str()).unwrap();
        if self.libros.len() > 0 {
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(&otro_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(&otro_libro) {
             logrado = true;
             self.libros[i as usize].cantidad_copias =  self.libros[i as usize].cantidad_copias + 1;
             let actualizacion = serde_json::to_string(&self.libros).unwrap();

             let mut archivo_actualizado = File::create("src/archivo_registros.json").expect("No se pudo crear el archivo");
             archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");
        }
       }
    
        return logrado
    }

    fn buscar_libro_pos(&mut self,un_libro:&Libro, pos:&mut i32) ->bool{
       
        let mut i: i32 = 0;
        let mut encontrado:bool = false;
        let path = "src/archivo_registros.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

        self.libros = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");
        if self.libros.len() > 0 {
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(un_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(un_libro){
            *pos = i;
            encontrado = true;
        }
        }
        return encontrado
    }

    /*fn cantidad_prestamos_cliente(&self, unCliente:&Cliente) ->i32{ //METODO ORIGINAL DE LA PRACTICA 3
        let mut cant:i32 = 0;
        let mut i:i32 = 0;
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].cliente.Eq(unCliente){
                if self.prestamos[i as usize].estado.Eq(&Estado::ENPRESTAMO){
                    cant = cant + 1;
                }
            }
            i = i + 1;
        }
        return cant
    }*/

    fn cantidad_prestamos_cliente(&mut self, unCliente: &Cliente) -> i32 {
        let mut cant: i32 = 0;
        let mut i: i32 = 0;

       
        let path = "src/archivo_prestamos.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

       
        self.prestamos = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");

        while (i as usize) < self.prestamos.len() {
            if self.prestamos[i as usize].cliente.eq(unCliente) {
                if self.prestamos[i as usize].estado == Estado::ENPRESTAMO {
                    cant = cant + 1;
                }
            }
            i = i + 1;
        }
        return cant;
    }

   /*  fn realizar_prestamo(&mut self, un_libro: &'a Libro, un_cliente: &'a Cliente, fecha_dev: Fecha, fecha_ven: Fecha) -> bool {  //MODULO ORIGINAL DE LA PRACTICA 3
        if self.cantidad_prestamos_cliente(un_cliente) > 5 {
            return false;
        } else {
            if self.obtener_cantidad_copias(un_libro) == 0 {
                return false;
            } else {
                let mut pos: i32 = 0;
                self.buscar_libro_pos(un_libro, &mut pos);
                self.libros[pos as usize].cantidad_copias =   self.libros[pos as usize].cantidad_copias - 1;
                
                let un_prestamo = Prestamo::new(un_libro, un_cliente, fecha_ven, fecha_dev, Estado::ENPRESTAMO);
                self.prestamos.push_front(un_prestamo);
                
                return true;
            }
        }
    }*/
    fn agregar_nuevotitulo_a_registros(&mut self, un_registro:Registro){
        let path = "src/archivo_registros.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

        self.libros = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");
        self.libros.push(un_registro);
        let actualizacion = serde_json::to_string(&self.libros).expect("No se pudo serealizar");

       let mut archivo_actualizado = File::create("src/archivo_registros.json").expect("No se pudo crear el archivo");
       archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");
    }


    fn realizar_prestamo(&mut self, un_libro:&Libro, un_cliente:&Cliente, fecha_dev: Fecha, fecha_ven: Fecha) -> bool {
        if self.cantidad_prestamos_cliente(un_cliente) > 5 {
            return false;
        } else {
            if self.obtener_cantidad_copias(un_libro) == 0 {
                return false;
            } else {
                let mut pos: i32 = 0;
                self.buscar_libro_pos(un_libro, &mut pos);
                let path = "src/archivo_registros.json";
                let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
                let mut contenido = String::new();
                archivo.read_to_string(&mut contenido).expect("No se pudo deserializar el archivo");
        
                self.libros = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");

                self.libros[pos as usize].cantidad_copias =   self.libros[pos as usize].cantidad_copias - 1;
                
                let actualizacion = serde_json::to_string(&self.libros).expect("No se pudo serealizar");

                let mut archivo_actualizado = File::create("src/archivo_registros.json").expect("No se pudo crear el archivo");
                archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");

                let un_prestamo = Prestamo::new(un_libro.codigo_identificador, un_cliente.clone(), fecha_ven, fecha_dev, Estado::ENPRESTAMO);
                let path2 = "src/archivo_prestamos.json";
                let mut archivo2 = File::open(path2).expect("No se pudo abrir el archivo");
                let mut contenido2 = String::new();
                 archivo2.read_to_string(&mut contenido2).expect("No se pudo leer el archivo");

       
                self.prestamos = serde_json::from_str(&contenido2.as_str()).expect("No se pudo parsear el archivo");
                self.prestamos.push_front(un_prestamo);
                let actualizacion = serde_json::to_string(&self.prestamos).expect("No se pudo serealizar el archivo");
                let mut archivo_actualizado = File::create("src/archivo_prestamos.json").expect("No se pudo crear el archivo");
                archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");
                 return true
            }
        }
    }
    
    fn prestamos_a_vencer(&mut self, fecha_actual:Fecha, dias:i32) -> Vec<Prestamo>{
        let mut fecha_limite = fecha_actual.clonar();
        fecha_limite.sumar_dias(dias);
        let mut prestamos_a_vencerse = Vec::new();
        let mut i:i32 = 0;
        let path = "src/archivo_prestamos.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
         archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

  
        self.prestamos = serde_json::from_str(&contenido).expect("No se pudo parsear el archivo");
        if self.prestamos.len() > 0{
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].fecha_vencimiento.es_mayor(&fecha_actual) && fecha_limite.es_mayor(&self.prestamos[i as usize].fecha_vencimiento){
                if self.prestamos[i as usize].estado.eq(&Estado::ENPRESTAMO){
                prestamos_a_vencerse.push(self.prestamos[i as usize].clone());
                }
            }
            i = i + 1;
        }
         }
        return prestamos_a_vencerse
    }

    fn prestamos_vencidos(&mut self, dias:Fecha) -> Vec<Prestamo>{
        let mut prestamos_vencidos = Vec::new();
        let mut i:i32 = 0;
        let path = "src/archivo_prestamos.json";
        let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
        let mut contenido = String::new();
         archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

        self.prestamos = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");
        if self.prestamos.len() > 0 {
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].fecha_vencimiento.es_mayor(&dias){
                if self.prestamos[i as usize].estado.eq(&Estado::ENPRESTAMO){
                prestamos_vencidos.push(self.prestamos[i as usize].clone());
                }
            }
            i = i +1;
        }
       }
        return prestamos_vencidos
    }

    fn buscar_prestamo(&mut self, un_libro:&Libro, un_cliente:&Cliente) -> Option<&Prestamo>{
            let mut i:i32 = 0;
            let mut el_prestamo = std::option::Option::None;
            let path = "src/archivo_prestamos.json";
         let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
           let mut contenido = String::new();
            archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");
           self.prestamos = serde_json::from_str(&contenido.as_str()).expect("No se pudo parsear el archivo");
           if self.prestamos.len() > 0{
              while (i as usize) < self.prestamos.len()  && !self.prestamos[i as usize].cliente.eq(un_cliente) && !self.prestamos[i as usize].codigo_libro != un_libro.codigo_identificador{
                i = i + 1;
              }
              if self.prestamos[i as usize].cliente.eq(un_cliente) && self.prestamos[i as usize].codigo_libro == un_libro.codigo_identificador{
                el_prestamo = std::option::Option::Some(&self.prestamos[i as usize]);
              }
            }
            return el_prestamo
    }

    fn devolver_libro(&mut self, un_libro:&Libro, un_cliente:&Cliente, fecha_actual:Fecha) -> bool{
          let mut hecho:bool = false;
          let mut i:i32 = 0;
          let mut path = "src/archivo_prestamos.json";
         let mut archivo = File::open(path).expect("No se pudo abrir el archivo");
           let mut contenido = String::new();
            archivo.read_to_string(&mut contenido).expect("No se pudo leer el archivo");

            self.prestamos = serde_json::from_str(&contenido).expect("No se pudo parsear el archivo");
            
          while (i as usize) < self.prestamos.len() && !self.prestamos[i as usize].codigo_libro == un_libro.codigo_identificador{
            i = i + 1;
          }
          if self.prestamos[i as usize].codigo_libro == un_libro.codigo_identificador{
            self.prestamos[i as usize].fecha_devolucion = fecha_actual.clonar();
            self.prestamos[i as usize].estado = Estado::DEVUELTO;
            
            let actualizacion = serde_json::to_string(&self.prestamos).expect("No se pudo serealizar el archivo");

            let mut archivo_actualizado = File::create("src/archivo_prestamos.json").expect("No se pudo crear el archivo");
            archivo_actualizado.write_all(actualizacion.as_bytes()).expect("No se pudo actualizar el archivo");
            
             let mut pos:i32 = 0;
            self.buscar_libro_pos(&un_libro, &mut pos);
            path = "src/archivo_registros.json";
            let mut file = File::open(path).expect("No se pudo abrir el archivo");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("No se pudo leer el archivo");
            
            self.libros = serde_json::from_str(&contents).expect("No se pudo parsear el archivo");
           
            

            self.libros[pos as usize].cantidad_copias = self.libros[pos as usize].cantidad_copias + 1;
            let actualizacion2 = serde_json::to_string(&self.libros).expect("No se pudo serealizar el archivo");

              let mut archivo_actualizado2 = File::create("src/archivo_registros.json").expect("No se pudo crear el archivo");
              archivo_actualizado2.write_all(actualizacion2.as_bytes()).expect("No se pudo actualizar el archivo ");
              hecho = true
          }else{
              hecho = false;
          }
          return hecho
    }
}
#[test]
fn biblioteca_test(){
    let mut una_biblioteca = Biblioteca::new("Gran Biblioteca".to_string(), "Camino centenario y 502".to_string()); 
    let libro1 = Libro::new(233,"La Biblia".to_string(), "Gabriel ".to_string(), 324, Genero::NOVELA);
    let libro2 = Libro::new(122,"Caballeros".to_string(), "Walker".to_string(), 526, Genero::NOVELA);
    let libro3 = Libro::new(124,"Ferrari".to_string(), "Robin hood".to_string(), 279, Genero::OTRO);
    let libro4 = Libro::new(126,"Amigos".to_string(), "Maria".to_string(), 80, Genero::INFANTIL);
    let libro5 = Libro::new(132,"Java".to_string(), "Weiss".to_string(), 630, Genero::TECNICO);
  
    let cliente1 = Cliente::new("Carlos Uno".to_string(), 4731812,"carlos_uno@hotmail.com".to_string());
    let cliente2 = Cliente::new("Maria Dos".to_string(), 4872020,"maria_dos@hotmail.com".to_string());
    let cliente3 = Cliente::new("Juan Tres".to_string(), 4921918,"maria_dos@hotmail.com".to_string());
    let registro1 = Registro::new(libro1.clone(), 4);
    let registro2 = Registro::new(libro2.clone(), 3);
    una_biblioteca.agregar_nuevotitulo_a_registros(registro1);
    una_biblioteca.agregar_nuevotitulo_a_registros(registro2);
   let prestamo1_logrado:bool = una_biblioteca.realizar_prestamo(&libro1, &cliente1, Fecha::new(24, 02, 2024),Fecha::new(28, 02, 2024));
    assert_eq!(prestamo1_logrado, true);
    let otro_prestamo = una_biblioteca.buscar_prestamo(&libro1, &cliente1);
    let prestamo2_logrado:bool = una_biblioteca.realizar_prestamo(&libro2, &cliente2, Fecha::new(28,6,2023), Fecha::new(17,7,2023));
   let prestamos_vencidos1 = una_biblioteca.prestamos_vencidos(Fecha::new(22,6,23));
   let prestamos_a_vencer1 = una_biblioteca.prestamos_a_vencer(Fecha::new(26,6,23), 4);
   let copias_decrementadas = una_biblioteca.decrementar_cantidad_copias(&libro1);
   let copias_decrementadas2 = una_biblioteca.decrementar_cantidad_copias(&libro2);
   assert_eq!(copias_decrementadas, true);
   assert_eq!(copias_decrementadas2, true);
   let copias_aumentadas = una_biblioteca.incrementar_cantidad_copias(&libro1);
   assert_eq!(copias_aumentadas, true);
   let libro_devuelto = una_biblioteca.devolver_libro(&libro2, &cliente2, Fecha::new(29,6,2023));
}

fn main(){
   
 
}