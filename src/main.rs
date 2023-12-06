use rand::Rng;
use rand::prelude::ThreadRng;

//TODO struct Individu
//TODO struct Population
//TODO crossover, mutation, solve problem

#[derive(Debug)]
struct Individual {
    genes: Vec<u8>,
    gene_length: u32,
    fitness: i32,
}

impl Individual {
    fn fitness(&mut self) {
        for gene in self.genes.iter() {
            if *gene == 1 {
                self.fitness = self.fitness + 1;
            }
        }
    }

    fn generate(&mut self, rng: &mut ThreadRng) {
        for _ in 0..self.gene_length {
            self.genes.push(rng.gen_range(0, 2))
        }
    }

    fn new(gene_length: u32) -> Individual {
        Individual {
            genes: Vec::new(),
            gene_length,
            fitness: 0
        }
    }
}

#[derive(Debug)]
struct Population {
    individuals: Vec<Individual>,
    pop_size: u32,
    gene_length: u32,
    fittest_score: i32,
}

impl Population {
    fn generate(&mut self, rng: &mut ThreadRng) {
        for _ in 0..self.pop_size {
            let mut ind = Individual::new(self.gene_length);
            ind.generate(rng);

            self.individuals.push(ind);
        }
    }

    fn select_fittest(&mut self) -> Option<&Individual> {
        let mut max_fit: i32 = -1;
        let mut index: usize = 0;

        for (i, individual) in self.individuals.iter().enumerate() {
            if max_fit <= individual.fitness {
                max_fit = individual.fitness;
                index = i;
            }
        }

        self.fittest_score = match self.individuals.get(index) {
            Some(individual) => individual.fitness,
            None => self.fittest_score
        };

        self.individuals.get(index)
    }

    fn calculate_fitness(&mut self) {
        for individual in &mut self.individuals {
            individual.fitness();
        }
    }

    fn new(pop_size: u32, gene_length: u32) -> Population {
        Population {
            individuals: Vec::new(),
            pop_size,
            gene_length,
            fittest_score: 0,
        }
    }
}

fn main() {
    let number_of_genes: u32 = 20;
    let number_of_individuals: u32 = 5;

    // random generator
    let mut rng = rand::thread_rng();

    let mut p1 = Population::new(number_of_genes, number_of_individuals);
    p1.generate(&mut rng);

    println!("{:?}", p1);
}
