use std::collections::VecDeque;
use std::vec::Vec;
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
                                 } else{  //si es impar
                                    if self.dia > 0 && self.dia <= 31 {
                                        valido = true;
                                     }
                                 }
                             }
                       } else { //si cae de Agosto en adelante
                        if self.mes % 2 == 0{  //si es par
                            if self.dia > 0 && self.dia <= 31{
                                valido = true;
                             } else{  //si es impar
                                if self.dia > 0 && self.dia <= 30 {
                                    valido = true;
                                 }
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
                           self.mes = 1
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
                if self.anio < otra_fecha.anio{
                  mayor = true;
                }else if self.anio == otra_fecha.anio{
                  if self.mes < otra_fecha.mes{
                    mayor = true;
                  }else if self.mes == otra_fecha.mes{
                    if self.dia < otra_fecha.dia{
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

struct Cliente{
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
    fn Eq(&self, otro:&Cliente) -> bool{
        let mut equivale:bool = false;
        if self.nombre == otro.nombre && self.telefono == otro.telefono && self.direccion == otro.direccion{
            equivale = true;
        }
        return equivale
    }
}
enum Genero{
    NOVELA,
    INFANTIL,
    TECNICO,
    OTRO,
}
impl Genero{
    fn Eq(&self, otro:&Genero) -> bool{
        match self{
            Genero::NOVELA => match otro{
                Genero::NOVELA => return true,
                _ => return false,
            }
            Genero::INFANTIL => match otro{
                Genero::INFANTIL => return true,
                _ => return false,
            }
            Genero::TECNICO => match otro{
                Genero::TECNICO => return true,
                _ => return false,
            }
            Genero::OTRO => match otro{
                Genero::OTRO => return true,
                _ => return false,
            }
        }
    }
}
struct Libro{
    titulo:String,
    autor:String,
    cantidad_paginas:i32,
    genero:Genero,
}

impl Libro{
    fn new(unTitulo:String, unAutor:String, unaCantidad:i32, unGenero:Genero) -> Libro{
        Libro{
            titulo:unTitulo,
            autor:unAutor,
            cantidad_paginas:unaCantidad,
            genero:unGenero,
        }
    }
    fn Eq(&self, otro:&Libro) ->bool{
        let mut equivale:bool = false;
        if self.titulo == otro.titulo && self.autor == otro.autor && self.cantidad_paginas == otro.cantidad_paginas && self.genero.Eq(&otro.genero){
            equivale = true;
        }
        return equivale
    }
}
struct Prestamo<'a>{
    libro:&'a Libro,
    cliente:&'a Cliente,
    fecha_vencimiento:Fecha,
    fecha_devolucion:Fecha,
    estado:Estado,
}
impl<'a> Prestamo<'a>{
    fn new(unLibro:&'a Libro, unCliente:&'a Cliente, unaFecha:Fecha, otraFecha:Fecha, unEstado:Estado) -> Prestamo<'a>{
        Prestamo{
            libro:unLibro,
            cliente:unCliente,
            fecha_vencimiento:unaFecha,
            fecha_devolucion:otraFecha,
            estado:unEstado,
        }
    }
    fn Eq(&self, otro:&Prestamo) -> bool{
        let mut equivale:bool = false;
        if self.libro.Eq(&otro.libro) && self.cliente.Eq(&otro.cliente) {
                equivale = true;
        }
        return equivale
    }
    fn clone(&self) -> Prestamo<'a>{
        let clon = Prestamo::new(self.libro, self.cliente, self.fecha_vencimiento.clonar(), self.fecha_devolucion.clonar(), self.estado.Clonar());
        return clon
    }
}
struct Registro{
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
enum Estado{
    DEVUELTO,
    ENPRESTAMO,
}
impl Estado{
    fn Eq(&self, otro:&Estado) ->bool{
        match self{
            Estado::DEVUELTO => match otro{
                Estado::DEVUELTO => return true,
                _ => return false,
            }
            Estado::ENPRESTAMO => match otro{
                Estado::ENPRESTAMO => return true,
                _ => return false,
            }
    }
 }
 fn Clonar(&self) ->Estado{
    match self{
        Estado::DEVUELTO => return Estado::DEVUELTO,
        Estado::ENPRESTAMO => return Estado::ENPRESTAMO,
    }
 }
}

struct Biblioteca<'a> {
    nombre: String,
    direccion: String,
    libros: Vec<Registro>,
    prestamos: VecDeque<Prestamo<'a>>,
}

impl<'a> Biblioteca<'a> {

    fn new(unNombre:String, unaDireccion:String) ->Biblioteca<'static>{
        Biblioteca{
            nombre:unNombre,
            direccion:unaDireccion,
            libros:Vec::new(),
            prestamos:VecDeque::new(),
        }
    }
    fn obtener_cantidad_copias(&self, otro_libro:&Libro) -> i32{
        let mut i:i32 = 0;
        let mut cant:i32 = 0;
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(&otro_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(&otro_libro){
            cant = self.libros[i as usize].cantidad_copias;
        }
        return cant

    }
    fn decrementar_cantidad_copias(&mut self, otro_libro:&Libro) -> bool{
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
    }
    fn incrementar_cantidad_copias(&mut self, otro_libro:&Libro) -> bool{
        let mut logrado:bool = false;
        let mut i:i32 = 0;
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(&otro_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(&otro_libro) {
             logrado = true;
             self.libros[i as usize].cantidad_copias =  self.libros[i as usize].cantidad_copias + 1;
        }
        return logrado
    }
    fn buscar_libro_pos(&self,un_libro:&Libro, pos:&mut i32) ->bool{
        let mut i:i32 = 0;
        while (i as usize) < self.libros.len() && !self.libros[i as usize].libro.Eq(un_libro){
            i = i + 1;
        }
        if self.libros[i as usize].libro.Eq(un_libro){
            *pos = i;
            return true
        }else{
            return false
        }
    }
    fn cantidad_prestamos_cliente(&self, unCliente:&Cliente) ->i32{
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
    }
    fn realizar_prestamo(&mut self, un_libro: &'a Libro, un_cliente: &'a Cliente, fecha_dev: Fecha, fecha_ven: Fecha) -> bool {
        if self.cantidad_prestamos_cliente(un_cliente) > 5 {
            return false;
        } else {
            if self.obtener_cantidad_copias(un_libro) == 0 {
                return false;
            } else {
                let mut pos: i32 = 0;
                self.buscar_libro_pos(un_libro, &mut pos);
                self.libros[pos as usize].cantidad_copias -= 1;
                
                let un_prestamo = Prestamo::new(un_libro, un_cliente, fecha_ven, fecha_dev, Estado::ENPRESTAMO);
                self.prestamos.push_front(un_prestamo);
                
                return true;
            }
        }
    }
    
    fn prestamos_a_vencer(&self, dias:Fecha) -> Vec<Prestamo>{
        let mut prestamos_a_vencerse = Vec::new();
        let mut i:i32 = 0;
        while (i as usize) < self.prestamos.len(){
            if dias.es_mayor(&self.prestamos[i as usize].fecha_vencimiento){
                prestamos_a_vencerse.push(self.prestamos[i as usize].clone());
            }
            i = i +1;
        }
        return prestamos_a_vencerse
    }
    fn prestamos_vencidos(&self, dias:Fecha) -> Vec<Prestamo>{
        let mut prestamos_vencidos = Vec::new();
        let mut i:i32 = 0;
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].fecha_vencimiento.es_mayor(&dias){
                prestamos_vencidos.push(self.prestamos[i as usize].clone());
            }
            i = i +1;
        }
        return prestamos_vencidos
    }
    fn buscar_prestamo(&self, un_libro:&Libro, un_cliente:&Cliente) -> Option<&Prestamo>{
            let mut i:i32 = 0;
            let mut el_prestamo = std::option::Option::None;
            while (i as usize) < self.prestamos.len() && !self.prestamos[i as usize].cliente.Eq(un_cliente){
                i = i + 1;
            }
            if self.prestamos[i as usize].cliente.Eq(un_cliente){
                el_prestamo = std::option::Option::Some(&self.prestamos[i as usize]);
            }
            return el_prestamo
    }
    fn devolver_libro(&mut self, un_libro:&Libro, un_cliente:&Cliente, fecha_actual:&Fecha) -> bool{
          let mut hecho:bool = false;
          let mut i:i32 = 0;
          while (i as usize) < self.prestamos.len() && !self.prestamos[i as usize].libro.Eq(un_libro){
            i = i + 1;
          }
          if self.prestamos[i as usize].libro.Eq(un_libro){
            self.prestamos[i as usize].fecha_devolucion = fecha_actual.clonar();
            self.prestamos[i as usize].estado = Estado::DEVUELTO;
            let mut pos:i32 = 0;
            self.buscar_libro_pos(&un_libro, &mut pos);
            self.libros[pos as usize].cantidad_copias = self.libros[pos as usize].cantidad_copias + 1; 
            hecho = true
          }else{
              hecho = false;
          }
          return hecho
    }
}

fn main(){

}