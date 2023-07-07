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
                assert_eq!(fecha2.es_mayor(&fecha1), true);
                fecha1.sumar_dias(23);
                fecha1.restar_dias(34);
                assert_eq!(fecha2.es_bisiesto(), false);
                assert_eq!(fecha2.es_fecha_valida(), true);
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
    fn Clone(&self) ->Cliente{
        let clon = Cliente::new(self.nombre.clone(), self.telefono.clone(), self.direccion.clone());
        return clon
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
    fn Clone(&self) -> Genero{
        match self{
            Genero::NOVELA => return Genero::NOVELA,
            Genero::INFANTIL => return Genero::INFANTIL,
            Genero::TECNICO => return Genero::TECNICO,
            Genero::OTRO => return Genero::OTRO,
            }
        }
    
}
struct Libro{
    codigo_identificador:i32,
    titulo:String,
    autor:String,
    cantidad_paginas:i32,
    genero:Genero,
}

impl Libro{
    fn new(un_cod:i32,unTitulo:String, unAutor:String, unaCantidad:i32, unGenero:Genero) -> Libro{
        Libro{
            codigo_identificador:un_cod,
            titulo:unTitulo,
            autor:unAutor,
            cantidad_paginas:unaCantidad,
            genero:unGenero,
        }
    }
    fn Eq(&self, otro:&Libro) ->bool{
        let mut equivale:bool = false;
        if self.codigo_identificador == otro.codigo_identificador{
            equivale = true;
        }
        return equivale
    }
    fn Clone(&self) -> Libro{
        let clon = Libro::new(self.codigo_identificador.clone(), self.titulo.clone(), self.autor.clone(), self.cantidad_paginas.clone(), self.genero.Clone());
        return clon
    }
}
struct Prestamo{
    codigo_libro:i32,
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
        if self.codigo_libro == otro.codigo_libro && self.cliente.Eq(&otro.cliente) {
                equivale = true;
        }
        return equivale
    }
    fn clone(&self) -> Prestamo{

        let clon = Prestamo::new(self.codigo_libro.clone(), self.cliente.Clone(), self.fecha_vencimiento.clonar(), self.fecha_devolucion.clonar(), self.estado.Clonar());
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

struct Biblioteca{
    nombre: String,
    direccion: String,
    libros: Vec<Registro>,
    prestamos: VecDeque<Prestamo>,
}

impl Biblioteca {

    fn new(unNombre:String, unaDireccion:String) ->Biblioteca{
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
        while (i as usize) < self.libros.len()  && !self.libros[i as usize].libro.Eq(&otro_libro){
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

    fn agregar_nuevotitulo_a_registros(&mut self, un_registro:Registro){
        self.libros.push(un_registro);
    }

    fn realizar_prestamo(&mut self, un_libro:Libro, un_cliente:Cliente, fecha_dev: Fecha, fecha_ven: Fecha) -> bool {
        if self.cantidad_prestamos_cliente(&un_cliente) > 5 {
            return false;
        } else {
            if self.obtener_cantidad_copias(&un_libro) == 0 {
                return false;
            } else {
                let mut pos: i32 = 0;
                self.buscar_libro_pos(&un_libro, &mut pos);
                self.libros[pos as usize].cantidad_copias -= 1;
                
                let un_prestamo = Prestamo::new(un_libro.codigo_identificador, un_cliente, fecha_ven, fecha_dev, Estado::ENPRESTAMO);
                self.prestamos.push_front(un_prestamo);
                
                return true;
            }
        }
    }
    
    fn prestamos_a_vencer(&self, fecha_actual:Fecha, dias:i32) -> Vec<Prestamo>{
        let mut fecha_limite = fecha_actual.clonar();
        fecha_limite.sumar_dias(dias);
        let mut prestamos_a_vencerse = Vec::new();
        let mut i:i32 = 0;
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].fecha_vencimiento.es_mayor(&fecha_actual) && fecha_limite.es_mayor(&self.prestamos[i as usize].fecha_vencimiento){
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
            if dias.es_mayor(&self.prestamos[i as usize].fecha_vencimiento) && self.prestamos[i as usize].estado.Eq(&Estado::ENPRESTAMO){
                prestamos_vencidos.push(self.prestamos[i as usize].clone());
            }
            i = i +1;
        }
        return prestamos_vencidos
    }
    fn buscar_prestamo(&self, un_libro:&Libro, un_cliente:&Cliente) -> Option<&Prestamo>{
            let mut i:i32 = 0;
            let mut el_prestamo = std::option::Option::None;
            if self.prestamos.len() > 0{
            while (i as usize) < self.prestamos.len() && !self.prestamos[i as usize].cliente.Eq(un_cliente){
                i = i + 1;
            }
            if self.prestamos[i as usize].cliente.Eq(un_cliente){
                el_prestamo = std::option::Option::Some(&self.prestamos[i as usize]);
            }
        }
            return el_prestamo
    }
    fn devolver_libro(&mut self, un_libro:&Libro, un_cliente:&Cliente, fecha_actual:&Fecha) -> bool{
          let mut hecho:bool = false;
          let mut i:i32 = 0;
          while (i as usize) < self.prestamos.len() && self.prestamos[i as usize].codigo_libro != un_libro.codigo_identificador{
            i = i + 1;
          }
          if self.prestamos[i as usize].codigo_libro == un_libro.codigo_identificador{
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
    fn emails_de_clientes_que_tuvieron_mora(&self) ->Vec<String>{
        let mut i:i32 = 0;    //INCIALIZO CONTADO EN 0
        let mut vector_tuvieron_demora = Vec::new(); //INICIALIZO EL VECTOR
        while (i as usize) < self.prestamos.len(){
            if self.prestamos[i as usize].fecha_devolucion.es_mayor(&self.prestamos[i as usize].fecha_vencimiento){   //SI LA FECHA DE DEVOLUCION ES MAYOR A LA DE VENCIMIENTO
                let mut mail = String::new();          //CREO VARIABLE STRING
                mail = self.prestamos[i as usize].cliente.direccion.clone();   //CLONO LOS DATOS DEL MAIL DEL CLIENTE DEL PRESTAMO
                vector_tuvieron_demora.push(mail);//AGREGO AL VECTOR
           }
           i = i + 1;    
           }
        for i in 0..vector_tuvieron_demora.len(){  //DE 0 HASTA EL FINAL DEL VECTOR
            let mut aux = String::new();  //CREO VARIABLE STRING
            aux = vector_tuvieron_demora[i].clone();    //COPIO EL PRIMER ELEMENTO
            for j in (i + 1)..vector_tuvieron_demora.len(){    //DESDE EL SIGUIENTE HASTA EL FINAL DEL VECTOr       
                    while aux  == vector_tuvieron_demora[j]{ //SI SE REPITE LO ELIMINO CUANTAS VECES SEA NECESARIO
                    vector_tuvieron_demora.remove(j);
                }
            }

        }
    
    return vector_tuvieron_demora
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
    let registro1 = Registro::new(libro1.Clone(), 1);
    let registro2 = Registro::new(libro2.Clone(), 1);
    una_biblioteca.agregar_nuevotitulo_a_registros(registro1);
    una_biblioteca.agregar_nuevotitulo_a_registros(registro2);
   let prestamo1_logrado:bool = una_biblioteca.realizar_prestamo(libro1.Clone(), cliente1.Clone(), Fecha::new(24, 02, 2024),Fecha::new(28, 02, 2024));
    assert_eq!(prestamo1_logrado, true);
    let otro_prestamo = una_biblioteca.buscar_prestamo(&libro1, &cliente1);
   let prestamos_vencidos1 = una_biblioteca.prestamos_vencidos(Fecha::new(22,6,23));
   let prestamos_a_vencer1 = una_biblioteca.prestamos_a_vencer(Fecha::new(26,6,23), 7);
   let clientes_con_mora = una_biblioteca.emails_de_clientes_que_tuvieron_mora();

}
fn main(){

}