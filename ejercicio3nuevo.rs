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
                if (self.anio < otra_fecha.anio){
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
        



fn main(){
    let mut nuevafecha = Fecha ::new(23,12,2025);
    println!("{}", nuevafecha.es_fecha_valida());
    nuevafecha.sumar_dias(43);
    println!("La fecha de hoy es {} de {} de {}", nuevafecha.dia, nuevafecha.mes, nuevafecha.anio);
}