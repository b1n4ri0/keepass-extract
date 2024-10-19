use keepass_ng::{
    db::{with_node, Database, Entry, Group, Node, NodeIterator},
    error::DatabaseOpenError,
    DatabaseKey,
};
use std::error::Error;
use std::fs::File;
use std::ffi::OsString;
use colored::*;
use clap::Parser;
use csv::Writer;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    database: OsString,

    #[arg(short, long)]
    password: String,

    #[arg(short, long, default_value = "Database.csv")]
    output: OsString,
}

fn open_database( database_path: OsString, master_pass: String)-> Result<Database, DatabaseOpenError> {
    let mut file = File::open(database_path)?;
    let key = DatabaseKey::new().with_password(&master_pass);
    let db = Database::open(&mut file, key)?;
    Ok(db)
}

fn read_database(db: &Database) {
    let mut contador = 1;
    for node in NodeIterator::new(&db.root).into_iter() {
        with_node::<Group, _, _>(&node, |group| {
            println!("[*] Grupo {0}:\n\t[-] Notas de grupo: {1:?}\n",group.get_title().unwrap_or("").blue(), group.get_notes().unwrap_or(""));
        });
        with_node::<Entry, _, _>(&node, |e| {
            let title = e.get_title().unwrap_or("").cyan();
            let user = e.get_username().unwrap_or("").green();
            let pass = e.get_password().unwrap_or("").bright_green().bold();
            let url = e.get_url().unwrap_or("").yellow();
            let notes = e.get_notes().unwrap_or("").bright_red();
            println!("\t[+] Entrada {0} |\tTitulo: '{1}';\tUsuario: '{2}';\tContraseña: '{3}';\tURL: '{4}'\n\n\t[+] Nota {0}: '{5}'\n", contador, title, user, pass, url, notes);
            contador += 1;
        });
}
}


fn db_to_csv(db: &Database, output_file: OsString) -> Result<(), Box<dyn Error>> {
    let file = File::create(&output_file)?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&["Titulo", "Usuario", "Contraseña", "URL", "Notas"])?;

    for node in NodeIterator::new(&db.root).into_iter() {
        with_node::<Entry, _, _>(&node, |e| {
            let title = e.get_title().unwrap_or("");
            let user = e.get_username().unwrap_or("");
            let pass = e.get_password().unwrap_or("");
            let url = e.get_url().unwrap_or("");
            let notes = e.get_notes().unwrap_or("");
            let record = [title, user, pass, url, notes];
            wtr.write_record(&record).expect("Error al escribir el registro CSV");
        });
    }
    wtr.flush()?;

    println!("[!] Exportado a CSV correctamente: {}", output_file.to_string_lossy().bright_magenta().bold());
    Ok(())
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let db = open_database(args.database, args.password)?;
    read_database(&db);
    db_to_csv(&db, args.output.clone())?;
    Ok(())
}