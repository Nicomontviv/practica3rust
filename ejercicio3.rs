struct Fecha{
    dia:i32,
    mes:i32,
    anio:i32,
}
impl Fecha{
    fn new(unDia:i32, unMes:i32, unAnio:i32)-> Fecha{
        Fecha{
         dia:unDia,
         mes:unMes,
         anio:unAnio,
        }
    }
    fn es_fecha_valida(&self) -> bool{
        let mut valida:bool = false;
        if self.dia > 0 && self.dia < 31{
            if self.mes > 0 && self.mes < 13 {
                if self.anio > 0 && self.anio < 3000{
                    valida = true;
                }
            }
        }
        return valida
    }
    fn es_bisiesto(&self) ->bool{
        let mut bisiesto:bool = false;
        if self.anio % 4 == 0{
            bisiesto = true
        }
        return bisiesto
    }
    fn sumar_dias(&mut self, suma: &i32){
        self.dia = self.dia + suma;
        if self.dia > 31{
            while self.dia > 31 {
                self.mes = self.mes + 1;
                self.dia = self.dia - 31;
            }
            if self.mes > 12{
                while self.mes > 12 {
                    self.anio = self.anio + 1;
                    self.mes = self.mes - 12;
                }
            }
        }
    }
    fn imprimir(&self){
        println!("La fecha es {} del {} de {}", self.dia, self.mes, self.anio);
    }
}
fn main(){
    let mut fecha1 = Fecha::new(14, 3,2023);
    fecha1.imprimir();
    fecha1.sumar_dias(&400);
    fecha1.imprimir();
}