extern crate bio;

use bio::alignment::pairwise::Aligner as PwAligner;
use bio::alignment::pairwise::Scoring;
use bio::alignment::poa::Aligner as PoaAligner;
use bio::scores::blosum62::blosum62;

fn main() {
    println!("Pairwise alignment example");
    alignment_example();
    println!("--------------------------------------------------");

    println!("Multiple alignment example");
    multiple_alignment_example();
    println!("--------------------------------------------------");
}

fn alignment_example() {
    let x = b"CCGTCCGGCAAGGG";
    let y = b"AAAAACCGTTGACGGCCAA";

    // gap open score: -5, gap extension score: -1
    let mut aligner = PwAligner::with_capacity(x.len(), y.len(), -1, -1, &blosum62);
    let alignment = aligner.semiglobal(x, y);

    // x is global (target sequence) and y is local (reference sequence)
    println!("y start: {}", alignment.ystart);
    println!("x start: {}", alignment.xstart);
    println!("alignment score: {}\n", alignment.score);
    println!("alignment visualization:\n\n{}", alignment.pretty(x, y, 25));
}

fn multiple_alignment_example() {
    let x = b"AAAAAAA";
    let y = b"AABBBAA";
    let z = b"AABCBAA";

    let scoring = Scoring::new(-1, -1, &blosum62);
    let mut aligner = PoaAligner::new(scoring, x);

    aligner.global(z).add_to_graph();
    aligner.global(y).add_to_graph();

    let consensus = aligner.consensus();
    let consensus_str = String::from_utf8_lossy(&consensus);
    println!("consensus sequence: {}\n", consensus_str);

    for seq in [x, y, z] {
        let mut pw_aligner =
            PwAligner::with_capacity(consensus.len(), seq.len(), -1, -1, &blosum62);
        let x_align = pw_aligner.global(&consensus, seq);
        println!("{}", x_align.pretty(&consensus, seq, 50));
    }
}
