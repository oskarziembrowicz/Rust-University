#[derive(Debug, Clone)]
enum Surowiec {
    Drewno(f32),
    Glina(f32),
    Zelazo(i32),
}

#[derive(PartialEq, Debug, Clone)]
enum Transport {
    Ciezarowka,
    Pociag,
}

#[derive(Clone)]
struct Magazyn {
    adres: String,
    pojemnosc: f32,
    posiadaTerminalKolejowy: bool,
}

impl Magazyn {
    fn new(adres: String, pojemnosc: f32, posiadaTerminalKolejowy: bool) -> Self {
        Magazyn {
            adres,
            pojemnosc,
            posiadaTerminalKolejowy,
        }
    }

    fn getAdres(self) -> String {
        self.adres
    }

    fn getPojemnosc(self) -> f32 {
        self.pojemnosc
    }

    fn czyPosiadaTerminalKolejowy(self) -> bool {
        self.posiadaTerminalKolejowy
    }

    fn setAdres(&mut self, adres: String) -> () {
        self.adres = adres;
    }

    fn setPojemnosc(&mut self, pojemnosc: f32) -> () {
        self.pojemnosc = pojemnosc;
    }

    fn setPosiadaTerminalKolejowy(&mut self, wartosc: bool) -> () {
        self.posiadaTerminalKolejowy = wartosc;
    }
}

#[derive(Clone)]
struct Przewoz {
    adresPoczatkowy: String,
    adresDocelowy: String,
    przewozonySurowiec: Surowiec,
    srodekTransportu: Transport,
}

impl Przewoz {
    fn new(adresPoczatkowy: String, adresDocelowy: String,
           przewozonySurowiec: Surowiec, srodekTransportu: Transport) -> Self {
        Przewoz {
            adresPoczatkowy,
            adresDocelowy,
            przewozonySurowiec,
            srodekTransportu,
        }
    }

    fn getAdresPoczatkowy(self) -> String {
        self.adresPoczatkowy
    }
    fn getAdresDocelowy(self) -> String {
        self.adresDocelowy
    }
    fn getPrzewozonySurowiec(self) -> Surowiec {
        self.przewozonySurowiec
    }
    fn getSrodekTransportu(self) -> Transport {
        self.srodekTransportu
    }

    fn setAdresPoczatkowy(&mut self, adres: String) -> () {
        self.adresPoczatkowy = adres;
    }
    fn setSrodekTransportu(&mut self, transport: Transport) -> () {
        self.srodekTransportu = transport;
    }

    fn zaladuj(&mut self, surowiec: Surowiec, transport: Transport) -> () {
        self.przewozonySurowiec = surowiec;
        self.srodekTransportu = transport;
    }

    fn dostawa_do(&mut self, magazyn: Magazyn) -> bool {
        if (self.srodekTransportu == Transport::Pociag) && !magazyn.clone().czyPosiadaTerminalKolejowy() {
            return false;
        }
        let mut masaTowaru = 0.0;
        match self.przewozonySurowiec {
            Surowiec::Zelazo(sztabki) => masaTowaru = sztabki as f32 * 0.005,
            Surowiec::Drewno(masa) => masaTowaru = masa,
            Surowiec::Glina(masa) => masaTowaru = masa,
        }
        if masaTowaru > magazyn.clone().getPojemnosc() {
            return false;
        }
        self.adresDocelowy = magazyn.clone().getAdres();
        true
    }
}

fn main() {
    let d = Surowiec::Drewno(3.6);
    let g = Surowiec::Glina(2.3);
    let z = Surowiec::Zelazo(5);
    println!("{:?}", d);
    println!("{:?}", g);
    println!("{:?}", z);

    let mut magazyn = Magazyn::new("Poligonowa 12".to_string(), 300.4, true);
    println!("\nAdres magazynu: {}", magazyn.clone().getAdres());
    println!("Pojemnosc magazynu w tonach: {}", magazyn.clone().getPojemnosc());
    println!("Czy magazyn posiada terminal kolejowy: {}", magazyn.clone().czyPosiadaTerminalKolejowy());
    magazyn.setAdres("Ruska 26".to_string());
    magazyn.setPojemnosc(500.0);
    magazyn.setPosiadaTerminalKolejowy(false);
    println!("\nAdres magazynu: {}", magazyn.clone().getAdres());
    println!("Pojemnosc magazynu w tonach: {}", magazyn.clone().getPojemnosc());
    println!("Czy magazyn posiada terminal kolejowy: {}", magazyn.clone().czyPosiadaTerminalKolejowy());


    let mut przewoz = Przewoz::new("Walecznych 14".to_string(), "Stokrotkowa 38".to_string(), Surowiec::Drewno(200.0), Transport::Ciezarowka);
    println!("\nPierwszy przewoz:\nAdres poczatkowy: {}", przewoz.clone().getAdresPoczatkowy());
    println!("Adres docelowy: {}", przewoz.clone().getAdresDocelowy());
    println!("Przewozony surowiec: {:?}", przewoz.clone().getPrzewozonySurowiec());
    println!("Srodek transportu: {:?}", przewoz.clone().getSrodekTransportu());

    przewoz.zaladuj(Surowiec::Glina(456.0), Transport::Pociag);
    println!("\nDrugi przewoz:\nAdres poczatkowy: {}", przewoz.clone().getAdresPoczatkowy());
    println!("Adres docelowy: {}", przewoz.clone().getAdresDocelowy());
    println!("Przewozony surowiec: {:?}", przewoz.clone().getPrzewozonySurowiec());
    println!("Srodek transportu: {:?}", przewoz.clone().getSrodekTransportu());

    println!("\nCzy moża przewieźć do magazynu: {}", przewoz.dostawa_do(magazyn.clone()));
    magazyn.setPosiadaTerminalKolejowy(true);
    println!("\nCzy moża przewieźć do magazynu: {}", przewoz.dostawa_do(magazyn.clone()));

    println!("\nTrzeci przewoz:\nAdres poczatkowy: {}", przewoz.clone().getAdresPoczatkowy());
    println!("Adres docelowy: {}", przewoz.clone().getAdresDocelowy());
    println!("Przewozony surowiec: {:?}", przewoz.clone().getPrzewozonySurowiec());
    println!("Srodek transportu: {:?}", przewoz.clone().getSrodekTransportu());
}
