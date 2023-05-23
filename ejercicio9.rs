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


          fn es_mayor(&self, otra_fecha:Fecha) ->bool{
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
          

            }
struct Veterinaria{
    nombre:String,
    direccion:String,
    id:i32,
    cola:VecDeque<Mascota>,
    registros:Vec<Registro>,
}
impl Veterinaria{
    fn new(unNombre:String, unaDireccion:String, unId:i32)-> Veterinaria{
        Veterinaria{
           nombre:unNombre,
           direccion:unaDireccion,
           id:unId,
           cola:VecDeque::new(),
           registros:Vec::new(),
        }
    }
    fn agregar_nueva_mascota(&mut self,unaMascota:Mascota){
        self.cola.push_front(unaMascota);
    }
    fn agregar_nueva_mascota_urgente(&mut self, unaMascota:Mascota){
        self.cola.push_back(unaMascota);
    }
    fn registrar_atencion(&mut self, unRegistro:Registro){
        self.registros.push(unRegistro);
    }
    fn atender_proximo(&mut self) -> bool{
         let atendido = self.cola.pop_back();
         match atendido{
            Some(a) => return true,
            None => return false,
         }
    }
    fn buscar_una_atencion(&self, unNombre:String, nombreMascota:String, unTelefono:i32) -> Option<&Registro>{
        let mut i:usize = 0;
        while i < self.registros.len() && self.registros[i].mascota.nombre != nombreMascota && self.registros[i].mascota.duenio.nombre != unNombre && self.registros[i].mascota.duenio.telefono != unTelefono{
            i = i + 1;
        }
        if self.registros[i].mascota.nombre == nombreMascota && self.registros[i].mascota.duenio.nombre == unNombre && self.registros[i].mascota.duenio.telefono == unTelefono{
            return Some(&self.registros[i])
        }else{return None}
    }
    fn eliminar_mascota(&mut self, unaMascota:Mascota) ->bool{
        let mut eliminado:bool = false;
        let mut i:usize = 0;
        while i < self.cola.len() && !self.cola[i].Eq(&unaMascota) {
            i = i + 1;
        }
        if self.cola[i].Eq(&unaMascota){
            let mascotilla = self.cola.remove(i);
            eliminado = true;
        }
        return eliminado
    }
    fn modificar_diagnostico_registro(&mut self,un_registro:Registro, nuevo_diagnostico:String)->bool{
        let mut i:i32 = 0;
        let mut encontrado:bool = false;
        while (i as usize) < self.registros.len() && !self.registros[i as usize].Eq(&un_registro){
            i = i + 1;
        }
        if self.registros[i as usize].Eq(&un_registro){
           encontrado = true;
           self.registros[i as usize].modificar_diagnostico(nuevo_diagnostico);
        }
        return encontrado

    }

}
struct Duenio{
      nombre:String,
      direccion:String,
      telefono:i32,
}
impl Duenio{
    fn new(unNombre:String, unaDireccion:String, unTelefono:i32) -> Duenio{
        Duenio{
            nombre:unNombre,
            direccion:unaDireccion,
            telefono:unTelefono,
        }
    } 
    fn Eq(&self, otro:&Duenio) ->bool{
        let mut equivale:bool = false;
        if self.nombre == otro.nombre && self.direccion == otro.direccion && self.telefono == otro.telefono {
            equivale = true;
        }
        return equivale
    }
}
enum Animal{
    PERRO,
    GATO, 
    CABALLO,
    OTRO,
}
impl Animal{
    fn Eq(&self, otro:&Animal)->bool{
        match self{
            Animal::PERRO => match otro{
                Animal::PERRO => return true,
                _ => return false,
            }
            Animal::GATO => match otro{
                Animal::GATO => return true,
                _ => return false,
            }
            Animal::CABALLO => match otro{
                Animal::CABALLO => return true,
                _ => return false,
            }
            Animal::OTRO => match otro{
                Animal::OTRO => return true,
                _ => return false,
            }
        }
    }
}
struct Mascota{
    nombre:String,
    edad:i32,
    tipo:Animal,
    duenio:Duenio,
}
impl Mascota{
    fn new(unNombre:String, unaEdad:i32, unTipo:Animal, unDuenio:Duenio) ->Mascota{
           Mascota{
            nombre:unNombre,
            edad:unaEdad,
            tipo:unTipo,
            duenio:unDuenio,
           }
    }
    fn Eq(&self, otra:&Mascota) -> bool{
        let mut equivale:bool = false;
        if self.nombre == otra.nombre && self.edad == otra.edad && self.tipo.Eq(&otra.tipo)&&self.duenio.Eq(&otra.duenio){
            equivale = true;
        }
        return equivale
    }
}
struct Registro{
    mascota:Mascota,
    diagnostico:String,
    tratamiento:String,
    fecha_proxima_visita:Option<Fecha>,
}
impl Registro{
    fn new(unaMascota:Mascota, unDiagnostico:String, unTratamiento:String, unaFecha:Option<Fecha>)->Registro{
    Registro{
        mascota:unaMascota,
        diagnostico:unDiagnostico,
        tratamiento:unTratamiento,
        fecha_proxima_visita:unaFecha,
    }
}
    fn Eq(&self, otro:&Registro) ->bool{
        let mut equivale:bool = false;
        if self.mascota.Eq(&otro.mascota) && self.diagnostico == otro.diagnostico && self.tratamiento == otro.tratamiento{
              equivale = true;
        }
        return equivale
    }
   
   fn modificar_diagnostico(&mut self,nuevo_diagnostico:String){
        self.diagnostico = nuevo_diagnostico;
   }
}


fn main(){
    let mut veterinaria_villaelisa = Veterinaria::new("Villa Elisa".to_string(), "Frente a la plaza".to_string(), 3245);
    let unDuenioNuevo = Duenio::new("Nicolas".to_string(), "Calle 118 y 404".to_string(), 1141619459);
    let nuevaMascota = Mascota::new("Titan".to_string(), 19, Animal::PERRO, unDuenioNuevo);
    veterinaria_villaelisa.agregar_nueva_mascota(nuevaMascota);
    let aux = veterinaria_villaelisa.cola.pop_back();
    match aux{
       Some(a) => println!("La mascota {} vino con su dueño {} y estan siendo atendidos",a.nombre, a.duenio.nombre ),
       None => println!("No se encontro a la mascota"),
    }
    
}