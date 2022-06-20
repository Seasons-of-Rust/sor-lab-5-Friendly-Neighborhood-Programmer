use personnel::{AstronautJob, Candidate};

fn main() {
    //parse input file into a Vec
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();

    //sort the candidates in ascending order
    candidates.sort_by_key(get_candidate_score);

    //print the best candidate with their score
    let best = candidates.last().unwrap();
    print!("{:?}, score: {}", best, get_candidate_score(best));
}

fn get_job_code(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn get_job_score(candidate: &Candidate) -> u32 {
    let score = get_job_code(&candidate.primary_job);

    match &candidate.secondary_job {
        Some(job) => (score * get_job_code(job)) % 577,
        None => (score * score) % 577,
    }
}

fn get_candidate_score(cand: &Candidate) -> u32 {
    ((get_job_score(cand) + cand.health as u32) * cand.age as u32) % 3929
}

// Testing begins
#[test]
fn test_get_job_code() {
    assert_eq!(251, get_job_code(&AstronautJob::Biogeochemist));
    assert_eq!(283, get_job_code(&AstronautJob::Scientist));
    assert_eq!(281, get_job_code(&AstronautJob::RoverOp));
}

#[test]
fn test_get_candidate_score() {
    let harry = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::RoverOp),
        age: 44,
        health: 89,
    };
    assert_eq!(2086, get_candidate_score(&harry));

    let marge = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: Some(AstronautJob::Medic),
        age: 53,
        health: 83,
    };
    assert_eq!(3491, get_candidate_score(&marge));
}

#[test]
fn test_candidate_sorting() {
    let mut group = vec![
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: Some(AstronautJob::RoverOp),
            age: 44,
            health: 89,
        }, // score: 2086
        Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: Some(AstronautJob::Geologist),
            age: 74,
            health: 32,
        }, // score 3038
        Candidate {
            primary_job: AstronautJob::Mechanic,
            secondary_job: Some(AstronautJob::Medic),
            age: 53,
            health: 83,
        }, // score 3491
        Candidate {
            primary_job: AstronautJob::Engineer,
            secondary_job: None,
            age: 69,
            health: 24,
        },
    ]; // score 1209

    group.sort_by_key(get_candidate_score);
    assert_eq!(3491, get_candidate_score(group.last().unwrap()));
}
