use crate::file_io::fasta::Fragment;

pub fn naive(genome: String, fragments: Vec<Fragment>) -> Vec<usize> {

    let mut indicies: Vec<usize> = Vec::new();

    for frag in fragments {
        println!("#");
        if let Some(index) = genome.find(&frag.bases().replace("U", "T")) {
            indicies.push(index);
            println!("\n---Found a match!---");
        }
    }
    indicies
}